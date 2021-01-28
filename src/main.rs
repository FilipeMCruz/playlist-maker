#[macro_use]
extern crate pest_derive;

use std::path::{Path, PathBuf};
use std::process::exit;

use clap::App;
use clap::load_yaml;
use pest::Parser;
use walkdir::{DirEntry, WalkDir};

use playlist::Playlist;

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
            .map(|s| s.ends_with("mp3") || s.ends_with("m3u"))
            .unwrap_or(false)
}

fn walk(dir: &str) -> Vec<PathBuf> {
    let mut ret = Vec::new();
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| is_song(e)) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            ret.push(entry.path().to_owned());
        }
    }
    ret
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
            vec = walk(dir);
        }
        inputs = inputs + dir + ";";
    }
    println!("Value for input: {}\nFound entries: {}", inputs, vec.len());

    let parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    });

    for token_2 in parse_result {
        match token_2.as_rule() {
            Rule::token_2 => {
                println!("{}", token_2);
                //TODO: for now only one token is accepted
                let token = token_2.into_inner()
                    .next() //get token_2
                    .unwrap()
                    .into_inner()
                    .next() //get first token
                    .unwrap();
                let vec = filter_token(&vec, token);
                for song in vec.iter() {
                    println!("{}", song.as_path().display());
                }
            }
            _ => {
                println!("Error parsing query: {}", token_2.as_str());
                exit(2);
            }
        }
    }
}


fn filter_token(vec: &Vec<PathBuf>, pair: pest::iterators::Pair<Rule>) -> Vec<PathBuf> {
    match pair.as_rule() {
        Rule::complex_token => {
            let mut pairs = pair.into_inner();
            let first = pairs.next().unwrap();
            if first.as_str() == "!" { // check if it's a _not_
                let second = pairs.next().unwrap(); // playlist or tag
                let to_remove = filter_simple_token(vec, second);
                let mut ret_vec = Vec::new();
                for song in vec {
                    if !to_remove.contains(song) {
                        ret_vec.push(song.to_owned());
                    }
                }
                ret_vec
            } else {
                filter_simple_token(vec, first)
            }
        }
        Rule::rec_token => {
            println!("{}", pair);
            unimplemented!("No parenthesis support for now")
        }
        _ => {
            println!("Error parsing filter_token: {} - {}", pair.to_string(), pair.as_str());
            exit(2);
        }
    }
}

fn filter_simple_token(vec: &Vec<PathBuf>, pair: pest::iterators::Pair<Rule>) -> Vec<PathBuf> {
    match pair.as_rule() {
        Rule::playlist => {
            let first = pair.into_inner().next().unwrap();
            println!("{}", first);
            // get playlist content and parse it to vec<str>
            let playlist = Playlist {
                songs: Vec::new()
            };
            playlist.filter(vec)
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
