use serde_json;
use serde;
use tauri::Window;
use crate::sevrices::yt_dlp;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Thumbs {
    url: String,
    height: i32,
    width: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Format {
    format_id: String,
    resolution: String,
    audio_ext: String,
    video_ext: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Video {
    id: String,
    title: String,
    duration: i32,
    view_count: i32,
    like_count: i32,
    channel: String,
    thumbnail: String,

    #[serde(default)]
    formats: Vec<Format>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SearchVideo {
    id: String,
    title: String,
    view_count: i32,
    thumbnails: Vec<Thumbs>,

    #[serde(default)]
    channel: Option<String>,

    #[serde(default)]
    duration: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DownloadData {
    pub id: String,
    pub path: String,
    pub format: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SearchData {
    pub query: String,
    pub limit: i32,
}

impl Video {
    pub async fn get_video_info(url: String) -> Result<Video, String> {
        let raw_data = yt_dlp::get_video_info(url);

        let json_data: Video = match serde_json::from_str(&raw_data[0..]) {
            Ok(video) => video,
            Err(err) => return Err(format!("Some error {}", err)),
        };
        
        Ok(json_data)
    }

    pub async fn download_video(data: DownloadData, window: Window) -> Result<bool, ()> {
        yt_dlp::download_video(&data, &window);

        Ok(true)
    }

    pub async fn search_video(data: SearchData) -> Result<Vec<SearchVideo>, String> {
        let raw_data = yt_dlp::search_video(&data);

        let mut json_data: serde_json::Value = match serde_json::from_str(&raw_data[0..]) {
            Ok(value) => value,
            Err(err) => return Err(format!("Some error {}", err)),
        };
        
        let videos: Vec<SearchVideo> = match serde_json::from_value(json_data["entries"].take()) {
            Ok(data) => data,
            Err(err) => return Err(format!("Some error {}", err)),
        };

        Ok(videos)
    }
}

// window.__TAURI_INVOKE__("search_video", {query: "andrux"})
