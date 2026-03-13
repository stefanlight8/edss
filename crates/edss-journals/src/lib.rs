use {
    crate::events::Event,
    chrono::{DateTime, Local},
    serde::Deserialize,
};

pub mod events;
pub mod reader;
pub mod utils;

#[derive(Deserialize, Debug)]
pub struct JournalEntry {
    pub timestamp: DateTime<Local>,
    #[serde(flatten)]
    pub event: Event,
}
