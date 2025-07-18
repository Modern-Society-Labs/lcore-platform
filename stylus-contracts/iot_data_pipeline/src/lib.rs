#![cfg_attr(not(feature = "export-abi"), no_main)]
// no_std removed for Stylus SDK 0.9 compatibility

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use alloy_sol_types::{sol, SolCall, SolValue};
use stylus_sdk::{
    alloy_primitives::{Address, B256, U256, U8},
    crypto::keccak,
    prelude::*,
    ArbResult,
    call::static_call,
};

// Define all on-chain events for the data pipeline.
sol! {
    event EncryptedDataStored(bytes32 indexed data_hash, address indexed device_address, uint256 timestamp);
    event AnalyticsComputed(bytes32 indexed analytics_hash, bytes32 indexed data_hash, uint8 analytics_type);
    event ZkProofStored(bytes32 indexed proof_hash, bytes32 indexed analytics_hash, bool is_valid);
}

sol_storage! {
    #[entrypoint]
    pub struct IoTDataPipeline {
        /* --- Admin and Configuration --- */
        address admin;
        address rollup_contract_address;
        address device_registry_address; // Address of the DeviceRegistry contract

        /* --- Data & Proof Storage --- */
        mapping(bytes32 => EncryptedDataRecord) encrypted_data_records;
        mapping(address => bytes32[]) device_data_hashes;
        mapping(bytes32 => AnalyticsComputation) analytics_computations;
        mapping(bytes32 => ZkProof) zk_proofs;

        /* --- Configuration for Analytics --- */
        mapping(uint8 => AnalyticsConfig) analytics_configs;

        /* --- Counters --- */
        uint256 total_records;
        uint256 total_analytics;
        uint256 total_proofs;
    }

    // --- Data Structures ---
    pub struct EncryptedDataRecord {
        bytes encrypted_payload;
        address device_address;
        uint256 timestamp;
        bool processed;
    }

    pub struct AnalyticsComputation {
        bytes32 data_hash;
        address device_address;
        bytes analytics_payload;
        uint8 analytics_type;
        uint256 timestamp;
        bool zk_proof_required;
    }

    pub struct ZkProof {
        bytes32 analytics_hash;
        bytes proof_payload;
        bytes public_inputs;
        bool is_valid;
    }

    pub struct AnalyticsConfig {
        bool enabled;
        uint256 processing_fee;
        bool requires_zk_proof;
    }
}

/// Implementation of the IoTDataPipeline contract.
#[public]
impl IoTDataPipeline {
    /// Initializes the contract, setting the admin, rollup, and registry addresses.
    pub fn initialize(&mut self, rollup_address: Address, registry_address: Address) -> Result<(), Vec<u8>> {
        if !self.admin.get().is_zero() {
            return Err(b"Contract already initialized".to_vec());
        }
        self.admin.set(self.vm().msg_sender());
        self.rollup_contract_address.set(rollup_address);
        self.device_registry_address.set(registry_address);
        self._initialize_analytics_configs();
        Ok(())
    }

    /// The main entrypoint called by the CartesiDApp.sol wrapper contract.
    /// For this PoC, we assume the payload *is* the device_id for verification.
    /// A production system would have a more complex payload format.
    pub fn submit_cartesi_result(&mut self, payload: Vec<u8>) -> Result<(), Vec<u8>> {
        // only rollup may call
        if self.vm().msg_sender() != self.rollup_contract_address.get() {
            return Err(b"Unauthorized: caller is not the rollup".to_vec());
        }

        let device_id_bytes = payload.clone();
        let mock_encrypted_data = b"mock_encrypted_data".to_vec();
        let data_hash = keccak(&mock_encrypted_data).into();
        let device_id_hash = keccak(&device_id_bytes).into();

        // ─── verify registration ───────────────────────────────────────────────
        let registry_addr = self.device_registry_address.get();
        let calldata = IIoTexDeviceRegistry::is_device_registeredCall {
            device_id_hash,
        }
        .abi_encode();

        let bytes = static_call(&mut *self, registry_addr, &calldata)?;
        
        let (is_registered,) = <(bool,)>::abi_decode(&bytes, true)
            .map_err(|_| b"Failed to decode registry response".to_vec())?;
        if !is_registered {
            return Err(b"Device not registered or inactive".to_vec());
        }
        // ────────────────────────────────────────────────────────────────────────

        // TODO: parse real device_address from payload
        let mock_device_address = self.vm().msg_sender();
        self._store_encrypted_data(data_hash, mock_device_address, mock_encrypted_data);
        self._trigger_analytics(data_hash, mock_device_address, device_id_bytes);

        Ok(())
    }

    /// Stores a ZK proof for a given analytics computation.
    /// This function would be called by a trusted prover or the rollup itself.
    pub fn store_zk_proof(
        &mut self,
        analytics_hash: B256,
        proof_payload: Vec<u8>,
        public_inputs: Vec<u8>,
        is_valid: bool,
    ) -> Result<(), Vec<u8>> {
        if self.analytics_computations.getter(analytics_hash).timestamp.get().is_zero() {
            return Err(b"Analytics computation not found".to_vec());
        }

        let proof_hash: B256 = keccak(&proof_payload).into();
        let mut proof = self.zk_proofs.setter(proof_hash);
        proof.analytics_hash.set(analytics_hash);
        proof.proof_payload.set_bytes(proof_payload);
        proof.public_inputs.set_bytes(public_inputs);
        proof.is_valid.set(is_valid);
        
        self.total_proofs.set(self.total_proofs.get() + U256::from(1));

        log(self.vm(), ZkProofStored {
            proof_hash,
            analytics_hash,
            is_valid,
        });

        Ok(())
    }

