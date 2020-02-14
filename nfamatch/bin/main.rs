#![allow(non_snake_case)]
use nfamatch::Nfa;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use dfa_optimizer::{Row, Table};

use log::*;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    /// Path to input file.
    file: PathBuf,
    /// Path to output DFA,
    out: PathBuf,
    /// Path to output the optimized DFA
    rest: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let args = Args::from_args();

    info!("Creating NFA from: {}", args.file.display());
    let nfa = Nfa::from_file(args.file);

    info!("Opening output file: {}", args.out.display());
    let output_file = File::open(args.out)?;
    let mut writer = BufWriter::new(output_file);

    info!("Creating DFA");
    let mut table = nfa.to_dfa();

    info!("Optimizing DFA");
    table.optimize();

    info!("Matching input tokens");
    for input in args.rest {
        debug!("TOKEN: {}", input);
        match table.does_match(&input, nfa.character_map()) {
            None => println!("OUTPUT :M:"),
            Some(i) => println!("OUTPUT {}", i),
        }
    }

    info!("Writing NFA to output file");
    for row in table.rows() {
        writeln!(writer, "{}", row)?;
    }

    writer.flush()?;

    Ok(())
}
