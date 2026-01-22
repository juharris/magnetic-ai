use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A content-addressed version identifier using SHA-256.
/// Forms a Merkle tree where each version references its parent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StateVersion {
    /// SHA-256 hash of the state content + parent hash
    pub hash: String,
    /// Hash of the parent version (None for initial state)
    pub parent: Option<String>,
    /// When this version was created
    pub timestamp: DateTime<Utc>,
    /// Which agent created this version
    pub author: String,
}

impl StateVersion {
    /// Creates a new version by hashing the content with the parent hash.
    #[must_use]
    pub fn new(content: &[u8], parent: Option<&Self>, author: String) -> Self {
        let parent_hash = parent.map(|p| p.hash.clone());

        let mut hasher = Sha256::new();
        hasher.update(content);
        if let Some(ref ph) = parent_hash {
            hasher.update(ph.as_bytes());
        }
        let hash = format!("{:x}", hasher.finalize());

        Self {
            hash,
            parent: parent_hash,
            timestamp: Utc::now(),
            author,
        }
    }

    /// Verifies that this version's hash matches the given content and parent.
    #[must_use]
    pub fn verify(&self, content: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(content);
        if let Some(ref ph) = self.parent {
            hasher.update(ph.as_bytes());
        }
        let computed = format!("{:x}", hasher.finalize());
        computed == self.hash
    }
}

/// Wraps state data with version information for the Merkle tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedState {
    pub version: StateVersion,
    pub data: serde_json::Value,
}

impl VersionedState {
    /// Creates a new versioned state from JSON data.
    ///
    /// # Panics
    /// Panics if JSON serialization fails (should not happen with valid `serde_json::Value`).
    #[must_use]
    pub fn new(data: serde_json::Value, parent: Option<&StateVersion>, author: &str) -> Self {
        let content = serde_json::to_vec(&data).expect("JSON serialization should not fail");
        let version = StateVersion::new(&content, parent, author.to_owned());
        Self { version, data }
    }

    /// Verifies the integrity of this versioned state.
    ///
    /// # Panics
    /// Panics if JSON serialization fails (should not happen with valid `serde_json::Value`).
    #[must_use]
    pub fn verify(&self) -> bool {
        let content = serde_json::to_vec(&self.data).expect("JSON serialization should not fail");
        self.version.verify(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_version_chain() {
        let v1 = VersionedState::new(
            json!({"goal": "test"}),
            None,
            "initializer",
        );
        assert!(v1.verify());

        let v2 = VersionedState::new(
            json!({"goal": "test", "insights": []}),
            Some(&v1.version),
            "augmenter",
        );
        assert!(v2.verify());
        assert_eq!(v2.version.parent, Some(v1.version.hash.clone()));
    }

    #[test]
    fn test_tamper_detection() {
        let mut state = VersionedState::new(
            json!({"goal": "original"}),
            None,
            "initializer",
        );
        assert!(state.verify());

        state.data = json!({"goal": "tampered"});
        assert!(!state.verify());
    }
}
