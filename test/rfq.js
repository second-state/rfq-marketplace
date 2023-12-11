const {
  time,
  loadFixture,
} = require("@nomicfoundation/hardhat-toolbox/network-helpers");
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { expect } = require("chai");

describe("RFQ", function () {
  async function deployRFQ() {
    const RFQFactory = await ethers.getContractFactory("RFQ");
    const requestLiveTime = 10 * 24 * 60 * 60 // 10 day in second
    const rfq = await RFQFactory.deploy(requestLiveTime);

    return { rfq, requestLiveTime };
  }

  async function deployToken(to, amount) {
    const tokenFactory = await ethers.getContractFactory("Token");
    const token = await tokenFactory.deploy(amount, to);

    return { token };
  }

  describe("Deployment", function () {
    it("Create exchange process", async function () {
      const [owner, exchangeCreator, buyer] = await ethers.getSigners();
      const { rfq, requestLiveTime } = await deployRFQ();
      const { token: tokenA } = await deployToken(exchangeCreator, 10);
      const { token: tokenB } = await deployToken(buyer, 10);
      
      tokenA.connect(exchangeCreator).approve(rfq.target, 10);
      let requestId = await rfq.connect(exchangeCreator).createExchange(tokenA.target, tokenB.target, 10);
      console.log(requestId)
      tokenB.connect(buyer).approve(rfq.target, 10);
      let buyerId = await rfq.connect(buyer).bidToken(requestId, 10);
      await (await rfq.acceptBid(requestId, buyerId)).wait();

      await (await rfq.connect(exchangeCreator).withdraw(requestId)).wait();
      await (await rfq.connect(buyer).acceptBid(requestId, buyerId)).wait();

      expect(await tokenA.balance(buyer)).to.equal(10);
      expect(await tokenB.balance(exchangeCreator)).to.equal(10);
    });

  });
});
