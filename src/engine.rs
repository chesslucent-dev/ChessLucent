use anyhow::{Result, Context, anyhow};
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::io::{BufRead, BufReader, Write};
use vampirc_uci::{UciMessage, UciInfoAttribute, parse_one};

pub struct Engine {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Engine {
    pub fn new(path: &str) -> Result<Self> {
        let mut process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start chess engine")?;

        let stdin = process.stdin.take().context("Failed to open stdin")?;
        let stdout = BufReader::new(process.stdout.take().context("Failed to open stdout")?);

        let mut engine = Engine { process, stdin, stdout };
        
        // Initialize UCI
        engine.send_command("uci")?;
        engine.wait_for("uciok")?;
        engine.send_command("isready")?;
        engine.wait_for("readyok")?;
        
        Ok(engine)
    }

    fn send_command(&mut self, cmd: &str) -> Result<()> {
        writeln!(self.stdin, "{}", cmd)?;
        self.stdin.flush()?;
        Ok(())
    }

    fn wait_for(&mut self, expected: &str) -> Result<()> {
        let mut line = String::new();
        while self.stdout.read_line(&mut line)? > 0 {
            if line.trim() == expected {
                return Ok(());
            }
            line.clear();
        }
        Err(anyhow!("Engine did not respond with '{}'", expected))
    }

    pub fn evaluate(&mut self, fen: &str, depth: u8) -> Result<i32> {
        // Set position
        self.send_command(&format!("position fen {}", fen))?;
        
        // Start analysis
        self.send_command(&format!("go depth {}", depth))?;
        
        // Read until we get bestmove
        let mut best_score = None;
        let mut line = String::new();
        
        while self.stdout.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            
            if trimmed.starts_with("bestmove") {
                break;
            }
            
            // Parse UCI info messages
            if let Ok(msg) = parse_one(trimmed) {
                if let UciMessage::Info(attributes) = msg {
                    // Look for score in the info
                    let mut found_score = None;
                    for attr in attributes {
                        match attr {
                            UciInfoAttribute::Score { cp: Some(cp), .. } => {
                                found_score = Some(cp);
                            }
                            UciInfoAttribute::Score { mate: Some(mate), .. } => {
                                // Convert mate scores to large centipawn values
                                found_score = Some(if mate > 0 { 10000 } else { -10000 });
                            }
                            _ => {}
                        }
                    }
                    if let Some(score) = found_score {
                        best_score = Some(score);
                    }
                }
            }
            
            line.clear();
        }
        
        best_score.ok_or_else(|| anyhow!("No evaluation found"))
    }

    pub fn shutdown(&mut self) -> Result<()> {
        self.send_command("quit")?;
        self.process.wait()?;
        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        let _ = self.send_command("quit");
        let _ = self.process.kill();
    }
}
