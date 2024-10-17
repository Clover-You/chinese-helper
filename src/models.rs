#[derive(serde::Serialize)]
pub struct SearchParams {
  pub tbname: String,
  pub show: String,
  pub tempid: i8,
  pub keyboard: String,
}

#[derive(Default, Debug)]
pub struct ProverbsData {
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
