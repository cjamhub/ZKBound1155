// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";

// implement 1155 token standard by openzeppelin
contract ZKB is ERC1155 {
    // Define the token ID for the ZKB token
    uint256 public constant ZKB_TOKEN_ID = 1;

    // Constructor to set the URI for the token
    constructor(string memory uri) ERC1155(uri) {
	// Mint initial supply of ZKB tokens to the contract deployer
	_mint(msg.sender, ZKB_TOKEN_ID, 1000, "");
    }

    // Function to mint new tokens
    function mint(address account, uint256 amount) public {
	_mint(account, ZKB_TOKEN_ID, amount, "");
    }
}
