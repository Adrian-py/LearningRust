// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::collections::HashMap;
use tauri::async_runtime::block_on;

#[derive(Debug, Deserialize)]
struct LinkData {
    tiny_url: String,
}

#[derive(Debug, Deserialize)]
struct LinkResponse {
    code: u8,
    data: LinkData,
}

async fn send_request(url: &str) -> Result<LinkResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // constructing request body
    let mut request_body: HashMap<&str, &str> = HashMap::new();
    request_body.insert("url", url);

    let shortened_link = client
        .post("https://api.tinyurl.com/create")
        .query(&[(
            "api_token",
            "85Ur3Dr54J9oVKIAOlpavRXuXvHblY5ChjiFWaD9x8mpgGFQGiqRWvBMqSmW",
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
    match block_on(link) {
        Ok(shortened_link) => shortened_link.data.tiny_url,
        Err(err) => format!("{}", err),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shorten_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
