use std::path::PathBuf;
use std::process::exit;

use pest::iterators::Pair;
use pest::Parser;

use crate::playlist::Playlist;
use crate::song_tag::{SearchType, SongTag};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct ExprParser;

pub fn query_walk(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, query: &str) -> Vec<PathBuf> {
    let parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    }).next().unwrap();

    filter_query_expr(&vec, &playlist_vec, parse_result)
}

fn filter_query_expr(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Vec<PathBuf> {
    let mut pairs = pair.into_inner();
    let mut final_songs = filter_token(&vec, &playlist_vec, pairs.next().unwrap());
    loop {
        let next = pairs.next();
        if next.is_none() {
            return final_songs;
        }
        if next.unwrap().as_str() == "&" {
            final_songs = filter_token(&final_songs, &playlist_vec, pairs.next().unwrap());
        } else {
            final_songs.extend(filter_token(&vec, &playlist_vec, pairs.next().unwrap()));
        }
    }
}

fn filter_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Vec<PathBuf> {
    let mut pairs = pair.into_inner();
    let first = pairs.next().unwrap();
    if first.as_str() == "!" { // check if it's a _not_
        let to_remove = filter_token_type(vec, playlist_vec, pairs.next().unwrap());

        return vec.into_iter()
            .filter(|song| !to_remove.contains(song))
            .map(|song| song.to_owned())
            .collect();
    } else {
        filter_token_type(vec, playlist_vec, first)
    }
}

fn filter_token_type(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Vec<PathBuf> {
    match pair.as_rule() {
        Rule::simple_token => filter_simple_token(vec, playlist_vec, pair.into_inner().next().unwrap()),
        Rule::rec_token => filter_query_expr(&vec, &playlist_vec, pair.into_inner().next().unwrap()),
        _ => {
            println!("Error parsing token: {} - {}", pair.to_string(), pair.as_str());
            exit(2);
        }
    }
}

fn filter_simple_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Vec<PathBuf> {
    match pair.as_rule() {
        Rule::playlist => {
            let first: String = pair.into_inner()
                .next()// get string
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
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
                let metadata = pair.next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                song_tag = SongTag::new(metadata, tag_type, SearchType::REGEX);
            } else if first == "C_" {
                let tag_type = pair.next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                let metadata = pair.next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                song_tag = SongTag::new(metadata, tag_type, SearchType::CONTAINS);
            } else {
                let metadata = pair.next()// get string
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                song_tag = SongTag::new(metadata, first, SearchType::LITERAL);
            }
            song_tag.filter_tag(vec)
        }
        _ => {
            println!("Error parsing filter_simple_token: {} - {}", pair.to_string(), pair.as_str());
            exit(2);
        }
    }
}
