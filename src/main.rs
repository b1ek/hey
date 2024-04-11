use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use clap::Parser;

const GREEN: &str = "\x1b[1;32m";
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
    pub model: String
}

async fn simulate_browser_reqs(cli: &Client) -> Result<(), Box<dyn Error>> {
    let req = cli.get("https://duckduckgo.com/country.json")
        .header("Host", "duckduckgo.com")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Referer", "https://duckduckgo.com/")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
        .header("DNT", "1")
        .header("Sec-GPC", "1")
        .header("Connection", "keep-alive")
        .header("Cookie", "dcm=3")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .build()?;
    cli.execute(req).await?;
    Ok(())
}

async fn get_vqd(cli: &Client) -> Result<String, Box<dyn Error>> {
    let req = cli.get("https://duckduckgo.com/duckchat/v1/status")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Host", "duckduckgo.com")
        .header("Referer", "https://duckduckgo.com/")
        .header("Cache-Control", "no-store")
        .header("x-vqd-accept", "1")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Cookie", "dcm=3")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Sec-GPC", "1")
        .header("TE", "trailers")
        .build()?;

    let res = cli.execute(req).await?;

    // stdout().write(res.bytes().await.unwrap().to_vec().as_slice()).unwrap();

    let data = res.headers().iter().find(|x| x.0 == "x-vqd-4").map(|x| x.1.clone());
    if let Some(data) = data {
        Ok(data.to_str()?.to_string())
    } else {
        Err("No VQD header returned".into())
    }
}

async fn get_res<T: Into<String>>(cli: &Client, query: T, vqd: String) {
    let query = query.into();
    let payload = ChatPayload {
        model: "claude-instant-1.2".into(),
        messages: vec![ ChatMessagePayload { role: "user".into(), content: query } ]
    };
    let payload = serde_json::to_string(&payload).unwrap();

    let req = cli.post("https://duckduckgo.com/duckchat/v1/chat")
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
        .header("x-vqd-4", vqd)
        .body(payload)
        .build().unwrap();

    let mut res = cli.execute(req).await.unwrap();
    while let Some(chunk) = res.chunk().await.unwrap() {
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
#[clap(trailing_var_arg=true)]
struct Args {
    #[arg()]
    pub query: Vec<String>
}

#[tokio::main]
async fn main() {
    femme::start();

    println!("{GREEN}Contacting DuckDuckGo chat AI...{RESET}");

    let args = Args::parse();
    let query = args.query.join(" ");

    let cli = Client::new();
    simulate_browser_reqs(&cli).await.unwrap();
    let vqd = get_vqd(&cli).await.unwrap();
    get_res(&cli, query, vqd).await;

}
