use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use rayon::prelude::*;
use walkdir::WalkDir;
use crate::playlist::Playlist;
use crate::tag::details::TagDetails;
use crate::utils::matching::ExtensionExtractor;

pub fn get_songs(input: Vec<PathBuf>) -> Vec<TagDetails> {
    input
        .into_iter()
        .filter(|dir| dir.is_dir() || dir.is_file())
        .flat_map(|dir| if dir.is_dir() { walk(dir) } else { export(dir) })
        .collect::<HashSet<TagDetails>>()
        .into_iter()
        .collect::<Vec<TagDetails>>()
}

fn export(file: PathBuf) -> Vec<TagDetails> {
    csv::ReaderBuilder::new()
        .delimiter(b',')
        .double_quote(true)
        .has_headers(true)
        .from_path(file.into_os_string())
        .expect("Invalid File")
        .deserialize::<TagDetails>()
        .filter_map(|record| record.ok())
        .collect::<Vec<TagDetails>>()
}

fn walk(dir: PathBuf) -> Vec<TagDetails> {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|entry| entry.path().is_dir_or_has_extension("mp3"))
        .par_bridge()
        .filter_map(|entry| entry.map(|e| e.into_path()).ok())
        .filter(|entry| entry.is_file())
        .filter_map(|path| TagDetails::try_from(&path).ok())
        .collect::<Vec<TagDetails>>()
}

pub fn get_playlists(playlists: Vec<PathBuf>) -> Vec<Playlist> {
    let mut playlist_vec = Vec::new();
    for playlist in playlists {
        let path = Path::new(&playlist);
        if path.has_extension("m3u") {
            playlist_vec.push(Playlist {
                name: path.file_stem().unwrap().to_string_lossy().to_string(),
                songs: BufReader::new(File::open(path).unwrap())
                    .lines()
                    .filter_map(|line| line.ok())
                    .collect(),
            });
        } else {
            println!(
                "playlist `{}` does not exist or is invalid (not m3u)!",
                playlist.display()
            );
            exit(2);
        }
    }
    playlist_vec
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::utils::fs::{export, get_songs};

    #[test]
    fn ensure_fn_export_works_as_expected() {
        let songs = export(PathBuf::from("test-data/index.csv"));
        assert_eq!(songs.len(), 17)
    }

    #[test]
    fn ensure_fn_get_songs_works_as_expected() {
        let songs = get_songs(vec![PathBuf::from("test-data/index.csv")]);
        assert_eq!(songs.len(), 13)
    }
}
