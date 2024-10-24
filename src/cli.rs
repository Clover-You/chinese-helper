use std::str::FromStr;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum CliType {
  Idiom,
  Word,
  Lexicon,
}

impl FromStr for CliType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "idiom" => Ok(CliType::Idiom),
      "word" => Ok(CliType::Word),
      "lexicon" => Ok(CliType::Lexicon),
      _ => Err("unknown command".to_string()),
    }
  }
}

impl CliType {
  fn support() -> &'static [&'static str] {
    &["idiom", "word", "lexicon"]
  }
}

// const CLI_TYPS: &[&str] = &["idiom", "word", "lexicon"];

#[derive(StructOpt, Debug)]
#[structopt(name = "Chinese Helper")]
pub struct Cli {
  pub content: String,
  #[structopt(short = "t", long = "type", default_value = "idiom", possible_values = CliType::support(), help = "Query classification.")]
  pub typ: CliType,
}
