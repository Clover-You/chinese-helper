mod cli;
mod glue;
mod models;
mod network;
mod status_code;
mod utils;

use cli::Cli;
use glue::UniversalTranslator;
use models::idiom::Idiom;
use status_code::StatusEnum;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::from_args();

  let idiom_interpreter = Idiom::default();

  match args.typ {
    cli::CliType::Idiom => {
      let result = idiom_interpreter.translate(&args.content).await?;
      println!("{:?}", result);
    }
    _ => println!("{:?}", StatusEnum::UNDER_DEVELOPMENT),
  }

  Ok(())
}
