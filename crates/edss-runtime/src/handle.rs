use {
    anyhow::Result,
    edss_rpc::command::Command,
    edss_sessions::session::{GameSession, GameSessionMeta},
    std::path::PathBuf,
    tokio::sync::{mpsc, oneshot},
};

pub struct RuntimeHandle {
    pub command_tx: mpsc::Sender<Command>,
}

impl RuntimeHandle {
    pub fn load_sessions(
        &mut self,
        journals_path: PathBuf,
    ) -> oneshot::Receiver<Result<Vec<GameSessionMeta>>> {
        let (tx, rx) = oneshot::channel::<Result<Vec<GameSessionMeta>>>();

        let _ = self.command_tx.try_send(Command::LoadSessions {
            respond_to: tx,
            journals_path,
        });

        rx
    }

    pub fn load_session(
        &mut self,
        session: GameSessionMeta,
    ) -> oneshot::Receiver<Result<GameSession>> {
        let (tx, rx) = oneshot::channel::<Result<GameSession>>();

        let _ = self.command_tx.try_send(Command::LoadSession {
            respond_to: tx,
            session,
        });

        rx
    }
}
