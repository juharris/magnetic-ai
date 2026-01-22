use crate::options::get_options_provider;
use optify::provider::{GetOptionsPreferences, OptionsRegistry};
use serde::Deserialize;
use std::collections::HashMap;

/// Defines what an agent can do when examining shared state.
/// Agents may have multiple capabilities and decide whether to act on each turn.
///
/// Future consideration: Agents could "bid" based on how much they can contribute,
/// allowing dynamic turn ordering based on state relevance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// Can create initial state structure from a prompt/goal
    Initialize,
    /// Can add information to existing fields without changing schema
    Augment,
    /// Can summarize/compress state to prevent unbounded growth
    Summarize,
    /// Can restructure state, define new schema, reorganize
    Restructure,
    /// Can find unexpected connections across state sections - the "flash of genius"
    Serendipity,
    /// Can evaluate state and mark items as validated/rejected
    Evaluate,
    /// Can declare state "complete" for a phase or goal
    Finalize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfig {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub model: ModelConfig,
    pub capabilities: Vec<Capability>,
    pub system_instructions: String,
}

pub fn get_agents() -> Result<HashMap<String, AgentConfig>, String> {
    let provider = get_options_provider();
    let feature_names: Vec<&str> = vec!["agents"];

    let mut preferences = GetOptionsPreferences::new();
    preferences.are_configurable_strings_enabled = true;

    let value = provider.get_all_options(&feature_names, None, Some(&preferences))?;

    let agents_value = value
        .get("agents")
        .ok_or_else(|| "No 'agents' key in configuration".to_string())?;

    serde_json::from_value(agents_value.clone())
        .map_err(|e| format!("Failed to deserialize agents: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_agent() {
        let agents = get_agents().expect("Should get all agents");
        let agent = agents
            .values()
            .next()
            .expect("Should have at least one agent");

        assert!(!agent.name.is_empty());
        assert!(agent.system_instructions.len() > 50);
    }
}
