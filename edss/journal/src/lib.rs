mod events;

use std::fs;
use std::path::PathBuf;
use std::io::Result;

use events::Event;

pub fn read_journal(path: PathBuf) -> Result<Vec<Event>> {
    let file: String = fs::read_to_string(path)?;
    let mut events: Vec<Event> = vec![];
    for line in file.lines() {
        let event: Event = serde_json::from_str(line)?;
        let _ = &events.push(event);
    }
    Ok(events)
}
