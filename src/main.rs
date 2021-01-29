#[macro_use]
extern crate pest_derive;

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::io::Write;

use clap::App;
use clap::load_yaml;
use walkdir::{DirEntry, WalkDir};

use playlist::Playlist;

use crate::query_walk::query_walk;

mod song_tag;
mod playlist;
mod query_walk;

pub fn get_type(playlist_type: &str) -> &str {
    return match playlist_type.to_lowercase().as_ref() {
        "local" => Ok("local"),
        "spotify" => Err("Type spotify not supported yet!".to_owned()),
        "soundcloud" => Err("Type soundcloud not supported yet!".to_owned()),
        _ => Err(format!("Type {playlist_type} not supported!", playlist_type = playlist_type))
    }.unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    });
}

fn is_song(entry: &DirEntry) -> bool {
    entry.path().is_dir() ||
        entry.file_name()
            .to_str()
            .map(|s| s.ends_with("mp3"))
            .unwrap_or(false)
}

fn walk(dir: &str, ret: &mut Vec<PathBuf>) {
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| is_song(e)) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            ret.push(entry.path().to_owned());
        }
    }
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    //Nothing to do for now, all playlist must be local
    let _playlist_type = get_type(matches.value_of("type").unwrap_or("local"));

    let output = matches.value_of("output");

    let query = matches.value_of("query").unwrap();

    let input = matches.values_of("input").unwrap();
    let mut vec = Vec::new();
    for dir in input {
        if !Path::new(dir).exists() {
            println!("Folder {} does not exist!", dir);
            exit(2);
        } else {
            walk(dir, vec.borrow_mut());
        }
    }

    let mut playlist_vec = Vec::new();
    let playlists_opt = matches.values_of("playlist");
    if playlists_opt.is_some() {
        let playlists = playlists_opt.unwrap();
        for playlist in playlists {
            let path = Path::new(playlist);
            if !path.exists() || !path.extension().unwrap().eq("m3u") {
                println!("playlist {} does not exist or is invalid (not m3u)!", playlist);
                exit(2);
            } else {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let mut songs = Vec::new();
                for line in reader.lines() {
                    songs.push(PathBuf::from(line.unwrap()));
                }
                let playlist = Playlist {
                    name: path.file_stem().unwrap().to_str().unwrap().to_string(),
                    songs,
                };
                playlist_vec.push(playlist);
            }
        }
    }

    let final_playlist = query_walk(&vec, &playlist_vec, query);

    if output.is_none() {
        for song in final_playlist.iter() {
            println!("{}", song.as_path().display());
        }
    } else {
        let mut file = File::create(output.unwrap()).unwrap();
        for song in final_playlist.iter() {
            writeln!(&mut file, "{}", song.as_path().display()).unwrap();
        }
    }
}
