use serde::{Serialize};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
use mp3_metadata::MP3Metadata;

struct Artist {
    name: String,
	albums: Vec<Album>,
}

struct Song {
    title: String,
	artists: Vec<Artist>,
	album: Album,
	length: u32,
	path: PathBuf,
}


struct Album {
    cover: Path,
	name: String,
	artist: Artist,
	songs: Vec<Song>,
}

async fn get_library() -> Result<Vec<Album>> {
    let mut library = Vec::new();

	if let Some(music_dir) = directories::UserDirs::new().unwrap().audio_dir() {
		for entry in WalkDir::new(music_dir) {
			if let Ok(entry) = entry {
				if entry.file_type().is_file() {
					let path = entry.path();
					if let Some(metadata) = get_m {
						
					};

				}
			};

		}
	};

}

fn get_metadata(file_path: &PathBuf) -> Option<Song> {
    // Handle different file types here, e.g., MP3, FLAC, etc.
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "mp3" => {
                if let Ok(metadata) = MP3Metadata::read_from_path(&file_path) {
                    return Some(Song {
                        path: file_path,
                        title: metadata.title,
                        artist: metadata.artist,
                        album: metadata.album,
                    });
                }
            }
            _ => {
                // Handle other file types if needed
            }
        }
    }

    None
}

