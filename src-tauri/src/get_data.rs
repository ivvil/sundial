use directories::UserDirs;
use serde::Serialize;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
	command
};
use walkdir::WalkDir;
use std::path::PathBuf;
use audiotags::Tag;
use anyhow::{Error, Result};

use self::uri::build_image_uri;

mod uri;

#[derive(Serialize)]
struct Song {
    title: String,
    artists: Option<Vec<String>>,
    album: Option<Album>,
    duration: f64,
    path: PathBuf,
}

#[derive(Serialize)]
struct Album {
    cover: String,
    name: String,
    artist: Option<String>,
}


#[command]
fn get_library() -> Result<Vec<Song>, String> {
    let mut library = Vec::new();

	if let Some(music_dir) = UserDirs::new().unwrap().audio_dir() {
        for entry in WalkDir::new(music_dir) {
			let entry = entry.map_err(|e| format!("Error reading directory: {}", e))?;
            if entry.file_type().is_file() {
                let path = entry.path().to_owned();
                if let Some(metadata) = get_metadata(&path) {
                    library.push(metadata);
                }
            }
        }
    }

    Ok(library)
}

fn get_metadata(file_path: &PathBuf) -> Option<Song> {
    if let Ok(tag) = Tag::new().read_from_path(file_path) {
		let artists = tag.artists().map(|vec| {
			vec.iter().map(|s| s.to_string()).collect()
		});

		let image = tag.album_cover().unwrap();
		
        let song = Song {
            title: tag.title().to_owned().unwrap_or_else(|| "").to_string(),
            artists,
				album: Some(Album {
                cover: build_image_uri(image.mime_type, image.data.to_vec()),
                name: tag.album_title().unwrap_or_else(|| "").to_string(),
                artist: Some(tag.album_artist().unwrap_or_else(|| "").to_string()),
            }),
            duration: tag.duration().unwrap_or(0.0),
            path: file_path.to_path_buf().clone(),
        };
        Some(song)
    } else {
        None
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("get_data")
        .invoke_handler(tauri::generate_handler![get_library])
        .build()
}
