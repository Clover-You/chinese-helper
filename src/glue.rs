use std::error::Error;

pub trait UniversalTranslator<T> {
  async fn translate(&self, text: &str) -> Result<T, Box<dyn Error>>;
}
