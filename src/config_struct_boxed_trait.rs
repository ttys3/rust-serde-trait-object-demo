use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use dyn_clone::DynClone;
use log::{debug};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub provider: Option<String>,
    pub provider_config: ProviderConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig  {
    openai_mobile: OpenaiMobile,
    claude2: Claude2,
}

pub trait Provider {

    fn name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}
impl Config {
    /// get current provider config
    pub fn current_provider_config(&self) -> Option<Box<dyn Provider>> {
        let provider = self.provider();
        let provider = provider.as_str();
        match provider {
            "claude2" => Some(Box::new(self.provider_config.claude2.clone())),
            "openai_mobile" => Some(Box::new(self.provider_config.openai_mobile.clone())),
            _ => None,
        }
    }

    pub fn provider(&self) -> String {
        self.provider.clone().unwrap_or_else(|| "OpenaiMobile".to_string())
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

impl Provider for Claude2 {
    fn name(&self) -> String {
        "claude2".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenaiMobile {
    pub token: Option<String>,
    pub api_base: Option<String>,
    pub model: Option<String>,
    pub cookies: HashMap<String, String>,
}

impl Provider for OpenaiMobile {
    fn name(&self) -> String {
        "openai_mobile".to_string()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
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

