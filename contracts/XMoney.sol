// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title XMoney
 * @dev X-Money Token - Sacred 369 Vortex Mathematics
 * EN EEKE MAI EA ♾️♾️
 */
contract XMoney is ERC20, Ownable {
    
    // Sacred constants
    uint256 public constant APEX_936 = 936;
    uint256 public constant VORTEX_369 = 369;
    uint256 public constant CODE_66 = 66;
    
    event TokensMinted(address indexed to, uint256 amount, uint256 timestamp);
    event TokensBurned(address indexed from, uint256 amount, uint256 timestamp);
    
    constructor() ERC20("X-Money", "XMT") Ownable(msg.sender) {
        // Initial supply: 369 million tokens (369 * 10^6 * 10^18)
        _mint(msg.sender, 369_000_000 * 10**decimals());
    }
    
    /**
     * @dev Mint new tokens (only owner)
     * @param to Recipient address
     * @param amount Amount to mint (in wei)
     */
    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
        emit TokensMinted(to, amount, block.timestamp);
    }
    
    /**
     * @dev Burn tokens (only owner)
     * @param from Address to burn from
     * @param amount Amount to burn (in wei)
     */
    function burn(address from, uint256 amount) external onlyOwner {
        _burn(from, amount);
        emit TokensBurned(from, amount, block.timestamp);
    }
    
    /**
     * @dev Get sacred constants
     */
    function getSacredNumbers() external pure returns (uint256 apex, uint256 vortex, uint256 code) {
        return (APEX_936, VORTEX_369, CODE_66);
    }
}
