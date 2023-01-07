#[macro_use]
extern crate pest_derive;

mod song_tag_checker;
mod playlist;
mod query_walk;
mod path_matching;
mod string_extractor;
mod song_metadata;
mod song;

use std::{thread};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};

use clap::Parser;

use walkdir::WalkDir;

use playlist::Playlist;
use path_matching::ExtensionExtractor;

use crate::query_walk::{query_type_is_play, query_walk};
use crate::song::Song;
use crate::song::Song::Indexed;
use crate::song_metadata::IndexDetails;

#[macro_use]
extern crate serde_derive;

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

    let export_type = query_type_is_play(&cli.query);

    let final_play = query_songs(&cli.query, playlist_vec, chunks_songs);

    print(&final_play, cli.output.as_deref(), export_type);
}

fn divide_songs_by_threads(all_songs: Vec<Song>) -> Vec<Vec<Song>> {
    all_songs.chunks(all_songs.len() / num_cpus::get())
        .map(|songs| songs.to_vec())
        .collect::<Vec<Vec<_>>>()
}

fn query_songs(query: &str, playlist_vec: Vec<Playlist>, chunks_songs: Vec<Vec<Song>>) -> Vec<String> {
    let mut handles = Vec::new();
    let final_play = Arc::new(Mutex::new(Vec::new()));
    for chunk in chunks_songs {
        let query_copy = query.to_owned().clone();

        let playlists = playlist_vec.clone();

        let cloned_v = final_play.clone();
        let handle = thread::spawn(move || {
            if let Some(arr) = query_walk(&chunk, &playlists, &query_copy) {
                cloned_v.lock().unwrap().extend(arr)
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|handle| handle.join().expect("Could not join on main threads"));

    let final_playlist = final_play.lock().unwrap().to_vec();
    final_playlist
}

fn get_songs(input: Vec<PathBuf>) -> Vec<Song> {
    input.iter()
        .filter(|dir| dir.is_dir() || dir.is_file())
        .flat_map(|dir| {
            if dir.is_dir() {
                walk(dir.to_owned())
            } else {
                export(dir.to_owned())
            }
        })
        .collect()
}

fn export(file: PathBuf) -> Vec<Song> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(true)
        .from_path(file.into_os_string())
        .expect("Invalid File")
        .deserialize::<IndexDetails>()
        .filter(|record| record.is_ok())
        .map(|record| Indexed(record.unwrap()))
        .collect::<Vec<Song>>()
}

fn walk(dir: PathBuf) -> Vec<Song> {
    WalkDir::new(dir).into_iter()
        .filter_entry(|entry| entry.path().match_extension_or_dir("mp3"))
        .map(|entry| entry.unwrap().into_path())
        .filter(|entry| entry.is_file())
        .map(Song::Real)
        .collect::<Vec<Song>>()
}

fn get_playlists(playlists: Vec<PathBuf>) -> Vec<Playlist> {
    let mut playlist_vec = Vec::new();
    for playlist in playlists {
        let path = Path::new(&playlist);
        if path.match_extension("m3u") {
            playlist_vec.push(Playlist {
                name: path.display().to_string(),
                songs: BufReader::new(File::open(path).unwrap()).lines()
                    .map(|line| PathBuf::from(line.unwrap())).collect(),
            });
        } else {
            println!("playlist `{}` does not exist or is invalid (not m3u)!", playlist.display());
            exit(2);
        }
    }
    playlist_vec
}

fn print(vec: &[String], output: Option<&Path>, is_play: bool) {
    match (output, is_play) {
        (None, true) => {
            vec.iter().for_each(|song| println!("{}", song))
        }
        (None, false) => {
            println!("{}", IndexDetails::headers());
            vec.iter().for_each(|song| println!("{}", song));
        }
        (Some(out), true) => {
            let mut file = File::create(out).unwrap();
            vec.iter().for_each(|song| writeln!(&mut file, "{}", song).unwrap());
        }
        (Some(out), false) => {
            let mut file = File::create(out).unwrap();
            writeln!(&mut file, "{}", IndexDetails::headers()).unwrap();
            vec.iter().for_each(|song| writeln!(&mut file, "{}", song).unwrap());
        }
    }
}
