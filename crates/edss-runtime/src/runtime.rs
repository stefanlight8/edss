use {
    crate::handle::RuntimeHandle,
    anyhow::Result,
    edss_journals::{reader::JournalReader, utils::fs::get_journals},
    edss_rpc::{command::Command, event::Event},
    edss_sessions::session::{GameSession, GameSessionMeta},
    std::path::PathBuf,
    tokio::sync::mpsc,
};

pub struct Runtime {
    command_rx: mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
    runtime: tokio::runtime::Runtime,
}

impl Runtime {
    pub fn new(event_tx: mpsc::Sender<Event>) -> (Runtime, RuntimeHandle) {
        let (command_tx, command_rx) = mpsc::channel::<Command>(128);
        let runtime = tokio::runtime::Runtime::new().unwrap();

        (
            Runtime {
                command_rx,
                event_tx,
                runtime,
            },
            RuntimeHandle { command_tx },
        )
    }

    pub fn start(&mut self) {
        let command_handler = Runtime::command_handler(&mut self.command_rx);
        self.runtime.block_on(command_handler);
    }

    async fn command_load_sessions(journals_path: PathBuf) -> Result<Vec<GameSessionMeta>> {
        let mut sessions: Vec<GameSessionMeta> = Vec::new();
        let journals = get_journals(journals_path)?;

        for journal_path in journals {
            let mut reader = JournalReader::open(&journal_path).await?;
            let first = match reader.read_first().await {
                Ok(entry) => entry,
                Err(e) => {
                    tracing::warn!(
                        "skipped {} because of error: {:?}",
                        journal_path.display(),
                        e
                    );
                    continue;
                }
            };
            let timestamp = first.timestamp;
            sessions.push(GameSessionMeta {
                timestamp,
                journal_path: journal_path.clone(),
            });
        }

        Ok(sessions)
    }

    async fn command_load_session(session_meta: GameSessionMeta) -> Result<GameSession> {
        let session = GameSession::load(session_meta.journal_path).await?;

        Ok(session)
    }

    pub async fn command_handler(command_rx: &mut mpsc::Receiver<Command>) {
        loop {
            match command_rx.recv().await {
                Some(Command::LoadSessions {
                    respond_to,
                    journals_path,
                }) => {
                    let output = Runtime::command_load_sessions(journals_path).await;
                    let _ = respond_to
                        .send(output)
                        .inspect_err(|e| tracing::error!("failed to send command output: {:?}", e));
                }
                Some(Command::LoadSession {
                    session,
                    respond_to,
                }) => {
                    let output = Runtime::command_load_session(session).await;
                    let _ = respond_to
                        .send(output)
                        .inspect_err(|e| tracing::error!("failed to send command output: {:?}", e));
                }
                None => (),
            }
        }
    }
}
