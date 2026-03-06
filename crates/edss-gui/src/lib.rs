use {
    crate::components::{Session, Sessions},
    edss_rpc::event::Event,
    edss_runtime::handle::RuntimeHandle,
    edss_sessions::session::GameSessionMeta,
    tokio::sync::mpsc,
};

mod components;

pub struct App {
    runtime: RuntimeHandle,
    event_rx: mpsc::Receiver<Event>,

    sessions: Sessions,
    session: Session,

    current_session_meta: Option<GameSessionMeta>,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        runtime: RuntimeHandle,
        event_rx: mpsc::Receiver<Event>,
        pixels_per_point: Option<f32>,
    ) -> Self {
        if let Some(pixels_per_point) = pixels_per_point {
            todo!()
        } else {
            let scale = cc.egui_ctx.pixels_per_point();
            cc.egui_ctx.set_pixels_per_point(scale * 1.2);
        }
        Self {
            runtime,
            event_rx,
            sessions: Sessions::None,
            session: Session::None,
            current_session_meta: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Plus)) {
            ctx.set_pixels_per_point(ctx.pixels_per_point() * 1.1);
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Minus)) {
            ctx.set_pixels_per_point(ctx.pixels_per_point() / 1.1);
        }

        egui::SidePanel::left("sessions").show(ctx, |ui| {
            let selected_session =
                self.sessions
                    .ui(ui, &mut self.runtime, self.current_session_meta.clone());

            if let Some(session) = selected_session {
                if self.current_session_meta.as_ref() != Some(&session) {
                    self.current_session_meta = Some(session.clone());
                }

                tracing::debug!("received {:?}", session);

                self.session = Session::Loading { session, rx: None };
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| self.session.ui(ui, &mut self.runtime));
    }
}
