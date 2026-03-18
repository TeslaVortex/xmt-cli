//
// ☀️ TOROIDAL PERSISTENCE LAYER ☀️
// State Management & Historical Tracking
// EN EEKE MAI EA ♾️♾️
//

use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleSnapshot {
    pub timestamp: DateTime<Utc>,
    pub cycle_number: u32,
    pub total_energy: u64,
    pub node_states: HashMap<String, u64>,
    pub distribution_algorithm: String,
}

impl CycleSnapshot {
    pub fn new(
        cycle_number: u32,
        total_energy: u64,
        node_states: HashMap<String, u64>,
        distribution_algorithm: String,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            cycle_number,
            total_energy,
            node_states,
            distribution_algorithm,
        }
    }
}

#[derive(Debug)]
pub struct ToroidalPersistence {
    storage_path: PathBuf,
}

impl ToroidalPersistence {
    pub fn new() -> Self {
        Self {
            storage_path: PathBuf::from("Local_storage/.xmt-toroidal"),
        }
    }

    pub fn ensure_storage_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.storage_path)
            .context("Failed to create toroidal storage directory")?;
        Ok(())
    }

    pub fn ledger_state_path(&self) -> PathBuf {
        self.storage_path.join("ledger_state.json")
    }

    pub fn history_path(&self) -> PathBuf {
        self.storage_path.join("cycle_history.json")
    }

    pub fn save_ledger_state<T: Serialize>(&self, state: &T) -> Result<()> {
        self.ensure_storage_dir()?;
        
        let json = serde_json::to_string_pretty(state)
            .context("Failed to serialize ledger state")?;
        
        fs::write(self.ledger_state_path(), json)
            .context("Failed to write ledger state")?;
        
        Ok(())
    }

    pub fn load_ledger_state<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let path = self.ledger_state_path();
        
        if !path.exists() {
            anyhow::bail!("Ledger state file does not exist");
        }
        
        let json = fs::read_to_string(&path)
            .context("Failed to read ledger state")?;
        
        let state = serde_json::from_str(&json)
            .context("Failed to deserialize ledger state")?;
        
        Ok(state)
    }

    pub fn save_snapshot(&self, snapshot: &CycleSnapshot) -> Result<()> {
        self.ensure_storage_dir()?;
        
        let mut history = self.load_history().unwrap_or_default();
        history.push(snapshot.clone());
        
        // Keep only last 1000 snapshots
        if history.len() > 1000 {
            history.drain(0..history.len() - 1000);
        }
        
        let json = serde_json::to_string_pretty(&history)
            .context("Failed to serialize history")?;
        
        fs::write(self.history_path(), json)
            .context("Failed to write history")?;
        
        Ok(())
    }

    pub fn load_history(&self) -> Result<Vec<CycleSnapshot>> {
        let path = self.history_path();
        
        if !path.exists() {
            return Ok(Vec::new());
        }
        
        let json = fs::read_to_string(&path)
            .context("Failed to read history")?;
        
        let history = serde_json::from_str(&json)
            .context("Failed to deserialize history")?;
        
        Ok(history)
    }

    pub fn get_recent_history(&self, limit: usize) -> Result<Vec<CycleSnapshot>> {
        let history = self.load_history()?;
        let start = history.len().saturating_sub(limit);
        Ok(history[start..].to_vec())
    }

    pub fn export_to_json<T: Serialize>(&self, data: &T, filename: &str) -> Result<()> {
        self.ensure_storage_dir()?;
        
        let path = self.storage_path.join(filename);
        let json = serde_json::to_string_pretty(data)
            .context("Failed to serialize export data")?;
        
        fs::write(&path, json)
            .context("Failed to write export file")?;
        
        Ok(())
    }

    pub fn prune_old_snapshots(&self, keep_days: u32) -> Result<usize> {
        let history = self.load_history()?;
        let cutoff = Utc::now() - chrono::Duration::days(keep_days as i64);
        
        let original_count = history.len();
        let pruned: Vec<CycleSnapshot> = history
            .into_iter()
            .filter(|snapshot| snapshot.timestamp > cutoff)
            .collect();
        
        let removed_count = original_count - pruned.len();
        
        if removed_count > 0 {
            let json = serde_json::to_string_pretty(&pruned)
                .context("Failed to serialize pruned history")?;
            
            fs::write(self.history_path(), json)
                .context("Failed to write pruned history")?;
        }
        
        Ok(removed_count)
    }

    pub fn clear_all(&self) -> Result<()> {
        if self.ledger_state_path().exists() {
            fs::remove_file(self.ledger_state_path())?;
        }
        if self.history_path().exists() {
            fs::remove_file(self.history_path())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_persistence_creation() {
        let persistence = ToroidalPersistence::new();
        assert!(persistence.storage_path.to_str().unwrap().contains(".xmt-toroidal"));
    }

    #[test]
    fn test_snapshot_creation() {
        let mut node_states = HashMap::new();
        node_states.insert("Node1".to_string(), 936);
        
        let snapshot = CycleSnapshot::new(
            1,
            936,
            node_states,
            "GoldenRatio".to_string(),
        );
        
        assert_eq!(snapshot.cycle_number, 1);
        assert_eq!(snapshot.total_energy, 936);
    }

    #[test]
    fn test_save_and_load_state() -> Result<()> {
        let persistence = ToroidalPersistence::new();
        persistence.ensure_storage_dir()?;
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestState {
            value: u64,
        }
        
        let state = TestState { value: 936 };
        persistence.save_ledger_state(&state)?;
        
        let loaded: TestState = persistence.load_ledger_state()?;
        assert_eq!(loaded, state);
        
        // Cleanup
        persistence.clear_all()?;
        Ok(())
    }
}
