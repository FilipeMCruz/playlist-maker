#[macro_use]
extern crate pest_derive;

mod song_tag;
mod playlist;
mod query_walk;
mod path_matching;
mod string_extractor;

use std::{thread};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};

use clap::Parser;
use walkdir::{WalkDir};

use playlist::Playlist;
use path_matching::ExtensionExtractor;

use crate::query_walk::query_walk;

#[derive(Parser)]
#[command(name = "playlist-maker")]
#[command(author = "FilipeMCruz <filipeCruz@tuta.io>")]
#[command(version = "0.3.0")]
#[command(about = "Create playlists using a query language", long_about = None)]
struct Cli {
    ///Directory with songs to query from (can be repeated if needed)
    #[arg(short, long)]
    input: Vec<PathBuf>,
    ///File to write the playlist to (if not specified send to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    ///Path to playlist to be used in the query (can be repeated if needed)
    #[arg(short, long)]
    playlist: Vec<PathBuf>,
    ///Query to execute
    #[arg(short, long)]
    query: String,
}

fn main() {
    let cli = Cli::parse();

    let playlist_vec = get_playlists(cli.playlist);

    let all_songs = get_songs(cli.input);

    let chunks_songs = divide_songs_by_threads(all_songs);

    let final_play = query_songs(&cli.query, playlist_vec, chunks_songs);

    print(&final_play, cli.output.as_deref());
}

fn divide_songs_by_threads(all_songs: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    all_songs.chunks(all_songs.len() /*/ num_cpus::get()*/)
        .map(|songs| songs.to_vec())
        .collect::<Vec<Vec<_>>>()
}

fn query_songs(query: &str, playlist_vec: Vec<Playlist>, chunks_songs: Vec<Vec<PathBuf>>) -> Vec<PathBuf> {
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

fn get_songs(input: Vec<PathBuf>) -> Vec<PathBuf> {
    input.iter()
        .filter(|dir| dir.is_dir())
        .flat_map(|dir| walk(dir.to_owned()))
        .collect()
}

fn walk(dir: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dir).into_iter()
        .filter_entry(|entry| entry.path().match_extension_or_dir("mp3"))
        .map(|entry| entry.unwrap().into_path())
        .filter(|entry| entry.is_file())
        .collect::<Vec<PathBuf>>()
}

fn get_playlists(playlists: Vec<PathBuf>) -> Vec<Playlist> {
    let mut playlist_vec = Vec::new();
    for playlist in playlists {
        let path = Path::new(&playlist);
        if path.match_extension("m3u") {
            playlist_vec.push(Playlist {
                name: path.file_stem().unwrap().to_str().unwrap().to_string(),
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

fn print(vec: &[PathBuf], output: Option<&Path>) {
    let map = vec.iter()
        .map(|song| song.as_path().display());

    match output {
        None => map.for_each(|song| println!("{}", song)),
        Some(out) => {
            let mut file = File::create(out).unwrap();
            map.for_each(|song| writeln!(&mut file, "{}", song).unwrap());
        }
    }
}
