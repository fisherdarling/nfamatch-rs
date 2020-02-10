#![allow(non_snake_case)]
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use structopt::StructOpt;

use dfa_optimizer::{Row, Table};

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

    // TODO: Read Rows and create separate NFA row type.
    let rows: Vec<Row> = Vec::new();
    let mut table = Table::from(rows);
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
