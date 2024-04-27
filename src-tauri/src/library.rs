use std::collections::HashMap;

use directories::UserDirs;
use serde::Serialize;
use tauri::{
	plugin::{Builder, TauriPlugin},
	Runtime,
};
use walkdir::WalkDir;

use self::metadata::{get_album_name, get_metadata};

pub mod metadata;
pub mod uri;

#[derive(Serialize)]
struct Song {
	title: String,
	artists: Option<Vec<String>>,
	year: Option<i32>,
	duration: f64,
	cover: String,
}

#[derive(Serialize)]
struct Album {
	title: String,
	artists: Option<Vec<String>>,
	songs: Vec<Song>
}

#[tauri::command]
fn get_files() -> Vec<Album> {
	let mut albums: HashMap<String, Album> = HashMap::new();
		
	if let Some(music_dir) = UserDirs::new().unwrap().audio_dir() {
		for file in WalkDir::new(music_dir) {
			if let Ok(file) = file {
				if file.file_type().is_file() {
					if let Some(album_name) = get_album_name(file.path().to_path_buf()) {
						let album = albums
							.entry(album_name.clone())
							.or_insert_with(|| Album {
								title: album_name,
								artists: None,
								songs: Vec::new(),
							});

						if let Some(song) = get_metadata(file.into_path()) {
							album.songs.push(song);
						}
					};

				}
			};
		}
	};	
	albums.into_values().collect()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("library")
    .invoke_handler(tauri::generate_handler![get_files])
    .build()
}
