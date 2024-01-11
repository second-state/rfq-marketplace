require("@nomicfoundation/hardhat-toolbox");

const priKey1 = "Your private key.";

module.exports = {
  solidity: "0.8.20",
  networks: {
    cyber: {
      url: "https://mainnet.cybermiles.io",
      gas: 1000000000,
      accounts:[priKey1]
    }
  },
};
