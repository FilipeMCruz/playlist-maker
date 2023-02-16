use std::process::exit;

use pest::iterators::Pair;
use pest::Parser;

use crate::playlist::Playlist;
use crate::query::string_extractor::{InnerStringExtractor, RuleExtractor, StringExtractor};
use crate::tag::checker::{SearchType, TagChecker};
use crate::tag::details::TagDetails;

#[derive(Parser)]
#[grammar = "query/grammar.pest"] // relative to src
pub struct ExprParser;

#[derive(Default, Debug, Eq, PartialEq)]
pub enum QueryType {
    #[default]
    Play,
    Index,
}

pub fn process(
    songs: &[TagDetails],
    playlists: &[Playlist],
    query: &str,
) -> Option<Vec<TagDetails>> {
    let mut parse_result = ExprParser::parse(Rule::query, query).ok()?;

    parse_result.next();

    filter_query_expr(songs, playlists, parse_result.next()?)
}

pub fn get_type(query: &str) -> QueryType {
    let query_type = ExprParser::parse(Rule::query, query).unwrap_or_else(|error| {
        println!("{}", error);
        exit(2);
    }).next().unwrap().as_rule();

    match query_type {
        Rule::play => QueryType::Play,
        Rule::index => QueryType::Index,
        _ => unreachable!(),
    }
}

fn filter_query_expr(
    vec: &[TagDetails],
    playlists: &[Playlist],
    query_expr: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    let mut pairs = query_expr.into_inner();
    let mut output = filter_maybe_not_token(vec, playlists, pairs.next()?)?;

    while let Some(operator) = pairs.next() {
        match operator.inner_rule()? {
            Rule::and => output = filter_maybe_not_token(&output, playlists, pairs.next()?)?,
            Rule::or => output.extend(filter_maybe_not_token(vec, playlists, pairs.next()?)?),
            _ => unreachable!(),
        }
    }
    Some(output)
}

fn filter_maybe_not_token(
    vec: &[TagDetails],
    playlists: &[Playlist],
    maybe_not_token: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    let mut pairs = maybe_not_token.into_inner();
    let first = pairs.next()?;

    match first.as_rule() {
        Rule::not => {
            let to_remove = filter_token(vec, playlists, pairs.next()?)?;
            Some(
                vec.iter()
                    .filter(|song| !to_remove.contains(song))
                    .map(|song| song.to_owned())
                    .collect(),
            )
        }
        Rule::token => filter_token(vec, playlists, first),
        _ => unreachable!(),
    }
}

fn filter_token(
    vec: &[TagDetails],
    playlists: &[Playlist],
    token: Pair<Rule>,
) -> Option<Vec<TagDetails>> {
    let pair = token.into_inner().next()?;
    match pair.as_rule() {
        Rule::playlist => {
            let first = pair.inner_str()?;
            Some(
                playlists
                    .iter()
                    .find(|&playlist| playlist.name == first)?
                    .filter(vec),
            )
        }
        Rule::tag => filter_tag(vec, pair),
        Rule::rec_token => filter_query_expr(vec, playlists, pair.into_inner().next()?),
        _ => unreachable!(),
    }
}

fn filter_tag(vec: &[TagDetails], tag: Pair<Rule>) -> Option<Vec<TagDetails>> {
    let pair = &mut tag.into_inner();

    let search_type = match pair.next()?.as_rule() {
        Rule::regex => SearchType::Regex,
        Rule::contains => SearchType::Contains,
        Rule::empty => SearchType::Literal,
        _ => unreachable!(),
    };

    let tag_type = pair.next_str()?;
    let metadata = pair.next_str()?;

    TagChecker::try_from(metadata, tag_type, search_type).map(|checker| checker.filter(vec))
}

#[cfg(test)]
mod tests {
    use crate::playlist::Playlist;
    use crate::query::processor::*;
    use crate::tag::details::TagDetails;
    use pest::Parser;

