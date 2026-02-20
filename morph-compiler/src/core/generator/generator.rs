use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::Path;
use serde::Deserialize;
use crate::core::Packet;
use crate::utils::{MorphError, MorphResult};

#[derive(Debug)]
pub struct GenerationError {
    pub message: String,
}

impl Display for GenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GenerationError: {}", self.message)
    }
}

impl MorphError for GenerationError {
    fn message(&self) -> String {
        format!("GenerationError: {}", self.message)
    }
}

#[derive(Debug, Deserialize)]
pub struct GenerationConfig {
    #[serde(flatten)]
    pub lang_configs: HashMap<String, toml::Value>,
}

impl GenerationConfig {

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: GenerationConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn parse_lang_options<T: serde::de::DeserializeOwned>(
        &self,
        lang: &str,
    ) -> anyhow::Result<T> {

        let val = self.lang_configs
            .get(lang)
            .ok_or_else(|| anyhow::anyhow!("No config section [{}] in configuration", lang))?;

        let opts = val.clone().try_into()?;
        Ok(opts)
    }

}


pub trait Generator {
    fn generate(&self, packets: &Vec<Packet>) -> MorphResult<()>;

}
