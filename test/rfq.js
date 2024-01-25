const {
  time,
  loadFixture,
} = require("@nomicfoundation/hardhat-toolbox/network-helpers");
const { anyValue } = require("@nomicfoundation/hardhat-chai-matchers/withArgs");
const { expect } = require("chai");

describe("OtomicMarket", function () {
  async function deployRFQ() {
    const RFQFactory = await ethers.getContractFactory("OtomicMarket");
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
    it("Full exchange process", async function () {
      const [owner, exchangeCreator, buyer] = await ethers.getSigners();
      const { rfq, requestLiveTime } = await deployRFQ();
      const { token: tokenA } = await deployToken(exchangeCreator, 10);
      const { token: tokenB } = await deployToken(buyer, 10);
      
      tokenA.connect(exchangeCreator).approve(rfq.target, 10);
      let createTx = await rfq.connect(exchangeCreator).submitRequest(tokenA.target, tokenB.target, 10);
      let receipt = await createTx.wait();
      let requestId = receipt.logs[1].args[1];
      tokenB.connect(buyer).approve(rfq.target, 10);
      let bidTx = await rfq.connect(buyer).submitResponse(requestId, 10);
      receipt = await bidTx.wait();
      let buyerId = receipt.logs[1].args[0];
      await (await rfq.connect(exchangeCreator).acceptBid(requestId, buyerId)).wait();
      await (await rfq.connect(exchangeCreator)["withdraw(uint256)"](requestId)).wait();
      await (await rfq.connect(buyer)["withdraw(uint256,uint256)"](requestId, buyerId)).wait();
      
      expect(await tokenA.balanceOf(buyer)).to.equal(10);
      expect(await tokenB.balanceOf(exchangeCreator)).to.equal(10);
    });

  });
});
