#[macro_use]
extern crate pest_derive;

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::App;
use clap::load_yaml;
use pest::Parser;
use walkdir::{DirEntry, WalkDir};

use playlist::Playlist;

use crate::operator::{Operator, OperatorType};
use crate::song_tag::SongTag;

mod song_tag;
mod operator;
mod playlist;

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

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct ExprParser;

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

    let playlist_type = get_type(matches.value_of("type").unwrap_or("local"));
    println!("Value for type: {}", playlist_type);

    let output = matches.value_of("output").unwrap_or("stdout");
    println!("Value for output: {}", output);

    let query = matches.value_of("query").unwrap();
    println!("Value for query: {}", query);

    let input = matches.values_of("input").unwrap();
    let mut inputs: String = String::new();
    let mut vec = Vec::new();
    for dir in input {
        if !Path::new(dir).exists() {
            println!("Folder {} does not exist!", dir);
            exit(2);
        } else {
            walk(dir, vec.borrow_mut());
        }
        inputs = inputs + dir + ";";
    }
    println!("Value for input: {}\nFound entries: {}", inputs, vec.len());

    let mut playlist_vec = Vec::new();
    let playlists_opt = matches.values_of("playlist");
    if playlists_opt.is_some() {
        let playlists = playlists_opt.unwrap();
        let mut playlists_str: String = String::new();
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
            playlists_str = playlists_str + playlist + ";";
        }
        println!("Value for playlists: {}\nFound entries: {}", playlists_str, playlist_vec.len());
    }

    let parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    });

    for query_expr in parse_result {
        match query_expr.as_rule() {
            Rule::query_expr => {
                let result = filter_query_expr(&vec, &playlist_vec, query_expr);
                for song in result.iter() {
                    println!("{}", song.as_path().display());
                }
            }
            _ => {
                println!("Error parsing query: {}", query_expr.as_str());
                exit(2);
            }
        }
    }
}

fn filter_query_expr(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: pest::iterators::Pair<Rule>) -> Vec<PathBuf> {
    //TODO: for now only one token is accepted
    println!("pair: {}", pair.as_str());
    let mut pairs = pair.into_inner();
    let token = pairs
        .next()//get query_expr
        .unwrap()
        .into_inner()
        .next()//get token
        .unwrap();
    let mut final_songs = filter_token(&vec, &playlist_vec, token);

    loop {
        let next = pairs.next();
        if next.is_none() {
            return final_songs;
        }
        let operator: Operator;
        if next.unwrap().as_str() == "&" {
            operator = Operator {
                operator_type: OperatorType::AND
            };
            let token = pairs.next().unwrap();
            let second_songs = filter_token(&final_songs, &playlist_vec, token.into_inner()
                .next()//get token
                .unwrap());
            final_songs = operator.filter(&final_songs, &second_songs);
        } else {
            operator = Operator {
                operator_type: OperatorType::OR
            };
            let token = pairs.next().unwrap();
            let second_songs = filter_token(&vec, &playlist_vec, token.into_inner()
                .next()//get token
                .unwrap());
            final_songs = operator.filter(&final_songs, &second_songs);
        }
    }
}

fn filter_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: pest::iterators::Pair<Rule>) -> Vec<PathBuf> {
    match pair.as_rule() {
        Rule::simple_token => {
            let mut pairs = pair.into_inner();
            let first = pairs.next().unwrap();
            if first.as_str() == "!" { // check if it's a _not_
                let second = pairs.next().unwrap(); // playlist or tag
                let to_remove = filter_simple_token(vec, playlist_vec, second);
                let mut ret_vec = Vec::new();
                for song in vec {
                    if !to_remove.contains(song) {
                        ret_vec.push(song.to_owned());
                    }
                }
                ret_vec
            } else {
                filter_simple_token(vec, playlist_vec, first)
            }
        }
        Rule::rec_token => {
            println!("rec_token: {}", pair);
            unimplemented!("No parenthesis support for now")
        }
        _ => {
            println!("Error parsing token: {} - {}", pair.to_string(), pair.as_str());
            exit(2);
        }
    }
}

fn filter_simple_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: pest::iterators::Pair<Rule>) -> Vec<PathBuf> {
    match pair.as_rule() {
        Rule::playlist => {
            let first = pair.into_inner()
                .next()// get string_literal
                .unwrap()
                .into_inner()
                .next()// get string
                .unwrap()
                .as_str().to_string();
            playlist_vec.iter()
                .find(|&playlist| playlist.name == first)
                .unwrap()
                .filter(vec)
        }
        Rule::tag => {
            let mut pair = pair.into_inner();
            let first = pair.next()
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let song_tag: SongTag;
            if first == "R_" { // check if it's a _regex_
                let tag_type = pair.next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                let metadata = pair.next()// get string_literal
                    .unwrap()
                    .into_inner()
                    .next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                song_tag = SongTag {
                    metadata,
                    tag_type,
                    is_regex: true,
                };
            } else {
                let metadata = pair.next()// get string_literal
                    .unwrap()
                    .into_inner()
                    .next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                song_tag = SongTag {
                    metadata,
                    tag_type: first,
                    is_regex: false,
                };
            }
            song_tag.filter_tag(vec)
        }
        _ => {
            println!("Error parsing filter_simple_token: {}", pair.as_str());
            exit(2);
        }
    }
}
