mod binary;
mod json;
mod utils;

use std::io::{Read, Write};

use binary::{encode_complex_vector, encode_real_vector};
use clap::{Args, Parser, Subcommand};
use num::complex::{Complex32, Complex64};

use crate::binary::{decode_complex_vector, decode_real_vector};

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Args, Clone, Copy, Debug, PartialEq, Eq)]
struct Opts {
    /// Complex numbers
    #[arg(short)]
    complex: bool,

    /// Double precision
    #[arg(short)]
    double: bool,
}

#[derive(Debug, Clone, Copy, Subcommand)]
enum Command {
    /// JSON -> Binary
    Encode(Opts),
    /// Binary -> JSON
    Decode(Opts),
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

fn encode(opts: Opts) -> anyhow::Result<()> {
    let input = read_stdin_string()?;
    let bytes = match opts {
        Opts {
            complex: false,
            double: false,
        } => {
            let arr: Vec<f32> = serde_json::from_str(&input)?;
            encode_real_vector(&arr)
        }
        Opts {
            complex: false,
            double: true,
        } => {
            let arr: Vec<f64> = serde_json::from_str(&input)?;
            encode_real_vector(&arr)
        }
        Opts {
            complex: true,
            double: false,
        } => {
            let arr: Vec<Complex32> = serde_json::from_str(&input)?;
            encode_complex_vector(&arr)
        }
        Opts {
            complex: true,
            double: true,
        } => {
            let arr: Vec<Complex64> = serde_json::from_str(&input)?;
            encode_complex_vector(&arr)
        }
    };

    std::io::stdout().write_all(&bytes)?;

    Ok(())
}

fn decode(opts: Opts) -> anyhow::Result<()> {
    let input = read_stdin_bytes()?;
    let output = match opts {
        Opts {
            complex: false,
            double: false,
        } => {
            let arr: Vec<f32> = decode_real_vector(&input)?;
            serde_json::to_string(&arr)?
        }
        Opts {
            complex: false,
            double: true,
        } => {
            let arr: Vec<f64> = decode_real_vector(&input)?;
            serde_json::to_string(&arr)?
        }
        Opts {
            complex: true,
            double: false,
        } => {
            let arr: Vec<Complex32> = decode_complex_vector(&input)?;
            serde_json::to_string(&arr)?
        }
        Opts {
            complex: true,
            double: true,
        } => {
            let arr: Vec<Complex64> = decode_complex_vector(&input)?;
            serde_json::to_string(&arr)?
        }
    };

    println!("{}", output);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode(opts) => encode(opts)?,
        Command::Decode(opts) => decode(opts)?,
    }

    Ok(())
}
