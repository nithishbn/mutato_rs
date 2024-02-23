use clap::Parser;
use indicatif::ParallelProgressIterator;
use mutato_rs::{generate_all_mutations_given_a_sequence, insert_mutation_in_sequence};
use rayon::prelude::*;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs::File, io};
use tracing::error;
use tracing::info;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    sequence_file: PathBuf,

    #[arg(short, long)]
    mutations_file: Option<PathBuf>,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.sequence_file).expect("could not read file");
    let path = args.output;
    let mutations_list_option: Option<Vec<String>> =
        if let Some(mutations_file) = args.mutations_file {
            let mutations_file_lines = std::fs::read_to_string(mutations_file)
                .expect("could not read mutations list file");
            Some(
                mutations_file_lines
                    .lines()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .to_vec(),
            )
        } else {
            None
        };
    let file_contents: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mutated_sequences: Vec<String> = file_contents
        .par_iter()
        .progress_count(file_contents.len() as u64)
        .flat_map(|sequence| {
            let sequence_line = sequence.to_string();
            let mutations_list = mutations_list_option
                .as_ref()
                .cloned()
                .unwrap_or_else(|| generate_all_mutations_given_a_sequence(&sequence.to_string()));
            mutations_list
                .par_iter()
                .filter_map(move |mutation| {
                    let mut sequence = sequence_line.clone();
                    if let Err(why) = insert_mutation_in_sequence(&mut sequence, mutation) {
                        error!("{}", why); // Corrected logging
                        None
                    } else {
                        Some(sequence)
                    }
                })
                .collect::<Vec<_>>() // Collect into Vec to extend the lifetime
        })
        .collect();

    let mut f = File::create(path.clone()).expect("could not create file");
    for (seq_index, mutated_sequence) in mutated_sequences.iter().enumerate() {
        if let Err(why) = write_to_file(mutated_sequence, &mut f) {
            error!("! {:?}", why.kind());
        }
        info!("{}", seq_index + 1);
    }
}

fn write_to_file(s: &str, f: &mut File) -> io::Result<()> {
    f.write_all(format!("{}\n", s).as_bytes())
}
