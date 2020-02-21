use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use structopt::StructOpt;

use dfa_optimizer::{Row, Table};

use std::collections::BTreeMap;

/// dfa reads in a formatted DFA file and spits
/// out an optimized form of given DFA.
#[derive(Debug, Clone, StructOpt)]
struct Args {
    /// Print the input and output DFAs.
    #[structopt(short, long)]
    verbose: bool,
    /// Path to DFA to optimize
    #[structopt(short, long)]
    file: PathBuf,
    /// Path to output the optimized DFA
    #[structopt(short, long)]
    out: PathBuf,
    /// The alphabet of our DFA
    #[structopt(short, long)]
    alphabet: Vec<char>,
    /// Tokens to match the DFA against
    #[structopt(short, long)]
    tokens: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let args = Args::from_args();

    let file = File::open(args.file)?;
    let reader = BufReader::new(file);

    let rows: Vec<Row> = reader
        .lines()
        .flatten()
        .map(|r| r.parse().unwrap())
        .collect();

    let mut table = Table::from(rows);

    if args.verbose {
        println!("Input DFA:");
        print!("{}", table);
    }

    table.optimize();

    if args.verbose {
        println!("\nOptimal DFA:");
        print!("{}", table);
        println!();
    }

    // args.alphabet.sort();
    let mut mapping = BTreeMap::new();

    for (i, c) in args.alphabet.iter().enumerate() {
        mapping.insert(c.clone(), i);
    }

    if args.verbose {
        println!("Alphabet: {:?}", args.alphabet);
        println!("Tokens: {:?}", args.tokens);
        println!("Mapping: {:#?}", mapping);
        println!();
    }

    for token in args.tokens {
        println!("{}: {:?}", token, table.does_match(&token, &mapping));
    }

    let new_file = File::create(args.out)?;
    let mut writer = BufWriter::new(new_file);

    for row in table.rows() {
        writeln!(writer, "{}", row)?;
    }

    writer.flush()?;

    Ok(())
}
