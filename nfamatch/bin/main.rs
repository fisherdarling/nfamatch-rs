#![allow(non_snake_case)]
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use structopt::StructOpt;

<<<<<<< HEAD
use dfa::{Row, Table};
use nfa;
=======
use dfa_optimizer::{Row, Table};
>>>>>>> 177587e73486bc423eb3252e1f11c1ee0f8934aa

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

    let input_file = File::open(args.file)?;
    let output_file = File::open(args.out)?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

<<<<<<< HEAD
    let nfa = nfa::from(reader);
    let mut table = nfa.to_dfa();
=======
    // TODO: Read Rows and create separate NFA row type.
    let rows: Vec<Row> = Vec::new();
    let mut table = Table::from(rows);
>>>>>>> 177587e73486bc423eb3252e1f11c1ee0f8934aa
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

    writer.flush()?;

    Ok(())
}