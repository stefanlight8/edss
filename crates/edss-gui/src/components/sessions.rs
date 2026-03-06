use {
    crate::components::ErrorView, anyhow::Result, edss_runtime::handle::RuntimeHandle,
    edss_sessions::session::GameSessionMeta, tokio::sync::oneshot, tracing,
};

pub enum Sessions {
    None,
    Loading(oneshot::Receiver<Result<Vec<GameSessionMeta>>>),
    Sessions(Vec<GameSessionMeta>),
    Error(anyhow::Error),
}

impl Sessions {
    fn load_sessions(
        &mut self,
        runtime: &mut RuntimeHandle,
    ) -> oneshot::Receiver<Result<Vec<GameSessionMeta>>> {
        let journals_path = std::path::PathBuf::from("/Users/stefanlight/Documents/Journals");
        // TODO: replace with journals path from app settings
        runtime.load_sessions(journals_path)
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        runtime: &mut RuntimeHandle,
        current_session: Option<GameSessionMeta>,
    ) -> Option<GameSessionMeta> {
        match self {
            Sessions::None => {
                if ui.button("Load sessions").clicked() {
                    let rx = self.load_sessions(runtime);
                    *self = Sessions::Loading(rx);
                }

                return None;
            }
            Sessions::Loading(rx) => {
                ui.spinner();

                match rx.try_recv() {
                    Ok(Ok(res)) => {
                        *self = Sessions::Sessions(res);
                        return None;
                    }
                    Ok(Err(error)) => {
                        *self = Sessions::Error(error);
                        return None;
                    }
                    Err(oneshot::error::TryRecvError::Empty) => (),
                    Err(err) => tracing::error!("failed to get command output: {}", err),
                }

                return None;
            }
            Sessions::Sessions(sessions) => {
                let output = egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("Total sessions: {}", sessions.len()));
                        });

                        ui.separator();

                        for session in sessions {
                            let date_label = session.timestamp.format("%d.%m.%Y %H:%M").to_string();
                            let selected = current_session.as_ref() == Some(session);

                            if ui.selectable_label(selected, date_label).clicked() {
                                if !selected {
                                    return Some(session.clone());
                                }
                            }
                        }
                        return None;
                    });

                return output.inner;
            }
            Sessions::Error(error) => {
                let view = ErrorView { error };
                if view.ui(ui, true) {
                    *self = Sessions::None;
                };
                return None;
            }
        }
    }
}
