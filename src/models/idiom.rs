use std::error::Error;

use crate::{glue::UniversalTranslator, network};

use super::SearchParams;

#[derive(Default, Debug)]
pub struct Idiom {
  pub explanation: String,
  pub source: String,
  pub eg: String,
  pub pinyin: String,
  pub zhuyin: String,
  pub traditional: String,
  pub feelings: String,
  pub usage: String,
  pub shape_disc: String,
  pub riddle: String,
  pub synonyms: String,
  pub antonym: String,
  pub english: String,
  pub russian: String,
}

impl UniversalTranslator<Idiom> for Idiom {
  async fn translate(&self, text: &str) -> Result<Idiom, Box<dyn Error>> {
    let params = SearchParams {
      tbname: "chengyu".into(),
      show: "title".into(),
      tempid: 3,
      keyboard: text.into(),
    };

    let uri = network::fetch_target_uri(params).await?;
    let target_url = format!("https://www.xhzidian.com/{}", uri);

    let data = network::fetch_idiom_data(&target_url).await?;

    Ok(data)
  }
}
