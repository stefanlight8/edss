use {
    crate::statistics::{event_handler::EventHandler, utils::per_hour},
    edss_journals::events::Event,
};

#[derive(Debug, Default)]
pub struct CombatStatistics {
    pub npc_kills: u64,

    pub pvp_kills: u64,
    pub pvp_deaths: u64,

    pub bounties_gained: u64,
    pub combat_bounds_gained: u64,

    pub destroyed: u64,
}

impl CombatStatistics {
    pub fn npc_kills_per_hour(&self, duration: f64) -> f64 {
        per_hour(self.npc_kills, duration)
    }

    pub fn bounties_per_hour(&self, duration: f64) -> f64 {
        per_hour(self.bounties_gained, duration)
    }

    pub fn combat_bounds_per_hour(&self, duration: f64) -> f64 {
        per_hour(self.combat_bounds_gained, duration)
    }
}

impl EventHandler for CombatStatistics {
    fn handle(&mut self, event: &Event) {
        match event {
            Event::Bounty { total_reward, .. } => {
                self.bounties_gained += total_reward;
                self.npc_kills += 1;
            }
            Event::FactionKillBond { reward, .. } => {
                self.combat_bounds_gained += reward;
                self.npc_kills += 1
            }
            // faction kill bond doesn't gurantee that it was received from npc kill
            // so npc kills is a approximately amount
            Event::PVPKill => self.pvp_kills += 1,
            Event::Died {
                killer_name: Some(killer_name),
                ..
            } => {
                if killer_name.starts_with("cmdr") {
                    self.pvp_deaths += 1;
                }
            }
            Event::Died {
                killers: Some(_), ..
            } => {
                self.pvp_deaths += 1;
            }
            Event::Resurrect { option, .. } if option == "rebuy" => {
                self.destroyed += 1;
            }
            _ => (),
        }
    }
}
