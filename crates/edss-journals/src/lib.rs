use {
    crate::events::Event,
    chrono::{DateTime, Local},
    serde::{Deserialize, Serialize},
};

pub mod events;
pub mod reader;
pub mod utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    pub timestamp: DateTime<Local>,
    #[serde(flatten)]
    pub event: Event,
}
