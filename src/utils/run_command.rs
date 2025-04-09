use anyhow::{Context, Result};
use tokio::process::Command;

pub async fn execute(prompt: &str) -> Result<()> {
  let commands: Vec<&str> = prompt.split("&&").collect();

  for command in commands {
    let words = shellwords::split(command).context("Mismatched quotes")?;

    Command::new(&words[0])
      .args(&words[1..])
      .stdout(std::process::Stdio::piped())
      .output()
      .await
      .context("Failed to run command")?;
  }

  Ok(())
}
