pub struct ErrorView<'a> {
    pub error: &'a anyhow::Error,
}

impl<'a> ErrorView<'a> {
    pub fn ui(&self, ui: &mut egui::Ui, show_retry_button: bool) -> bool {
        let mut retry = false;

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.colored_label(egui::Color32::RED, egui::RichText::new("Error").strong());

                ui.label(self.error.to_string());

                if ui
                    .collapsing("Details", |ui| {
                        ui.label(format!("{:#?}", self.error));
                    })
                    .header_response
                    .clicked()
                {}

                ui.separator();

                if show_retry_button && ui.button("Retry").clicked() {
                    retry = true;
                }
            });
        });

        retry
    }
}
