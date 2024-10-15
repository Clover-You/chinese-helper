fn main() -> Result<(), Box<dyn std::error::Error>> {
  // load env profile
  dotenv::dotenv()?;

  println!("Hello, world!");

  Ok(())
}
