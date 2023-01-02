use anony_run::run;
use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to execute
    #[arg(short, long)]
    path: String,

    /// Size of the file to execute (MB)
    #[arg(short, long)]
    size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut file = File::open(args.path.as_str())?;
    let mut buf = vec![0; args.size * 1024 * 1024];
    let len = file.read(&mut buf)?;

    run(&buf[..len])
}
