// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod library;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use directories::{ProjectDirs, UserDirs};
use serde_json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
	print!("{}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// remember to call `.manage(MyState::default())`

#[tauri::command]
fn getPlaying() -> Result<(), String> {
  Ok(())
}

#[tauri::command]
fn getFiles() {
    if let Some(user_dirs) = UserDirs::new() {
		user_dirs.audio_dir();
	}
}

#[tauri::command]
fn play(path: &str) {
	print!("{}", path);
	
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let file = BufReader::new(File::open(path).unwrap());

	let source = Decoder::new(file).unwrap();

	stream_handle.play_raw(source.convert_samples());

	std::thread::sleep(std::time::Duration::from_secs(10))
}

// #[tauri::command]
// fn open_file_dialog(app: AppHandle) -> Option<PathBuf> {
//     let filter = tauri_dialog::open_file_dialog().filter("Audio Files", "*.mp3").unwrap();
//     let file = app
//         .invoke_handler(move |_app| {
//             Ok(filter.load())
//         })
//         .expect("Failed to invoke file dialog");
//     file
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, play])
        .plugin(library::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}










