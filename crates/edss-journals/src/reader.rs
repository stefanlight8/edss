use {
    crate::JournalEntry,
    std::{io::Error, path::Path},
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
};

pub struct JournalReader {
    file_reader: BufReader<File>,
}

impl JournalReader {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path).await?;
        let file_reader = BufReader::new(file);

        Ok(Self { file_reader })
    }

    pub async fn read(&mut self) -> Result<Vec<JournalEntry>, Error> {
        let file_reader = &mut self.file_reader;
        let mut lines = file_reader.lines();

        let mut events: Vec<JournalEntry> = vec![];

        while let Some(line) = lines.next_line().await? {
            let event: JournalEntry = serde_json::from_str(&line)?;
            events.push(event);
        }

        Ok(events)
    }

    pub async fn read_first(&mut self) -> Result<JournalEntry, Error> {
        let mut line = String::new();
        self.file_reader.read_line(&mut line).await?;
        let entry = serde_json::from_str(&line)?;
        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Event;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_journal_reader() {
        let path = "test_journal.log";
        let mut file = File::create(path).await.unwrap();

        file.write_all(
            br#"{ "timestamp":"2026-02-13T16:16:00Z", "event":"Commander", "Name":"Verglass" }"#,
        )
        .await
        .unwrap();

        let mut reader = JournalReader::open(path).await.unwrap();
        let events = reader.read().await.unwrap();
        assert_eq!(events.len(), 1);
        match &events[0].event {
            Event::Commander { name } => assert_eq!(name, "Verglass"),
            Event::Unknown => (),
            _ => {}
        }
        let _ = tokio::fs::remove_file(path).await;
    }
}
