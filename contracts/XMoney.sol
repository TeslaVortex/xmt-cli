// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/metatx/ERC2771Context.sol";

/**
 * @title XMoney
 * @dev X-Money Token - Sacred 369 Vortex Mathematics
 * EN EEKE MAI EA ♾️♾️
 */
contract XMoney is ERC20, AccessControl, ERC2771Context {
    
    // Role definitions
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    
    // Sacred constants
    uint256 public constant APEX_936 = 936;
    uint256 public constant VORTEX_369 = 369;
    uint256 public constant CODE_66 = 66;
    
    event TokensMinted(address indexed to, uint256 amount, uint256 timestamp);
    event TokensBurned(address indexed from, uint256 amount, uint256 timestamp);
    
    constructor(address trustedForwarder) 
        ERC20("X-Money", "XMT") 
        ERC2771Context(trustedForwarder) 
    {
        // Grant deployer all roles
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
        _grantRole(BURNER_ROLE, msg.sender);
        
        // Initial supply: 369 million tokens (369 * 10^6 * 10^18)
        _mint(msg.sender, 369_000_000 * 10**decimals());
    }
    
    /**
     * @dev Mint new tokens (requires MINTER_ROLE)
     * @param to Recipient address
     * @param amount Amount to mint (in wei)
     */
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        _mint(to, amount);
        emit TokensMinted(to, amount, block.timestamp);
    }
    
    /**
     * @dev Burn tokens (requires BURNER_ROLE)
     * @param from Address to burn from
     * @param amount Amount to burn (in wei)
     */
    function burn(address from, uint256 amount) external onlyRole(BURNER_ROLE) {
        _burn(from, amount);
        emit TokensBurned(from, amount, block.timestamp);
    }
    
    /**
     * @dev Get sacred constants
     */
    function getSacredNumbers() external pure returns (uint256 apex, uint256 vortex, uint256 code) {
        return (APEX_936, VORTEX_369, CODE_66);
    }
    
    /**
     * @dev Override _msgSender to support meta-transactions
     */
    function _msgSender() internal view virtual override(Context, ERC2771Context) returns (address) {
        return ERC2771Context._msgSender();
    }
    
    /**
     * @dev Override _msgData to support meta-transactions
     */
    function _msgData() internal view virtual override(Context, ERC2771Context) returns (bytes calldata) {
        return ERC2771Context._msgData();
    }
    
    /**
     * @dev Override _contextSuffixLength to support meta-transactions
     */
    function _contextSuffixLength() internal view virtual override(Context, ERC2771Context) returns (uint256) {
        return ERC2771Context._contextSuffixLength();
    }
}