    #[test]
    fn ensure_fn_filter_tag_works_as_expected_1() {
        let rule = ExprParser::parse(Rule::tag, r#"Album("Black")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();

        let output = filter_tag(songs.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_tag_works_as_expected_2() {
        let rule = ExprParser::parse(Rule::tag, r#"C_Album("Black")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();

        let output = filter_tag(songs.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 2);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/2.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_tag_works_as_expected_3() {
        let rule = ExprParser::parse(Rule::tag, r#"R_Album(".*B.*")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();

        let output = filter_tag(songs.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 3);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/2.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/3.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_tag_works_as_expected_4() {
        let rule = ExprParser::parse(Rule::tag, r#"None(".*B.*")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();

        let output = filter_tag(songs.as_slice(), rule);

        assert!(output.is_none());
    }

    #[test]
    fn ensure_fn_filter_token_works_as_expected_1() {
        let rule = ExprParser::parse(Rule::token, r#"InPlaylist("def")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_token(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_token_works_as_expected_2() {
        let rule = ExprParser::parse(Rule::token, r#"AlbumArtist("Surf")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_token(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/3.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_token_works_as_expected_3() {
        let rule = ExprParser::parse(Rule::token, r#"InPlaylist("missing")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlist = Playlist {
            name: "def".to_string(),
            songs: vec!["test-data/songs/1.mp3".to_string()],
        };
        let playlists = vec![playlist];

        let output = filter_token(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_none());
    }

    #[test]
    fn ensure_fn_filter_token_works_as_expected_4() {
        let rule = ExprParser::parse(Rule::token, r#"(AlbumArtist("Surf") | Album("Black"))"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_token(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 2);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/3.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_maybe_not_token_works_as_expected_1() {
        let rule = ExprParser::parse(Rule::maybe_not_token, r#"!InPlaylist("def")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_maybe_not_token(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 4);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/2.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/3.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/4.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/5.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_maybe_not_token_works_as_expected_2() {
        let rule = ExprParser::parse(Rule::maybe_not_token, r#"!AlbumArtist("Surf")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_maybe_not_token(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 4);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/2.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/4.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/5.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_query_expr_works_as_expected_1() {
        let rule = ExprParser::parse(
            Rule::query_expr,
            r#"AlbumArtist("Surf") | InPlaylist("def")"#,
        )
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_query_expr(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 2);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/3.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_query_expr_works_as_expected_2() {
        let rule = ExprParser::parse(
            Rule::query_expr,
            r#"AlbumArtist("Surf") & InPlaylist("def")"#,
        )
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_query_expr(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 0);
    }

    #[test]
    fn ensure_fn_filter_query_expr_works_as_expected_3() {
        let rule = ExprParser::parse(Rule::query_expr, r#"Album("Black") & InPlaylist("def")"#)
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_query_expr(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_filter_token_works_as_expected_5() {
        let rule = ExprParser::parse(
            Rule::query_expr,
            r#"C_Album("Black") & (AlbumArtist("Surf") | Track("1"))"#,
        )
            .unwrap()
            .next()
            .unwrap();
        let songs = default_songs();
        let playlists = default_playlist();

        let output = filter_query_expr(songs.as_slice(), playlists.as_slice(), rule);

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_process_works_as_expected_1() {
        let songs = default_songs();
        let playlists = vec![];

        let output = process(
            songs.as_slice(),
            playlists.as_slice(),
            r#"Play(Album("Black"))"#,
        );

        assert!(output.is_some());
        let selected = output.unwrap();
        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
    }

    #[test]
    fn ensure_fn_get_type_works_as_expected_1() {
        let output = get_type(r#"Play(Album("Black"))"#);

        assert_eq!(output, QueryType::Play);
    }

    #[test]
    fn ensure_fn_get_type_works_as_expected_2() {
        let output = get_type(r#"Index(Album("Black"))"#);

        assert_eq!(output, QueryType::Index);
    }

    fn default_playlist() -> Vec<Playlist> {
        let playlist = Playlist {
            name: "def".to_string(),
            songs: vec!["test-data/songs/1.mp3".to_string()],
        };
        let playlists = vec![playlist];
        playlists
    }

    fn default_songs() -> Vec<TagDetails> {
        let info1 = TagDetails {
            path: "test-data/songs/1.mp3".to_string(),
            album: Some("Black".to_string()),
            track: Some("1".to_string()),
            ..Default::default()
        };
        let info2 = TagDetails {
            path: "test-data/songs/2.mp3".to_string(),
            album: Some("Also Black".to_string()),
            track: Some("3".to_string()),
            ..Default::default()
        };
        let info3 = TagDetails {
            path: "test-data/songs/3.mp3".to_string(),
            album: Some("Blue".to_string()),
            album_artist: Some("Surf".to_string()),
            ..Default::default()
        };
        let info4 = TagDetails {
            path: "test-data/songs/4.mp3".to_string(),
            album: Some("Orange".to_string()),
            ..Default::default()
        };
        let info5 = TagDetails {
            path: "test-data/songs/5.mp3".to_string(),
            artist: Some("Cap".to_string()),
            ..Default::default()
        };
        vec![info1, info2, info3, info4, info5]
    }
}
