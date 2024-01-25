// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import './IERC20.sol';
import './OpenZeppelin_v4_9_0/openzeppelin-contracts/contracts/access/Ownable.sol';

/**
 * @title OtomicMarket
 * @dev OtomicMarket logic
 */
contract OtomicMarket is Ownable {
    
    event exchangeEvent(address indexed owner, uint indexed requestId, address tokenA, address tokenB, uint amount);
    event bidEvent(uint indexed responseId, address buyer, uint indexed requestId, uint amount);

    // Buyer detai;
    struct depositInfo {
        address owner;
        uint amount;
    }
    
    // Exchange require detail
    struct requestInfo {
        address owner;
        address tokenA;
        address tokenB; 
        uint lockAmount;
        uint depositSize;
        uint expireTime;
        bool finish;
        uint buyer;
    }
    
    
    requestInfo[] private requestList;
    uint private requestLiveTime;
    mapping(uint => mapping(uint => depositInfo)) private depositList;

    /**
    * @dev Initialize RFQ necessary parameters.
    * @param _requestLiveTime Request validity period.
    */
    constructor(uint _requestLiveTime) Ownable(msg.sender){
        requestLiveTime = _requestLiveTime;
    }

    /**
    * @dev Modifier to verify request status
    */
    modifier checkRequestStatus(uint id, bool isValid) {
        if(isValid){
            require(block.timestamp <= requestList[id].expireTime || !requestList[id].finish, "");
        } else {
            require(block.timestamp > requestList[id].expireTime || requestList[id].finish, "");
        }
        _;
    }

    /**
    * @dev Modifier to verify if user approve enough token to contract.
    */
    modifier checkApprove(address token, uint amount) {
        require(IERC20(token).allowance(msg.sender, address(this)) >= amount, "You should approve enough token.");
        _;
    }

    /**
    * @dev Set requestLiveTime
    * @param _requestLiveTime new requestLiveTime value
    */
    function setRequestLiveTime(uint _requestLiveTime) external onlyOwner() {
        requestLiveTime = _requestLiveTime;
    }

    /**
    * @dev Get requestLiveTime
    * @return requestLiveTime value
    */
    function getRequestLiveTime() external view returns(uint) {
        return requestLiveTime;
    }

    /**
    * @dev Create an exchange request.
    * @param tokenA ERC 20 contract address that the caller wants to swap out.
    * @param tokenB ERC 20 contract address that the caller wants to swap in.
    * @param amount Amount of tokenA that the caller wants to swap out.
    * @return Request id.
    */
    function submitRequest(address tokenA, address tokenB, uint amount) external checkApprove(tokenA, amount) returns(uint) {
        IERC20(tokenA).transferFrom(msg.sender, address(this), amount);
        uint requestId = requestList.length;
        requestList.push();
        requestInfo storage newRequest = requestList[requestId];
        newRequest.owner = msg.sender;
        newRequest.tokenA = tokenA;
        newRequest.tokenB = tokenB;
        newRequest.lockAmount = amount;
        newRequest.expireTime = block.timestamp + requestLiveTime;
        newRequest.finish = false;
        emit exchangeEvent(msg.sender, requestId, tokenA, tokenB, amount);
        return requestId;
    }


    /**
    * @dev Response an exchange request with the amount of tokenB to buy tokenA.
    * @param requestId The exchange request id.
    * @param amount The amount of tokenB that the caller wants to swap out.
    * @return The responseId of the caller response.
    */
    function submitResponse(uint requestId, uint amount) 
    external 
    checkRequestStatus(requestId, true)
    checkApprove(requestList[requestId].tokenB, amount)
    returns(uint){
        IERC20(requestList[requestId].tokenB).transferFrom(msg.sender, address(this), amount);
        depositInfo memory newDeposit;
        newDeposit.owner = msg.sender;
        newDeposit.amount = amount;
        uint responseId = requestList[requestId].depositSize;
        requestList[requestId].depositSize += 1;
        depositList[requestId][responseId] = newDeposit;
        emit bidEvent(responseId, msg.sender, requestId, amount);
        return responseId;
    }


    /**
    * @dev Accept the reponse of the exchange request.
    * @param requestId The exchange request id.
    * @param responseId Accept the exchange with the response id, which id is the index of depositList.
    */
    function acceptBid(uint requestId, uint responseId) external checkRequestStatus(requestId, true) {
        require(requestList[requestId].owner == msg.sender , "");
        requestList[requestId].finish = true;
        requestList[requestId].buyer = responseId;
    }

    /**
    * @dev Exchange request owner whithdraw token from a ended exchange request.
    * @param requestId The exchange request id.
    */
    function withdraw(uint requestId) external checkRequestStatus(requestId, false) {
        require(requestList[requestId].owner == msg.sender, "");
        if(requestList[requestId].finish == true){
            uint responseId = requestList[requestId].buyer;
            uint amount = depositList[requestId][responseId].amount;
            depositList[requestId][responseId].amount -= amount;
            IERC20(requestList[requestId].tokenB).transfer(msg.sender, amount);
        } else {
            uint amount = requestList[requestId].lockAmount;
            requestList[requestId].lockAmount -= amount;
            IERC20(requestList[requestId].tokenA).transfer(msg.sender, amount);
        }
    }

    /**
    * @dev Exchange request buyer whithdraw token from a ended exchange request.
    * @param requestId The exchange request id.
    */
    function withdraw(uint requestId, uint responseId) external {
        require(depositList[requestId][responseId].owner == msg.sender, "");
        if(requestList[requestId].finish == true && depositList[requestId][requestList[requestId].buyer].owner == msg.sender){
            uint amount = requestList[requestId].lockAmount;
            requestList[requestId].lockAmount -= amount;
            IERC20(requestList[requestId].tokenA).transfer(msg.sender, amount);
        } else {
            uint amount = depositList[requestId][responseId].amount;
            depositList[requestId][responseId].amount -= amount;
            IERC20(requestList[requestId].tokenB).transfer(msg.sender, amount);
        }
    }

    /**
    * @dev Query request list length.
    * @return Request list length.
    */
    function getRequestLength() external view returns(uint) {
        return requestList.length;
    }

    /**
    * @dev Query request from request id.
    * @param requestId request id.
    * @return Request rquest detail.
    */
    function getRequest(uint requestId) external view returns(requestInfo memory){
        return requestList[requestId];
    }

    /**
    * @dev Query the number of buyer of exchange request.
    * @param requestId request id.
    * @return Request the number of buyer.
    */
    function getBuyerLength(uint requestId) external view returns(uint) {
        return requestList[requestId].depositSize;
    }

    /**
    * @dev Query buyer detail.
    * @param requestId request id.
    * @param responseId response id.
    * @return Buyer detail.
    */
    function getBuyer(uint requestId, uint responseId) external view returns(depositInfo memory){
        return depositList[requestId][responseId];
    }

}