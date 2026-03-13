use {crate::statistics::event_handler::EventHandler, edss_journals::events::Event};

#[derive(Debug, Default)]
pub struct TemplateStatistics {
    // fields
}

impl EventHandler for TemplateStatistics {
    fn handle(&mut self, event: &Event) {
        match event {
            // event handling
            _ => (),
        }
    }
}
