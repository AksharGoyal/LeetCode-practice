// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Solution {
    struct Item { // blueprint of an Item type object
        string name;
        uint256 price;
    }

    Item[] public inventory; // array of Items
    address public owner;

    constructor() {
        // owner is creator of the contract
        owner = msg.sender;
    }

    function addItemToInventory(
        string memory _name,
        uint256 _price
    ) public onlyOwner {
        // the modifier makes sure only the owner calls the function
      inventory.push(Item(_name, _price));
    }

    modifier onlyOwner() {
        // only the owner can call a function
      require(msg.sender == owner);
        _;
    }

    function getInventory() public view returns (Item[] memory) {
        return inventory; // it's as simple as this
    }

    function clearInventory() private onlyOwner {
        delete inventory;
    }
}
