#![cfg_attr(not(feature = "export-abi"), no_main)]
// no_std removed for Stylus SDK 0.9 compatibility

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use alloy_sol_types::{sol, SolCall, SolValue};
use stylus_sdk::{
    alloy_primitives::{Address, B256, U256},
    crypto::keccak,
    prelude::*,
};

// Simplified events for basic data marketplace
sol! {
    event DataSubmitted(
        bytes32 indexed data_hash, 
        bytes32 indexed device_id_hash, 
        address indexed device_owner, 
        uint256 timestamp
    );
    
    event MarketplaceConfigUpdated(
        uint256 base_fee
        // bool is_paused  // REMOVED: Anti-decentralization pattern
    );
}

sol_storage! {
    #[entrypoint]
    pub struct IoTDataPipeline {
        /// Admin controls
        address admin;
        address rollup_contract_address;
        address device_registry_address;
        // bool is_paused;  // REMOVED: Anti-decentralization pattern
        
        /// Basic marketplace configuration
        uint256 base_submission_fee;
        
        /// Data submission tracking
        mapping(bytes32 => DataSubmission) data_submissions;      // data_hash -> submission info
        mapping(bytes32 => uint256) device_submission_counts;     // device_id_hash -> count
        mapping(address => bytes32[]) owner_data_hashes;          // owner -> list of data hashes
        
        /// Access control for data marketplace
        mapping(address => mapping(address => bool)) marketplace_access; // owner -> consumer -> allowed
        
        /// Counters
        uint256 total_submissions;
    }

    /// Simplified data submission record
    pub struct DataSubmission {
        bytes32 device_id_hash;
        address device_owner;
        uint256 timestamp;
        uint256 submission_fee_paid;
        bool is_processed;
    }
}

/// Interface for DeviceRegistry contract calls
sol! {
    interface IDeviceRegistry {
        function is_device_registered(bytes32 device_id_hash) external view returns (bool);
        function get_device_owner(bytes32 device_id_hash) external view returns (address);
        function has_access(address owner, address consumer) external view returns (bool);
    }
}

#[public]
impl IoTDataPipeline {
    /// Initialize the contract with basic marketplace configuration
    pub fn initialize(
        &mut self, 
        rollup_address: Address, 
        registry_address: Address,
        base_fee: U256
    ) -> Result<(), Vec<u8>> {
        if !self.admin.get().is_zero() {
            return Err(b"Contract already initialized".to_vec());
        }
        
        self.admin.set(self.vm().msg_sender());
        self.rollup_contract_address.set(rollup_address);
        self.device_registry_address.set(registry_address);
        self.base_submission_fee.set(base_fee);
        // self.is_paused.set(false); // REMOVED: Anti-decentralization pattern
        self.total_submissions.set(U256::ZERO);
        
        Ok(())
    }

    /// Main entrypoint called by Cartesi rollup
    /// Expects payload format: device_id (raw bytes for verification)
    pub fn submit_cartesi_result(&mut self, payload: Vec<u8>) -> Result<(), Vec<u8>> {
        // if self.is_paused.get() { // REMOVED: Anti-decentralization pattern
        //     return Err(b"Pipeline is paused".to_vec());
        // }
        
        // Only Cartesi rollup can call this
        if self.vm().msg_sender() != self.rollup_contract_address.get() {
            return Err(b"Unauthorized: only rollup can submit".to_vec());
        }

        // Generate device_id_hash from payload 
        let device_id_hash: B256 = keccak(&payload).into();
        
        // Verify device is registered
        let registry_addr = self.device_registry_address.get();
        let is_registered = self._verify_device_registration(registry_addr, device_id_hash)?;
        if !is_registered {
            return Err(b"Device not registered".to_vec());
        }

        // Get device owner
        let device_owner = self._get_device_owner(registry_addr, device_id_hash)?;
        if device_owner == Address::ZERO {
            return Err(b"Invalid device owner".to_vec());
        }

        // Create data submission record
        let data_hash: B256 = keccak(&[&device_id_hash.0[..], &payload].concat()).into();
        let timestamp = self.vm().block_timestamp();
        
        let mut submission = self.data_submissions.setter(data_hash);
        submission.device_id_hash.set(device_id_hash);
        submission.device_owner.set(device_owner);
        submission.timestamp.set(U256::from(timestamp));
        submission.submission_fee_paid.set(U256::ZERO); // No fee for Cartesi submissions
        submission.is_processed.set(true);

        // Update tracking counters
        let current_count = self.device_submission_counts.getter(device_id_hash).get();
        self.device_submission_counts.setter(device_id_hash).set(current_count + U256::from(1));
        
        self.owner_data_hashes.setter(device_owner).grow().set(data_hash);
        
        let new_total = self.total_submissions.get() + U256::from(1);
        self.total_submissions.set(new_total);

        log(self.vm(), DataSubmitted {
            data_hash,
            device_id_hash,
            device_owner,
            timestamp: U256::from(timestamp),
        });

        Ok(())
    }

