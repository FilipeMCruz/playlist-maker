use std::path::PathBuf;
use std::process::exit;

use pest::iterators::Pair;
use pest::Parser;

use crate::playlist::Playlist;
use crate::song_tag::{SearchType, SongTag};
use crate::string_extractor::{InnerStringExtractor, RuleExtractor, StringExtractor};

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct ExprParser;

pub fn query_walk(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, query: &str) -> Option<Vec<PathBuf>> {
    let parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    }).next()?;

    filter_query_expr(vec, playlist_vec, parse_result)
}

fn filter_query_expr(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    let mut pairs = pair.into_inner();
    let mut final_songs = filter_token(vec, playlist_vec, pairs.next()?)?;
    loop {
        match pairs.next() {
            None => return Some(final_songs),
            Some(operator) => {
                let second = pairs.next()?;
                match operator.inner_rule()? {
                    Rule::and => final_songs = filter_token(&final_songs, playlist_vec, second)?,
                    _ => final_songs.extend(filter_token(vec, playlist_vec, second)?)
                }
            }
        }
    }
}

fn filter_token(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    let mut pairs = pair.into_inner();
    let first = pairs.next()?;

    match first.as_rule() {
        Rule::not => {
            let to_remove = filter_token_type(vec, playlist_vec, pairs.next()?)?;
            Some(vec.iter()
                .filter(|song| !to_remove.contains(song))
                .map(|song| song.to_owned())
                .collect())
        }
        _ => filter_token_type(vec, playlist_vec, first)
    }
}

fn filter_token_type(vec: &Vec<PathBuf>, playlist_vec: &Vec<Playlist>, pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    match pair.as_rule() {
        Rule::simple_token => filter_simple_token(vec, playlist_vec, pair.into_inner().next()?),
        Rule::rec_token => filter_query_expr(vec, playlist_vec, pair.into_inner().next()?),
        _ => {
            println!("Error parsing token: {} - {}", pair, pair.as_str());
            exit(2);
        }
    }
}

fn filter_simple_token(vec: &[PathBuf], playlist_vec: &[Playlist], pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    match pair.as_rule() {
        Rule::playlist => {
            let first = pair.inner_str()?;
            Some(playlist_vec.iter()
                .find(|&playlist| playlist.name == first)?
                .filter(vec))
        }
        Rule::tag => filter_songs_by_tag(vec, pair),
        _ => {
            println!("Error parsing filter_simple_token: {} - {}", pair, pair.as_str());
            exit(2);
        }
    }
}

fn filter_songs_by_tag(vec: &[PathBuf], pair: Pair<Rule>) -> Option<Vec<PathBuf>> {
    let pair = &mut pair.into_inner();

    let search_type = match pair.next()?.as_rule() {
        Rule::regex => SearchType::Regex,
        Rule::contains => SearchType::Contains,
        _ => SearchType::Literal
    };

    let tag_type = pair.next_str()?;
    let metadata = pair.next_str()?;

    SongTag::new(metadata, tag_type, search_type).filter_tag(vec)
}
