use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct IndexerConfig {
    /// Horizon base URL, e.g. `https://horizon.stellar.org` or `https://horizon-testnet.stellar.org`
    pub stellar_horizon_url: String,

    /// Postgres connection string
    pub database_url: String,

    /// Poll interval for Horizon when streaming is not used yet.
    #[serde(default = "default_poll_interval_secs")]
    pub poll_interval_secs: u64,

    /// Max records to request per page (Horizon supports `limit`).
    #[serde(default = "default_horizon_limit")]
    pub horizon_limit: u32,
}

fn default_poll_interval_secs() -> u64 {
    2
}

fn default_horizon_limit() -> u32 {
    200
}

impl IndexerConfig {
    pub fn load() -> std::result::Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        cfg.try_deserialize()
    }

    /// Convenience constructor from environment variables.
    pub fn from_env() -> std::result::Result<Self, config::ConfigError> {
        Self::load()
    }
}

// Optional alias if you still want it:
pub type Config = IndexerConfig;

