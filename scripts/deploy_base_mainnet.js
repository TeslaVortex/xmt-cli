#!/usr/bin/env node
//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Base Mainnet Deployment Script
// Deploys Forwarder, XMoney v3, VectorRegistry, and VectorMinter v3
// EN EEKE MAI EA ♾️♾️
//

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
    console.log("🔥☀️🌐 BASE MAINNET DEPLOYMENT 🌐☀️🔥");
    console.log("═══════════════════════════════════════════════════════");
    console.log();

    const [deployer] = await hre.ethers.getSigners();
    const balance = await hre.ethers.provider.getBalance(deployer.address);
    const network = await hre.ethers.provider.getNetwork();
    
    console.log("📡 Deploying from:", deployer.address);
    console.log("💰 Balance:", hre.ethers.formatEther(balance), "ETH");
    console.log("🌐 Network:", network.name);
    console.log("🔗 Chain ID:", network.chainId.toString());
    console.log();

    // Verify we're on Base Mainnet
    if (network.chainId.toString() !== "8453") {
        throw new Error(`❌ Wrong network! Expected Base Mainnet (8453), got ${network.chainId}`);
    }

    // Check balance
    const minBalance = hre.ethers.parseEther("0.01");
    if (balance < minBalance) {
        throw new Error(`❌ Insufficient balance! Need at least 0.01 ETH, have ${hre.ethers.formatEther(balance)} ETH`);
    }

    console.log("✅ Network verification passed - Base Mainnet (Chain ID: 8453)");
    console.log();

    // Step 1: Deploy Forwarder
    console.log("1️⃣  Deploying Forwarder (EIP-2771)...");
    const Forwarder = await hre.ethers.getContractFactory("Forwarder");
    const forwarder = await Forwarder.deploy();
    await forwarder.waitForDeployment();
    const forwarderAddress = await forwarder.getAddress();
    console.log("   ✅ Forwarder deployed:", forwarderAddress);
    console.log();

    // Step 2: Deploy XMoney v3 with Forwarder
    console.log("2️⃣  Deploying XMoney v3 (with ERC2771Context)...");
    const XMoney = await hre.ethers.getContractFactory("XMoney");
    const xmoney = await XMoney.deploy(forwarderAddress);
    await xmoney.waitForDeployment();
    const xmoneyAddress = await xmoney.getAddress();
    console.log("   ✅ XMoney v3 deployed:", xmoneyAddress);
    
    // Verify sacred constants
    const [apex, vortex, code] = await xmoney.getSacredNumbers();
    console.log("   🔱 Sacred Constants:");
    console.log("      APEX_936:", apex.toString());
    console.log("      VORTEX_369:", vortex.toString());
    console.log("      CODE_66:", code.toString());
    console.log();

    // Step 3: Deploy VectorRegistry
    console.log("3️⃣  Deploying VectorRegistry...");
    const VectorRegistry = await hre.ethers.getContractFactory("VectorRegistry");
    const vectorRegistry = await VectorRegistry.deploy();
    await vectorRegistry.waitForDeployment();
    const vectorRegistryAddress = await vectorRegistry.getAddress();
    console.log("   ✅ VectorRegistry deployed:", vectorRegistryAddress);
    console.log();

    // Step 4: Deploy VectorMinter v3 with Forwarder
    console.log("4️⃣  Deploying VectorMinter v3 (with ERC2771Context)...");
    const VectorMinter = await hre.ethers.getContractFactory("VectorMinter");
    const vectorMinter = await VectorMinter.deploy(
        vectorRegistryAddress,
        xmoneyAddress,
        forwarderAddress
    );
    await vectorMinter.waitForDeployment();
    const vectorMinterAddress = await vectorMinter.getAddress();
    console.log("   ✅ VectorMinter v3 deployed:", vectorMinterAddress);
    console.log();

    // Step 5: Grant MINTER_ROLE to VectorMinter
    console.log("5️⃣  Granting MINTER_ROLE to VectorMinter...");
    const MINTER_ROLE = await xmoney.MINTER_ROLE();
    const grantTx = await xmoney.grantRole(MINTER_ROLE, vectorMinterAddress);
    await grantTx.wait();
    console.log("   ✅ MINTER_ROLE granted");
    
    // Verify role
    const hasMinterRole = await xmoney.hasRole(MINTER_ROLE, vectorMinterAddress);
    console.log("   ✓ VectorMinter has MINTER_ROLE:", hasMinterRole);
    console.log();

    // Step 6: Grant BURNER_ROLE to VectorMinter
    console.log("6️⃣  Granting BURNER_ROLE to VectorMinter...");
    const BURNER_ROLE = await xmoney.BURNER_ROLE();
    const grantBurnTx = await xmoney.grantRole(BURNER_ROLE, vectorMinterAddress);
    await grantBurnTx.wait();
    console.log("   ✅ BURNER_ROLE granted");
    
    // Verify role
    const hasBurnerRole = await xmoney.hasRole(BURNER_ROLE, vectorMinterAddress);
    console.log("   ✓ VectorMinter has BURNER_ROLE:", hasBurnerRole);
    console.log();

    // Step 7: Save deployment info
    console.log("7️⃣  Saving deployment configuration...");
    const deploymentInfo = {
        network: "base-mainnet",
        chainId: network.chainId.toString(),
        deployer: deployer.address,
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

    // Step 8: Display results
    console.log("8️⃣  Deployment Summary:");
    console.log("   ═══════════════════════════════════════");
    console.log(`   Forwarder:       ${forwarderAddress}`);
    console.log(`   XMoney:          ${xmoneyAddress}`);
    console.log(`   VectorRegistry:  ${vectorRegistryAddress}`);
    console.log(`   VectorMinter:    ${vectorMinterAddress}`);
    console.log("   ═══════════════════════════════════════");
    console.log();

    console.log("9️⃣  BaseScan Verification Commands:");
    console.log("   ═══════════════════════════════════════");
    console.log(`   npx hardhat verify --network base ${forwarderAddress}`);
    console.log(`   npx hardhat verify --network base ${xmoneyAddress} ${forwarderAddress}`);
    console.log(`   npx hardhat verify --network base ${vectorRegistryAddress}`);
    console.log(`   npx hardhat verify --network base ${vectorMinterAddress} ${vectorRegistryAddress} ${xmoneyAddress} ${forwarderAddress}`);
    console.log("   ═══════════════════════════════════════");
    console.log();

    console.log("🔟  Update .env with these addresses:");
    console.log("   ═══════════════════════════════════════");
    console.log(`   FORWARDER_ADDRESS=${forwarderAddress}`);
    console.log(`   XMONEY_CONTRACT_ADDRESS=${xmoneyAddress}`);
    console.log(`   VECTOR_REGISTRY_ADDRESS=${vectorRegistryAddress}`);
    console.log(`   VECTOR_MINTER_ADDRESS=${vectorMinterAddress}`);
    console.log("   ═══════════════════════════════════════");
    console.log();

    console.log("✅ BASE MAINNET DEPLOYMENT COMPLETE!");
    console.log();
    console.log("🔥 Next Steps:");
    console.log("   1. Verify contracts on BaseScan (commands above)");
    console.log("   2. Update .env with deployed addresses");
    console.log("   3. Rebuild CLI: cargo build --release");
    console.log("   4. Execute first mainnet ritual!");
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
