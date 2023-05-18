#![allow(unused)]

use std::io::{self, Write};
use std::time::Duration;
use anyhow::{Context, Result};
use std::thread;
use clap::Parser;
use indicatif::{ProgressBar, ProgressIterator, ProgressState, ProgressStyle};


#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {

    // arg parser
    let args = Cli::parse();

    // console write buffer
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();


    writeln!(stdout_handle, "Your pattern is: {pattern:?} & path is {path:?}\n",
             pattern=args.pattern, path=args.path);
    writeln!(stdout_handle, "Initiating Search:");

    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => {content}
    //     Err(error) => {panic!("Error: {}", error);}
    // };

    // let content = std::fs::read_to_string(&args.path).unwrap();
    // let path = args.path;
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;

    let mut res = rust_grepClone::find_matches(content, &args.pattern);

    writeln!(stdout_handle, "\nHere are the matching results: \n------------------------------\n");

    for line in &res {
        writeln!(stdout_handle, "{}", line);
    }


    Ok(())
}
