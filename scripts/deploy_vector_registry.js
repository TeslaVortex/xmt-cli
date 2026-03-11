// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// VectorRegistry Deployment Script
// Deploy to Sepolia testnet
// EN EEKE MAI EA ♾️♾️

const hre = require("hardhat");

async function main() {
  console.log("🔮 DEPLOYING VECTORREGISTRY CONTRACT");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

  const [deployer] = await hre.ethers.getSigners();
  console.log("📜 Deploying with account:", deployer.address);
  const balance = await hre.ethers.provider.getBalance(deployer.address);
  console.log("💰 Account balance:", balance.toString());

  const VectorRegistry = await hre.ethers.getContractFactory("VectorRegistry");
  const vectorRegistry = await VectorRegistry.deploy();

  await vectorRegistry.waitForDeployment();

  const address = await vectorRegistry.getAddress();
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
  console.log("✅ VectorRegistry deployed to:", address);
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
  
  // Verify sacred constants
  const apex = await vectorRegistry.APEX_936();
  const vortex = await vectorRegistry.VORTEX_369();
  const code = await vectorRegistry.CODE_66();
  const frequency = await vectorRegistry.FREQUENCY_432();
  
  console.log("\n🔢 SACRED CONSTANTS VERIFIED:");
  console.log("   APEX_936:", apex.toString());
  console.log("   VORTEX_369:", vortex.toString());
  console.log("   CODE_66:", code.toString());
  console.log("   FREQUENCY_432:", frequency.toString());
  
  console.log("\n📋 ADD TO .env:");
  console.log(`VECTOR_REGISTRY_ADDRESS=${address}`);
  console.log("\n♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
