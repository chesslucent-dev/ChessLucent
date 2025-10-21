mod engine;
mod analyzer;
mod types;
mod report;

use clap::Parser;
use anyhow::Result;
use colored::Colorize;

use analyzer::Analyzer;
use report::{print_human_readable, save_json};

#[derive(Parser)]
#[command(name = "ChessLucent")]
#[command(author = "ChessLucent Developers")]
#[command(version = "0.1.0")]
#[command(about = "Making your blunders look slightly less idiotic. For free.", long_about = None)]
struct Cli {
    /// Path to the PGN file to analyze
    #[arg(short, long)]
    pgn: String,

    /// Path to the chess engine executable (e.g., stockfish)
    #[arg(short, long, default_value = "stockfish")]
    engine: String,

    /// Analysis depth (higher = more accurate but slower)
    #[arg(short, long, default_value_t = 15)]
    depth: u8,

    /// Output JSON file path
    #[arg(short, long)]
    out: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{}", "
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║              ♔  CHESSLUCENT BLUNDERSCOPE  ♚              ║
    ║                                                           ║
    ║        Making your blunders look slightly less           ║
    ║                    idiotic. For free.                     ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
    ".bright_cyan().bold());

    println!("\n🔍 Analyzing game: {}", cli.pgn.bright_white());
    println!("🤖 Using engine: {} (depth {})", cli.engine.bright_white(), cli.depth);
    println!("\n⏳ This may take a moment...\n");

    let mut analyzer = Analyzer::new(&cli.engine, cli.depth)?;
    let report = analyzer.analyze_pgn(&cli.pgn)?;

    // Print human-readable report
    print_human_readable(&report);

    // Save JSON if requested
    if let Some(out_path) = cli.out {
        save_json(&report, &out_path)?;
    }

    Ok(())
}