    // ========== Access Control Functions ==========

    /// Grant marketplace access to a consumer (called by data owner)
    pub fn grant_marketplace_access(&mut self, consumer: Address) -> Result<(), Vec<u8>> {
        if consumer == Address::ZERO {
            return Err(b"Invalid consumer address".to_vec());
        }
        
        let owner = self.vm().msg_sender();
        self.marketplace_access.setter(owner).setter(consumer).set(true);
        
        Ok(())
    }

    /// Revoke marketplace access from a consumer (called by data owner)
    pub fn revoke_marketplace_access(&mut self, consumer: Address) -> Result<(), Vec<u8>> {
        let owner = self.vm().msg_sender();
        self.marketplace_access.setter(owner).setter(consumer).set(false);
        
        Ok(())
    }

    /// Check if consumer has marketplace access to owner's data
    pub fn has_marketplace_access(&mut self, owner: Address, consumer: Address) -> Result<bool, Vec<u8>> {
        // Check both contract-level and registry-level permissions
        let marketplace_permission = self.marketplace_access.getter(owner).getter(consumer).get();
        
        if !marketplace_permission {
            // Fall back to DeviceRegistry permissions
            let registry_addr = self.device_registry_address.get();
            return self._check_registry_access(registry_addr, owner, consumer);
        }
        
        Ok(true)
    }

    // ========== Query Functions ==========

    /// Get total number of data submissions
    pub fn total_submissions(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_submissions.get())
    }

    /// Get submission count for a specific device
    pub fn get_device_submission_count(&self, device_id_hash: B256) -> Result<U256, Vec<u8>> {
        Ok(self.device_submission_counts.getter(device_id_hash).get())
    }

    /// Get data hashes submitted by an owner
    pub fn get_owner_data_hashes(&self, owner: Address) -> Result<Vec<B256>, Vec<u8>> {
        let hashes = self.owner_data_hashes.getter(owner);
        let mut result = Vec::with_capacity(hashes.len() as usize);
        for i in 0..hashes.len() {
            if let Some(hash) = hashes.get(i) {
                result.push(hash);
            }
        }
        Ok(result)
    }

    /// Get submission information by data hash
    pub fn get_submission_info(&self, data_hash: B256) -> Result<(B256, Address, U256, bool), Vec<u8>> {
        let submission = self.data_submissions.getter(data_hash);
        Ok((
            submission.device_id_hash.get(),
            submission.device_owner.get(),
            submission.timestamp.get(),
            submission.is_processed.get(),
        ))
    }

    // ========== Admin Functions ==========

    /// Get contract owner
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.admin.get())
    }

    /// Update rollup contract address
    pub fn set_rollup_contract(&mut self, new_address: Address) -> Result<(), Vec<u8>> {
        self.only_admin()?;
        self.rollup_contract_address.set(new_address);
        Ok(())
    }

    /// Update device registry address
    pub fn set_device_registry(&mut self, new_address: Address) -> Result<(), Vec<u8>> {
        self.only_admin()?;
        self.device_registry_address.set(new_address);
        Ok(())
    }

    /// Update base submission fee
    pub fn set_base_fee(&mut self, new_fee: U256) -> Result<(), Vec<u8>> {
        self.only_admin()?;
        self.base_submission_fee.set(new_fee);
        
        log(self.vm(), MarketplaceConfigUpdated {
            base_fee: new_fee,
        });
        
        Ok(())
    }

    /// Set paused state
    // pub fn set_paused(&mut self, paused: bool) -> Result<(), Vec<u8>> {  // REMOVED: Anti-decentralization
    //     self.only_admin()?;
    //     // self.is_paused.set(paused); // REMOVED: Anti-decentralization pattern
    //     
    //     log(self.vm(), MarketplaceConfigUpdated {
    //         base_fee: self.base_submission_fee.get(),
    //         // is_paused: paused, // REMOVED: Anti-decentralization pattern
    //     });
    //     
    //     Ok(())
    // }

    /// Simple liveness check
    pub fn ping(&self) -> Result<U256, Vec<u8>> {
        Ok(U256::from(1))
    }
}

