use std::path::Path;
use std::process::exit;

use clap::App;
use clap::load_yaml;

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
    for dir in input {
        if !Path::new(dir).exists() {
            println!("Folder {} does not exist!", dir);
            exit(2);
        }
        inputs = inputs + dir + ";";
    }
    println!("Value for input: {}", inputs);
}
