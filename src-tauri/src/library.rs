use std::path::PathBuf;

use directories::UserDirs;
use tauri::{
	plugin::{Builder, TauriPlugin},
	Runtime,
};
use walkdir::WalkDir;

#[tauri::command]
fn get_files() -> Vec<PathBuf> {
	let mut file_paths = Vec::new();
	
	if let Some(music_dir) = UserDirs::new().unwrap().audio_dir() {
		for file in WalkDir::new(music_dir) {
			if let Ok(file) = file {
				if file.file_type().is_file() {
					file_paths.push(file.path().to_path_buf());
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