// Private helper functions
impl IoTDataPipeline {
    /// Ensure only admin can call
    fn only_admin(&self) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(b"Only admin can call this function".to_vec());
        }
        Ok(())
    }

    /// Verify device registration via static call to DeviceRegistry
    fn _verify_device_registration(&mut self, registry_addr: Address, device_id_hash: B256) -> Result<bool, Vec<u8>> {
        let calldata = IDeviceRegistry::is_device_registeredCall { device_id_hash }.abi_encode();
        let response = self.vm().static_call(&self, registry_addr, &calldata)?;
        
        let (is_registered,) = <(bool,)>::abi_decode(&response, true)
            .map_err(|_| b"Failed to decode registry response".to_vec())?;
        
        Ok(is_registered)
    }

    /// Get device owner via static call to DeviceRegistry
    fn _get_device_owner(&mut self, registry_addr: Address, device_id_hash: B256) -> Result<Address, Vec<u8>> {
        let calldata = IDeviceRegistry::get_device_ownerCall { device_id_hash }.abi_encode();
        let response = self.vm().static_call(&self, registry_addr, &calldata)?;
        
        let (owner,) = <(Address,)>::abi_decode(&response, true)
            .map_err(|_| b"Failed to decode owner response".to_vec())?;
        
        Ok(owner)
    }

    /// Check registry-level access permissions
    fn _check_registry_access(&mut self, registry_addr: Address, owner: Address, consumer: Address) -> Result<bool, Vec<u8>> {
        let calldata = IDeviceRegistry::has_accessCall { owner, consumer }.abi_encode();
        let response = self.vm().static_call(&self, registry_addr, &calldata)?;
        
        let (has_access,) = <(bool,)>::abi_decode(&response, true)
            .map_err(|_| b"Failed to decode access response".to_vec())?;
        
        Ok(has_access)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::{test_helpers, alloy_primitives::address};

    #[test]
    fn test_basic_initialization() {
        let mut contract = IoTDataPipeline::new();
        let admin_address = address!("0000000000000000000000000000000000000001");
        let rollup_address = address!("0000000000000000000000000000000000000002");
        let registry_address = address!("0000000000000000000000000000000000000003");

        test_helpers::with_sender(admin_address, || {
            let result = contract.initialize(rollup_address, registry_address, U256::from(100));
            assert!(result.is_ok(), "Initialization should succeed");
            assert_eq!(contract.owner().unwrap(), admin_address);
            assert_eq!(contract.total_submissions().unwrap(), U256::ZERO);
        });
    }

    #[test]
    fn test_marketplace_access_control() {
        let mut contract = IoTDataPipeline::new();
        let owner_address = address!("0000000000000000000000000000000000000001");
        let consumer_address = address!("0000000000000000000000000000000000000002");

        test_helpers::with_sender(owner_address, || {
            // Grant access
            let result = contract.grant_marketplace_access(consumer_address);
            assert!(result.is_ok(), "Access grant should succeed");

            // Check access
            let has_access = contract.marketplace_access.getter(owner_address).getter(consumer_address).get();
            assert!(has_access, "Consumer should have access");

            // Revoke access
            let result = contract.revoke_marketplace_access(consumer_address);
            assert!(result.is_ok(), "Access revoke should succeed");

            // Check access revoked
            let has_access = contract.marketplace_access.getter(owner_address).getter(consumer_address).get();
            assert!(!has_access, "Consumer should not have access");
        });
    }
} 