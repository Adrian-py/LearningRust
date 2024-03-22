// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::block_on;
use serde_json::json;
use dotenv;

#[derive(Debug, Deserialize)]
struct LinkData {
    tiny_url: String,
}

#[derive(Debug, Deserialize)]
struct LinkResponse {
    code: u8,
    data: LinkData,
}
#[derive(Debug, Serialize)]
struct ResponseObject {
    code: u8,
    url: String,
}

async fn send_request(url: &str) -> Result<LinkResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // constructing request body
    let mut request_body: HashMap<&str, &str> = HashMap::new();
    request_body.insert("url", url);

    // retrieve API Key from environment variables
    let api_key = dotenv::var("API_KEY").unwrap();

    let shortened_link = client
        .post("https://api.tinyurl.com/create")
        .query(&[(
            "api_token",
            api_key,
        )])
        .json(&request_body)
        .send()
        .await?
        .json::<LinkResponse>()
        .await?;

    if shortened_link.code != 0 {
        return Err("Failed to shorten link!".into());
    }

    Ok(shortened_link)
}

#[tauri::command]
fn shorten_url(url: &str) -> String {
    let link = send_request(&url);
    let mut response = ResponseObject {
        code: 1,
        url: String::new(),
    };

    match block_on(link) {
        Ok(shortened_link) => {
            response.url = shortened_link.data.tiny_url;
        },
        Err(_) => {
            response.code = 0;
        }, 
    };

    json!(response).to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shorten_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
