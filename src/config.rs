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

pub async fn read() -> Config {
  let yml = std::fs::read_to_string(PathBuf::from("altair.yaml"))
    .expect("Failed to read config");

  serde_yml::from_str::<Config>(&yml)
    .map_err(|_| ())
    .expect("Failed to parse config")
}
