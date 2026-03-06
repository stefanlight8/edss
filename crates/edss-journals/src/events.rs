use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum Event {
    BookTaxi {
        cost: u64,
    },
    Bounty {
        rewards: Vec<Reward>,
        total_reward: u64,
        victim_faction: String,
    },
    BuyAmmo {
        cost: u64,
    },
    BuyDrones {
        count: u64,
        total_cost: u64,
    },
    BuyWeapon {
        name_localised: String,
        price: u64,
    },
    CarrierJump {
        docked: bool,
        star_system: String,
    },
    CockpitBreached,
    CodexEntry {
        is_new_entry: Option<bool>,
        voucher_amount: bool,
    },
    Commander {
        name: String,
    },
    CommitCrime,
    CommunityGoalReward {
        reward: u64,
    },
    CrimeVictim {
        bounty: Option<u64>,
        fine: Option<u64>,
    },
    DatalinkVoucher {
        reward: u64,
    },
    Died {
        killer_name: Option<String>,
        killers: Option<Killer>,
    },
    FactionKillBond {
        reward: u64,
        victim_faction: String,
        #[serde(alias = "VictimFaction_Localised")]
        victim_faction_localised: Option<String>,
    },
    Fileheader {
        odyssey: bool,
        #[serde(alias = "gameversion")]
        game_version: String,
    },
    FSDJump {
        jump_dist: f64,
        star_system: String,
    },
    LoadGame {
        commander: String,
        game_mode: Option<String>,
        horizons: bool,
        odyssey: Option<bool>,
        ship: Option<String>,
    },
    Loadout {
        ship: String,
    },
    Location {
        star_system: String,
    },
    MarketBuy {
        count: u32,
        total_cost: u64,
    },
    MarketSell {
        count: u32,
        total_sale: u64,
    },
    MissionAbandoned,
    MissionAccepted,
    MissionCompleted {
        donated: Option<u64>,
    },
    MissionFailed,
    ModuleSell {
        sell_price: u64,
    },
    ModuleSellRemote {
        sell_price: u64,
    },
    MultiSellExplorationData {
        total_earnings: u64,
    },
    PayBounties,
    PayFines {
        amount: u64,
    },
    Powerplay {
        merits: u64,
        power: String,
        rank: u16,
        time_pledged: i64,
    },
    PowerplayMerits {
        power: String,
        total_merits: u64,
    },
    PVPKill,
    RedeemVoucher {
        amount: u64,
    },
    RefuelAll {
        cost: u64,
    },
    RefuelPartial {
        cost: u64,
    },
    RepairAll {
        cost: u64,
    },
    Resurrect {
        cost: u64,
        option: String,
    },
    RestockVehicle {
        cost: u64,
    },
    SAAScanComplete,
    ScanOrganic,
    Screenshot,
    SellDrones {
        count: u64,
        total_sale: u64,
    },
    SellOrganicData,
    SellWeapon {
        price: u64,
    },
    ShipyardBuy {
        ship_price: u64,
    },
    ShipyardSell {
        ship_price: u64,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Reward {
    pub faction: String,
    pub reward: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Killer {
    pub name: String,
}
