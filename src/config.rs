use crate::common::Chain;
use crate::errors::SwrError;
use figment::providers::Env;
use figment::providers::Format;
use figment::providers::Toml;
use figment::Figment;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub chain: Chain,
    pub subgraph_name: String,
    pub subgraph_id: Option<String>,
    pub manifest: String,
    pub transforms: Option<TransformConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TransformConfig {
    Ethereum {
        block: String,
        transactions: String,
        logs: String,
    },
    Mock,
}

impl Config {
    pub fn load() -> Result<Self, SwrError> {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("SWR_"))
            .extract()
            .map_err(|e| SwrError::ConfigLoadFail(e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn test_config() {
        let config = Config::load().unwrap();
        ::log::info!("Config = {:?}", config);
    }
}
