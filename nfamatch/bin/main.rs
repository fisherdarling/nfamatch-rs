#![allow(non_snake_case)]
use nfamatch::Nfa;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use dfa_optimizer::{Row, Table};

#[derive(Debug, Clone, StructOpt)]
struct Args {
    /// Path to input file.
    #[structopt(short, long)]
    file: PathBuf,
    /// Path to output DFA,
    #[structopt(short, long)]
    out: PathBuf,
    /// Path to output the optimized DFA
    #[structopt(short, long)]
    rest: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();

    let output_file = File::open(args.out)?;
    let mut writer = BufWriter::new(output_file);

    // TODO: Read Rows and create separate NFA row type.
    let nfa: Nfa = Nfa::from_file(args.file).expect("Unable to read input file");

    let mut table = nfa.to_dfa();
    table.optimize();

    for input in args.rest {
        match table.does_match(&input, nfa.character_map()) {
            None => println!("OUTPUT :M:"),
            Some(i) => println!("OUTPUT {}", i),
        }
    }

    for row in table.rows() {
        writeln!(writer, "{}", row)?;
    }

    writer.flush()?;

    Ok(())
}
