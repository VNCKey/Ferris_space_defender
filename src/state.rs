use serde::{Deserialize, Serialize};
use std::fs;

#[derive(bevy::prelude::States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    NameInput,
    Playing,
    GameOver,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
    pub wave: u32,
    pub date: String,
}

#[derive(bevy::prelude::Resource)]
pub struct Leaderboard {
    pub entries: Vec<ScoreEntry>,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::load()
    }
}

impl Leaderboard {
    const FILE_PATH: &'static str = "highscores.json";

    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string(Self::FILE_PATH) {
            if let Ok(entries) = serde_json::from_str::<Vec<ScoreEntry>>(&content) {
                return Self { entries };
            }
        }
        Self {
            entries: vec![
                ScoreEntry {
                    name: "Ferris_Pro".to_string(),
                    score: 4200,
                    wave: 6,
                    date: "Record".to_string(),
                },
                ScoreEntry {
                    name: "Rustacean_PE".to_string(),
                    score: 2800,
                    wave: 4,
                    date: "Top 2".to_string(),
                },
            ],
        }
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            let _ = fs::write(Self::FILE_PATH, json);
        }
    }

    pub fn add_score(&mut self, name: String, score: u32, wave: u32) {
        let entry = ScoreEntry {
            name: if name.trim().is_empty() {
                "Anonimo".to_string()
            } else {
                name.trim().to_string()
            },
            score,
            wave,
            date: "Hoy".to_string(),
        };
        self.entries.push(entry);
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        if self.entries.len() > 15 {
            self.entries.truncate(15);
        }
        self.save();
    }
}
