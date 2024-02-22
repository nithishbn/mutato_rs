use std::path::PathBuf;

use mutato_rs::insert_mutation_in_sequence;
use mutato_rs::parse_mutation;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    sequence_file: PathBuf,

    #[arg(short, long)]
    mutations_list: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.sequence_file).expect("could not read file");
    let mutations_list =
        std::fs::read_to_string(&args.mutations_list).expect("could not read mutations list file");
    let sequence_line = content.lines().next().unwrap().to_string();
    for mutation_str in mutations_list.lines() {
        let mut sequence = sequence_line.clone();
        let mutation = mutation_str.to_string();
        let (wt, pos, mutant) = parse_mutation(&mutation).unwrap();
        match insert_mutation_in_sequence(&mut sequence, &mutation) {
            Ok(()) => {}
            Err(why) => {
                println!("{why}");
            }
        };
        println!("{}", sequence);
    }
}
