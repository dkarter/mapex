use std::io::{self, Read};
use clap::Parser;

#[derive(Parser)]
struct Cli {}

fn main() {
    let _args = Cli::parse();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from STDIN");

    let elixir_map = mapex::convert(&buffer);
    println!("{}", elixir_map);
}
