use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
struct Cli {
    // Pretty-print the output
    #[arg(short, long)]
    pretty: Option<bool>,
    // convert keys to atoms
    #[arg(short, long)]
    atom_keys: Option<bool>,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            pretty: Some(true),
            atom_keys: Some(false),
        }
    }
}

fn main() {
    let parsed_args = Cli::parse();
    let default_args = Cli::default();

    let args = Cli {
        pretty: parsed_args.pretty.or(default_args.pretty),
        atom_keys: parsed_args.atom_keys.or(default_args.pretty),
    };

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from STDIN");

    let elixir_map = mapex::convert(
        &buffer,
        mapex::ConvertOptions {
            pretty: args.pretty,
            atom_keys: args.atom_keys,
        },
    );
    println!("{}", elixir_map);
}
