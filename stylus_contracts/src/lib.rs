#![cfg_attr(not(feature = "export-abi"), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::{Address, B256, U256}, prelude::*};

/// @title SimpleStorageStylus
/// @dev A simple Rust contract for testing deployment on Arbitrum Stylus
sol_storage! {
    #[entrypoint]
    pub struct MVPIoTProcessor {
        /// Mapping from task ID to processing result.
        mapping(B256 => ProcessingResult) results;
    }

    /// Represents the result of a processed task.
    pub struct ProcessingResult {
        /// Encrypted result data.
        bytes encrypted_result;
        /// Hash of the proof.
        bytes32 proof_hash;
        /// Timestamp of the submission.
        uint256 timestamp;
        /// Address of the node that submitted the result.
        address submitter;
    }
}

#[public]
impl MVPIoTProcessor {
    pub fn submit_result(
        &mut self,
        task_id: B256,
        encrypted_result: Vec<u8>,
        proof_hash: B256,
    ) -> Result<(), Vec<u8>> {
        let timestamp = self.vm().block_timestamp();
        let submitter = self.vm().msg_sender();

        let mut result_storage = self.results.setter(task_id);
        result_storage.encrypted_result.set_bytes(&encrypted_result);
        result_storage.proof_hash.set(proof_hash);
        result_storage.timestamp.set(U256::from(timestamp));
        result_storage.submitter.set(submitter);

        Ok(())
    }

    pub fn get_result(&self, task_id: B256) -> Result<(Vec<u8>, B256, U256, Address), Vec<u8>> {
        let result = self.results.get(task_id);
        Ok((
            result.encrypted_result.get_bytes(),
            result.proof_hash.get(),
            result.timestamp.get(),
            result.submitter.get(),
        ))
    }
}
