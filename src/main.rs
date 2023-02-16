#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate serde_derive;

mod path;
mod playlist;
mod query;
mod tag;

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

use walkdir::WalkDir;

use crate::path::matching::ExtensionExtractor;
use crate::playlist::Playlist;
use crate::query::processor;
use crate::tag::details::TagDetails;

/// Create playlists using a query language
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    ///Directory with songs or file with indexed songs to query from (can be repeated if needed)
    #[arg(short, long)]
    input: Vec<PathBuf>,
    ///File to write the query results to (if not specified send to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    ///Path to m3u playlist to be used in the query (can be repeated if needed)
    #[arg(short, long)]
    playlist: Vec<PathBuf>,
    ///Query to execute
    #[arg(short, long)]
    query: String,
}

pub fn build_cli() -> Cli {
    Cli::parse()
}

fn main() {
    let cli = build_cli();

    let playlist_vec = get_playlists(cli.playlist);

    let all_songs = get_songs(cli.input);

    let chunks_songs = divide_songs_by_threads(all_songs);

    let export_type = processor::is_play(&cli.query);

    let final_play = query_songs(cli.query, playlist_vec, chunks_songs);

    print(&final_play, cli.output.as_deref(), export_type);
}

fn divide_songs_by_threads(all_songs: Vec<TagDetails>) -> Vec<Vec<TagDetails>> {
    if all_songs.is_empty() {
        return vec![];
    }
    all_songs
        .chunks(all_songs.len() / num_cpus::get())
        .map(|songs| songs.to_vec())
        .collect::<Vec<Vec<_>>>()
}

fn query_songs(
    query: String,
    playlist_vec: Vec<Playlist>,
    chunks_songs: Vec<Vec<TagDetails>>,
) -> Vec<TagDetails> {
    let mut handles = Vec::new();
    let final_play = Arc::new(Mutex::new(Vec::new()));
    for chunk in chunks_songs {
        let query_copy = query.clone();

        let playlists = playlist_vec.clone();

        let cloned_v = final_play.clone();
        let handle = thread::spawn(move || {
            if let Some(arr) = processor::process(&chunk, &playlists, &query_copy) {
                cloned_v.lock().unwrap().extend(arr)
            }
        });
        handles.push(handle);
    }
    handles
        .into_iter()
        .for_each(|handle| handle.join().expect("Could not join on main thread"));

    let final_playlist = final_play.lock().unwrap().to_vec();
    final_playlist
}

fn get_songs(input: Vec<PathBuf>) -> Vec<TagDetails> {
    input
        .into_iter()
        .filter(|dir| dir.is_dir() || dir.is_file())
        .flat_map(|dir| {
            if dir.is_dir() {
                walk(dir)
            } else {
                export(dir)
            }
        })
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

fn get_playlists(playlists: Vec<PathBuf>) -> Vec<Playlist> {
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

fn print(info: &[TagDetails], output: Option<&Path>, is_play: bool) {
    let content = info
        .iter()
        .map(|tag| {
            if is_play {
                tag.path.clone()
            } else {
                tag.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    match (output, is_play) {
        (None, true) => println!("{}", content),
        (None, false) => {
            println!("{}", TagDetails::headers());
            println!("{}", content);
        }
        (Some(out), true) => {
            let mut file = File::create(out).unwrap();
            writeln!(&mut file, "{}", content).unwrap();
        }
        (Some(out), false) => {
            let mut file = File::create(out).unwrap();
            writeln!(&mut file, "{}", TagDetails::headers()).unwrap();
            writeln!(&mut file, "{}", content).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{export, get_songs};
    use std::path::PathBuf;

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
