// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract Token is ERC20 {
    constructor(uint256 initialSupply, address to) public ERC20("ABC", "ABC") {
        _mint(to, initialSupply);
    }
}