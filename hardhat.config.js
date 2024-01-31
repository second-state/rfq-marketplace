require("@nomicfoundation/hardhat-toolbox");

const priKey1 = undefined; // Please provide your private key to deploy contract.

module.exports = {
  solidity: "0.8.20",
  networks: {
    cybermiles: {
      url: "https://mainnet.cybermiles.io",
      gas: 1000000000,
      accounts: (priKey1 ? [priKey1] : [])
    }
  },
};
