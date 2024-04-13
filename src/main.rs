use std::{error::Error, path::PathBuf, process::exit};

use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde::{Deserialize, Serialize};

use clap::Parser;
use std::io::{stdout, IsTerminal};

use crate::{cache::Cache, config::Config};

mod cache;
mod config;

const GREEN: &str = "\x1b[1;32m";
const RED:   &str = "\x1b[1;31m";
const WARN:  &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

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
    pub role: String,
    pub message: String,
    pub created: u64,
    pub id: String,
    pub action: String,
    pub model: String,
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
    map.insert("Accept", HeaderValue::from_static("*/*"));
    map.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.5"));
    map.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    map.insert("Referer", HeaderValue::from_static("https://duckduckgo.com/"));
    map.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0"));
    map.insert("DNT", HeaderValue::from_static("1"));
    map.insert("Sec-GPC", HeaderValue::from_static("1"));
    map.insert("Connection", HeaderValue::from_static("keep-alive"));
    map.insert("Cookie", HeaderValue::from_static("dcm=3"));
    map.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    map.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    map.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    map.insert("TE", HeaderValue::from_static("trailers"));

    map
}

async fn simulate_browser_reqs(cli: &Client) -> Result<(), Box<dyn Error>> {
    let req = cli.get("https://duckduckgo.com/country.json")
        .headers(get_headers())
        .header("X-Requested-With", "XMLHttpRequest")
        .build()?;
    cli.execute(req).await?;
    Ok(())
}

async fn get_vqd(cli: &Client) -> Result<String, Box<dyn Error>> {

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

async fn get_res<'a>(cli: &Client, query: String, vqd: String, cache: &'a mut Cache) {
    let payload = ChatPayload {
        model: "claude-instant-1.2".into(),
        messages: vec![ ChatMessagePayload { role: "user".into(), content: query } ]
    };
    let payload = serde_json::to_string(&payload).unwrap();

    // println!("{payload}\n\n{:#?}", headers);return;

    let req = cli.post("https://duckduckgo.com/duckchat/v1/chat")
        // .headers(get_headers())
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

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "false", required = false, help = "If you want to agree to the DuckDuckGo TOS")]
    pub agree_tos: bool,
    #[arg()]
    pub query: Vec<String>,
}

#[tokio::main]
async fn main() {
    femme::start();
    
    if ! stdout().is_terminal() {
        eprintln!("{RED}Refusing to run in a non-terminal environment{RESET}");
        eprintln!("This is done to prevent API scraping.");
        exit(2)
    }

    let args = Args::parse();
    let query = args.query.join(" ");

    let mut cache = Cache::load().unwrap();
    let mut config = Config::load().unwrap();

    if args.agree_tos {
        if ! config.tos {
            println!("{GREEN}TOS accepted{RESET}");
        }
        config.tos = true;
        config.save().expect("Error saving config");
    }

    if ! config.tos {
        eprintln!("{RED}You need to agree to duckduckgo AI chat TOS to continue.{RESET}");
        eprintln!("{RED}Visit it on this URL: https://duckduckgo.com/?q=duckduckgo+ai+chat&ia=chat{RESET}");
        eprintln!("Once you read it, pass --agree-tos parameter to agree.");
        eprintln!("{WARN}Note: if you want to, modify `tos` parameter in {}{RESET}", Config::get_path::<PathBuf>().join(Config::get_file_name::<String>()).display());
        exit(3);
    }

    println!("{GREEN}Contacting DuckDuckGo chat AI...{RESET}");

    let cli = Client::new();
    simulate_browser_reqs(&cli).await.unwrap();

    let vqd = match cache.get_last_vqd() {
        Some(v) => { println!("using cached vqd"); v},
        None => get_vqd(&cli).await.unwrap()
    };

    get_res(&cli, query, vqd, &mut cache).await;

}
