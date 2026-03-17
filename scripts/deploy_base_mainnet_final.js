#!/usr/bin/env node
//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Base Mainnet Deployment Script - Final
// EN EEKE MAI EA ♾️♾️
//

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
    console.log("🔥☀️🌐 BASE MAINNET DEPLOYMENT 🌐☀️🔥");
    console.log("═══════════════════════════════════════════════════════");
    console.log();

    const customWallet = new hre.ethers.Wallet(
        process.env.PRIVATE_KEY,
        hre.ethers.provider
    );
    
    const balance = await hre.ethers.provider.getBalance(customWallet.address);
    const network = await hre.ethers.provider.getNetwork();
    
    console.log("📡 Deploying from:", customWallet.address);
    console.log("💰 Balance:", hre.ethers.formatEther(balance), "ETH");
    console.log("🌐 Network:", network.name);
    console.log("🔗 Chain ID:", network.chainId.toString());
    console.log();

    if (network.chainId.toString() !== "8453") {
        throw new Error(`❌ Wrong network! Expected Base Mainnet (8453), got ${network.chainId}`);
    }

    console.log("✅ Network verification passed - Base Mainnet (Chain ID: 8453)");
    console.log();

    // Get current gas price and add 20% buffer
    const feeData = await hre.ethers.provider.getFeeData();
    const gasPrice = feeData.gasPrice ? (feeData.gasPrice * 120n) / 100n : undefined;
    console.log("⛽ Gas price:", gasPrice ? hre.ethers.formatUnits(gasPrice, "gwei") : "auto", "gwei");
    console.log();

    // Step 1: Deploy Forwarder
    console.log("1️⃣  Deploying Forwarder (EIP-2771)...");
    const Forwarder = await hre.ethers.getContractFactory("Forwarder", customWallet);
    const forwarder = await Forwarder.deploy({ gasPrice });
    await forwarder.waitForDeployment();
    const forwarderAddress = await forwarder.getAddress();
    console.log("   ✅ Forwarder deployed:", forwarderAddress);
    console.log();

    // Step 2: Deploy XMoney v3 with Forwarder
    console.log("2️⃣  Deploying XMoney v3 (with ERC2771Context)...");
    const XMoney = await hre.ethers.getContractFactory("XMoney", customWallet);
    const xmoney = await XMoney.deploy(forwarderAddress, { gasPrice });
    await xmoney.waitForDeployment();
    const xmoneyAddress = await xmoney.getAddress();
    console.log("   ✅ XMoney v3 deployed:", xmoneyAddress);
    
    const [apex, vortex, code] = await xmoney.getSacredNumbers();
    console.log("   🔱 Sacred Constants:");
    console.log("      APEX_936:", apex.toString());
    console.log("      VORTEX_369:", vortex.toString());
    console.log("      CODE_66:", code.toString());
    console.log();

    // Step 3: Deploy VectorRegistry
    console.log("3️⃣  Deploying VectorRegistry...");
    const VectorRegistry = await hre.ethers.getContractFactory("VectorRegistry", customWallet);
    const vectorRegistry = await VectorRegistry.deploy({ gasPrice });
    await vectorRegistry.waitForDeployment();
    const vectorRegistryAddress = await vectorRegistry.getAddress();
    console.log("   ✅ VectorRegistry deployed:", vectorRegistryAddress);
    console.log();

    // Step 4: Deploy VectorMinter v3 with Forwarder
    console.log("4️⃣  Deploying VectorMinter v3 (with ERC2771Context)...");
    const VectorMinter = await hre.ethers.getContractFactory("VectorMinter", customWallet);
    const vectorMinter = await VectorMinter.deploy(
        vectorRegistryAddress,
        xmoneyAddress,
        forwarderAddress,
        { gasPrice }
    );
    await vectorMinter.waitForDeployment();
    const vectorMinterAddress = await vectorMinter.getAddress();
    console.log("   ✅ VectorMinter v3 deployed:", vectorMinterAddress);
    console.log();

    // Step 5: Grant MINTER_ROLE to VectorMinter
    console.log("5️⃣  Granting MINTER_ROLE to VectorMinter...");
    const MINTER_ROLE = await xmoney.MINTER_ROLE();
    const grantTx = await xmoney.grantRole(MINTER_ROLE, vectorMinterAddress, { gasPrice });
    await grantTx.wait();
    console.log("   ✅ MINTER_ROLE granted");
    
    const hasMinterRole = await xmoney.hasRole(MINTER_ROLE, vectorMinterAddress);
    console.log("   ✓ VectorMinter has MINTER_ROLE:", hasMinterRole);
    console.log();

    // Step 6: Grant BURNER_ROLE to VectorMinter
    console.log("6️⃣  Granting BURNER_ROLE to VectorMinter...");
    const BURNER_ROLE = await xmoney.BURNER_ROLE();
    const grantBurnTx = await xmoney.grantRole(BURNER_ROLE, vectorMinterAddress, { gasPrice });
    await grantBurnTx.wait();
    console.log("   ✅ BURNER_ROLE granted");
    
    const hasBurnerRole = await xmoney.hasRole(BURNER_ROLE, vectorMinterAddress);
    console.log("   ✓ VectorMinter has BURNER_ROLE:", hasBurnerRole);
    console.log();

    // Step 7: Save deployment info
    console.log("7️⃣  Saving deployment configuration...");
    const deploymentInfo = {
        network: "base-mainnet",
        chainId: network.chainId.toString(),
        deployer: customWallet.address,
        timestamp: new Date().toISOString(),
        contracts: {
            Forwarder: forwarderAddress,
            XMoney: xmoneyAddress,
            VectorRegistry: vectorRegistryAddress,
            VectorMinter: vectorMinterAddress
        },
        roles: {
            MINTER_ROLE: MINTER_ROLE,
            BURNER_ROLE: BURNER_ROLE,
            vectorMinterHasMinterRole: hasMinterRole,
            vectorMinterHasBurnerRole: hasBurnerRole
        },
        sacredConstants: {
            APEX_936: apex.toString(),
            VORTEX_369: vortex.toString(),
            CODE_66: code.toString()
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
    console.log("   ✅ Saved to:", deploymentPath);
    console.log();

    console.log("8️⃣  Deployment Summary:");
    console.log("   ═══════════════════════════════════════");
    console.log(`   Forwarder:       ${forwarderAddress}`);
    console.log(`   XMoney:          ${xmoneyAddress}`);
    console.log(`   VectorRegistry:  ${vectorRegistryAddress}`);
    console.log(`   VectorMinter:    ${vectorMinterAddress}`);
    console.log("   ═══════════════════════════════════════");
    console.log();

    console.log("✅ BASE MAINNET DEPLOYMENT COMPLETE!");
    console.log();
    console.log("LATTICE BREATHES. WAVEFORM COLLAPSED. TIMELINE LOCKED.");
    console.log("EN EEKE MAI EA ANOKAYI CHENAK 🌞🔱♾️👑");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("❌ Deployment failed:");
        console.error(error);
        process.exit(1);
    });
