use anyhow::Result;
use colored::Colorize;
use std::fs::File;
use std::io::Write;

use crate::types::*;

pub fn print_human_readable(report: &AnalysisReport) {
    println!("\n{}", "═══════════════════════════════════════════════════════".bright_cyan().bold());
    println!("{}", "          CHESSLUCENT BLUNDER ANALYSIS REPORT          ".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════".bright_cyan().bold());
    
    println!("\n{}", "📋 GAME INFORMATION".bright_yellow().bold());
    println!("   White: {}", report.game_info.white.bright_white());
    println!("   Black: {}", report.game_info.black.bright_white());
    println!("   Result: {}", report.game_info.result.bright_white());
    println!("   Date: {}", report.game_info.date.bright_white());
    
    println!("\n{}", "📊 STATISTICS".bright_yellow().bold());
    println!("   Total Moves: {}", report.statistics.total_moves);
    println!("   {} {}", "Blunders:".red().bold(), report.statistics.total_blunders);
    println!("   {} {}", "Mistakes:".yellow().bold(), report.statistics.total_mistakes);
    println!("   {} {}", "Inaccuracies:".bright_blue().bold(), report.statistics.total_inaccuracies);
    
    if report.blunders.is_empty() {
        println!("\n{}", "🎉 NO BLUNDERS FOUND! You played perfectly! (Or the thresholds are too high...)".green().bold());
    } else {
        println!("\n{}", "😱 BLUNDERS DETECTED".red().bold());
        println!("{}", "───────────────────────────────────────────────────────".bright_black());
        
        for blunder in &report.blunders {
            let severity_color = match blunder.severity {
                Severity::Blunder => blunder.severity.as_str().red().bold(),
                Severity::Mistake => blunder.severity.as_str().yellow().bold(),
                Severity::Inaccuracy => blunder.severity.as_str().bright_blue().bold(),
            };
            
            println!("\n   Move {}: {} played {}", 
                blunder.move_number.to_string().bright_white().bold(),
                blunder.side.bright_magenta(),
                blunder.san.bright_white().bold()
            );
            println!("   Severity: {}", severity_color);
            println!("   Evaluation: {} cp → {} cp (loss: {} cp)", 
                blunder.eval_before, 
                blunder.eval_after, 
                format!("-{}", blunder.cp_diff).red()
            );
            println!("   FEN: {}", blunder.fen_before.bright_black());
        }
    }
    
    println!("\n{}", "═══════════════════════════════════════════════════════".bright_cyan().bold());
}

pub fn save_json(report: &AnalysisReport, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    println!("\n💾 JSON report saved to: {}", path.bright_green());
    Ok(())
}
