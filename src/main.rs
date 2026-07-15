#![allow(non_snake_case)]
mod steg;
use steg::tencryptimage;
use steg::imagedecrypttext;

// default clap template for making the code into a cli
// basically copy paste from https://docs.rs/clap/latest/clap/
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt {
            // if encrypt take 3 inputs
            input,
            output,
            text,
        } => {
            match tencryptimage(&text, &input, &output) {
                Ok(_) => println!("Encrypted successfully"),
                Err(err) => eprintln!("Error: {err}"),
            }
        }

        Commands::Decrypt {
            // if decrypt take 1 input
            input,
        } => {
            match imagedecrypttext(&input) {
                Ok(text) => println!("{text}"),
                Err(err) => eprintln!("Error: {err}"),
            }
        }
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        input: String,
        output: String,
        text: String,
    },
    Decrypt {
        input: String,
    },
}
