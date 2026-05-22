use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HighScores {
    pub scores: HashMap<String, u32>,
}

impl HighScores {
    pub fn load() -> Self {
        let path = Path::new("highscores.json");
        if path.exists() {
            if let Ok(mut file) = File::open(path) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    if let Ok(high_scores) = serde_json::from_str(&content) {
                        return high_scores;
                    }
                }
            }
        }
        
        // Return default scores
        let mut scores = HashMap::new();
        scores.insert("Tetris".to_string(), 0);
        scores.insert("2048".to_string(), 0);
        scores.insert("Minesweeper".to_string(), 0);
        scores.insert("Space Invaders".to_string(), 0);
        scores.insert("Roguelike".to_string(), 0);
        
        HighScores { scores }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Path::new("highscores.json");
        let content = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn get_score(&self, game: &str) -> u32 {
        *self.scores.get(game).unwrap_or(&0)
    }

    pub fn update_score(&mut self, game: &str, score: u32) -> bool {
        let current = self.get_score(game);
        if score > current {
            self.scores.insert(game.to_string(), score);
            let _ = self.save();
            true // new high score!
        } else {
            false
        }
    }
}
