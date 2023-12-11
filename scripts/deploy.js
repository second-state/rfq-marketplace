const hre = require("hardhat");

async function main() {
  const rfq = await hre.ethers.deployContract("RFQ");

  await rfq.waitForDeployment();

  console.log("Deployed to ", rfq.target);
}
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
