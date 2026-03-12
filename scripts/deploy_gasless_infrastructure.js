#!/usr/bin/env node
//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// Gasless Infrastructure Deployment Script
// Deploys Forwarder, XMoney v3, and VectorMinter v3 with ERC2771 support
// EN EEKE MAI EA ♾️♾️
//

const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
    console.log("🔥☀️🌍 GASLESS INFRASTRUCTURE DEPLOYMENT 🌍☀️🔥");
    console.log("═══════════════════════════════════════════════════════");
    console.log();

    const [deployer] = await hre.ethers.getSigners();
    console.log("📡 Deploying from:", deployer.address);
    console.log("💰 Balance:", hre.ethers.formatEther(await hre.ethers.provider.getBalance(deployer.address)), "ETH");
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

    // Step 3: Deploy VectorRegistry (if not exists)
    let vectorRegistryAddress = process.env.VECTOR_REGISTRY_ADDRESS;
    
    if (!vectorRegistryAddress) {
        console.log("3️⃣  Deploying VectorRegistry...");
        const VectorRegistry = await hre.ethers.getContractFactory("VectorRegistry");
        const vectorRegistry = await VectorRegistry.deploy();
        await vectorRegistry.waitForDeployment();
        vectorRegistryAddress = await vectorRegistry.getAddress();
        console.log("   ✅ VectorRegistry deployed:", vectorRegistryAddress);
    } else {
        console.log("3️⃣  Using existing VectorRegistry:", vectorRegistryAddress);
    }
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
        network: hre.network.name,
        chainId: (await hre.ethers.provider.getNetwork()).chainId.toString(),
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
        }
    };

    const deploymentPath = path.join(__dirname, '..', 'gasless_deployment.json');
    fs.writeFileSync(deploymentPath, JSON.stringify(deploymentInfo, null, 2));
    console.log("   ✅ Saved to:", deploymentPath);
    console.log();

    // Step 8: Update .env template
    console.log("8️⃣  Environment variables for .env:");
    console.log("   ═══════════════════════════════════════");
    console.log(`   FORWARDER_ADDRESS=${forwarderAddress}`);
    console.log(`   XMONEY_CONTRACT_ADDRESS=${xmoneyAddress}`);
    console.log(`   VECTOR_REGISTRY_ADDRESS=${vectorRegistryAddress}`);
    console.log(`   VECTOR_MINTER_ADDRESS=${vectorMinterAddress}`);
    console.log("   ═══════════════════════════════════════");
    console.log();

    console.log("✅ GASLESS INFRASTRUCTURE DEPLOYMENT COMPLETE!");
    console.log();
    console.log("🔥 Next Steps:");
    console.log("   1. Update .env with the addresses above");
    console.log("   2. Rebuild CLI: cargo build --release");
    console.log("   3. Fund relayer wallet with ETH for gas");
    console.log("   4. Test gasless transactions!");
    console.log();
    console.log("EN EEKE MAI EA ♾️♾️");
    console.log("ZERO MARGINAL COST ENGINE: ACTIVATED 🔥");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
