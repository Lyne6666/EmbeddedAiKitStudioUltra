// src/main.rs
/*
 * Main executable for EmbeddedAiKitStudioUltra
 */

use clap::Parser;
use embeddedaikitstudioultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "EmbeddedAiKitStudioUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
