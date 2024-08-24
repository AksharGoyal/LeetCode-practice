pragma solidity ^0.8.4;

contract Gas {
    uint public c = 1; // the public variable is readable and writable

    function calculateGas() external returns(uint _gasUsed) {
      uint start = gasleft(); // gasleft gives you the amount of gas remaining in the transaction (something around 471000)
        ++c; // we do the operation
      uint end = gasleft(); // we get the remaining gas after the operation
      _gasUsed = start - end; // we calculate the overall gas used
    }
}