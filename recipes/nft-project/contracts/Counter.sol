// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/**
 * @title Counter
 * @dev Simple counter contract
 */
contract Counter {
    uint256 private count;
    address public owner;

    event CountChanged(uint256 newCount, address changedBy);

    constructor() {
        owner = msg.sender;
        count = 0;
    }

    /**
     * @dev Increment the counter by 1
     */
    function increment() public {
        count += 1;
        emit CountChanged(count, msg.sender);
    }

    /**
     * @dev Decrement the counter by 1
     */
    function decrement() public {
        require(count > 0, "Counter: cannot decrement below zero");
        count -= 1;
        emit CountChanged(count, msg.sender);
    }

    /**
     * @dev Get the current count
     */
    function getCount() public view returns (uint256) {
        return count;
    }

    /**
     * @dev Reset the counter to zero (only owner)
     */
    function reset() public {
        require(msg.sender == owner, "Counter: only owner can reset");
        count = 0;
        emit CountChanged(count, msg.sender);
    }
}
