use {
    anyhow::Result,
    edss_sessions::session::{GameSession, GameSessionMeta},
    std::path::PathBuf,
    tokio::sync::oneshot,
};

pub enum Command {
    LoadSessions {
        journals_path: PathBuf,
        respond_to: oneshot::Sender<Result<Vec<GameSessionMeta>>>,
    },
    LoadSession {
        session: GameSessionMeta,
        respond_to: oneshot::Sender<Result<GameSession>>,
    },
}
