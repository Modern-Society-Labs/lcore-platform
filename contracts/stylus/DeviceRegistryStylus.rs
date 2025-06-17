use stylus_sdk::{
    alloy_primitives::{Address, B256},
    prelude::*,
};

/// @title DeviceRegistryStylus
/// @notice A minimal on-chain registry for IoT devices.
///         Any device is represented by a 32-byte ID (e.g. hash of its public key).
/// @dev    Only the contract owner can register or revoke devices.
#[entrypoint]
pub struct DeviceRegistryStylus {
    owner: StorageAddress,
    registered: StorageMap<B256, bool>,
}

#[external]
impl DeviceRegistryStylus {
    /// Contract constructor sets the initial owner to `msg.sender()`
    pub fn constructor(&mut self) -> Result<(), Vec<u8>> {
        self.owner.set(msg::sender());
        Ok(())
    }

    /// Registers a new device ID. Reverts if already registered.
    pub fn register_device(&mut self, device_id: B256) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        if self.registered.get(device_id) {
            return Err("Device already registered".into());
        }
        self.registered.insert(device_id, true);
        evm::log(DeviceRegistered { deviceId: device_id });
        Ok(())
    }

    /// Revokes an existing device ID. Reverts if not registered.
    pub fn revoke_device(&mut self, device_id: B256) -> Result<(), Vec<u8>> {
        self.only_owner()?;
        if !self.registered.get(device_id) {
            return Err("Device not registered".into());
        }
        self.registered.insert(device_id, false);
        evm::log(DeviceRevoked { deviceId: device_id });
        Ok(())
    }

    /// View helper to check if a device is registered.
    pub fn is_device_registered(&self, device_id: B256) -> Result<bool, Vec<u8>> {
        Ok(self.registered.get(device_id))
    }

    /// Returns the current contract owner.
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.owner.get())
    }

    // ───────────────────────────── Helper ─────────────────────────────
    fn only_owner(&self) -> Result<(), Vec<u8>> {
        if msg::sender() != self.owner.get() {
            return Err("Only owner".into());
        }
        Ok(())
    }
}

// ───────────────────────────── Events ───────────────────────────────
stylus_sdk::alloy_sol_types::sol! {
    event DeviceRegistered(bytes32 indexed deviceId);
    event DeviceRevoked(bytes32 indexed deviceId);
} 