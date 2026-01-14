use crate::ai::client::Ai;

use super::AgentConfig;

pub struct Agent {
    config: AgentConfig,
    ai: Ai,
}

impl Agent {
    pub fn new(config: AgentConfig) -> Self {
        let ai = Ai::new(config.model.clone());
        Self { config, ai }
    }

    pub const fn config(&self) -> &AgentConfig {
        &self.config
    }

    pub const fn ai(&self) -> &Ai {
        &self.ai
    }
}
