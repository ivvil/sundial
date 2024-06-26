use std::path::PathBuf;
use audiotags::{Error, Picture, Tag};

use super::{uri::img_base64, Song};

pub fn get_cover(file: PathBuf) -> Option<String> {
    match Tag::new().read_from_path(file) {
        Ok(tag) => match tag.album_cover() {
            Some(cover) => Some(img_base64(&cover)),
            None => None,
        },
        Err(_) => None,
    }
}

pub fn get_album_name(file: PathBuf) -> Option<String> {
    if let Ok(tag) = Tag::new().read_from_path(file) {
		Some(tag.album_title().unwrap_or_else(|| "").to_string())
	} else {
		None
	}

}

pub fn get_album_artists(file: PathBuf) -> Option<Vec<String>> {
	match Tag::new().read_from_path(file) {
		Ok(tag) => match tag.album_artists() {
			Some(artists) => Some(artists.iter().map(|artist| artist.to_string()).collect()),
			None => match tag.album_artist() {
                Some(artist) => Some(vec![artist.to_string()]),
                None => None,
            },
		},
		Err(_) => None,
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
