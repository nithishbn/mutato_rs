use clap::Parser;
use mutato_rs::insert_mutation_in_sequence;
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
    mutations_list: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.sequence_file).expect("could not read file");
    let mutations_list =
        std::fs::read_to_string(&args.mutations_list).expect("could not read mutations list file");
    let sequence_line = content.lines().next().unwrap().to_string();
    let path = args.output;
    let mut f = File::create(&path).unwrap();
    for mutation_str in mutations_list.lines() {
        let mut sequence = sequence_line.clone();
        let mutation = mutation_str.to_string();
        match insert_mutation_in_sequence(&mut sequence, &mutation) {
            Ok(()) => {
                write_to_file(&sequence, &mut f).unwrap_or_else(|why| {
                    error!("! {:?}", why.kind());
                });
            }
            Err(why) => {
                error!("{why}");
            }
        };
        info!("{}", sequence);
    }
}

fn write_to_file(s: &str, f: &mut File) -> io::Result<()> {
    f.write_all(format!("{}\n", s).as_bytes())
}
