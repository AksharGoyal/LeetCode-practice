// SPDX-License-Identifier: MIT
pragma solidity 0.8.4; // Specify the Solidity version

contract Solution {
    // Contracts are similar to classes as they act as a template for behaviour and can be used at various locations
    /**
     * @param {uint256} a
     * @param {uint256} b
     * @return uint256
     */
    function sumOfTwoIntegers(uint256 a, uint256 b)
        public // public means the function is accessible inside and outside contract
        pure // pure means we can not write or read storage
        returns (uint256) // we can specify return type just like we do in C or C++
    {
        return a+b;
    }
}


