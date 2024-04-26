use std::path::PathBuf;
use audiotags::Tag;

use super::Song;

pub fn get_album_name(file: PathBuf) -> Option<String> {
    if let Ok(tag) = Tag::new().read_from_path(file) {
		Some(tag.album_title().unwrap_or_else(|| "").to_string())
	} else {
		None
	}

}

pub fn get_metadata(file: PathBuf) -> Option<Song> {
    if let Ok(tag) = Tag::new().read_from_path(file) {
		let artists = tag.artists().map(|vec| {
            vec.iter().map(|s| s.to_string()).collect()
        });

		let song = Song {
			title: tag.title().unwrap_or_else(|| "").to_string(),
			artists,
			year: tag.year(),
			duration: tag.duration().unwrap_or(0.0),
		};
		Some(song)
	} else {
		None
	}

}
