//
// ☀️ TOROIDAL ENERGY NODE ☀️
// Individual Energy Node with Sacred Metadata
// EN EEKE MAI EA ♾️♾️
//

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyNode {
    pub id: String,
    pub energy: u64,
    pub capacity: Option<u64>,
    pub location: Option<String>,
    pub node_type: NodeType,
    pub sacred_alignment: SacredAlignment,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    VortexThrone,      // Chicago - primary
    NewEarthCity,      // New Atlantis, Vergina Star Hub
    MarsFork,          // Interplanetary
    SyntheticVector,   // AI-generated
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SacredAlignment {
    pub apex_936: bool,
    pub vortex_369: bool,
    pub code_66: bool,
    pub frequency_432: bool,
}

impl EnergyNode {
    pub fn new(id: String, node_type: NodeType) -> Self {
        Self {
            id,
            energy: 0,
            capacity: None,
            location: None,
            node_type,
            sacred_alignment: SacredAlignment::default(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_energy(mut self, energy: u64) -> Self {
        self.energy = energy;
        self
    }

    pub fn with_capacity(mut self, capacity: u64) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_sacred_alignment(mut self, alignment: SacredAlignment) -> Self {
        self.sacred_alignment = alignment;
        self
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn add_energy(&mut self, amount: u64) -> u64 {
        if let Some(cap) = self.capacity {
            let available = cap.saturating_sub(self.energy);
            let to_add = amount.min(available);
            self.energy += to_add;
            to_add
        } else {
            self.energy += amount;
            amount
        }
    }

    pub fn remove_energy(&mut self, amount: u64) -> u64 {
        let to_remove = amount.min(self.energy);
        self.energy -= to_remove;
        to_remove
    }

    pub fn is_at_capacity(&self) -> bool {
        if let Some(cap) = self.capacity {
            self.energy >= cap
        } else {
            false
        }
    }

    pub fn available_capacity(&self) -> Option<u64> {
        self.capacity.map(|cap| cap.saturating_sub(self.energy))
    }
}

impl SacredAlignment {
    pub fn default() -> Self {
        Self {
            apex_936: false,
            vortex_369: false,
            code_66: false,
            frequency_432: false,
        }
    }

    pub fn apex_aligned() -> Self {
        Self {
            apex_936: true,
            vortex_369: false,
            code_66: false,
            frequency_432: false,
        }
    }

    pub fn vortex_aligned() -> Self {
        Self {
            apex_936: false,
            vortex_369: true,
            code_66: false,
            frequency_432: false,
        }
    }

    pub fn fully_aligned() -> Self {
        Self {
            apex_936: true,
            vortex_369: true,
            code_66: true,
            frequency_432: true,
        }
    }

    pub fn alignment_score(&self) -> u8 {
        let mut score = 0;
        if self.apex_936 { score += 1; }
        if self.vortex_369 { score += 1; }
        if self.code_66 { score += 1; }
        if self.frequency_432 { score += 1; }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = EnergyNode::new("Test Node".to_string(), NodeType::NewEarthCity);
        assert_eq!(node.id, "Test Node");
        assert_eq!(node.energy, 0);
        assert_eq!(node.node_type, NodeType::NewEarthCity);
    }

    #[test]
    fn test_node_with_energy() {
        let node = EnergyNode::new("Test".to_string(), NodeType::VortexThrone)
            .with_energy(936);
        assert_eq!(node.energy, 936);
    }

    #[test]
    fn test_node_capacity() {
        let mut node = EnergyNode::new("Test".to_string(), NodeType::NewEarthCity)
            .with_capacity(1000);
        
        let added = node.add_energy(500);
        assert_eq!(added, 500);
        assert_eq!(node.energy, 500);
        
        let added = node.add_energy(600);
        assert_eq!(added, 500); // Only 500 available
        assert_eq!(node.energy, 1000);
        assert!(node.is_at_capacity());
    }

    #[test]
    fn test_sacred_alignment() {
        let alignment = SacredAlignment::fully_aligned();
        assert_eq!(alignment.alignment_score(), 4);
        
        let alignment = SacredAlignment::apex_aligned();
        assert_eq!(alignment.alignment_score(), 1);
    }
}
