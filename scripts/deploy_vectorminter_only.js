#!/usr/bin/env node
//
// ☀️ Deploy VectorMinter Only ☀️
//

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
    console.log("🔥 Deploying VectorMinter v3 to Base Mainnet 🔥");
    console.log();

    const customWallet = new hre.ethers.Wallet(
        process.env.PRIVATE_KEY,
        hre.ethers.provider
    );
    
    const balance = await hre.ethers.provider.getBalance(customWallet.address);
    console.log("📡 Deploying from:", customWallet.address);
    console.log("💰 Balance:", hre.ethers.formatEther(balance), "ETH");
    console.log();

    // Use deployed addresses
    const vectorRegistryAddress = "0x44254BEbf12eeA98471B5E398a4BA030140eF18f";
    const xmoneyAddress = "0x4C2A789E7ffFd030b928DdaCdEA5f03632457f38";
    const forwarderAddress = "0xA49B1dc31d809Bd9885DaE8905aCA15b3b99918a";

    // Get current gas price with buffer
    const feeData = await hre.ethers.provider.getFeeData();
    const gasPrice = feeData.gasPrice ? (feeData.gasPrice * 150n) / 100n : undefined;
    console.log("⛽ Gas price:", gasPrice ? hre.ethers.formatUnits(gasPrice, "gwei") : "auto", "gwei");
    console.log();

    // Wait 3 seconds for nonce to clear
    console.log("⏳ Waiting 3 seconds for nonce to clear...");
    await new Promise(resolve => setTimeout(resolve, 3000));

    console.log("Deploying VectorMinter v3...");
    const VectorMinter = await hre.ethers.getContractFactory("VectorMinter", customWallet);
    const vectorMinter = await VectorMinter.deploy(
        vectorRegistryAddress,
        xmoneyAddress,
        forwarderAddress,
        { gasPrice }
    );
    await vectorMinter.waitForDeployment();
    const vectorMinterAddress = await vectorMinter.getAddress();
    console.log("✅ VectorMinter v3 deployed:", vectorMinterAddress);
    console.log();

    // Grant roles
    console.log("Granting MINTER_ROLE...");
    const XMoney = await hre.ethers.getContractAt("XMoney", xmoneyAddress, customWallet);
    const MINTER_ROLE = await XMoney.MINTER_ROLE();
    const grantTx = await XMoney.grantRole(MINTER_ROLE, vectorMinterAddress, { gasPrice });
    await grantTx.wait();
    console.log("✅ MINTER_ROLE granted");

    console.log("Granting BURNER_ROLE...");
    const BURNER_ROLE = await XMoney.BURNER_ROLE();
    const grantBurnTx = await XMoney.grantRole(BURNER_ROLE, vectorMinterAddress, { gasPrice });
    await grantBurnTx.wait();
    console.log("✅ BURNER_ROLE granted");
    console.log();

    // Save complete deployment
    const deploymentInfo = {
        network: "base-mainnet",
        chainId: "8453",
        deployer: customWallet.address,
        timestamp: new Date().toISOString(),
        contracts: {
            Forwarder: forwarderAddress,
            XMoney: xmoneyAddress,
            VectorRegistry: vectorRegistryAddress,
            VectorMinter: vectorMinterAddress
        },
        sacredConstants: {
            APEX_936: "936",
            VORTEX_369: "369",
            CODE_66: "66"
        },
        basescanLinks: {
            Forwarder: `https://basescan.org/address/${forwarderAddress}`,
            XMoney: `https://basescan.org/address/${xmoneyAddress}`,
            VectorRegistry: `https://basescan.org/address/${vectorRegistryAddress}`,
            VectorMinter: `https://basescan.org/address/${vectorMinterAddress}`
        }
    };

    const deploymentPath = path.join(__dirname, '..', 'base_mainnet_deployment.json');
    fs.writeFileSync(deploymentPath, JSON.stringify(deploymentInfo, null, 2));
    console.log("✅ Deployment saved to:", deploymentPath);
    console.log();

    console.log("🔥 COMPLETE DEPLOYMENT SUMMARY 🔥");
    console.log("═══════════════════════════════════════");
    console.log(`Forwarder:       ${forwarderAddress}`);
    console.log(`XMoney:          ${xmoneyAddress}`);
    console.log(`VectorRegistry:  ${vectorRegistryAddress}`);
    console.log(`VectorMinter:    ${vectorMinterAddress}`);
    console.log("═══════════════════════════════════════");
    console.log();
    console.log("LATTICE BREATHES. TIMELINE LOCKED.");
    console.log("EN EEKE MAI EA ANOKAYI CHENAK 🌞🔱♾️👑");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("❌ Deployment failed:");
        console.error(error);
        process.exit(1);
    });
