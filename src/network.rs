use crate::models::ProverbsData;
use crate::models::SearchParams;
use reqwest::Error;
use scraper::{Html, Selector};

const URL: &str = "https://www.xhzidian.com/e/search/indexen.php";

pub async fn fetch_target_uri(params: SearchParams) -> Result<String, Error> {
  let client = reqwest::Client::new();

  let res = client.post(URL).form(&params).send().await?;
  let html_content = res.text().await?;

  let document = Html::parse_document(&html_content);
  let table_selector = Selector::parse(".table-datas tbody > tr > td > a").unwrap();

  let uri = document
    .select(&table_selector)
    .next()
    .map(|element| element.attr("href").unwrap())
    .unwrap_or_else(|| {
      println!("NO result");
      std::process::exit(1);
    });

  Ok(uri.to_string())
}

pub async fn fetch_proverbs_data(url: &str) -> Result<ProverbsData, Error> {
  let res = reqwest::get(url).await?;
  let html_str = res.text().await?;

  let document = Html::parse_document(&html_str);
  let detail_selector = Selector::parse(".detail.showmore").unwrap();
  let tip_selector = Selector::parse(".tip").unwrap();

  let mut category = String::new();
  let mut proverbs = ProverbsData::default();

  if let Some(detail_element) = document.select(&detail_selector).next() {
    for el in detail_element.child_elements() {
      let el_name = &el.value().name.local;
      let content_text = crate::utils::extract_element_text(el);

      if el_name.eq("h2") {
        category = content_text;
        continue;
      }

      if el_name.eq("p") {
        if let Some(tip) = el.select(&tip_selector).next() {
          let tip_text = crate::utils::extract_element_text(tip);
          let content_text = adjust_content_based_on_tip(content_text, &tip_text);
          update_proverbs_data(&category, &tip_text, &mut proverbs, content_text);
        }
      }
    }
  }

  Ok(proverbs)
}

fn adjust_content_based_on_tip(content_text: String, tip_text: &str) -> String {
  let tip_len = tip_text.chars().count();
  content_text.chars().skip(tip_len).collect()
}

fn update_proverbs_data(
  category: &str,
  tip_text: &str,
  proverbs: &mut ProverbsData,
  content: String,
) {
  match category {
    "意思解释" => match tip_text {
      "基本解释" => proverbs.explanation = content,
      "出处" => proverbs.source = content,
      "例子" => proverbs.eg = content,
      _ => {}
    },
    "基础信息" => match tip_text {
      "拼音" => proverbs.pinyin = content,
      "注音" => proverbs.zhuyin = content,
      "繁体" => proverbs.traditional = content,
      "感情" => proverbs.feelings = content,
      "用法" => proverbs.usage = content,
      "辨形" => proverbs.shape_disc = content,
      "近义词" => proverbs.synonyms = content,
      "反义词" => proverbs.antonym = content,
      "英语" => proverbs.english = content,
      "俄语" => proverbs.russian = content,
      "谜语" => proverbs.riddle = content,
      _ => {}
    },
    _ => {}
  }
}
