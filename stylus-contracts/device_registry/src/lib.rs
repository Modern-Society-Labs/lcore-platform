// SPDX-License-Identifier: UNLICENSED
#![cfg_attr(not(feature = "export-abi"), no_main)]
// no_std removed for Stylus SDK 0.9

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, B256, U256},
    crypto, prelude::*,
};

// Simplified storage structure - focus on core device registry functionality
sol_storage! {
    #[entrypoint]
    pub struct DeviceRegistry {
        /// Core device mappings (device_id_hash -> data)
        mapping(bytes32 => string) device_dids;          // W3C DID documents
        mapping(bytes32 => string) device_public_keys;   // Public keys for verification
        mapping(bytes32 => string) device_types;         // Generic device type classification
        mapping(bytes32 => string) device_metadata;      // Generic metadata (JSON string)
        mapping(bytes32 => uint256) device_registered_at; // Registration timestamp
        
        /// Owner mappings (always wallet addresses)
        mapping(bytes32 => address) device_owners;       // device_id_hash -> owner address
        mapping(address => string[]) owner_devices;      // owner -> list of device_ids
        
        /// Access control mappings for data marketplace
        mapping(address => mapping(address => bool)) access_permissions; // owner -> consumer -> allowed
        mapping(address => mapping(address => uint256)) permission_expires; // owner -> consumer -> expiry timestamp
        
        /// Admin and configuration
        address admin;
        uint256 total_devices;
    }
}

// Simplified events
sol! {
    event DeviceRegistered(
        bytes32 indexed device_id_hash,
        address indexed owner,
        string device_type,
        uint256 timestamp
    );

    event AccessGranted(
        address indexed owner,
        address indexed consumer,
        uint256 expires_at
    );

    event AccessRevoked(
        address indexed owner,
        address indexed consumer
    );

    event OwnershipTransferred(
        address indexed previous_owner, 
        address indexed new_owner
    );
}

#[public]
impl DeviceRegistry {
    /// Initialize the contract with basic configuration
    pub fn initialize(&mut self) -> Result<(), Vec<u8>> {
        if self.admin.get() != Address::ZERO {
            return Err(b"Already initialized".to_vec());
        }
        self.admin.set(self.vm().msg_sender());
        self.total_devices.set(U256::ZERO);
        Ok(())
    }

    // ========== Device Registration ==========

    /// Register a new device (simplified parameters)
    #[payable]
    pub fn register_device(
        &mut self,
        device_id: String,
        did_document: String,
        public_key_hex: String,
        device_type: String,
        metadata: String, // Generic metadata as JSON string
    ) -> Result<(), Vec<u8>> {
        // if self.is_paused.get() {               // REMOVED: Anti-decentralization
        //     return Err(b"Registry is paused".to_vec());
        // }
        if device_id.is_empty() {
            return Err(b"Device ID cannot be empty".to_vec());
        }

        let device_id_hash: B256 = crypto::keccak(device_id.as_bytes()).into();

        if self.device_owners.getter(device_id_hash).get() != Address::ZERO {
            return Err(b"Device already registered".to_vec());
        }

        // Check registration fee
        // if self.vm().msg_value() < self.registry_fee.get() {  // REMOVED: Free Cartesi model
        //     return Err(b"Insufficient registration fee".to_vec());
        // }

        // Store device information
        self.device_dids.setter(device_id_hash).set_str(did_document);
        self.device_public_keys.setter(device_id_hash).set_str(public_key_hex);
        self.device_types.setter(device_id_hash).set_str(device_type.clone());
        self.device_metadata.setter(device_id_hash).set_str(metadata);
        
        let timestamp = self.vm().block_timestamp();
        self.device_registered_at.setter(device_id_hash).set(U256::from(timestamp));

        // Set ownership (ALWAYS wallet address)
        let owner = self.vm().msg_sender();
        self.device_owners.setter(device_id_hash).set(owner);
        self.owner_devices.setter(owner).grow().set_str(device_id);

        // Update counters
        let new_total = self.total_devices.get() + U256::from(1);
        self.total_devices.set(new_total);

        log(self.vm(), DeviceRegistered {
            device_id_hash,
            owner,
            device_type,
            timestamp: U256::from(timestamp),
        });

        Ok(())
    }

