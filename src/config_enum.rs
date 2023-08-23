use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use dyn_clone::DynClone;
use log::{debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: Option<String>,
    pub provider_config: ProviderConfigMap,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ProviderConfigMap(HashMap<String, ProviderConfig>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderConfig  {
    OpenaiMobile(OpenaiMobile),
    Claude2(Claude2),
}

impl Debug for ProviderConfigMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderConfigMap")
            .field("provider_config", &self.0.keys())
            .finish()
    }
}

impl ProviderConfigMap {
    pub fn get(&self, provider: &str) -> Option<&ProviderConfig> {
        self.0.get(provider)
    }

    pub fn set(&mut self, provider: &str, pcfg: ProviderConfig) {
        self.0.insert(provider.to_string(), pcfg);
    }
}

impl Default for ProviderConfigMap {
    fn default() -> Self {
        let mut provider_config: HashMap<String, ProviderConfig> = HashMap::new();
        provider_config.insert(
            "OpenaiMobile".to_string(),
            ProviderConfig::OpenaiMobile(OpenaiMobile::default()),
        );
        provider_config.insert(
            "Claude2".to_string(),
            ProviderConfig::Claude2(Claude2::default()),
        );
        ProviderConfigMap(provider_config)
    }
}

impl Default for Config {
    fn default() -> Self {
        let provider_config  = ProviderConfigMap::default();
        Config {
            provider: Some("OpenaiMobile".to_string()),
            provider_config,
        }
    }
}

impl Config {

    /// get current provider config
    pub fn current_provider_config(&self) -> Option<&ProviderConfig> {
        self.provider_config.get(self.provider().as_str())
    }

    pub fn provider(&self) -> String {
        self.provider.clone().unwrap_or_else(|| "OpenaiMobile".to_string())
    }

    pub fn from_str() -> anyhow::Result<Self> {
        let contents: Vec<u8> = r#"
provider: OpenaiMobile
#provider: Claude2
provider_config:
  Claude2: !Claude2
    provider: "Claude2"
    cookies:
        cookie1: "cookie1-value"
        cookie2: "cookie2-value"
    api_base: "https://claude.io"
  OpenaiMobile: !OpenaiMobile
    provider: "OpenaiMobile"
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claude2 {
    pub cookies: HashMap<String, String>,
    pub api_base: Option<String>,
}

impl Default for Claude2 {
    fn default() -> Self {
        Claude2 {
            cookies: HashMap::new(),
            api_base: "https://claude.io".to_string().into(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenaiMobile {
    pub token: Option<String>,
    pub api_base: Option<String>,
    pub model: Option<String>,
    pub cookies: HashMap<String, String>,
}

impl Default for OpenaiMobile {
    fn default() -> Self {
        OpenaiMobile {
            token: None,
            api_base: "https://api.openai.com".to_string().into(),
            model: "davinci".to_string().into(),
            cookies: HashMap::new(),
        }
    }
}

