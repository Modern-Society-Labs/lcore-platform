// SPDX-License-Identifier: UNLICENSED
#![cfg_attr(not(feature = "export-abi"), no_main)]
// no_std removed for Stylus SDK 0.9

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, B256, U256, U64, U8},
    crypto, prelude::*,
};

// All structs are defined within the sol_storage! macro to handle storage layout automatically.
sol_storage! {
    #[entrypoint]
    pub struct IoTexDeviceRegistry {
        /// We flatten the DeviceInfo struct into individual maps,
        /// mapping from a device ID hash to the specific field.
        /// The key is `keccak256(device_id)`.
        mapping(bytes32 => string) device_dids;
        mapping(bytes32 => string) device_pub_keys;
        mapping(bytes32 => string) device_types;
        mapping(bytes32 => string) device_manufacturers;
        mapping(bytes32 => string) device_models;
        mapping(bytes32 => string) device_firmware_versions;
        mapping(bytes32 => string) device_deployment_zones;
        mapping(bytes32 => address) device_city_contracts;
        mapping(bytes32 => uint64) device_expected_data_rates;
        mapping(bytes32 => uint256) device_registered_at;
        mapping(bytes32 => uint256) device_last_heartbeats;
        mapping(bytes32 => uint8) device_statuses;

        /// Mapping from device ID hash to the owner's address.
        mapping(bytes32 => address) device_owners;
        /// Mapping from an owner's address to a list of their device IDs (the original strings).
        mapping(address => string[]) owner_devices;
        /// Mapping from a city zone ID to its governing contract address.
        mapping(string => address) city_zones;
        /// Mapping from device ID hash to its data processing requirements.
        mapping(bytes32 => ProcessingRequirements) processing_requirements;

        /// General contract state
        string cartesi_vm_hash;
        string rollups_endpoint;
        address admin;
        uint256 registry_fee;
        uint256 total_devices;
        bool is_paused;
    }

    // Smart Cities processing requirements - this can remain a struct
    // as it's the value in a map, which is supported.
    pub struct ProcessingRequirements {
        bool requires_cartesi;
        uint8 encryption_level;    // 0: Standard, 1: Dual, 2: Quantum
        uint8 analytics_tier;      // 0: Basic, 1: Advanced, 2: ML
        uint8 settlement_priority; // 0: Low, 1: Normal, 2: High, 3: Critical
    }
}

// Events
sol! {
    event DeviceRegistered(
        bytes32 indexed device_id,
        address indexed owner,
        string device_type,
        string deployment_zone,
        uint256 timestamp
    );

    event SmartCityZoneUpdated(
        string indexed zone_id,
        address city_contract,
        uint256 timestamp
    );

    event ProcessingRequirementsUpdated(
        bytes32 indexed device_id,
        bool requires_cartesi,
        uint8 encryption_level,
        uint256 timestamp
    );

    event OwnershipTransferred(address indexed previous_owner, address indexed new_owner);
}

#[public]
impl IoTexDeviceRegistry {
    pub fn initialize(
        &mut self,
        cartesi_vm_hash: String,
        rollups_endpoint: String,
        registry_fee: U256,
    ) -> Result<(), Vec<u8>> {
        if self.admin.get() != Address::ZERO {
            return Err(b"Already initialized".to_vec());
        }
        self.admin.set(self.vm().msg_sender());
        self.cartesi_vm_hash.set_str(cartesi_vm_hash);
        self.rollups_endpoint.set_str(rollups_endpoint);
        self.registry_fee.set(registry_fee);
        self.is_paused.set(false);
        self.total_devices.set(U256::ZERO);
        Ok(())
    }

    // ========== Ownership and Admin Functions ==========

