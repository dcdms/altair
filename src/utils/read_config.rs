use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct ConfigCommand {
  pub name: String,
  pub run: String,
}

#[derive(serde::Deserialize)]
pub struct Config {
  pub commands: Vec<ConfigCommand>,
}

pub async fn execute() -> Result<Config> {
  let yaml = std::fs::read_to_string(PathBuf::from("altair.yaml"))
    .context("Failed to read configuration file")?;

  let config = serde_yml::from_str::<Config>(&yaml)
    .context("Failed to parse configuration file")?;

  Ok(config)
}
