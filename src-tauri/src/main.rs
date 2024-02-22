// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// crates
mod sevrices;
mod commands;

// main
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::download_video, commands::get_video_info, commands::search_video,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}