#![allow(non_snake_case)]
use nfamatch::Nfa;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use dfa_optimizer::{Row, Table};
use log::*;

use log::*;

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

// cargo run -- --file float.nfa --out out.dfa
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let args = Args::from_args();

    // TODO: Read Rows and create separate NFA row type.
    info!("Creating NFA table from: {}", args.file.display());
    let nfa: Nfa = Nfa::from_file(args.file)?;
    let mut table = nfa.to_dfa();
    info!("Optimizing DFA table");
    table.optimize();

    info!("Checking tokens");
    for input in args.rest {
        info!("Checking `{}`", input);
        match table.does_match(&input, nfa.character_map()) {
            None => println!("OUTPUT :M:"),
            Some(i) => println!("OUTPUT {}", i),
        }
    }

    info!("Writing output file: {}", args.out.display());
    let output_file = File::create(args.out)?;
    let mut writer = BufWriter::new(output_file);
    for row in table.rows() {
        writeln!(writer, "{}", row)?;
    }

    writer.flush()?;

    Ok(())
}
