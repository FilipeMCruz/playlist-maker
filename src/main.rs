#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate serde_derive;

mod playlist;
mod query;
mod tag;
mod utils;

use std::path::PathBuf;

use clap::Parser;
use rayon::prelude::*;

use crate::playlist::Playlist;
use crate::query::processor;
use crate::tag::details::TagDetails;
use crate::utils::fs::{get_playlists, get_songs};
use crate::utils::iter::AlmostEqualDivision;
use crate::utils::printer::{Output, Printer};

/// Create playlists using a query language
#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    ///Directory with songs or file with indexed songs to query from (can be repeated if needed)
    #[arg(short, long)]
    input: Vec<PathBuf>,
    ///File to write the query results to (if not specified send to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    ///Path to m3u playlist to be used in the query (can be repeated if needed)
    #[arg(short, long)]
    playlist: Vec<PathBuf>,
    ///Query to execute
    #[arg(short, long)]
    query: String,
}

fn main() {
    let cli = build_cli();

    let printer = build_printer(&cli);

    let outcome = filter_songs(
        cli.query,
        get_playlists(cli.playlist),
        get_songs(cli.input),
        num_cpus::get(),
    );

    printer.print(&outcome);
}

fn build_cli() -> Cli {
    Cli::parse()
}

fn build_printer(cli: &Cli) -> Printer {
    Printer {
        output: match cli.output.as_deref() {
            None => Output::Terminal,
            Some(file) => Output::File(file.into()),
        },
        print_type: processor::get_type(&cli.query),
    }
}

fn filter_songs(
    query: String,
    playlists: Vec<Playlist>,
    songs: Vec<TagDetails>,
    num_cpus: usize,
) -> Vec<TagDetails> {
    songs
        .divide_collection_by(num_cpus)
        .par_iter()
        .filter_map(|songs| processor::process(songs, &playlists, &query))
        .flatten()
        .collect::<Vec<TagDetails>>()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{build_printer, Cli, filter_songs};
    use crate::query::processor::QueryType;
    use crate::tag::details::TagDetails;
    use crate::utils::printer::Output;

    #[test]
    fn ensure_fn_build_printer_works_as_expected_1() {
        let cli = Cli {
            query: "Play(Artist('a'))".to_string(),
            input: vec![PathBuf::from("ii")],
            output: None,
            playlist: vec![],
        };
        let printer = build_printer(&cli);

        assert_eq!(printer.output, Output::Terminal);
        assert_eq!(printer.print_type, QueryType::Play);
    }

    #[test]
    fn ensure_fn_build_printer_works_as_expected_2() {
        let cli = Cli {
            query: "Index(Artist('a'))".to_string(),
            input: vec![PathBuf::from("ii")],
            output: Some(PathBuf::from("oo")),
            playlist: vec![],
        };
        let printer = build_printer(&cli);

        assert_eq!(printer.output, Output::File(PathBuf::from("oo")));
        assert_eq!(printer.print_type, QueryType::Index);
    }

    #[test]
    fn ensure_fn_process_works_as_expected_1() {
        let songs = default_songs();
        let playlists = vec![];

        let selected = filter_songs(
            r#"Play(Album("Black"))"#.to_string(),
            playlists,
            songs,
            3,
        );

        assert_eq!(selected.len(), 1);
        assert!(selected
            .iter()
            .find(|s| s.path.eq("test-data/songs/1.mp3"))
            .is_some());
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
        vec![info1, info2]
    }
}