    // --- Admin Functions ---

    /// Updates the address of the CartesiDApp wrapper contract.
    pub fn set_rollup_contract(&mut self, new_address: Address) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(b"Only admin".to_vec());
        }
        self.rollup_contract_address.set(new_address);
        Ok(())
    }

    /// Updates the address of the DeviceRegistry contract.
    pub fn set_device_registry(&mut self, new_addr: Address) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        self.device_registry_address.set(new_addr);
        Ok(())
    }

    /// Updates the configuration for a specific analytics tier.
    pub fn update_analytics_config(
        &mut self,
        analytics_type: u8,
        enabled: bool,
        fee: U256,
        zk_required: bool,
    ) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        let key_uint = U8::from(analytics_type);
        let mut config = self.analytics_configs.setter(key_uint);
        config.enabled.set(enabled);
        config.processing_fee.set(fee);
        config.requires_zk_proof.set(zk_required);
        Ok(())
    }

    // --- View Functions ---

    /// Returns the total number of encrypted data records stored.
    pub fn total_records(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_records.get())
    }

    /// Returns the total number of analytics computations performed.
    pub fn total_analytics(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_analytics.get())
    }

    /// Returns the total number of ZK proofs processed.
    pub fn total_proofs(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_proofs.get())
    }

    /// Returns the configuration for a specific analytics tier.
    pub fn get_analytics_config(&self, analytics_type: u8) -> Result<(bool, U256, bool), Vec<u8>> {
        let config = self.analytics_configs.getter(U8::from(analytics_type));
        Ok((
            config.enabled.get(),
            config.processing_fee.get(),
            config.requires_zk_proof.get(),
        ))
    }

    /// Returns the list of data hashes submitted by a specific device.
    pub fn get_device_data_hashes(&self, device_address: Address) -> Result<Vec<B256>, Vec<u8>> {
        let hashes = self.device_data_hashes.getter(device_address);
        let mut result = Vec::with_capacity(hashes.len() as usize);
        for i in 0..hashes.len() {
            result.push(hashes.get(i).unwrap());
        }
        Ok(result)
    }

    /// Simple liveness check; returns 1 if contract is reachable.
    pub fn ping(&self) -> Result<U256, Vec<u8>> {
        Ok(U256::from(1))
    }

    /// Minimal method solely to trigger ABI generation.
    pub fn abi_test(&self) -> ArbResult {
        Ok(Vec::new())
    }
}

/// Internal helper functions for the data pipeline.
impl IoTDataPipeline {
    /// mirror of registry.only_owner()
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err(b"Only admin".to_vec());
        }
        Ok(())
    }
    
    /// Stores the encrypted data payload and emits an event.
    fn _store_encrypted_data(&mut self, data_hash: B256, device_address: Address, payload: Vec<u8>) {
        let timestamp = self.vm().block_timestamp();
        let mut record = self.encrypted_data_records.setter(data_hash);
        
        record.encrypted_payload.set_bytes(payload.clone());
        record.device_address.set(device_address);
        record.timestamp.set(U256::from(timestamp));
        record.processed.set(false);

        self.device_data_hashes.setter(device_address).push(data_hash);
        self.total_records.set(self.total_records.get() + U256::from(1));

        log(self.vm(), EncryptedDataStored {
            data_hash,
            device_address,
            timestamp: U256::from(timestamp),
        });
    }

    /// Triggers an analytics computation based on the data.
    fn _trigger_analytics(&mut self, data_hash: B256, device_address: Address, payload: Vec<u8>) {
        // For now, we'll hardcode to the "basic" analytics type (0).
        // A real implementation might determine the type from the payload or device registry.
        let analytics_type = 0; 
        let analytics_hash: B256 = keccak(&payload).into();
        let timestamp = self.vm().block_timestamp();
        let config = self.analytics_configs.get(U8::from(analytics_type));
        let mut computation = self.analytics_computations.setter(analytics_hash);

        computation.data_hash.set(data_hash);
        computation.device_address.set(device_address);
        computation.analytics_payload.set_bytes(payload);
        computation.analytics_type.set(U8::from(analytics_type));
        computation.timestamp.set(U256::from(timestamp));
        computation.zk_proof_required.set(config.requires_zk_proof.get());
        
        let new_total = self.total_analytics.get() + U256::from(1);
        self.total_analytics.set(new_total);

        log(self.vm(), AnalyticsComputed {
            analytics_hash,
            data_hash,
            analytics_type,
        });
    }
    
    /// Sets up the default configurations for the analytics tiers.
    fn _initialize_analytics_configs(&mut self) {
        // Basic analytics config
        let mut basic_config = self.analytics_configs.setter(U8::from(0));
        basic_config.enabled.set(true);
        basic_config.processing_fee.set(U256::ZERO);
        basic_config.requires_zk_proof.set(false);

        // Advanced analytics config
        let mut advanced_config = self.analytics_configs.setter(U8::from(1));
        advanced_config.enabled.set(true);
        advanced_config.processing_fee.set(U256::from(10u64.pow(15))); // 0.001 ETH
        advanced_config.requires_zk_proof.set(true);

        // ML analytics config
        let mut ml_config = self.analytics_configs.setter(U8::from(2));
        ml_config.enabled.set(true);
        ml_config.processing_fee.set(U256::from(10u64.pow(16))); // 0.01 ETH
        ml_config.requires_zk_proof.set(true);
    }
} 

sol! {
    interface IIoTexDeviceRegistry {
        function is_device_registered(bytes32 device_id_hash) external view returns (bool);
    }
} 