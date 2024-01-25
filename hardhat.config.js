require("@nomicfoundation/hardhat-toolbox");

// const priKey1 = "Your private key.";
const priKey1 = process.env.PRIKEY1

module.exports = {
  solidity: "0.8.20",
  networks: {
    cybermiles: {
      url: "https://mainnet.cybermiles.io",
      gas: 1000000000,
      accounts:[priKey1]
    }
  },
};
