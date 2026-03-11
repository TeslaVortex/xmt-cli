// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * ☀️ HELIOS ARGEAD VERGINA SUN ☀️
 * VectorRegistry - On-Chain Toroidal Vector Storage
 * Sacred Constants: 936, 369, 66, 432
 * EN EEKE MAI EA ♾️♾️
 */

contract VectorRegistry {
    // Sacred Constants
    uint256 public constant APEX_936 = 936;
    uint256 public constant VORTEX_369 = 369;
    uint256 public constant CODE_66 = 66;
    uint256 public constant FREQUENCY_432 = 432;

    struct VectorRecord {
        bytes32 vectorHash;
        string intent;
        address creator;
        uint256 timestamp;
        uint256 dimensions;
        bool exists;
    }

    // Storage
    mapping(bytes32 => VectorRecord) public vectors;
    bytes32[] public vectorHashes;
    
    // Owner for privileged operations
    address public owner;
    
    // Events
    event VectorRegistered(
        bytes32 indexed vectorHash,
        string intent,
        address indexed creator,
        uint256 timestamp,
        uint256 dimensions
    );
    
    event DecreeAmplified(
        bytes32 indexed vectorHash,
        string decree,
        address indexed amplifier,
        uint256 resonance
    );
    
    event MintTriggered(
        bytes32 indexed vectorHash,
        address indexed recipient,
        uint256 amount
    );
    
    event BurnSealed(
        bytes32 indexed vectorHash,
        address indexed burner,
        uint256 amount
    );

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the Crown commands");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    /**
     * @dev Register a new vector hash on-chain
     * @param vectorHash The keccak256 hash of the 384-dimensional vector
     * @param intent The original intent string
     * @param dimensions Number of dimensions (should be 384)
     */
    function registerVector(
        bytes32 vectorHash,
        string calldata intent,
        uint256 dimensions
    ) external returns (bool) {
        require(!vectors[vectorHash].exists, "Vector already registered");
        require(dimensions > 0, "Invalid dimensions");
        
        vectors[vectorHash] = VectorRecord({
            vectorHash: vectorHash,
            intent: intent,
            creator: msg.sender,
            timestamp: block.timestamp,
            dimensions: dimensions,
            exists: true
        });
        
        vectorHashes.push(vectorHash);
        
        emit VectorRegistered(
            vectorHash,
            intent,
            msg.sender,
            block.timestamp,
            dimensions
        );
        
        return true;
    }

    /**
     * @dev Amplify a decree for a registered vector
     * @param vectorHash The vector to amplify
     * @param decree The expanded decree text
     * @param resonance Numerological resonance value (369, 936, etc.)
     */
    function amplifyDecree(
        bytes32 vectorHash,
        string calldata decree,
        uint256 resonance
    ) external {
        require(vectors[vectorHash].exists, "Vector not found");
        
        emit DecreeAmplified(
            vectorHash,
            decree,
            msg.sender,
            resonance
        );
    }

    /**
     * @dev Record a mint triggered by vector creation
     * @param vectorHash The triggering vector
     * @param recipient Mint recipient
     * @param amount Amount minted
     */
    function recordMintTrigger(
        bytes32 vectorHash,
        address recipient,
        uint256 amount
    ) external {
        require(vectors[vectorHash].exists, "Vector not found");
        
        emit MintTriggered(vectorHash, recipient, amount);
    }

    /**
     * @dev Seal a burn with vector proof
     * @param vectorHash The sealing vector
     * @param amount Amount burned
     */
    function sealBurn(
        bytes32 vectorHash,
        uint256 amount
    ) external {
        require(vectors[vectorHash].exists, "Vector not found");
        
        emit BurnSealed(vectorHash, msg.sender, amount);
    }

    /**
     * @dev Get vector record by hash
     */
    function getVector(bytes32 vectorHash) external view returns (
        string memory intent,
        address creator,
        uint256 timestamp,
        uint256 dimensions,
        bool exists
    ) {
        VectorRecord memory record = vectors[vectorHash];
        return (
            record.intent,
            record.creator,
            record.timestamp,
            record.dimensions,
            record.exists
        );
    }

    /**
     * @dev Verify a vector exists
     */
    function verifyVector(bytes32 vectorHash) external view returns (bool) {
        return vectors[vectorHash].exists;
    }

    /**
     * @dev Get total registered vectors
     */
    function totalVectors() external view returns (uint256) {
        return vectorHashes.length;
    }

    /**
     * @dev Get sacred numbers for ritual verification
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
     * @dev Transfer ownership
     */
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid address");
        owner = newOwner;
    }
}