    /// Register device from Cartesi rollup (called by rollup contract via voucher)
    pub fn register_device_from_cartesi(&mut self, payload: Vec<u8>) -> Result<(), Vec<u8>> {
        // if self.is_paused.get() {               // REMOVED: Anti-decentralization
        //     return Err(b"Paused".to_vec());
        // }

        let s = String::from_utf8(payload).map_err(|_| b"Bad UTF-8".to_vec())?;
        
        let device_id = Self::get_val(&s, "device_id").ok_or(b"No device_id".to_vec())?;
        let did_document = Self::get_val(&s, "did_document").ok_or(b"No did_document".to_vec())?;
        let public_key = Self::get_val(&s, "public_key").unwrap_or("".to_string());
        let device_type = Self::get_val(&s, "device_type").unwrap_or("iot".to_string());
        
        // Extract owner address from payload (NEW: Fix ownership tracking)
        let owner_address_str = Self::get_val(&s, "owner_address")
            .ok_or(b"No owner_address".to_vec())?;
        
        let owner = owner_address_str.parse::<Address>()
            .map_err(|_| b"Invalid owner address".to_vec())?;

        if device_id.is_empty() {
            return Err(b"Empty ID".to_vec());
        }

        let hash: B256 = crypto::keccak(device_id.as_bytes()).into();

        if self.device_owners.getter(hash).get() != Address::ZERO {
            return Err(b"Exists".to_vec());
        }

        self.device_dids.setter(hash).set_str(did_document);
        self.device_public_keys.setter(hash).set_str(public_key);
        self.device_types.setter(hash).set_str(device_type.clone());
        self.device_metadata.setter(hash).set_str("{}".to_string());
        
        let ts = self.vm().block_timestamp();
        self.device_registered_at.setter(hash).set(U256::from(ts));

        // Use extracted owner address instead of msg_sender
        self.device_owners.setter(hash).set(owner);
        self.owner_devices.setter(owner).grow().set_str(device_id);

        self.total_devices.set(self.total_devices.get() + U256::from(1));

        log(self.vm(), DeviceRegistered {
            device_id_hash: hash,
            owner,
            device_type,
            timestamp: U256::from(ts),
        });

        Ok(())
    }

    // ========== Access Control Functions ==========

    /// Grant data access to a consumer (called by device owner)
    pub fn grant_access(&mut self, consumer: Address, expires_at: U256) -> Result<(), Vec<u8>> {
        if consumer == Address::ZERO {
            return Err(b"Invalid consumer address".to_vec());
        }

        let owner = self.vm().msg_sender();
        
        // Set permission
        self.access_permissions.setter(owner).setter(consumer).set(true);
        
        // Set expiration (0 = never expires)
        if expires_at > U256::ZERO {
            self.permission_expires.setter(owner).setter(consumer).set(expires_at);
        }

        log(self.vm(), AccessGranted {
            owner,
            consumer,
            expires_at,
        });

        Ok(())
    }

    /// Revoke data access from a consumer (called by device owner)
    pub fn revoke_access(&mut self, consumer: Address) -> Result<(), Vec<u8>> {
        let owner = self.vm().msg_sender();
        
        self.access_permissions.setter(owner).setter(consumer).set(false);
        self.permission_expires.setter(owner).setter(consumer).set(U256::ZERO);

        log(self.vm(), AccessRevoked {
            owner,
            consumer,
        });

        Ok(())
    }

    /// Check if consumer has access to owner's data
    pub fn has_access(&self, owner: Address, consumer: Address) -> Result<bool, Vec<u8>> {
        let has_permission = self.access_permissions.getter(owner).getter(consumer).get();
        
        if !has_permission {
            return Ok(false);
        }

        // Check expiration
        let expires_at = self.permission_expires.getter(owner).getter(consumer).get();
        if expires_at > U256::ZERO && U256::from(self.vm().block_timestamp()) > expires_at {
            return Ok(false); // Permission expired
        }

        Ok(true)
    }

    // ========== Device Query Functions ==========

