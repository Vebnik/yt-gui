use tauri::Window;

use crate::sevrices::yt;

#[tauri::command]
pub async fn download_video(window: Window, data: yt::DownloadData) -> Result<bool, ()> {
    yt::Video::download_video(data, window).await
}

#[tauri::command]
pub async fn get_video_info(url: &str) -> Result<yt::Video, String> {
    yt::Video::get_video_info(String::from(url)).await
}

#[tauri::command]
pub async fn search_video(data: yt::SearchData) -> Result<Vec<yt::SearchVideo>, String> {
    yt::Video::search_video(data).await
}