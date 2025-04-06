use altair::config;
use tokio::{io::{ AsyncBufReadExt, BufReader }, process::Command, signal::ctrl_c};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[tokio::main]
async fn main() {
  let config = config::read().await;
  
  let tracker = TaskTracker::new();
  let token = CancellationToken::new();

  for command in config.commands {
    let token_clone = token.clone();
    let splitted = shellwords::split(&command.run).unwrap();

    tokio::spawn(async move {
      let mut child = Command::new(&splitted[0])
        .args(&splitted[1..])
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
      },
      Err(err) => {
        eprintln!("[crun] failed to listen to shutdown signal: {}", err)
      }
    }
  });

  tracker.wait().await;
}
