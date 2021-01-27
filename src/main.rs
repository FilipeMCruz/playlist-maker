use clap::App;
use clap::load_yaml;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let playlist_type = matches.value_of("type").unwrap_or("local");
    println!("Value for type: {}", playlist_type);

    let output = matches.value_of("output").unwrap_or("stdout");
    println!("Value for output: {}", output);

    let query = matches.value_of("query").unwrap();
    println!("Value for query: {}", query);

    let input = matches.value_of("input").unwrap();
    println!("Value for input: {}", input);
}
