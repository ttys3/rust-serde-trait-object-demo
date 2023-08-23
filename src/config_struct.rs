use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use dyn_clone::DynClone;
use log::{debug};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub provider: Option<String>,
    pub provider_config: HashMap<String, ProviderConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    pub api_base: Option<String>,
    pub token: Option<String>,
    pub model: Option<String>,
    pub cookies: HashMap<String, String>,
}

impl Config {
    /// get current provider config
    pub fn current_provider_config(&self) -> Option<ProviderConfig> {
        let provider = self.provider();
        let provider = provider.as_str();
        self.provider_config.get(provider).cloned()
    }

    pub fn provider(&self) -> String {
        self.provider.clone().unwrap_or_else(|| "openai_mobile".to_string())
    }

    pub fn from_str() -> anyhow::Result<Self> {
        let contents: Vec<u8> = r#"
provider: openai_mobile
#provider: claude2
provider_config:
  claude2:
    cookies:
        cookie1: "cookie1-value"
        cookie2: "cookie2-value"
    api_base: "https://claude.io"
  openai_mobile:
    token: "tk-xxxxxxxx-001"
    api_base: "https://api.openai.com"
    model: "davinci"
    cookies:
        cookie_foo: "foo_value"
        cookie_bar: "bar_value"
"#.into();
        let config: Config = serde_yaml::from_str(&String::from_utf8_lossy(contents.as_slice())).unwrap();

        debug!("read config success: {:?}", config);

        Ok(config)
    }

    pub fn to_string(&self) -> anyhow::Result<String> {
        Ok(serde_yaml::to_string(self).map_err(|e| anyhow::format_err!("serde config to yaml failed: {}", e))?)
    }
}

