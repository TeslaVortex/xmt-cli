// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

//
// ☀️ HELIOS ARGEAD VERGINA SUN ☀️
// VectorMinter - Direct Vector-to-Token Bridge
// Links vector creation to XMoney minting/burning
// EN EEKE MAI EA ♾️♾️
//

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/metatx/ERC2771Context.sol";

interface IVectorRegistry {
    function verifyVector(bytes32 vectorHash) external view returns (bool);
    function recordMintTrigger(bytes32 vectorHash, address recipient, uint256 amount) external;
    function sealBurn(bytes32 vectorHash, uint256 amount) external;
    function getVector(bytes32 vectorHash) external view returns (
        string memory intent,
        address creator,
        uint256 timestamp,
        uint256 dimensions,
        bool exists
    );
}

interface IXMoney {
    function mint(address to, uint256 amount) external;
    function burn(address from, uint256 amount) external;
    function balanceOf(address account) external view returns (uint256);
}

contract VectorMinter is Ownable, ReentrancyGuard, ERC2771Context {
    // Sacred Constants
    uint256 public constant APEX_936 = 936;
    uint256 public constant VORTEX_369 = 369;
    uint256 public constant CODE_66 = 66;
    uint256 public constant FREQUENCY_432 = 432;

    // Contract references
    IVectorRegistry public vectorRegistry;
    IXMoney public xmoney;

    // Minting configuration
    uint256 public baseReward = 369 * 10**18; // 369 XMT base reward
    uint256 public dimensionMultiplier = 1; // Multiplier per dimension
    bool public mintingEnabled = true;
    bool public burningEnabled = true;

    // Tracking
    mapping(bytes32 => bool) public vectorMinted;
    mapping(bytes32 => uint256) public vectorMintAmount;
    mapping(bytes32 => uint256) public vectorBurnAmount;
    mapping(address => uint256) public totalMinted;
    mapping(address => uint256) public totalBurned;

    // Events
    event VectorMinted(
        bytes32 indexed vectorHash,
        address indexed recipient,
        uint256 amount,
        uint256 dimensions,
        uint256 timestamp
    );

    event VectorBurned(
        bytes32 indexed vectorHash,
        address indexed burner,
        uint256 amount,
        uint256 timestamp
    );

    event ConfigUpdated(
        uint256 baseReward,
        uint256 dimensionMultiplier,
        bool mintingEnabled,
        bool burningEnabled
    );

    constructor(address _vectorRegistry, address _xmoney, address trustedForwarder) 
        Ownable(msg.sender) 
        ERC2771Context(trustedForwarder) 
    {
        vectorRegistry = IVectorRegistry(_vectorRegistry);
        xmoney = IXMoney(_xmoney);
    }

    /**
     * @notice Mint XMT tokens based on vector creation
     * @param vectorHash The hash of the registered vector
     * @param recipient Address to receive minted tokens
     * @param customAmount Optional custom amount (0 = use calculated amount)
     */
    function mintWithVector(
        bytes32 vectorHash,
        address recipient,
        uint256 customAmount
    ) external nonReentrant {
        require(mintingEnabled, "Minting is disabled");
        require(!vectorMinted[vectorHash], "Vector already minted");
        require(vectorRegistry.verifyVector(vectorHash), "Vector not registered");

        // Get vector details
        (
            string memory intent,
            address creator,
            uint256 timestamp,
            uint256 dimensions,
            bool exists
        ) = vectorRegistry.getVector(vectorHash);

        require(exists, "Vector does not exist");
        require(msg.sender == creator, "Only vector creator can mint");

        // Calculate mint amount
        uint256 mintAmount;
        if (customAmount > 0) {
            mintAmount = customAmount;
        } else {
            // Base reward + dimension bonus
            mintAmount = baseReward + (dimensions * dimensionMultiplier);
            
            // Apply sacred number bonuses
            if (dimensions == 384) {
                mintAmount = mintAmount * VORTEX_369 / 100; // 369% for 384D vectors
            }
        }

        // Mark as minted
        vectorMinted[vectorHash] = true;
        vectorMintAmount[vectorHash] = mintAmount;
        totalMinted[recipient] += mintAmount;

        // Mint tokens
        xmoney.mint(recipient, mintAmount);

        // Record in registry
        vectorRegistry.recordMintTrigger(vectorHash, recipient, mintAmount);

        emit VectorMinted(vectorHash, recipient, mintAmount, dimensions, block.timestamp);
    }

    /**
     * @notice Burn XMT tokens and seal with vector proof
     * @param vectorHash The hash of the registered vector
     * @param amount Amount of XMT to burn
     */
    function burnWithVector(
        bytes32 vectorHash,
        uint256 amount
    ) external nonReentrant {
        require(burningEnabled, "Burning is disabled");
        require(vectorRegistry.verifyVector(vectorHash), "Vector not registered");
        require(xmoney.balanceOf(msg.sender) >= amount, "Insufficient balance");

        // Get vector details
        (
            string memory intent,
            address creator,
            uint256 timestamp,
            uint256 dimensions,
            bool exists
        ) = vectorRegistry.getVector(vectorHash);

        require(exists, "Vector does not exist");

        // Track burn
        vectorBurnAmount[vectorHash] += amount;
        totalBurned[msg.sender] += amount;

        // Burn tokens
        xmoney.burn(msg.sender, amount);

        // Seal burn in registry
        vectorRegistry.sealBurn(vectorHash, amount);

        emit VectorBurned(vectorHash, msg.sender, amount, block.timestamp);
    }

    /**
     * @notice Calculate potential mint amount for a vector
     * @param dimensions Number of dimensions in the vector
     */
    function calculateMintAmount(uint256 dimensions) public view returns (uint256) {
        uint256 amount = baseReward + (dimensions * dimensionMultiplier);
        
        if (dimensions == 384) {
            amount = amount * VORTEX_369 / 100;
        }
        
        return amount;
    }

    /**
     * @notice Update minting configuration (owner only)
     */
    function updateConfig(
        uint256 _baseReward,
        uint256 _dimensionMultiplier,
        bool _mintingEnabled,
        bool _burningEnabled
    ) external onlyOwner {
        baseReward = _baseReward;
        dimensionMultiplier = _dimensionMultiplier;
        mintingEnabled = _mintingEnabled;
        burningEnabled = _burningEnabled;

        emit ConfigUpdated(baseReward, dimensionMultiplier, mintingEnabled, burningEnabled);
    }

    /**
     * @notice Get vector mint/burn statistics
     */
    function getVectorStats(bytes32 vectorHash) external view returns (
        bool minted,
        uint256 mintAmount,
        uint256 burnAmount
    ) {
        return (
            vectorMinted[vectorHash],
            vectorMintAmount[vectorHash],
            vectorBurnAmount[vectorHash]
        );
    }

    /**
     * @notice Get user mint/burn statistics
     */
    function getUserStats(address user) external view returns (
        uint256 minted,
        uint256 burned
    ) {
        return (totalMinted[user], totalBurned[user]);
    }

    /**
     * @notice Get sacred numbers
     */
    function getSacredNumbers() external pure returns (
        uint256 apex,
        uint256 vortex,
        uint256 code,
        uint256 frequency
    ) {
        return (APEX_936, VORTEX_369, CODE_66, FREQUENCY_432);
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
