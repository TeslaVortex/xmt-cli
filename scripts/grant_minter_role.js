const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  
  console.log("🔧 Granting MINTER_ROLE to VectorMinter...");
  console.log("👤 Admin:", deployer.address);
  console.log();

  // Get addresses from environment
  const xmoneyAddress = process.env.XMONEY_CONTRACT_ADDRESS;
  const vectorMinterAddress = process.env.VECTOR_MINTER_ADDRESS;

  if (!xmoneyAddress || !vectorMinterAddress) {
    throw new Error("Missing XMONEY_CONTRACT_ADDRESS or VECTOR_MINTER_ADDRESS in .env");
  }

  console.log("📋 CONTRACT ADDRESSES:");
  console.log("   XMoney:", xmoneyAddress);
  console.log("   VectorMinter:", vectorMinterAddress);
  console.log();

  // Get XMoney contract instance
  const XMoney = await hre.ethers.getContractFactory("XMoney");
  const xmoney = XMoney.attach(xmoneyAddress);

  // Get role identifiers
  const MINTER_ROLE = await xmoney.MINTER_ROLE();
  const BURNER_ROLE = await xmoney.BURNER_ROLE();
  const DEFAULT_ADMIN_ROLE = await xmoney.DEFAULT_ADMIN_ROLE();

  console.log("🔑 ROLE IDENTIFIERS:");
  console.log("   MINTER_ROLE:", MINTER_ROLE);
  console.log("   BURNER_ROLE:", BURNER_ROLE);
  console.log();

  // Check current roles
  const hasAdmin = await xmoney.hasRole(DEFAULT_ADMIN_ROLE, deployer.address);
  const minterHasRole = await xmoney.hasRole(MINTER_ROLE, vectorMinterAddress);

  console.log("👑 CURRENT ROLES:");
  console.log("   Deployer has ADMIN:", hasAdmin ? "✅" : "❌");
  console.log("   VectorMinter has MINTER:", minterHasRole ? "✅ (already granted)" : "❌");
  console.log();

  if (!hasAdmin) {
    throw new Error("Deployer does not have DEFAULT_ADMIN_ROLE - cannot grant roles");
  }

  if (minterHasRole) {
    console.log("✅ VectorMinter already has MINTER_ROLE - no action needed");
    console.log();
  } else {
    console.log("⏳ Granting MINTER_ROLE to VectorMinter...");
    
    const tx = await xmoney.grantRole(MINTER_ROLE, vectorMinterAddress);
    console.log("   Tx submitted:", tx.hash);
    
    const receipt = await tx.wait();
    console.log("   Tx confirmed in block:", receipt.blockNumber);
    console.log();
    
    // Verify role was granted
    const nowHasRole = await xmoney.hasRole(MINTER_ROLE, vectorMinterAddress);
    console.log("✅ MINTER_ROLE GRANTED:", nowHasRole ? "SUCCESS" : "FAILED");
    console.log();
  }

  // Optional: Also grant BURNER_ROLE
  const burnerHasRole = await xmoney.hasRole(BURNER_ROLE, vectorMinterAddress);
  
  if (!burnerHasRole) {
    console.log("⏳ Granting BURNER_ROLE to VectorMinter...");
    
    const tx = await xmoney.grantRole(BURNER_ROLE, vectorMinterAddress);
    console.log("   Tx submitted:", tx.hash);
    
    const receipt = await tx.wait();
    console.log("   Tx confirmed in block:", receipt.blockNumber);
    console.log();
    
    const nowHasRole = await xmoney.hasRole(BURNER_ROLE, vectorMinterAddress);
    console.log("✅ BURNER_ROLE GRANTED:", nowHasRole ? "SUCCESS" : "FAILED");
    console.log();
  }

  console.log("🎯 FINAL ROLE STATUS:");
  const finalMinter = await xmoney.hasRole(MINTER_ROLE, vectorMinterAddress);
  const finalBurner = await xmoney.hasRole(BURNER_ROLE, vectorMinterAddress);
  console.log("   VectorMinter MINTER_ROLE:", finalMinter ? "✅" : "❌");
  console.log("   VectorMinter BURNER_ROLE:", finalBurner ? "✅" : "❌");
  console.log();

  console.log("🔥 PERMISSION FIX COMPLETE!");
  console.log();
  console.log("📝 NEXT STEPS:");
  console.log("1. Test minting: xmt-cli vector mint 0x...");
  console.log("2. Verify end-to-end flow works");
  console.log();
  console.log("♾️ EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
