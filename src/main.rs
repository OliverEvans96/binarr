mod binary;
mod json;
mod utils;

use std::io::Read;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Opts {
    /// Complex numbers
    #[arg(short)]
    complex: bool,

    /// Double precision
    #[arg(short)]
    double: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// JSON -> Binary
    Encode,
    /// Binary -> JSON
    Decode,
}

fn read_stdin_bytes() -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    std::io::stdin().read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn read_stdin_string() -> std::io::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn main() -> anyhow::Result<()> {
    let _opts = Opts::parse();

    Ok(())
}
