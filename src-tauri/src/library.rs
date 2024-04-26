

use directories::UserDirs;
use serde::Serialize;
use tauri::{
	plugin::{Builder, TauriPlugin},
	Runtime,
};
use walkdir::WalkDir;

use self::metadata::get_metadata;

pub mod metadata;

#[derive(Serialize)]
struct Song {
	title: String,
	artists: Option<Vec<String>>,
	year: Option<i32>,
	duration: f64,
	album_title: String
}

#[tauri::command]
fn get_files() -> Vec<Song> {
	let mut file_paths = Vec::new();
	
	if let Some(music_dir) = UserDirs::new().unwrap().audio_dir() {
		for file in WalkDir::new(music_dir) {
			if let Ok(file) = file {
				if file.file_type().is_file() {
					if let Some(song) = get_metadata(file.path().to_path_buf()) {
						file_paths.push(song);
					};
				}
			};
		}
	};
	
	file_paths
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("library")
    .invoke_handler(tauri::generate_handler![get_files])
    .build()
}
