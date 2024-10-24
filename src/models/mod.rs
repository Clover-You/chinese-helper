pub mod idiom;

#[derive(serde::Serialize)]
pub struct SearchParams {
  pub tbname: String,
  pub show: String,
  pub tempid: i8,
  pub keyboard: String,
}
