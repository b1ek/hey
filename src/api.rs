
use std::error::Error;
use std::process::exit;

use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde::{Deserialize, Serialize};

use crate::{cache::Cache, config::Config};
use crate::{WARN, RED, RESET};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessagePayload {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatPayload {
    pub model: String,
    pub messages: Vec<ChatMessagePayload>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatChunk {
    pub role: Option<String>,
    pub message: String,
    pub created: u64,
    pub action: String,
    pub id: Option<String>,
    pub model: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrChatChunk {
    pub action: String,
    pub status: u64,
    #[serde(rename = "type")]
    pub err_type: String,
}

fn get_headers() -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert("Host", HeaderValue::from_static("duckduckgo.com"));
    map.insert("Accept", HeaderValue::from_static("text/event-stream"));
    map.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.5"));
    map.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    map.insert("Referer", HeaderValue::from_static("https://duckduckgo.com/"));
    map.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0"));
    map.insert("DNT", HeaderValue::from_static("1"));
    map.insert("Sec-GPC", HeaderValue::from_static("1"));
    map.insert("Connection", HeaderValue::from_static("keep-alive"));
    map.insert("Cookie", HeaderValue::from_static("dcm=3; ay=b"));
    map.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    map.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    map.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    map.insert("TE", HeaderValue::from_static("trailers"));

    map
}

pub async fn simulate_browser_reqs(cli: &Client) -> Result<(), Box<dyn Error>> {
    let req = cli.get("https://duckduckgo.com/country.json")
        .headers(get_headers())
        .header("X-Requested-With", "XMLHttpRequest")
        .build()?;
    cli.execute(req).await?;
    Ok(())
}

pub async fn get_vqd(cli: &Client) -> Result<String, Box<dyn Error>> {

    let mut headers = get_headers();
    headers.insert("Cache-Control", HeaderValue::from_static("no-store"));
    headers.insert("x-vqd-accept", HeaderValue::from_static("1"));

    let req = cli.get("https://duckduckgo.com/duckchat/v1/status")
        .headers(headers)
        .build()?;

    let res = cli.execute(req).await?;

    let data = res.headers().iter().find(|x| x.0 == "x-vqd-4").map(|x| x.1.clone());
    if let Some(data) = data {
        Ok(data.to_str()?.to_string())
    } else {
        Err("No VQD header returned".into())
    }
}

pub async fn get_res<'a>(cli: &Client, query: String, vqd: String, cache: &'a mut Cache, config: &Config) {
    let payload = ChatPayload {
        model: config.model.to_string(),
        messages: vec![ ChatMessagePayload { role: "user".into(), content: query } ]
    };
    let payload = serde_json::to_string(&payload).unwrap();

    let req = cli.post("https://duckduckgo.com/duckchat/v1/chat")
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
        .header("x-vqd-4", vqd.clone())
        .body(payload)
        .build().unwrap();

    let mut res = cli.execute(req).await.unwrap();
    let new_vqd = res.headers().iter().find(|x| x.0 == "x-vqd-4");
    let vqd_set_res = 
        if let Some(new_vqd) = new_vqd {
            cache.set_last_vqd(new_vqd.1.as_bytes().iter().map(|x| char::from(*x)).collect::<String>())
        } else {
            eprintln!("{WARN}Warn: DuckDuckGo did not return new VQD. Ignore this if everything else is ok.{RESET}");
            cache.set_last_vqd(vqd.clone())
        };
    if let Err(err) = vqd_set_res {
        eprintln!("{WARN}Warn: Could not save VQD to cache: {err}{RESET}");
    }

    while let Some(chunk) = res.chunk().await.unwrap() {

        if let Ok(obj) = serde_json::from_slice::<ErrChatChunk>(&chunk) {
            if obj.action == "error" {
                eprintln!("{RED}Error obtaining response: {} - {}{RESET}", obj.status, obj.err_type);
                exit(1);
            }
        }

        let chunk = String::from_utf8(chunk.to_vec()).unwrap();
        let chunk = chunk.replace("data: ", "");
        for line in chunk.lines() {
            if let Ok(obj) = serde_json::from_str::<ChatChunk>(line) {
                print!("{}", obj.message);
            }
        }
    }
    println!("\n");
}