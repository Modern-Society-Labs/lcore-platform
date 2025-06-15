// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@nomicfoundation/hardhat/console.sol";

/**
 * @title DeviceRegistry
 * @dev A simple on-chain registry for IoT devices.
 * For the MVP, this contract allows an owner to register devices
 * and allows anyone to check if a device is registered.
 */
contract DeviceRegistry {
    address public owner;
    mapping(bytes32 => bool) public registeredDevices;

    event DeviceRegistered(bytes32 indexed deviceId);
    event DeviceRevoked(bytes32 indexed deviceId);

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can call this function");
        _;
    }

    constructor() {
        owner = msg.sender;
        console.log("DeviceRegistry deployed by:", owner);
    }

    /**
     * @dev Registers a new device.
     * @param deviceId The unique identifier of the device (e.g., a hash of its public key).
     */
    function registerDevice(bytes32 deviceId) public onlyOwner {
        require(!registeredDevices[deviceId], "Device is already registered");
        registeredDevices[deviceId] = true;
        emit DeviceRegistered(deviceId);
        console.log("Device registered:", deviceId);
    }

    /**
     * @dev Revokes a device's registration.
     * @param deviceId The unique identifier of the device to revoke.
     */
    function revokeDevice(bytes32 deviceId) public onlyOwner {
        require(registeredDevices[deviceId], "Device is not registered");
        registeredDevices[deviceId] = false;
        emit DeviceRevoked(deviceId);
        console.log("Device revoked:", deviceId);
    }

    /**
     * @dev Checks if a device is registered.
     * @param deviceId The unique identifier of the device to check.
     * @return A boolean indicating whether the device is registered.
     */
    function isDeviceRegistered(bytes32 deviceId) public view returns (bool) {
        return registeredDevices[deviceId];
    }
} 