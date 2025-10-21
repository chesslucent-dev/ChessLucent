use serde::{Deserialize, Serialize};
use shakmaty::{Color, san::San};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "inaccuracy")]
    Inaccuracy,
    #[serde(rename = "mistake")]
    Mistake,
    #[serde(rename = "blunder")]
    Blunder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlunderEntry {
    pub move_number: u32,
    pub side: String,
    pub san: String,
    pub fen_before: String,
    pub eval_before: i32,
    pub eval_after: i32,
    pub cp_diff: i32,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub game_info: GameInfo,
    pub blunders: Vec<BlunderEntry>,
    pub statistics: Statistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub white: String,
    pub black: String,
    pub result: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_moves: u32,
    pub total_blunders: u32,
    pub total_mistakes: u32,
    pub total_inaccuracies: u32,
}

impl Severity {
    pub fn from_cp_loss(cp_loss: i32) -> Option<Self> {
        if cp_loss >= 300 {
            Some(Severity::Blunder)
        } else if cp_loss >= 150 {
            Some(Severity::Mistake)
        } else if cp_loss >= 75 {
            Some(Severity::Inaccuracy)
        } else {
            None
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Inaccuracy => "inaccuracy",
            Severity::Mistake => "mistake",
            Severity::Blunder => "blunder",
        }
    }
}
