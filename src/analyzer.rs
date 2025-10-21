use anyhow::{Result, Context};
use shakmaty::{Chess, Position, fen::Fen};
use shakmaty::san::San;
use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus, RawHeader};
use std::fs::File;
use std::io::BufReader;

use crate::engine::Engine;
use crate::types::*;

struct GameVisitor {
    moves: Vec<SanPlus>,
    headers: Vec<(String, String)>,
}

impl GameVisitor {
    fn new() -> Self {
        GameVisitor {
            moves: Vec::new(),
            headers: Vec::new(),
        }
    }
}

impl Visitor for GameVisitor {
    type Result = ();

    fn begin_game(&mut self) {
        self.moves.clear();
        self.headers.clear();
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        let key_str = String::from_utf8_lossy(key).to_string();
        let value_str = String::from_utf8_lossy(value.as_bytes()).to_string();
        self.headers.push((key_str, value_str));
    }

    fn san(&mut self, san: SanPlus) {
        self.moves.push(san);
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // Skip variations
    }

    fn end_game(&mut self) {}
}

pub struct Analyzer {
    engine: Engine,
    depth: u8,
}

impl Analyzer {
    pub fn new(engine_path: &str, depth: u8) -> Result<Self> {
        let engine = Engine::new(engine_path)?;
        Ok(Analyzer { engine, depth })
    }

    pub fn analyze_pgn(&mut self, pgn_path: &str) -> Result<AnalysisReport> {
        // Parse PGN
        let file = File::open(pgn_path).context("Failed to open PGN file")?;
        let reader = BufReader::new(file);
        let mut reader = BufferedReader::new(reader);
        
        let mut visitor = GameVisitor::new();
        reader.read_game(&mut visitor)?;

        // Extract game info
        let game_info = self.extract_game_info(&visitor.headers);

        // Analyze moves
        let mut blunders = Vec::new();
        let mut pos = Chess::default();
        let mut move_number = 1u32;
        
        let mut stats = Statistics {
            total_moves: visitor.moves.len() as u32,
            total_blunders: 0,
            total_mistakes: 0,
            total_inaccuracies: 0,
        };

        for san_plus in &visitor.moves {
            let fen_before = Fen::from_position(pos.clone(), shakmaty::EnPassantMode::Legal).to_string();
            let side = if pos.turn().is_white() { "white" } else { "black" };
            
            // Evaluate position before move
            let eval_before = self.engine.evaluate(&fen_before, self.depth)?;
            
            // Make the move
            let m = san_plus.san.to_move(&pos)
                .context(format!("Invalid move: {}", san_plus.san))?;
            pos = pos.play(&m)
                .context("Failed to play move")?;
            
            // Evaluate position after move (from opponent's perspective)
            let fen_after = Fen::from_position(pos.clone(), shakmaty::EnPassantMode::Legal).to_string();
            let mut eval_after = self.engine.evaluate(&fen_after, self.depth)?;
            
            // Flip evaluation for black's moves
            let (eval_before_normalized, eval_after_normalized) = if side == "black" {
                (-eval_before, -eval_after)
            } else {
                (eval_before, eval_after)
            };
            
            // Calculate centipawn loss
            let cp_diff = eval_before_normalized - eval_after_normalized;
            
            // Check if this is a blunder/mistake/inaccuracy
            if let Some(severity) = Severity::from_cp_loss(cp_diff) {
                match severity {
                    Severity::Blunder => stats.total_blunders += 1,
                    Severity::Mistake => stats.total_mistakes += 1,
                    Severity::Inaccuracy => stats.total_inaccuracies += 1,
                }
                
                blunders.push(BlunderEntry {
                    move_number,
                    side: side.to_string(),
                    san: san_plus.san.to_string(),
                    fen_before,
                    eval_before: eval_before_normalized,
                    eval_after: eval_after_normalized,
                    cp_diff,
                    severity,
                });
            }
            
            // Increment move number after black's move
            if side == "black" {
                move_number += 1;
            }
        }

        Ok(AnalysisReport {
            game_info,
            blunders,
            statistics: stats,
        })
    }

    fn extract_game_info(&self, headers: &[(String, String)]) -> GameInfo {
        let mut info = GameInfo {
            white: "Unknown".to_string(),
            black: "Unknown".to_string(),
            result: "*".to_string(),
            date: "????.??.??".to_string(),
        };

        for (key, value) in headers {
            match key.as_str() {
                "White" => info.white = value.clone(),
                "Black" => info.black = value.clone(),
                "Result" => info.result = value.clone(),
                "Date" => info.date = value.clone(),
                _ => {}
            }
        }

        info
    }
}
