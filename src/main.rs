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

    let lines: Vec<String> = io::stdin().lock().lines().map_while(Result::ok).collect();

    // Heuristic to check if the input is likely from `git status --porcelain`
    // It checks for two status characters followed by a space, e.g., "M  file.txt"
    let is_porcelain_output = lines.first().map_or(false, |line| {
        if line.len() < 4 {
            return false;
        }
        let status_part = &line[..2];
        let separator = &line[2..3];
        // Status part should not be empty after trim, and separator must be a space.
        !status_part.trim().is_empty() && separator == " "
    });

    let paths_with_status: Vec<(String, String)> = if is_porcelain_output {
        lines
            .iter()
            .filter_map(|line| {
                if line.len() < 4 {
                    return None;
                }
                let status_str = &line[..2];
                let path_str = &line[3..];

                let status = status_str.trim();
                if status.is_empty() {
                    return None;
                }

                // For renames "R  old -> new", we want to display the new path
                if status.starts_with('R') {
                    if let Some(separator) = path_str.find(" -> ") {
                        let new_path = path_str.split_at(separator + 4).1;
                        return Some((new_path.to_string(), status.to_string()));
                    }
                }
                Some((path_str.to_string(), status.to_string()))
            })
            .collect()
    } else {
        lines
            .iter()
            .map(|line| (line.clone(), String::new()))
            .collect()
    };

    print!("{}", generate_tree_from_paths(&paths_with_status, &opts));
}
