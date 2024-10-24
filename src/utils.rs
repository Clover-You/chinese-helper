use scraper::ElementRef;

pub fn extract_element_text(el: ElementRef) -> String {
  el.text().collect::<Vec<_>>().join("")
}
