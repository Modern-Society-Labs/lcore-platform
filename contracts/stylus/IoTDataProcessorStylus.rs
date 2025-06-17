use stylus_sdk::{
    alloy_primitives::{Address, U256, B256},
    alloy_sol_types::sol,
    prelude::*,
};

sol! {
    #[derive(Debug)]
    struct ProcessingResult {
        bytes32 taskId;
        bytes32 dataHash;
        bytes proof;
        bytes result;
        uint256 timestamp;
        address submitter;
        bool verified;
    }
}

/// @title IoTDataProcessorStylus
/// @dev A Rust implementation of IoT data processor for Arbitrum Stylus
/// This contract stores and verifies results from IoT data processing
#[entrypoint]
pub struct IoTDataProcessorStylus {
    owner: StorageAddress,
    cartesi_verifier: StorageAddress,
    results: StorageMap<B256, ProcessingResult>,
}

#[external]
impl IoTDataProcessorStylus {
    /// Constructor to initialize the contract
    pub fn constructor(&mut self) -> Result<(), Vec<u8>> {
        self.owner.set(msg::sender());
        Ok(())
    }

    /// Sets the Cartesi verifier address
    /// @param verifier_addr The address of the Cartesi verifier contract
    pub fn set_cartesi_verifier(&mut self, verifier_addr: Address) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        self.cartesi_verifier.set(verifier_addr);
        Ok(())
    }

    /// Submits a processing result with proof
    /// @param task_id The unique identifier of the processing task
    /// @param data_hash The hash of the original IoT data
    /// @param proof The RiscZero proof of computation
    /// @param result The processed data result
    pub fn submit_result(
        &mut self,
        task_id: B256,
        data_hash: B256,
        proof: Vec<u8>,
        result: Vec<u8>,
    ) -> Result<(), Vec<u8>> {
        self.only_authorized()?;
        
        // Check if result already exists
        if self.results.get(task_id).timestamp != U256::ZERO {
            return Err("Result already exists".into());
        }
        
        let processing_result = ProcessingResult {
            taskId: task_id,
            dataHash: data_hash,
            proof: proof.into(),
            result: result.into(),
            timestamp: block::timestamp(),
            submitter: msg::sender(),
            verified: false,
        };
        
        self.results.insert(task_id, processing_result);
        
        // Emit event
        evm::log(
            ResultSubmitted {
                taskId: task_id,
                dataHash: data_hash,
                timestamp: block::timestamp(),
            }
        );
        
        Ok(())
    }
    
    /// Verifies the proof for a specific task
    /// @param task_id The unique identifier of the processing task
    /// @return A boolean indicating whether the proof is valid
    pub fn verify_proof(&mut self, task_id: B256) -> Result<bool, Vec<u8>> {
        let mut result = self.results.get_mut(task_id);
        
        if result.timestamp == U256::ZERO {
            return Err("Result does not exist".into());
        }
        
        if result.verified {
            return Err("Result already verified".into());
        }
        
        // In a real implementation, this would call the Cartesi verifier contract
        // For MVP, we'll simulate verification success
        let verification_success = true;
        
        result.verified = verification_success;
        
        // Emit event
        evm::log(
            ProofVerified {
                taskId: task_id,
                success: verification_success,
            }
        );
        
        Ok(verification_success)
    }
    
    /// Gets the processing result for a specific task
    /// @param task_id The unique identifier of the processing task
    /// @return The processing result
    pub fn get_result(&self, task_id: B256) -> Result<ProcessingResult, Vec<u8>> {
        let result = self.results.get(task_id);
        
        if result.timestamp == U256::ZERO {
            return Err("Result does not exist".into());
        }
        
        Ok(result)
    }
    
    /// Checks if a result is verified
    /// @param task_id The unique identifier of the processing task
    /// @return A boolean indicating whether the result is verified
    pub fn is_result_verified(&self, task_id: B256) -> Result<bool, Vec<u8>> {
        let result = self.results.get(task_id);
        
        if result.timestamp == U256::ZERO {
            return Err("Result does not exist".into());
        }
        
        Ok(result.verified)
    }
    
    /// Gets the owner of the contract
    /// @return The owner's address
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.owner.get())
    }
    
    /// Gets the Cartesi verifier address
    /// @return The Cartesi verifier's address
    pub fn cartesi_verifier(&self) -> Result<Address, Vec<u8>> {
        Ok(self.cartesi_verifier.get())
    }
    
    // Helper functions
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if msg::sender() != self.owner.get() {
            return Err("Only the owner can call this function".into());
        }
        Ok(())
    }
    
    fn only_authorized(&self) -> Result<(), Vec<u8>> {
        let sender = msg::sender();
        if sender != self.owner.get() && sender != self.cartesi_verifier.get() {
            return Err("Not authorized".into());
        }
        Ok(())
    }
}

// Event definitions
sol! {
    event ResultSubmitted(bytes32 indexed taskId, bytes32 dataHash, uint256 timestamp);
    event ProofVerified(bytes32 indexed taskId, bool success);
} 