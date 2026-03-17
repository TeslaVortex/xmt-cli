#!/usr/bin/env node
//
// Register First Vector on Base Mainnet
//

const hre = require("hardhat");
const { ethers } = require("ethers");
const crypto = require("crypto");

async function main() {
    console.log("🔥 Registering First Vector on Base Mainnet 🔥");
    console.log();

    const customWallet = new hre.ethers.Wallet(
        process.env.PRIVATE_KEY,
        hre.ethers.provider
    );
    
    const network = await hre.ethers.provider.getNetwork();
    console.log("Network:", network.name);
    console.log("Chain ID:", network.chainId.toString());
    console.log("Wallet:", customWallet.address);
    console.log();

    if (network.chainId.toString() !== "8453") {
        throw new Error(`Wrong network! Expected Base Mainnet (8453), got ${network.chainId}`);
    }

    const vectorRegistryAddress = "0x44254BEbf12eeA98471B5E398a4BA030140eF18f";
    const intent = "FIRST BASE MAINNET VECTOR - MARCH 17 2026 - GATE DETONATION - EN EEKE MAI EA ANOKAYI CHENAK - LATTICE BREATHES - TIMELINE LOCKED - 369 VORTEX ACTIVATED";
    
    // Generate 384D vector hash (simplified version matching Rust logic)
    const hash = crypto.createHash('sha256').update(intent).digest('hex');
    const vectorHash = '0x' + hash;
    
    console.log("Intent:", intent);
    console.log("Vector Hash:", vectorHash);
    console.log();

    // Load VectorRegistry contract
    const VectorRegistry = await hre.ethers.getContractAt("VectorRegistry", vectorRegistryAddress, customWallet);
    
    console.log("Registering vector on-chain...");
    const tx = await VectorRegistry.registerVector(vectorHash, intent, 384);
    console.log("Transaction sent:", tx.hash);
    
    const receipt = await tx.wait();
    console.log("✅ Transaction confirmed!");
    console.log("Block:", receipt.blockNumber);
    console.log("Gas used:", receipt.gasUsed.toString());
    console.log();
    
    console.log("BaseScan:", `https://basescan.org/tx/${tx.hash}`);
    console.log();
    
    // Verify registration
    const exists = await VectorRegistry.verifyVector(vectorHash);
    console.log("Vector verified:", exists ? "✅ YES" : "❌ NO");
    
    const totalVectors = await VectorRegistry.totalVectors();
    console.log("Total vectors:", totalVectors.toString());
    console.log();
    
    console.log("LATTICE BREATHES. TIMELINE LOCKED.");
    console.log("EN EEKE MAI EA ANOKAYI CHENAK 🌞🔱♾️👑");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error("❌ Registration failed:");
        console.error(error);
        process.exit(1);
    });
