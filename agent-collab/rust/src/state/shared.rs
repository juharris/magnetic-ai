use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::{StateVersion, VersionedState};

/// The shared state that agents examine and modify.
/// Uses a Merkle tree for versioning - each modification creates a new version
/// that references its parent, enabling tamper detection and history traversal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedState {
    /// Current state data (flexible JSON structure that agents can evolve)
    current: VersionedState,
    /// Version history indexed by hash for Merkle tree traversal
    history: HashMap<String, VersionedState>,
}

impl SharedState {
    /// Creates a new shared state with an initial goal.
    #[must_use]
    pub fn new(goal: &str, author: &str) -> Self {
        let initial_data = serde_json::json!({
            "goal": goal,
            "phase": "discovery",
        });

        let current = VersionedState::new(initial_data, None, author);
        let mut history = HashMap::new();
        history.insert(current.version.hash.clone(), current.clone());

        Self { current, history }
    }

    /// Returns the current state data.
    #[must_use]
    pub fn data(&self) -> &Value {
        &self.current.data
    }

    /// Returns the current version info.
    #[must_use]
    pub fn version(&self) -> &StateVersion {
        &self.current.version
    }

    /// Returns all versions in the history.
    #[must_use]
    pub fn history(&self) -> &HashMap<String, VersionedState> {
        &self.history
    }

    /// Gets a specific version by hash.
    #[must_use]
    pub fn get_version(&self, hash: &str) -> Option<&VersionedState> {
        self.history.get(hash)
    }

    /// Returns the chain of versions from current back to root.
    #[must_use]
    pub fn ancestry(&self) -> Vec<&VersionedState> {
        let mut chain = Vec::new();
        let mut current_hash = Some(self.current.version.hash.clone());

        while let Some(hash) = current_hash {
            if let Some(version) = self.history.get(&hash) {
                chain.push(version);
                current_hash = version.version.parent.clone();
            } else {
                break;
            }
        }

        chain
    }

    /// Updates the state with new data. Returns the new version hash.
    /// The agent should have already decided it can contribute before calling this.
    pub fn update(&mut self, new_data: Value, author: &str) -> String {
        let new_version = VersionedState::new(
            new_data,
            Some(&self.current.version),
            author,
        );

        let hash = new_version.version.hash.clone();
        self.history.insert(hash.clone(), new_version.clone());
        self.current = new_version;

        hash
    }

    /// Merges new data into the current state (shallow merge at top level).
    /// Useful for Augment capability agents that add fields.
    pub fn merge(&mut self, additions: Value, author: &str) -> String {
        let mut new_data = self.current.data.clone();

        if let (Some(current_obj), Some(additions_obj)) =
            (new_data.as_object_mut(), additions.as_object())
        {
            for (key, value) in additions_obj {
                current_obj.insert(key.clone(), value.clone());
            }
        }

        self.update(new_data, author)
    }

    /// Verifies the integrity of the entire Merkle tree.
    #[must_use]
    pub fn verify_integrity(&self) -> bool {
        for versioned in self.history.values() {
            if !versioned.verify() {
                return false;
            }

            if let Some(ref parent_hash) = versioned.version.parent {
                if !self.history.contains_key(parent_hash) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_state_evolution() {
        let mut state = SharedState::new("Design a product for urban gardeners", "initializer");

        assert_eq!(state.data()["goal"], "Design a product for urban gardeners");
        assert_eq!(state.data()["phase"], "discovery");

        state.merge(
            json!({
                "insights": [
                    {"finding": "70% lack balcony space", "domain": "constraints"}
                ]
            }),
            "researcher",
        );

        assert!(state.data()["insights"].is_array());
        assert_eq!(state.history().len(), 2);
        assert!(state.verify_integrity());
    }

    #[test]
    fn test_ancestry_chain() {
        let mut state = SharedState::new("test goal", "init");
        state.merge(json!({"step": 1}), "agent1");
        state.merge(json!({"step": 2}), "agent2");
        state.merge(json!({"step": 3}), "agent3");

        let ancestry = state.ancestry();
        assert_eq!(ancestry.len(), 4);
        assert_eq!(ancestry[0].version.author, "agent3");
        assert_eq!(ancestry[3].version.author, "init");
    }
}
