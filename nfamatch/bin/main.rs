#![allow(non_snake_case)]
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use structopt::StructOpt;

use dfa::{Row, Table};
use nfa;

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
    let args = Args::from_args();

    let file = File::open(args.file)?;
    let out = File::open(args.out)?;
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(out);

    let nfa = nfa::from(reader);
    let mut table = nfa.to_dfa();
    table.optimize();

    for input in args.rest {
        match table.does_match(input) {
            None => println!("OUTPUT :M:"),
            Some(i) => println!("OUTPUT {}", i),
        }
    }
    
    for row in table.rows() {
        writeln!(writer, "{}", row)?;
    }

    Ok(())
}