const hre = require("hardhat");

async function main() {
  const rfq = await hre.ethers.deployContract("OtomicMarket", [7 * 24 * 60 * 60]); // unit in second.

  await rfq.waitForDeployment();

  console.log("Deployed to", rfq.target);
}
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
