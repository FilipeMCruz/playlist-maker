use std::path::PathBuf;
use std::process::exit;

use pest::iterators::Pair;
use pest::Parser;

use crate::playlist::Playlist;
use crate::song_tag::{SearchType, SongTag};
use crate::pair_extended::{ExtendedRulePair, ExtendedRulePairs};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct ExprParser;

pub fn query_walk(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, query: &str) -> Option<Vec<PathBuf>> {
    let parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    }).next()?;

    filter_query_expr(&vec, &playlist_vec, parse_result)
}

fn filter_query_expr(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    let mut pairs = pair.into_inner();
    let mut final_songs = filter_token(&vec, &playlist_vec, pairs.next()?)?;
    loop {
        let next = pairs.next();
        if next.is_none() {
            return Some(final_songs);
        }
        if next.unwrap().as_str() == "&" {
            final_songs = filter_token(&final_songs, &playlist_vec, pairs.next()?)?;
        } else {
            final_songs.extend(filter_token(&vec, &playlist_vec, pairs.next()?)?);
        }
    }
}

fn filter_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    let mut pairs = pair.into_inner();
    let first = pairs.next()?;

    match first.as_rule() {
        Rule::not => {
            let to_remove = filter_token_type(vec, playlist_vec, pairs.next()?)?;

            Some(vec.into_iter()
                .filter(|song| !to_remove.contains(song))
                .map(|song| song.to_owned())
                .collect())
        }
        _ => {
            filter_token_type(vec, playlist_vec, first)
        }
    }
}

fn filter_token_type(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    match pair.as_rule() {
        Rule::simple_token => filter_simple_token(vec, playlist_vec, pair.into_inner().next()?),
        Rule::rec_token => filter_query_expr(vec, playlist_vec, pair.into_inner().next()?),
        _ => {
            println!("Error parsing token: {} - {}", pair.to_string(), pair.as_str());
            exit(2);
        }
    }
}

fn filter_simple_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    match pair.as_rule() {
        Rule::playlist => {
            let first = pair.inner_str()?;
            Some(playlist_vec.iter()
                .find(|&playlist| playlist.name == first)?
                .filter(vec))
        }
        Rule::tag => {
            filter_songs_by_tag(vec, pair)
        }
        _ => {
            println!("Error parsing filter_simple_token: {} - {}", pair.to_string(), pair.as_str());
            exit(2);
        }
    }
}

fn filter_songs_by_tag(vec: &Vec<PathBuf>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    let pair = &mut pair.into_inner();
    let first = pair.next_str()?;

    match first.as_str() {
        "R_" => {
            let tag_type = pair.next_str()?;
            let metadata = pair.next_str()?;
            SongTag::new(metadata, tag_type, SearchType::REGEX).filter_tag(vec)
        }
        "C_" => {
            let tag_type = pair.next_str()?;
            let metadata = pair.next_str()?;
            SongTag::new(metadata, tag_type, SearchType::CONTAINS).filter_tag(vec)
        }
        _ => {
            let metadata = pair.next_str()?;
            SongTag::new(metadata, first, SearchType::LITERAL).filter_tag(vec)
        }
    }
}
