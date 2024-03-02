// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn shorten_url(url: &str) -> String {
    format!("{}", url)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shorten_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
