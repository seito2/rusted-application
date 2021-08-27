pub use ::config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub server_address: String,
  pub pg: deadpool_postgres::Config,
}

impl Config {
  pub fn load_env() -> Result<Config, ConfigError> {
    let mut config = ::config::Config::new();
    config.merge(::config::Environment::new())?;
    config.try_into()
  }
}