    /// @notice Returns the address of the current owner.
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.admin.get())
    }

    /// @notice Transfers ownership of the contract to a new account (`new_owner`).
    /// Can only be called by the current owner.
    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        if new_owner == Address::ZERO {
            return Err(b"New owner cannot be the zero address".to_vec());
        }
        let previous_owner = self.admin.get();
        self.admin.set(new_owner);
        log(self.vm(), OwnershipTransferred { previous_owner, new_owner });
        Ok(())
    }

    /// @notice Leaves the contract without an owner. It will not be possible to call
    /// `only_owner` functions anymore. Can only be called by the current owner.
    /// @dev This is an IRREVERSIBLE action.
    pub fn renounce_ownership(&mut self) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        let previous_owner = self.admin.get();
        self.admin.set(Address::ZERO);
        log(self.vm(), OwnershipTransferred { previous_owner, new_owner: Address::ZERO });
        Ok(())
    }

    // ========== Configuration Functions ==========

    /// @notice Returns the current registration fee.
    pub fn registry_fee(&self) -> Result<U256, Vec<u8>> {
        Ok(self.registry_fee.get())
    }

    pub fn set_registry_fee(&mut self, new_fee: U256) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        self.registry_fee.set(new_fee);
        Ok(())
    }

    #[payable]
    pub fn register_device(
        &mut self,
        device_id: String,
        did_document: String,
        public_key_hex: String,
        device_type: String,
        manufacturer: String,
        model: String,
        firmware_version: String,
        deployment_zone: String,
        city_contract_address: Address,
        expected_data_rate: u64,
    ) -> Result<(), Vec<u8>> {
        if self.is_paused.get() {
            return Err(b"Registry is paused".to_vec());
        }
        if device_id.is_empty() {
            return Err(b"Device ID cannot be empty".to_vec());
        }
        
        let device_id_hash: B256 = crypto::keccak(device_id.as_bytes()).into();

        if self.device_owners.getter(device_id_hash).get() != Address::ZERO {
            return Err(b"Device already registered".to_vec());
        }
        if self.vm().msg_value() < self.registry_fee.get() {
            return Err(b"Insufficient registration fee".to_vec());
        }

        self.device_dids.setter(device_id_hash).set_str(did_document);
        self.device_pub_keys.setter(device_id_hash).set_str(public_key_hex);
        self.device_types.setter(device_id_hash).set_str(device_type.clone());
        self.device_manufacturers.setter(device_id_hash).set_str(manufacturer);
        self.device_models.setter(device_id_hash).set_str(model);
        self.device_firmware_versions.setter(device_id_hash).set_str(firmware_version);
        self.device_deployment_zones.setter(device_id_hash).set_str(deployment_zone.clone());
        self.device_city_contracts.setter(device_id_hash).set(city_contract_address);
        self.device_expected_data_rates.setter(device_id_hash).set(U64::from(expected_data_rate));
        
        let timestamp = self.vm().block_timestamp();
        self.device_registered_at.setter(device_id_hash).set(U256::from(timestamp));
        self.device_last_heartbeats.setter(device_id_hash).set(U256::from(timestamp));
        self.device_statuses.setter(device_id_hash).set(U8::from(0)); // 0 = Pending

        let sender = self.vm().msg_sender();
        self.device_owners.setter(device_id_hash).set(sender);
        self.owner_devices.setter(sender).grow().set_str(device_id);

        let mut default_processing = self.processing_requirements.setter(device_id_hash);
        default_processing.requires_cartesi.set(device_type == "air_quality_sensor" || device_type == "traffic_monitor");
        default_processing.encryption_level.set(U8::from(0)); // Standard
        default_processing.analytics_tier.set(U8::from(0));   // Basic
        default_processing.settlement_priority.set(U8::from(1)); // Normal

        let new_total = self.total_devices.get() + U256::from(1);
        self.total_devices.set(new_total);

        log(self.vm(), DeviceRegistered {
            device_id: device_id_hash,
            owner: sender,
            device_type,
            deployment_zone,
            timestamp: U256::from(timestamp),
        });

        Ok(())
    }

    /// Read-only function to check if a device is registered.
    pub fn is_device_registered(&self, device_id_hash: B256) -> Result<bool, Vec<u8>> {
        let owner = self.device_owners.getter(device_id_hash).get();
        Ok(owner != Address::ZERO)
    }
}

// Private helper functions
impl IoTexDeviceRegistry {
    /// @dev Throws if called by any account other than the owner.
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(b"Only owner can call this function".to_vec());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::{test_helpers, alloy_primitives::{address, U256}};

    /// Tests that the initialize function correctly sets the owner
    /// and that the only_owner modifier works as expected.
    #[test]
    fn test_owner_access_control() {
        // Deploy the contract in a mock environment
        let mut contract = IoTexDeviceRegistry::new();

        // Define two mock addresses: one for the owner, one for a random user
        let owner_address = address!("0000000000000000000000000000000000000001");
        let random_user = address!("0000000000000000000000000000000000000002");

        // Set the sender for the initialization call
        test_helpers::with_sender(owner_address, || {
            // Call the initialize function
            let _ = contract.initialize(
                "vm_hash".into(), 
                "endpoint".into(), 
                U256::from(100)
            ).unwrap();
        });

        // 1. Check that the owner is set correctly
        assert_eq!(contract.owner().unwrap(), owner_address);

        // 2. Check that the owner CAN call an admin function
        test_helpers::with_sender(owner_address, || {
            let result = contract.set_registry_fee(U256::from(200));
            assert!(result.is_ok(), "Owner should be able to set the fee");
        });
        // Verify the fee was actually changed
        assert_eq!(contract.registry_fee.get(), U256::from(200));

        // 3. Check that a random user CANNOT call an admin function
        test_helpers::with_sender(random_user, || {
            let result = contract.set_registry_fee(U256::from(300));
            // Assert that the call failed as expected
            assert!(result.is_err(), "Non-owner should not be able to set the fee");
            let err_msg = result.unwrap_err();
            assert_eq!(err_msg, b"Only owner can call this function".to_vec());
        });
        // Verify the fee was NOT changed by the random user
        assert_eq!(contract.registry_fee.get(), U256::from(200));
    }
}