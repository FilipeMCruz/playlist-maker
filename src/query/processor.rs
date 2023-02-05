use std::process::exit;

use pest::iterators::Pair;
use pest::Parser;

use crate::query::string_extractor::{InnerStringExtractor, RuleExtractor, StringExtractor};
use crate::song::info::SongInfo;
use crate::song::playlist::Playlist;
use crate::tag::checker::{SearchType, TagChecker};
use crate::tag::details::TagDetails;

#[derive(Parser)]
#[grammar = "query/grammar.pest"] // relative to src
pub struct ExprParser;

pub fn process(vec: &[SongInfo], playlist_vec: &Vec<Playlist>, query: &str) -> Option<Vec<String>> {
    let mut parse_result = ExprParser::parse(Rule::query, query).ok()?;

    let export = parse_result.next()?.as_rule();

    let details_vec = vec
        .iter()
        .filter_map(|e| e.extract_info())
        .collect::<Vec<TagDetails>>();

    let songs = filter_query_expr(&details_vec, playlist_vec, parse_result.next()?);

    match export {
        Rule::play => Some(songs?.iter().map(|song| song.path.clone()).collect()),
        Rule::index => Some(songs?.iter().map(|tag| tag.details()).collect()),
        _ => None,
    }
}

pub fn is_play(query: &str) -> bool {
    let mut parse_result = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    });

    matches!(parse_result.next().unwrap().as_rule(), Rule::play)
}

fn filter_query_expr(
    vec: &Vec<TagDetails>,
    playlist_vec: &Vec<Playlist>,
    pair: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    let mut pairs = pair.into_inner();
    let mut final_songs = filter_token(vec, playlist_vec, pairs.next()?)?;
    loop {
        match pairs.next() {
            None => return Some(final_songs),
            Some(operator) => match operator.inner_rule()? {
                Rule::and => final_songs = filter_token(&final_songs, playlist_vec, pairs.next()?)?,
                _ => final_songs.extend(filter_token(vec, playlist_vec, pairs.next()?)?),
            },
        }
    }
}

fn filter_token(
    vec: &Vec<TagDetails>,
    playlist_vec: &Vec<Playlist>,
    pair: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    let mut pairs = pair.into_inner();
    let first = pairs.next()?;

    match first.as_rule() {
        Rule::not => {
            let to_remove = filter_token_type(vec, playlist_vec, pairs.next()?)?;
            Some(
                vec.iter()
                    .filter(|song| !to_remove.contains(song))
                    .map(|song| song.to_owned())
                    .collect(),
            )
        }
        _ => filter_token_type(vec, playlist_vec, first),
    }
}

fn filter_token_type(
    vec: &Vec<TagDetails>,
    playlist_vec: &Vec<Playlist>,
    pair: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    match pair.as_rule() {
        Rule::simple_token => filter_simple_token(vec, playlist_vec, pair.into_inner().next()?),
        Rule::rec_token => filter_query_expr(vec, playlist_vec, pair.into_inner().next()?),
        _ => {
            println!("Error parsing token: {} - {}", pair, pair.as_str());
            exit(2);
        }
    }
}

fn filter_simple_token(
    vec: &[TagDetails],
    playlist_vec: &[Playlist],
    pair: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    match pair.as_rule() {
        Rule::playlist => {
            let first = pair.inner_str()?;
            Some(
                playlist_vec
                    .iter()
                    .find(|&playlist| playlist.name == first)?
                    .filter(vec),
            )
        }
        Rule::tag => filter_songs_by_tag(vec, pair),
        _ => {
            println!(
                "Error parsing filter_simple_token: {} - {}",
                pair,
                pair.as_str()
            );
            exit(2);
        }
    }
}

fn filter_songs_by_tag(vec: &[TagDetails], pair: Pair<Rule>) -> Option<Vec<TagDetails>> {
    let pair = &mut pair.into_inner();

    let search_type = match pair.next()?.as_rule() {
        Rule::regex => SearchType::Regex,
        Rule::contains => SearchType::Contains,
        _ => SearchType::Literal,
    };

    let tag_type = pair.next_str()?;
    let metadata = pair.next_str()?;

    TagChecker::new(metadata, tag_type, search_type).map(|checker| checker.filter(vec))
}
