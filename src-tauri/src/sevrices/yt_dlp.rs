use std::{io::{BufRead, BufReader}, process::{Command, Stdio}};
use tauri::Window;

use crate::sevrices::yt;

pub fn get_video_info(url: String) -> String {
    let out = Command::new("yt-dlp")
        .arg(url)
        .arg("--skip-download")
        .arg("--dump-single-json")
        .arg("--no-check-certificate")
        .arg("--restrict-filenames")
        .arg("--ignore-no-formats-error")
        .output()
        .expect("Some error in download_playlists");

    String::from_utf8(out.stdout)
        .expect("Error with parsing ytdlp out")
}

pub fn download_video(data: &yt::DownloadData, window: &Window) {
    let _ = window;
    let mut cmd = Command::new("yt-dlp")
        .arg(&data.id)
        .arg("-f")
        .arg(&data.format)
        .arg("-o")
        .arg(&data.path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    
    for line in stdout_reader.lines() {
        println!("Read: {:?}", line.unwrap());
    }

    cmd.wait().unwrap();
}

pub fn search_video(data: &yt::SearchData) -> String {
    let out = Command::new("yt-dlp")
        .arg(format!("ytsearch{}:{}", &data.limit, &data.query))
        .arg("--skip-download")
        .arg("--dump-single-json")
        .arg("--no-check-certificate")
        .arg("--flat-playlist")
        .arg("--ignore-no-formats-error")
        .output()
        .expect("Some error in download_playlists");

    String::from_utf8(out.stdout)
        .expect("Error with parsing ytdlp out")
}