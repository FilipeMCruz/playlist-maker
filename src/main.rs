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
    chunks_songs: Vec<TagDetails>,
    num_cpus: usize,
) -> Vec<TagDetails> {
    chunks_songs
        .divide_collection_by(num_cpus)
        .par_iter()
        .filter_map(|songs| processor::process(songs, &playlists, &query))
        .flatten()
        .collect::<Vec<TagDetails>>()
}
