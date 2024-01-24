use reqwest;
use futures::executor;
use serde_json::Value;
use std::fs;
use tauri::api::path::app_data_dir;

#[tauri::command]
pub fn fetch_boards() -> Value {
    let url: &str = "https://a.4cdn.org/boards.json";
    let json: Value = executor::block_on(fetch_json(url)).unwrap();
    json
}

#[tauri::command]
pub fn fetch_catalog(board: &str) -> Value {
    let url_str: String = format!("https://a.4cdn.org/{}/catalog.json", board);
    let url_string: &str = url_str.as_str();
    let json: Value = executor::block_on(fetch_json(url_string)).unwrap();
    json
}

#[tauri::command]
pub fn fetch_thread(board: &str, thread: &str) -> Value {
    let url_str = format!("https://a.4cdn.org/{}/thread/{}.json", board, thread);
    let url_string: &str = url_str.as_str();
    let json: Value = executor::block_on(fetch_json(url_string)).unwrap();
    json
}

#[tauri::command]
pub async fn fetch_thumbnail_from_thread(board: String, file_name: String, app_handle: tauri::AppHandle) -> String {
    let image_url = format!("https://i.4cdn.org/{}/{}", board, file_name);

    let data_dir = app_data_dir(&*app_handle.config()).unwrap();
    let thumbnail_dir_path = data_dir.join("thumbnails");
    fs::create_dir_all(&thumbnail_dir_path).unwrap();

    let thumbnail_file_path = thumbnail_dir_path.join(&file_name);

    if !thumbnail_file_path.exists() {
        let response = reqwest::get(&image_url).await.unwrap();
        let content = response.bytes().await.unwrap();
        let file_path = thumbnail_file_path.as_path();
        fs::write(file_path, content).unwrap();
    }

    thumbnail_file_path.into_os_string().into_string().unwrap()
}

async fn fetch_json(url: &str) -> Result<Value, reqwest::Error> {
    let json: Value = reqwest::get(url)
        .await?
        .json()
        .await?;
    Ok(json)
}
