use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub timestamp: DateTime<Local>,
    #[serde(flatten)]
    pub event: EventKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", rename_all_fields = "PascalCase")]
pub enum EventKind {
    Commander {
        name: String,
    },
    LoadGame {
        commander: String,
        ship: Option<String>,
        #[serde(alias = "Ship_Localised")]
        ship_localised: Option<String>,
        ship_name: Option<String>,
        #[serde(alias = "ShipID")]
        ship_id: Option<usize>,
        credits: usize,
    },    
    Progress {
        combat: u8,
        trade: u8,
        explore: u8,
        soldier: u8,
        exobiologist: u8,
        empire: u8,
        federation: u8,
    },
    Reputation {
        empire: f32,
        federation: f32,
        independent: f32,
        alliance: f32,
    },
    Loadout {
        ship: Option<String>,
        ship_name: Option<String>,
        #[serde(alias = "ShipID")]
        ship_id: Option<usize>,
    },
    Screenshot,
    Scan {
        scan_type: String,
        was_discovered: bool,
        was_footfalled: Option<bool>,
    },
    ScanBaryCentre {
        star_system: String,
        #[serde(alias = "BodyID")]
        body_id: u16,
    },
    FSSAllBodiesFound {
        system_name: String,
        count: u16,
    },
    SAAScanComplete {
        efficiency_target: u32,
    },
    ScanOrganic {
        scan_type: String,
        genus: String,
        #[serde(alias = "Genus_Localised")]
        genus_localised: String,
        species: String,
        #[serde(alias = "Species_Localised")]
        species_localised: String,
        was_logged: Option<bool>,
        variant: String,
        #[serde(alias = "Variant_Localised")]
        variant_localised: String
    },
    FSDJump {
        star_system: String,
        star_pos: [f32; 3],
        jump_dist: f32,
    },
    Docked {
        station_name: String,
        star_system: Option<String>,
        star_pos: Option<[f32; 3]>,
    },
    Undocked,
    Location {
        star_system: String,
        star_pos: [f32; 3],
        body: Option<String>,
        station_name: Option<String>,
    },
    ShipyardSwap {
        ship_type: String,
        #[serde(alias = "ShipType_Localised")]
        ship_type_localised: Option<String>,
        ship_name: Option<String>,
        #[serde(alias = "ShipID")]
        ship_id: Option<usize>,
    },
    Powerplay {
        rank: u16,
        merits: usize,
    },
    PowerplayMerits {
        power: String,
        merits_gained: u16,
        total_merits: usize,
    },
    Bounty {
        total_reward: usize,
        victim_faction: Option<String>,
    },
    FactionKillBond {
        reward: usize,
        awarding_faction: String,
    },
    PVPKill {
        victim: String,
    },
    CodexEntry {
        region: String,
        #[serde(alias = "Region_Localised")]
        region_localised: String,
        voucher_amount: Option<usize>,
    },
    DatalinkVoucher {
        reward: usize,
        payee_faction: String,
    },
    MissionCompleted {
        donated: Option<usize>,
        reward: Option<usize>,
    },
    SearchAndRescue {
        reward: usize,
    },
    SellExplorationData {
        total_earnings: usize,
    },
    SellOrganicData {
        bio_data: Vec<OrganicSale>,
    },
    MultiSellExplorationData {
        total_earnings: usize,
    },
    ModuleSell {
        sell_price: usize,
    },
    ModuleSellRemote {
        sell_price: usize,
    },
    MarketSell {
        #[serde(alias = "Type")]
        target: String,
    },
    SellDrones {
        sell_price: usize,
        total_sale: usize,
    },
    ShipyardSell {
        ship_price: usize,
    },
    ShipyardBuy {
        ship_price: usize,
    },
    ModuleBuy {
        buy_price: usize,
    },
    BuyDrones {
        buy_price: usize,
    },
    MarketBuy {
        #[serde(alias = "Type")]
        target: String,
    },
    BuyWeapon {
        price: usize,
    },
    RefuelAll {
        cost: usize,
    },
    RepairAll {
        cost: usize,
    },
    Repair {
        cost: usize,
    },
    BuyAmmo {
        cost: usize,
    },
    NpcCrewPaidWage {
        amount: usize,
    },
    PayFines {
        amount: usize,
    },
    PayBounties {
        amount: usize,
    },
    SelfDestruct {
    },
    Resurrect {
        cost: usize,
    },
    RestockVehicle {
        cost: usize,
    },
    Disembark {
        #[serde(alias = "SRV")]
        srv: bool,
    },
    Embark {
        #[serde(alias = "SRV")]
        srv: bool,
    },
    Died {
        killer_name: Option<String>,
        killer_ship: Option<String>,
        killers: Option<Vec<Killer>>,
    },
    Shutdown,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OrganicSale {
    pub genus: String,
    pub species: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Killer {
    name: String,
    ship: String,
}