// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract NFTLootBox is ERC721URIStorage, Ownable(msg.sender) {
    uint256 private _tokenIds;
    IERC20 token;
    uint256 nftPrice;

    constructor() ERC721("MyNFTs", "MNFT") {
        nftPrice = 20;
    }

    function mintNFT(
        address recipient,
        string memory tokenURI,
        address serverAddress
    ) public returns (uint256) {
        require(token.balanceOf(recipient) > nftPrice);

        token.transferFrom(recipient, serverAddress, nftPrice);

        _tokenIds += 1;
        uint256 newItemId = _tokenIds;
        _mint(recipient, newItemId);
        _setTokenURI(newItemId, tokenURI);

        return newItemId;
    }

    function setToken(address tokenAddress) public onlyOwner returns (bool) {
        require(tokenAddress != address(0x0));
        token = IERC20(tokenAddress);
        return true;
    }
}
