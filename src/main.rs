use clap::Parser;
use std::io::{self, BufRead};

mod tree_generator;
use tree_generator::{generate_tree_from_paths, Options};

#[derive(clap::Args, Debug)]
struct Opts {
    #[arg(short, long)]
    pub compact: bool,
    #[arg(long)]
    pub color: bool,
    #[arg(long, name = "no-color")]
    pub no_color: bool,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    options: Opts,
}

fn main() {
    let args = Args::parse();
    let opts = Options {
        compact: args.options.compact,
        color: args.options.color || !args.options.no_color,
    };
    let paths = io::stdin().lock().lines().map_while(Result::ok).collect();

    print!("{}", generate_tree_from_paths(&paths, &opts));
}
