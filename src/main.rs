use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use log::{error, info};
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
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
        let pattern = Regex::new(r"(.+)\.(jpg|png|gif)").unwrap();
        if let Some(text) = msg.text() {
            if let Some(captures) = pattern.captures(text) {
                let query = captures.get(1).unwrap().as_str();
                match image_search(query, captures.get(2).unwrap().as_str() == "gif").await {
                    Ok(image_urls) => {
                        for image_url in image_urls.iter().take(2) {
                            let result = if image_url.starts_with("data:image/") {
                                if let Some(base64_data) = image_url.split(',').nth(1) {
                                    match general_purpose::STANDARD.decode(base64_data) {
                                        Ok(decoded_image) => bot
                                            .send_photo(
                                                msg.chat.id,
                                                InputFile::memory(decoded_image),
                                            )
                                            .await
                                            .map_err(|e| {
                                                error!("Failed to send photo: {:?}", e);
                                                e
                                            }),
                                        Err(e) => {
                                            error!("Failed to decode base64 image: {:?}", e);
                                            continue;
                                        }
                                    }
                                } else {
                                    continue;
                                }
                            } else if let Ok(parsed_url) = Url::parse(image_url) {
                                info!("Parsed URL: {:?}", parsed_url);
                                bot.send_photo(msg.chat.id, InputFile::url(parsed_url))
                                    .await
                                    .map_err(|e| {
                                        error!("Failed to send photo: {:?}", e);
                                        e
                                    })
                            } else {
                                error!("Failed to parse URL: {}", image_url);
                                continue;
                            };

                            if result.is_ok() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Image search failed: {:?}", e);
                    }
                }
            }
        }
        Ok::<(), RequestError>(())
    })
    .await;
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
    let document = Html::parse_document(text);
    let selector = Selector::parse(".islir").unwrap();

    let elements: Vec<_> = document.select(&selector).collect();
    if elements.is_empty() {
        error!("No elements found with the given selector");
    }

    for element in elements.iter().take(3) {
        if let Some(alt) = element.value().attr("alt") {
            if alt == "Google" {
                continue;
            }
        }
        if let Some(src) = element.value().attr("src") {
            urls.push(src.to_string());
        }
    }
    urls
}
