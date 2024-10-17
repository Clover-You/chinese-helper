use scraper::ElementRef;

pub fn extract_element_text(el: ElementRef) -> String {
  el.text().collect::<Vec<_>>().join("")
}

pub fn read_user_input() -> Result<String, Box<dyn std::error::Error>> {
  let mut user_input = String::new();

  std::io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");

  Ok(user_input)
}
