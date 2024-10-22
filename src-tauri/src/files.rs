use std::{fs, io, path::{Path, PathBuf}};

fn is_audio_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy().to_lowercase();
			// KLUDGE This is a horrible way to do this			
            ext == "mp3" || ext == "wav" || ext == "flac" || ext == "ogg" || ext == "m4a" || ext == "aac"
        },
        None => false,
    }
}


// KLUDGE This isn't the best way to do this
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
			
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else if is_audio_file(&path) {
                cb(&path);
            }
        }
    }
    Ok(())
}

pub fn get_audio_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();

    visit_dirs(dir, &mut |path| {
        files.push(path.to_path_buf());
    })?;

    Ok(files)
}

