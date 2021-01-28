mod song_tag;
mod operator;
mod playlist;

#[macro_use]
extern crate pest_derive;

use std::borrow::BorrowMut;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::App;
use clap::load_yaml;
use pest::Parser;
use walkdir::{DirEntry, WalkDir};

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

fn walk(dir: &str, vec: &mut Vec<PathBuf>) {
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| is_song(e)) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            vec.push(entry.path().to_owned());
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

    let parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    });
}
