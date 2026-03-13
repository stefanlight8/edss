use {crate::statistics::event_handler::EventHandler, edss_journals::events::Event};

const THARGOID_FACTION: &'static str = "$faction_Thargoid;";

#[derive(Debug, Default)]
pub struct ThargoidStatistics {
    pub cyclops_killed: u64,
    pub basilisk_killed: u64,
    pub medusa_killed: u64,
    pub hydra_killed: u64,
    pub other_killed: u64,
}

impl EventHandler for ThargoidStatistics {
    fn handle(&mut self, event: &Event) {
        match event {
            Event::FactionKillBond {
                reward,
                victim_faction,
                ..
            } if victim_faction == THARGOID_FACTION => match reward {
                8_000_000 => self.cyclops_killed += 1,
                24_000_000 => self.basilisk_killed += 1,
                40_000_000 => self.medusa_killed += 1,
                60_000_000 => self.hydra_killed += 1,
                _ => self.other_killed += 1,
            },
            _ => (),
        }
    }
}
