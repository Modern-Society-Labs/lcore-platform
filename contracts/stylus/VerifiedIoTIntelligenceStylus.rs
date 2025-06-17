use stylus_sdk::{
    alloy_primitives::{Address, U256, B256},
    alloy_sol_types::sol,
    prelude::*,
};

sol! {
    #[derive(Debug)]
    struct IoTBatch {
        uint256 batchId;
        string deviceSummary;
        uint256 timestamp;
        bytes32 proofHash;
        bool verified;
    }
}

/// @title VerifiedIoTIntelligenceStylus
/// @dev A Rust implementation of VerifiedIoTIntelligence for Arbitrum Stylus
/// This contract leverages the IoTDataProcessor and CartesiVerifier to provide
/// fraud-proof verified IoT data intelligence.
#[entrypoint]
pub struct VerifiedIoTIntelligenceStylus {
    owner: StorageAddress,
    iot_data_processor: StorageAddress,
    cartesi_verifier: StorageAddress,
    query_interval: StorageU256,
    last_query_timestamp: StorageU256,
    iot_batches: StorageMap<U256, IoTBatch>,
    current_batch_id: StorageU256,
}

#[external]
impl VerifiedIoTIntelligenceStylus {
    /// Constructor to initialize the contract
    /// @param iot_processor The address of the IoT data processor contract
    /// @param cartesi_verifier The address of the Cartesi verifier contract
    pub fn constructor(&mut self, iot_processor: Address, cartesi_verifier: Address) -> Result<(), Vec<u8>> {
        self.owner.set(msg::sender());
        self.iot_data_processor.set(iot_processor);
        self.cartesi_verifier.set(cartesi_verifier);
        self.query_interval.set(U256::from(3600)); // Default: 1 hour (in seconds)
        self.last_query_timestamp.set(block::timestamp());
        self.current_batch_id.set(U256::from(1));
        Ok(())
    }

    /// Stores an IoT batch with device summary and proof
    /// @param device_summary A summary of the IoT devices processed
    /// @param proof_hash The hash of the RiscZero proof
    /// @return The batch ID of the stored data
    pub fn store_iot_batch(
        &mut self,
        device_summary: String,
        proof_hash: B256,
    ) -> Result<U256, Vec<u8>> {
        let sender = msg::sender();
        if sender != self.owner.get() && sender != self.iot_data_processor.get() {
            return Err("Not authorized".into());
        }
        
        let batch_id = self.current_batch_id.get();
        
        let iot_batch = IoTBatch {
            batchId: batch_id,
            deviceSummary: device_summary.clone().into(),
            timestamp: block::timestamp(),
            proofHash: proof_hash,
            verified: false,
        };
        
        self.iot_batches.insert(batch_id, iot_batch);
        self.current_batch_id.set(batch_id + U256::from(1));
        
        // Emit event
        evm::log(
            BatchStored {
                batchId: batch_id,
                deviceSummary: device_summary.into(),
                timestamp: block::timestamp(),
            }
        );
        
        Ok(batch_id)
    }
    
    /// Verifies the proof for a specific IoT batch
    /// @param batch_id The ID of the batch to verify
    /// @param epoch The Cartesi epoch number for verification
    /// @return A boolean indicating whether the verification was successful
    pub fn verify_batch(&mut self, batch_id: U256, epoch: U256) -> Result<bool, Vec<u8>> {
        let mut batch = self.iot_batches.get_mut(batch_id);
        
        if batch.timestamp == U256::ZERO {
            return Err("Batch does not exist".into());
        }
        
        if batch.verified {
            return Err("Batch already verified".into());
        }
        
        // Call the CartesiVerifier contract
        // In a real implementation, this would use cross-contract calls
        // For MVP, we'll simulate verification success
        let verification_success = true;
        
        if verification_success {
            batch.verified = true;
            
            // Emit event
            evm::log(
                ProofVerified {
                    batchId: batch_id,
                    proofHash: batch.proofHash,
                }
            );
        }
        
        Ok(verification_success)
    }
    
    /// Sets the query interval for automated processing
    /// @param interval The new interval in seconds
    pub fn set_query_interval(&mut self, interval: U256) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        
        if interval == U256::ZERO {
            return Err("Interval must be greater than zero".into());
        }
        
        self.query_interval.set(interval);
        Ok(())
    }
    
    /// Checks if a query is due based on the configured interval
    /// @return A boolean indicating whether a query is due
    pub fn is_query_due(&self) -> Result<bool, Vec<u8>> {
        let elapsed = block::timestamp() - self.last_query_timestamp.get();
        Ok(elapsed >= self.query_interval.get())
    }
    
    /// Triggers an automated query for IoT data processing
    pub fn trigger_automated_query(&mut self) -> Result<(), Vec<u8>> {
        let elapsed = block::timestamp() - self.last_query_timestamp.get();
        
        if elapsed < self.query_interval.get() {
            return Err("Query interval not reached".into());
        }
        
        self.last_query_timestamp.set(block::timestamp());
        
        // Emit event
        evm::log(
            QueryTriggered {
                timestamp: block::timestamp(),
            }
        );
        
        // In a real implementation, this would trigger the Cartesi VM to process IoT data
        Ok(())
    }
    
    /// Gets the latest IoT batch summary
    /// @return The device summary from the latest batch
    pub fn get_latest_iot_summary(&self) -> Result<String, Vec<u8>> {
        let current_id = self.current_batch_id.get();
        
        if current_id == U256::from(1) {
            return Err("No batches stored yet".into());
        }
        
        let latest_batch = self.iot_batches.get(current_id - U256::from(1));
        Ok(latest_batch.deviceSummary.to_string())
    }
    
    /// Gets details for a specific IoT batch
    /// @param batch_id The ID of the batch to retrieve
    /// @return The IoT batch details
    pub fn get_batch_details(&self, batch_id: U256) -> Result<IoTBatch, Vec<u8>> {
        let batch = self.iot_batches.get(batch_id);
        
        if batch.timestamp == U256::ZERO {
            return Err("Batch does not exist".into());
        }
        
        Ok(batch)
    }
    
    /// Gets the owner of the contract
    /// @return The owner's address
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.owner.get())
    }
    
    // Helper functions
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if msg::sender() != self.owner.get() {
            return Err("Only the owner can call this function".into());
        }
        Ok(())
    }
}

// Event definitions
sol! {
    event BatchStored(uint256 indexed batchId, string deviceSummary, uint256 timestamp);
    event ProofVerified(uint256 indexed batchId, bytes32 proofHash);
    event QueryTriggered(uint256 timestamp);
} 