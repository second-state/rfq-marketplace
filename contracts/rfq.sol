// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import './IERC20.sol';
import './OpenZeppelin_v4_9_0/openzeppelin-contracts/contracts/access/Ownable.sol';

/**
 * @title OtomicMarket
 * @dev OtomicMarket logic
 */
contract OtomicMarket is Ownable {
    
    event exchangeEvent(address indexed owner, uint indexed requestId, address tokenOut, address tokenIn, uint amount, uint expireTime);
    event bidEvent(uint indexed responseId, address buyer, uint indexed requestId, uint amount, uint expireTime);

    // Buyer detai;
    struct depositInfo {
        address owner;
        uint amount;
        uint expireTime;
    }
    
    // Exchange require detail
    struct requestInfo {
        address owner;
        address tokenOut;
        address tokenIn; 
        uint lockAmount;
        uint depositSize;
        uint expireTime;
        bool finish;
        uint buyer;
    }
    
    
    requestInfo[] private requestList;
    uint private requestLifetime;
    mapping(uint => mapping(uint => depositInfo)) private depositList;

    /**
    * @dev Initialize RFQ necessary parameters.
    * @param _requestLifetime Request validity period.
    */
    constructor(uint _requestLifetime) Ownable(msg.sender){
        requestLifetime = _requestLifetime;
    }

    /**
    * @dev Modifier to verify request status
    */
    modifier checkRequestStatus(uint id, bool isValid) {
        if(isValid){
            require(block.timestamp <= requestList[id].expireTime || !requestList[id].finish, "This request is not valid.");
        } else {
            require(block.timestamp > requestList[id].expireTime || requestList[id].finish, "This request is still valid.");
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
    * @dev Set requestLifetime
    * @param _requestLifetime new requestLifetime value
    */
    function setRequestLifetime(uint _requestLifetime) external onlyOwner() {
        requestLifetime = _requestLifetime;
    }

    /**
    * @dev Get requestLifetime
    * @return requestLifetime value
    */
    function getRequestLifetime() external view returns(uint) {
        return requestLifetime;
    }

    /**
    * @dev Create an exchange request.
    * @param tokenOut ERC 20 contract address that the caller wants to swap out.
    * @param tokenIn ERC 20 contract address that the caller wants to swap in.
    * @param amount Amount of tokenOut that the caller wants to swap out.
    * @return Request id.
    */
    function submitRequest(address tokenOut, address tokenIn, uint amount, uint lifetime) external checkApprove(tokenOut, amount) returns(uint) {
        IERC20(tokenOut).transferFrom(msg.sender, address(this), amount);
        uint requestId = requestList.length;
        requestList.push();
        requestInfo storage newRequest = requestList[requestId];
        newRequest.owner = msg.sender;
        newRequest.tokenOut = tokenOut;
        newRequest.tokenIn = tokenIn;
        newRequest.lockAmount = amount;
        if(lifetime == 0){
            newRequest.expireTime = block.timestamp + requestLifetime;
        }else{
            newRequest.expireTime = block.timestamp + lifetime;
        }
        newRequest.finish = false;
        emit exchangeEvent(msg.sender, requestId, tokenOut, tokenIn, amount, newRequest.expireTime);
        return requestId;
    }


    /**
    * @dev Response an exchange request with the amount of tokenIn to buy tokenOut.
    * @param requestId The exchange request id.
    * @param amount The amount of tokenIn that the caller wants to swap out.
    * @param lifetime The response lifetime.
    * @return The responseId of the caller response.
    */
    function submitResponse(uint requestId, uint amount, uint lifetime) 
    external 
    checkRequestStatus(requestId, true)
    checkApprove(requestList[requestId].tokenIn, amount)
    returns(uint){
        IERC20(requestList[requestId].tokenIn).transferFrom(msg.sender, address(this), amount);
        depositInfo memory newDeposit;
        newDeposit.owner = msg.sender;
        newDeposit.amount = amount;
        if(lifetime == 0){
            newDeposit.expireTime = requestList[requestId].expireTime;
        }else{
            newDeposit.expireTime = block.timestamp + lifetime;
        }
        uint responseId = requestList[requestId].depositSize;
        requestList[requestId].depositSize += 1;
        depositList[requestId][responseId] = newDeposit;
        emit bidEvent(responseId, msg.sender, requestId, amount, newDeposit.expireTime);
        return responseId;
    }


    /**
    * @dev Accept the reponse of the exchange request.
    * @param requestId The exchange request id.
    * @param responseId Accept the exchange with the response id, which id is the index of depositList.
    */
    function acceptBid(uint requestId, uint responseId) external checkRequestStatus(requestId, true) {
        require(requestList[requestId].owner == msg.sender , "You are not the reequest owner.");
        require(block.timestamp <= depositList[requestId][responseId].expireTime , "This response has expired.");
        requestList[requestId].finish = true;
        requestList[requestId].buyer = responseId;
    }

    /**
    * @dev Exchange request owner whithdraw token from a ended exchange request.
    * @param requestId The exchange request id.
    */
    function withdraw(uint requestId) external checkRequestStatus(requestId, false) {
        require(requestList[requestId].owner == msg.sender, "You are not the reequest owner.");
        if(requestList[requestId].finish == true){
            uint responseId = requestList[requestId].buyer;
            uint amount = depositList[requestId][responseId].amount;
            depositList[requestId][responseId].amount -= amount;
            IERC20(requestList[requestId].tokenIn).transfer(msg.sender, amount);
        } else {
            uint amount = requestList[requestId].lockAmount;
            requestList[requestId].lockAmount -= amount;
            IERC20(requestList[requestId].tokenOut).transfer(msg.sender, amount);
        }
    }

    /**
    * @dev Exchange request buyer whithdraw token from a ended exchange request.
    * @param requestId The exchange request id.
    */
    function withdraw(uint requestId, uint responseId) external {
        require(depositList[requestId][responseId].owner == msg.sender, "You are not the response owner.");
        require(block.timestamp > requestList[requestId].expireTime || requestList[requestId].finish || block.timestamp > depositList[requestId][responseId].expireTime, "This request or response is still valid.");
        if(requestList[requestId].finish == true && depositList[requestId][requestList[requestId].buyer].owner == msg.sender){
            uint amount = requestList[requestId].lockAmount;
            requestList[requestId].lockAmount -= amount;
            IERC20(requestList[requestId].tokenOut).transfer(msg.sender, amount);
        } else {
            uint amount = depositList[requestId][responseId].amount;
            depositList[requestId][responseId].amount -= amount;
            IERC20(requestList[requestId].tokenIn).transfer(msg.sender, amount);
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