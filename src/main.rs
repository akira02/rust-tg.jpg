use anyhow::Result;
use log::{error, info};
use regex::Regex;
use reqwest::Client;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::RequestError;
use url::Url;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();
  info!("Starting image search bot...");

  let bot = Bot::from_env();

  teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
    if let Err(e) = handle_message(&bot, &msg).await {
      error!("Error handling message: {:?}", e);
    }
    Ok::<(), RequestError>(())
  })
  .await;
}

async fn handle_message(bot: &Bot, msg: &Message) -> Result<(), anyhow::Error> {
  let text = match msg.text() {
    Some(text) => text,
    None => return Ok(()),
  };

  let pattern = Regex::new(r"^(.+?)\.(jpg|png|gif)$")?;
  let captures = pattern.captures(text).ok_or(anyhow::anyhow!(""))?;

  let query = captures.get(1).unwrap().as_str();
  info!("Query: {}", query);
  let is_gif = captures.get(2).unwrap().as_str() == "gif";

  let image_urls = image_search(query, is_gif).await?;

  for image_url in image_urls.iter().take(2) {
    let parsed_url = match Url::parse(image_url) {
      Ok(url) => url,
      Err(_) => {
        error!("Failed to parse URL: {}", image_url);
        continue;
      }
    };

    match bot
      .send_photo(msg.chat.id, InputFile::url(parsed_url))
      .await
    {
      Ok(_) => break,
      Err(e) => {
        error!("Failed to send photo {}: {:?}", image_url, e);
        continue;
      }
    }
  }

  Ok(())
}

async fn image_search(query: &str, is_gif: bool) -> Result<Vec<String>, anyhow::Error> {
  let endpoint = "https://www.google.com/search";
  let tbs = if is_gif { "ift:gif" } else { "ift:jpg" };

  let params = [("q", query), ("tbs", tbs), ("tbm", "isch"), ("hl", "zh-TW")];

  let client = Client::new();
  let res = client
    .get(endpoint)
    .query(&params)
    .header(
      "User-Agent",
      "Opera/9.80 (J2ME/MIDP; Opera Mini/9.80 (J2ME/23.377; U; en) Presto/2.5.25 Version/10.54",
    )
    .send()
    .await?;

  let html = res.text().await?;
  Ok(extract_image_urls(&html))
}

fn extract_image_urls(text: &str) -> Vec<String> {
  let mut urls = Vec::new();
  let data_ou_regex = regex::Regex::new(r#"data-ou="(.*?)""#).unwrap();

  for cap in data_ou_regex.captures_iter(text).take(3) {
    if let Some(url_match) = cap.get(1) {
      let decoded_url = urlencoding::decode(url_match.as_str())
        .unwrap_or_default()
        .into_owned();
      let clean_url = decoded_url.split('?').next().unwrap_or("").to_string();
      urls.push(clean_url);
    }
  }
  urls
}
