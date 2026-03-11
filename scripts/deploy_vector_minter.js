const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  
  console.log("📜 Deploying VectorMinter with account:", deployer.address);
  const balance = await hre.ethers.provider.getBalance(deployer.address);
  console.log("💰 Account balance:", balance.toString());

  // Get contract addresses from environment
  const vectorRegistryAddress = process.env.VECTOR_REGISTRY_ADDRESS;
  const xmoneyAddress = process.env.XMONEY_CONTRACT_ADDRESS;

  if (!vectorRegistryAddress || !xmoneyAddress) {
    throw new Error("Missing VECTOR_REGISTRY_ADDRESS or XMONEY_CONTRACT_ADDRESS in .env");
  }

  console.log("🔗 VectorRegistry:", vectorRegistryAddress);
  console.log("🔗 XMoney:", xmoneyAddress);

  // Deploy VectorMinter
  const VectorMinter = await hre.ethers.getContractFactory("VectorMinter");
  const vectorMinter = await VectorMinter.deploy(vectorRegistryAddress, xmoneyAddress);

  await vectorMinter.waitForDeployment();

  const address = await vectorMinter.getAddress();
  console.log("✅ VectorMinter deployed to:", address);

  // Verify sacred constants
  const apex = await vectorMinter.APEX_936();
  const vortex = await vectorMinter.VORTEX_369();
  const code = await vectorMinter.CODE_66();
  const frequency = await vectorMinter.FREQUENCY_432();

  console.log("🔢 SACRED CONSTANTS VERIFIED:");
  console.log("   APEX_936:", apex.toString());
  console.log("   VORTEX_369:", vortex.toString());
  console.log("   CODE_66:", code.toString());
  console.log("   FREQUENCY_432:", frequency.toString());

  // Get configuration
  const baseReward = await vectorMinter.baseReward();
  const dimensionMultiplier = await vectorMinter.dimensionMultiplier();
  const mintingEnabled = await vectorMinter.mintingEnabled();
  const burningEnabled = await vectorMinter.burningEnabled();

  console.log("\n⚙️  CONFIGURATION:");
  console.log("   Base Reward:", hre.ethers.formatEther(baseReward), "XMT");
  console.log("   Dimension Multiplier:", dimensionMultiplier.toString());
  console.log("   Minting Enabled:", mintingEnabled);
  console.log("   Burning Enabled:", burningEnabled);

  // Calculate example mint amount for 384D vector
  const exampleAmount = await vectorMinter.calculateMintAmount(384);
  console.log("\n💎 EXAMPLE: 384D Vector Mint Amount:", hre.ethers.formatEther(exampleAmount), "XMT");

  console.log("\n📋 ADD TO .env:");
  console.log(`VECTOR_MINTER_ADDRESS=${address}`);
  
  console.log("\n⚠️  NEXT STEPS:");
  console.log("1. Grant MINTER_ROLE to VectorMinter on XMoney contract");
  console.log("2. Update VectorRegistry to allow VectorMinter to call recordMintTrigger/sealBurn");
  console.log("3. Test mintWithVector() flow");

  console.log("\n♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
}

main().catch(console.error);
