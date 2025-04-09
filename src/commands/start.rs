use crate::utils::{read_config, run_command};
use anyhow::Result;
use clap::Command;
use tokio::{
  io::{AsyncBufReadExt, BufReader},
  process::Command as ProcessCommand,
  signal::ctrl_c,
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

pub fn build() -> Command {
  Command::new("start").about("Start altair")
}

pub async fn execute() -> Result<()> {
  let config = read_config::execute().await?;

  let tracker = TaskTracker::new();
  let token = CancellationToken::new();

  for command in config.commands {
    let token_clone = token.clone();

    if let Some(preflight) = command.preflight {
      run_command::execute(&preflight).await?;
    }
 
    let words = shellwords::split(&command.run).unwrap();

    tokio::spawn(async move {
      let mut child = ProcessCommand::new(&words[0])
        .args(&words[1..])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

      let stdout = child.stdout.take().unwrap();
      let mut reader = BufReader::new(stdout).lines();

      println!("[{}] started", &command.name);

      loop {
        tokio::select! {
          maybe_line = reader.next_line() => {
            if let Some(line) = maybe_line.unwrap() {
              println!("[{}] {}", &command.name, line)
            } else {
              break
            }
          },
          _ = token_clone.cancelled() => {
            child.kill().await.unwrap();
            break
          }
        }
      }

      token_clone.cancelled().await;
    });
  }

  let token_clone = token.clone();
  let tracker_clone = tracker.clone();

  tokio::spawn(async move {
    match ctrl_c().await {
      Ok(()) => {
        tracker_clone.close();
        token_clone.cancel();
      }
      Err(err) => {
        eprintln!("[altair] failed to listen to shutdown signal: {}", err)
      }
    }
  });

  tracker.wait().await;

  Ok(())
}
