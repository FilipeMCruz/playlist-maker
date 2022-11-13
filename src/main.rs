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

    print(&final_play, cli.output.as_deref());
}

fn divide_songs_by_threads(songs: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut chunks_songs: Vec<Vec<PathBuf>> = Vec::new();

    songs.chunks(songs.len() / num_cpus::get()).collect::<Vec<&[PathBuf]>>()
        .iter()
        .for_each(|chunk_copy| chunks_songs.push(chunk_copy.iter()
            .map(|elem| elem.clone())
            .collect::<Vec<PathBuf>>()));

    chunks_songs
}

fn query_songs(query: &str, playlist_vec: Vec<Playlist>, chunks_songs: Vec<Vec<PathBuf>>) -> Vec<PathBuf> {
    let mut handles = Vec::new();
    let final_play = Arc::new(Mutex::new(Vec::new()));
    for chunk in chunks_songs {
        let query_copy = query.to_owned().clone();

        let playlist_songs = playlist_vec.clone();

        let cloned_v = final_play.clone();
        let handle = thread::spawn(move || {
            let final_playlist = query_walk(&chunk, &playlist_songs, &query_copy);
            cloned_v.lock().unwrap().extend(final_playlist);
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|handle| handle.join().unwrap());

    let final_playlist = final_play.lock().unwrap().to_vec();
    final_playlist
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
    WalkDir::new(dir).into_iter()
        .filter_entry(|entry| is_song(entry))
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .for_each(|entry| ret.push(entry.path().to_owned()));
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
            playlist_vec.push(Playlist {
                name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                songs: BufReader::new(File::open(path).unwrap()).lines()
                    .map(|line| PathBuf::from(line.unwrap())).collect(),
            });
        }
    }
    playlist_vec
}

fn print(vec: &Vec<PathBuf>, output: Option<&Path>) {
    let map = vec.iter()
        .map(|song| song.as_path().display());
    if output.is_none() {
        map.for_each(|song| println!("{}", song));
    } else {
        let mut file = File::create(output.unwrap()).unwrap();
        map.for_each(|song| writeln!(&mut file, "{}", song).unwrap());
    }
}
