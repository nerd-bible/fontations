//! binary subset tool
//!
//! Takes a font file and a subset input which describes the desired subset, and output is a new
//! font file containing only the data specified in the input.
//!

use clap::Parser;
use klippa::{subset_bytes, Args};

#[derive(Parser, Debug)]
// Allow name_IDs, so we keep the option name consistent with HB and fonttools
#[allow(non_snake_case)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// The input font file.
    #[arg(short, long)]
    path: std::path::PathBuf,

    /// The output font file
    #[arg(short, long)]
    output_file: std::path::PathBuf,

    #[command(flatten)]
    args: Args,
}

fn main() {
    let args = CliArgs::parse();

    let font_bytes = std::fs::read(&args.path)
        .unwrap_or_else(|err| panic!("Failed to read file {path:?}.\n{err}", path = &args.path));
    let bytes = match subset_bytes(font_bytes, args.args) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    std::fs::write(&args.output_file, bytes).unwrap_or_else(|err| {
        panic!(
            "Failed to write output to {path:?}.\n{err}",
            path = &args.output_file
        )
    });
}
