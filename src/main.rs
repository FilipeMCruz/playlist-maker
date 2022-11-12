#[macro_use]
extern crate pest_derive;

use std::{thread};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{Arc, Mutex};

use clap::Parser;
use walkdir::{DirEntry, WalkDir};

use playlist::Playlist;

use crate::query_walk::query_walk;

mod song_tag;
mod playlist;
mod query_walk;

#[derive(Parser)]
#[command(name = "playlist-maker")]
#[command(author = "FilipeMCruz <filipeCruz@tuta.io>")]
#[command(version = "0.2.1")]
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

    let chunks_songs = divide_songs_by_threads(get_songs(cli.input));

    let final_play = query_songs(&cli.query, playlist_vec, chunks_songs);

    if let Some(output) = cli.output.as_deref() {
        println!("Value for output: {}", output.display());
    }

    print(&final_play, cli.output.as_deref());
}

fn divide_songs_by_threads(songs: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let threads = num_cpus::get();

    let chunk_size = songs.len() / threads;

    let chunks: Vec<&[PathBuf]> = songs.chunks(chunk_size).collect();

    let mut chunks_songs: Vec<Vec<PathBuf>> = Vec::new();
    for chunk_copy in chunks {
        let mut vec = Vec::new();
        for song in chunk_copy {
            vec.push(song.clone())
        }
        chunks_songs.push(vec);
    }
    chunks_songs
}

fn query_songs(query: &str, playlist_vec: Vec<Playlist>, chunks_songs: Vec<Vec<PathBuf>>) -> Vec<PathBuf> {
    let mut handles = Vec::new();
    let final_play = Arc::new(Mutex::new(Vec::new()));
    for chunk in chunks_songs {
        let mut query_copy = String::new();
        query_copy.clone_from(&query.to_owned());

        let playlist_songs = playlist_vec.clone();

        let cloned_v = final_play.clone();
        let handle = thread::spawn(move || {
            let final_playlist = query_walk(&chunk, &playlist_songs, &query_copy);
            cloned_v.lock().unwrap().extend(final_playlist);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_playlist = final_play.lock().unwrap().to_vec();
    final_playlist
}

pub fn get_type(playlist_type: &str) -> &str {
    return match playlist_type.to_lowercase().as_ref() {
        "local" => Ok("local"),
        "spotify" => Err("Type spotify not supported yet!".to_owned()),
        "soundcloud" => Err("Type soundcloud not supported yet!".to_owned()),
        _ => Err(format!("Type {} not supported!", playlist_type))
    }.unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    });
}

fn get_songs(input: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    for dir in input {
        if !dir.exists() {
            println!("Folder {} does not exist!", dir.display());
            exit(2);
        } else {
            walk(dir, vec.borrow_mut());
        }
    }
    vec
}

fn walk(dir: PathBuf, ret: &mut Vec<PathBuf>) {
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| is_song(e)) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            ret.push(entry.path().to_owned());
        }
    }
}

fn is_song(entry: &DirEntry) -> bool {
    entry.path().is_dir() ||
        entry.file_name()
            .to_str()
            .map(|s| s.to_lowercase().ends_with("mp3"))
            .unwrap_or(false)
}

fn get_playlists(playlists: Vec<PathBuf>) -> Vec<Playlist> {
    let mut playlist_vec = Vec::new();
    for playlist in playlists {
        let path = Path::new(&playlist);
        if !path.exists() || !path.extension().unwrap().eq("m3u") {
            println!("playlist `{}` does not exist or is invalid (not m3u)!", playlist.display());
            exit(2);
        } else {
            let playlist = Playlist {
                name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                songs: BufReader::new(File::open(path).unwrap()).lines()
                    .map(|line| PathBuf::from(line.unwrap())).collect(),
            };
            playlist_vec.push(playlist);
        }
    }
    playlist_vec
}

fn print(vec: &Vec<PathBuf>, output: Option<&Path>) {
    if output.is_none() {
        for song in vec.iter() {
            println!("{}", song.as_path().display());
        }
    } else {
        let mut file = File::create(output.unwrap()).unwrap();
        for song in vec.iter() {
            writeln!(&mut file, "{}", song.as_path().display()).unwrap();
        }
    }
}
