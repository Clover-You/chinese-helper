mod models;
mod network;
mod utils;

use models::SearchParams;
use utils::read_user_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  loop {
    let user_input = read_user_input()?;

    let params = SearchParams {
      tbname: "chengyu".into(),
      show: "title".into(),
      tempid: 3,
      keyboard: user_input.trim().into(),
    };

    let uri = network::fetch_target_uri(params).await?;
    let target_url = format!("https://www.xhzidian.com/{}", uri);

    let proverbs = network::fetch_proverbs_data(&target_url).await?;
    println!("{:?}", proverbs);
  }
}
