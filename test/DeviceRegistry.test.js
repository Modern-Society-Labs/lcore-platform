const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("DeviceRegistry", function () {
    let DeviceRegistry;
    let deviceRegistry;
    let owner;
    let addr1;

    beforeEach(async function () {
        [owner, addr1] = await ethers.getSigners();
        const DeviceRegistryFactory = await ethers.getContractFactory("DeviceRegistry");
        deviceRegistry = await DeviceRegistryFactory.deploy();
        await deviceRegistry.waitForDeployment();
    });

    it("Should set the deployer as the owner", async function () {
        expect(await deviceRegistry.owner()).to.equal(owner.address);
    });

    it("Should allow the owner to register a device", async function () {
        const deviceId = ethers.keccak256(ethers.toUtf8Bytes("device-pub-key-1"));
        await expect(deviceRegistry.registerDevice(deviceId))
            .to.emit(deviceRegistry, "DeviceRegistered")
            .withArgs(deviceId);
        expect(await deviceRegistry.isDeviceRegistered(deviceId)).to.be.true;
    });

    it("Should prevent non-owners from registering a device", async function () {
        const deviceId = ethers.keccak256(ethers.toUtf8Bytes("device-pub-key-2"));
        await expect(
            deviceRegistry.connect(addr1).registerDevice(deviceId)
        ).to.be.revertedWith("Only the owner can call this function");
    });

    it("Should prevent registering an already registered device", async function () {
        const deviceId = ethers.keccak256(ethers.toUtf8Bytes("device-pub-key-1"));
        await deviceRegistry.registerDevice(deviceId);
        await expect(
            deviceRegistry.registerDevice(deviceId)
        ).to.be.revertedWith("Device is already registered");
    });
}); 