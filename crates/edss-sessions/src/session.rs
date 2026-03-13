use {
    crate::statistics::{
        combat::CombatStatistics, event_handler::EventHandler, thargoids::ThargoidStatistics,
    },
    anyhow::{Context, Result},
    chrono::{DateTime, Local},
    edss_journals::{JournalEntry, reader::JournalReader},
    std::path::PathBuf,
};

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

    pub combat_statistics: CombatStatistics,
    pub thargoid_statistics: ThargoidStatistics,
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

    pub async fn compile(entries: Vec<JournalEntry>) -> Self {
        let duration = if entries.len() >= 2 {
            let (first, last) = (entries.first().unwrap(), entries.last().unwrap());

            (last.timestamp - first.timestamp).as_seconds_f32()
        } else {
            0.0
        };

        let mut session = GameSession {
            duration,
            ..Default::default()
        };

        for entry in entries {
            session.combat_statistics.handle(&entry.event);
            session.thargoid_statistics.handle(&entry.event);
        }

        println!("{:?}", session);

        session
    }
}
