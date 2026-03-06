use edss_journals::events::Event;

pub trait EventHandler {
    fn handle(&mut self, event: &Event);
}
