use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use edss_journals::{JournalEntry, reader::JournalReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameSessionMeta {
    pub timestamp: DateTime<Local>,
    pub journal_path: PathBuf,
}

#[derive(Debug, Default)]
pub struct GameSession {
    pub duration: f32,

    pub commander_name: Option<String>,
    pub odyssey: bool,
}

impl GameSession {
    pub async fn load(journal_path: PathBuf) -> Result<Self> {
        let mut reader = JournalReader::open(&journal_path)
            .await
            .with_context(|| "Failed to open journal reader")?;

        let events = reader
            .read()
            .await
            .with_context(|| format!("Failed to read events from {:?}", journal_path))?;

        let session = GameSession::compile(events).await;

        Ok(session)
    }

    pub async fn compile(events: Vec<JournalEntry>) -> Self {
        let duration = if events.len() >= 2 {
            let (first, last) = (events.first().unwrap(), events.last().unwrap());

            (last.timestamp - first.timestamp).as_seconds_f32()
        } else {
            0.0
        };

        Self {
            duration,
            commander_name: None,
            odyssey: false,
        }
    }
}
