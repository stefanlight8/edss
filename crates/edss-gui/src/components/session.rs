use {
    crate::components::ErrorView,
    anyhow::Result,
    edss_runtime::handle::RuntimeHandle,
    edss_sessions::session::{GameSession, GameSessionMeta},
    tokio::sync::oneshot,
};

pub enum Session {
    None,
    Loading {
        session: GameSessionMeta,
        rx: Option<oneshot::Receiver<Result<GameSession>>>,
    },
    Session(GameSession),
    Error(anyhow::Error),
}

impl Session {
    pub fn ui(&mut self, ui: &mut egui::Ui, runtime: &mut RuntimeHandle) {
        match self {
            Session::None => (),
            Session::Loading {
                session: session_meta,
                rx,
            } => {
                ui.label(format!("Loading {}", session_meta.journal_path.display()));

                if let Some(rx) = rx {
                    match rx.try_recv() {
                        Ok(Ok(session)) => {
                            *self = Session::Session(session);
                        }
                        Ok(Err(error)) => {
                            *self = Session::Error(error);
                        }
                        Err(oneshot::error::TryRecvError::Empty) => (),
                        Err(err) => tracing::error!("failed to get command output: {}", err),
                    }
                } else {
                    *rx = Some(runtime.load_session(session_meta.clone()));
                }
            }
            Session::Session(session) => {
                ui.label(format!("duration: {} seconds", session.duration));
            }
            Session::Error(error) => {
                let view = ErrorView { error };
                view.ui(ui, false);
            }
        }
    }
}