    /// Check if a device is registered
    pub fn is_device_registered(&self, device_id_hash: B256) -> Result<bool, Vec<u8>> {
        let owner = self.device_owners.getter(device_id_hash).get();
        Ok(owner != Address::ZERO)
    }

    /// Get device owner address
    pub fn get_device_owner(&self, device_id_hash: B256) -> Result<Address, Vec<u8>> {
        Ok(self.device_owners.getter(device_id_hash).get())
    }

    /// Get devices owned by an address
    // ========== Admin Functions ==========

    /// Get contract owner
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.admin.get())
    }

    /// Transfer contract ownership
    pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        if new_owner == Address::ZERO {
            return Err(b"New owner cannot be zero address".to_vec());
        }
        let previous_owner = self.admin.get();
        self.admin.set(new_owner);
        log(self.vm(), OwnershipTransferred { previous_owner, new_owner });
        Ok(())
    }

    /// Set registration fee
    // pub fn set_registry_fee(&mut self, new_fee: U256) -> Result<(), Vec<u8>> {  // REMOVED: Free Cartesi model
    //     self.only_owner()?;
    //     self.registry_fee.set(new_fee);
    //     Ok(())
    // }

    /// Get registration fee
    // pub fn registry_fee(&self) -> Result<U256, Vec<u8>> {  // REMOVED: Free Cartesi model
    //     Ok(self.registry_fee.get())
    // }

    /// Pause/unpause contract
    // pub fn set_paused(&mut self, paused: bool) -> Result<(), Vec<u8>> {  // REMOVED: Anti-decentralization
    //     self.only_owner()?;
    //     self.is_paused.set(paused);
    //     Ok(())
    // }

    /// Check if contract is paused
    // pub fn is_paused(&self) -> Result<bool, Vec<u8>> {  // REMOVED: Anti-decentralization
    //     Ok(self.is_paused.get())
    // }

    /// Get total devices registered
    pub fn total_devices(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_devices.get())
    }
}

// Private helper functions
impl DeviceRegistry {
    /// Ensure only contract owner can call
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(b"Only owner can call this function".to_vec());
        }
        Ok(())
    }

    /// Helper to extract a string from a JSON payload manually (simplified)
    /// Minimal JSON value extractor
    fn get_val(s: &str, k: &str) -> Option<String> {
        let p = format!("\"{}\":\"", k);
        let i = s.find(&p)? + p.len();
        let e = s[i..].find('"')?;
        Some(s[i..i + e].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::{test_helpers, alloy_primitives::{address, U256}};

    #[test]
    fn test_device_registration() {
        let mut contract = DeviceRegistry::new();
        let owner_address = address!("0000000000000000000000000000000000000001");

        test_helpers::with_sender(owner_address, || {
            // Initialize contract
            let _ = contract.initialize();

            // Register device
            test_helpers::with_value(U256::from(100), || {
                let result = contract.register_device(
                    "did:lcore:test-device".into(),
                    "test_did_document".into(),
                    "test_public_key".into(),
                    "environmental_sensor".into(),
                    "{}".into(), // empty metadata
                );
                assert!(result.is_ok(), "Device registration should succeed");
            });

            // Check device is registered
            let device_hash = stylus_sdk::crypto::keccak(b"did:lcore:test-device").into();
            assert!(contract.is_device_registered(device_hash).unwrap());
            assert_eq!(contract.get_device_owner(device_hash).unwrap(), owner_address);
        });
    }

    #[test]
    fn test_access_control() {
        let mut contract = DeviceRegistry::new();
        let owner_address = address!("0000000000000000000000000000000000000001");
        let consumer_address = address!("0000000000000000000000000000000000000002");

        test_helpers::with_sender(owner_address, || {
            // Grant access
            let result = contract.grant_access(consumer_address, U256::ZERO);
            assert!(result.is_ok(), "Access grant should succeed");

            // Check access
            assert!(contract.has_access(owner_address, consumer_address).unwrap());

            // Revoke access
            let result = contract.revoke_access(consumer_address);
            assert!(result.is_ok(), "Access revoke should succeed");

            // Check access revoked
            assert!(!contract.has_access(owner_address, consumer_address).unwrap());
        });
    }
}