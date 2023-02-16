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

    let printer = get_printer(&cli);

    let playlist_vec = get_playlists(cli.playlist);

    let all_songs = get_songs(cli.input);

    let chunks_songs = divide_songs_by_threads(all_songs, num_cpus::get());

    let final_play = filter_songs(cli.query, playlist_vec, chunks_songs);

    printer.print(&final_play);
}

fn build_cli() -> Cli {
    Cli::parse()
}

fn get_printer(cli: &Cli) -> Printer {
    Printer {
        output: match cli.output.as_deref() {
            None => Output::Terminal,
            Some(file) => Output::File(file.into()),
        },
        print_type: processor::get_type(&cli.query),
    }
}

fn divide_songs_by_threads(all_songs: Vec<TagDetails>, div: usize) -> Vec<Vec<TagDetails>> {
    if all_songs.is_empty() {
        return vec![];
    }
    all_songs
        .chunks(all_songs.len() / div)
        .map(|songs| songs.to_vec())
        .collect::<Vec<Vec<_>>>()
}

fn filter_songs(
    query: String,
    playlists: Vec<Playlist>,
    chunks_songs: Vec<Vec<TagDetails>>,
) -> Vec<TagDetails> {
    chunks_songs
        .par_iter()
        .filter_map(|songs| processor::process(songs, &playlists, &query))
        .flatten()
        .collect::<Vec<TagDetails>>()
}
