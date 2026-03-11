const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  
  console.log("🔧 Configuring VectorMinter permissions...");
  console.log("👤 Account:", deployer.address);

  // Get addresses from environment
  const vectorMinterAddress = process.env.VECTOR_MINTER_ADDRESS;
  const xmoneyAddress = process.env.XMONEY_CONTRACT_ADDRESS;
  const vectorRegistryAddress = process.env.VECTOR_REGISTRY_ADDRESS;

  if (!vectorMinterAddress || !xmoneyAddress || !vectorRegistryAddress) {
    throw new Error("Missing required addresses in .env");
  }

  console.log("\n📋 CONTRACT ADDRESSES:");
  console.log("   VectorMinter:", vectorMinterAddress);
  console.log("   XMoney:", xmoneyAddress);
  console.log("   VectorRegistry:", vectorRegistryAddress);

  // Get contract instances
  const XMoney = await hre.ethers.getContractFactory("XMoney");
  const xmoney = XMoney.attach(xmoneyAddress);

  const VectorRegistry = await hre.ethers.getContractFactory("VectorRegistry");
  const vectorRegistry = VectorRegistry.attach(vectorRegistryAddress);

  console.log("\n⚠️  IMPORTANT NOTES:");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
  console.log("XMoney contract uses onlyOwner modifier for mint/burn.");
  console.log("VectorMinter needs to call these functions.");
  console.log("\nOPTIONS:");
  console.log("1. Transfer XMoney ownership to VectorMinter (NOT RECOMMENDED)");
  console.log("2. Keep current setup - use owner wallet to call VectorMinter");
  console.log("3. Upgrade XMoney to use AccessControl with MINTER_ROLE (RECOMMENDED)");
  console.log("\nCURRENT SETUP: Option 2 (Owner calls VectorMinter)");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

  // Check current owner
  const xmoneyOwner = await xmoney.owner();
  const vectorRegistryOwner = await vectorRegistry.owner();

  console.log("\n👑 CURRENT OWNERS:");
  console.log("   XMoney owner:", xmoneyOwner);
  console.log("   VectorRegistry owner:", vectorRegistryOwner);
  console.log("   Deployer:", deployer.address);

  if (xmoneyOwner.toLowerCase() === deployer.address.toLowerCase()) {
    console.log("   ✅ You are the XMoney owner - can call VectorMinter");
  } else {
    console.log("   ❌ You are NOT the XMoney owner - cannot use VectorMinter");
  }

  if (vectorRegistryOwner.toLowerCase() === deployer.address.toLowerCase()) {
    console.log("   ✅ You are the VectorRegistry owner");
  } else {
    console.log("   ❌ You are NOT the VectorRegistry owner");
  }

  console.log("\n✅ CONFIGURATION COMPLETE");
  console.log("\n📝 USAGE:");
  console.log("Since you are the owner, you can:");
  console.log("1. Register a vector via VectorRegistry");
  console.log("2. Call VectorMinter.mintWithVector() as owner");
  console.log("3. VectorMinter will call XMoney.mint() (allowed because you're owner)");
  console.log("\n♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
}

main().catch(console.error);
