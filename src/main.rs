use altairsh::commands::start;
use clap::Command;

#[tokio::main]
async fn main() {
  let matches = Command::new("altair")
    .author("dcdms <dcdmsx@gmail.com>")
    .about("Run multiple commands simultaneously")
    .subcommands([start::build()])
    .get_matches();

  match matches.subcommand() {
    Some(("start", _sub_m)) => match start::execute().await {
      Ok(_) => {}
      Err(error) => {
        eprintln!("Error: {:#}", error);
        std::process::exit(1);
      }
    },
    _ => match start::execute().await {
      Ok(_) => {}
      Err(error) => {
        eprintln!("Error: {:#}", error);
        std::process::exit(1);
      }
    },
  }
}
