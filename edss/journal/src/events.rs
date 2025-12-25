use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BaseEvent {
    timestamp: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "PascalCase")]
pub enum Event {
    Commander {
        #[serde(flatten)]
        base: BaseEvent,
        name: String,
    },
    Progress {
        #[serde(flatten)]
        base: BaseEvent,
        combat: u8,
        trade: u8,
        explore: u8,
        soldier: u8,
        exobiologist: u8,
        empire: u8,
        federation: u8,
    },
    Reputation {
        #[serde(flatten)]
        base: BaseEvent,
        empire: f32,
        federation: f32,
        independent: f32,
        alliance: f32,
    },
    Powerplay {
        #[serde(flatten)]
        base: BaseEvent,
        rank: u16,
        merits: usize,
    },
    PowerplayMerits {
        #[serde(flatten)]
        base: BaseEvent,
        power: String,
        merits_gained: u16,
        total_merits: usize,
    },
    Bounty {
        #[serde(flatten)]
        base: BaseEvent,
        total_reward: usize,
        victim_faction: String,
    },
    FactionKillBond {
        #[serde(flatten)]
        base: BaseEvent,
        reward: usize,
        awarding_faction: String,
    },
    CodexEntry {
        #[serde(flatten)]
        base: BaseEvent,
        voucher_amount: usize,
    },
    DatalinkVoucher {
        #[serde(flatten)]
        base: BaseEvent,
        reward: usize,
        payee_faction: String,
    },
    RedeemVoucher {
        #[serde(flatten)]
        base: BaseEvent,
        amount: usize,
        factions: Vec<String>,
        faction: String,
    },
    MissionCompleted {
        #[serde(flatten)]
        base: BaseEvent,
        donated: usize,
        reward: usize,
    },
    SearchAndRescue {
        #[serde(flatten)]
        base: BaseEvent,
        reward: usize,
    },
    MultiSellExplorationData {
        #[serde(flatten)]
        base: BaseEvent,
        total_earnings: usize,
    },
    ModuleSell {
        #[serde(flatten)]
        base: BaseEvent,
        sell_price: usize,
    },
    ModuleSellRemote {
        #[serde(flatten)]
        base: BaseEvent,
        sell_price: usize,
    },
    MarketSell {
        #[serde(flatten)]
        base: BaseEvent,
        #[serde(alias = "type")]
        target: String,
    },
    SellDrones {
        #[serde(flatten)]
        base: BaseEvent,
        sell_price: usize,
        total_sale: usize,
    },
    ShipyardSell {
        #[serde(flatten)]
        base: BaseEvent,
        ship_price: usize,
    },
    ShipyardBuy {
        #[serde(flatten)]
        base: BaseEvent,
        ship_price: usize,
    },
    ModuleBuy {
        #[serde(flatten)]
        base: BaseEvent,
        buy_price: usize,
    },
    BuyDrones {
        #[serde(flatten)]
        base: BaseEvent,
        buy_price: usize,
    },
    MarketBuy {
        #[serde(flatten)]
        base: BaseEvent,
        #[serde(alias = "type")]
        target: String,
    },
    BuyWeapon {
        #[serde(flatten)]
        base: BaseEvent,
        price: usize,
    },
    RefuelAll {
        #[serde(flatten)]
        base: BaseEvent,
        cost: usize,
    },
    RepairAll {
        #[serde(flatten)]
        base: BaseEvent,
        cost: usize,
    },
    Repair {
        #[serde(flatten)]
        base: BaseEvent,
        cost: usize,
    },
    BuyAmmo {
        #[serde(flatten)]
        base: BaseEvent,
        cost: usize,
    },
    NpcCrewPaidWage {
        #[serde(flatten)]
        base: BaseEvent,
        amount: usize,
    },
    PayFines {
        #[serde(flatten)]
        base: BaseEvent,
        amount: usize,
    },
    PayBounties {
        #[serde(flatten)]
        base: BaseEvent,
        amount: usize,
    },
    SelfDestruct {
        #[serde(flatten)]
        base: BaseEvent,
    },
    Resurrect {
        #[serde(flatten)]
        base: BaseEvent,
        cost: usize,
    },
    RestockVehicle {
        #[serde(flatten)]
        base: BaseEvent,
        cost: usize,
    },
    Died {
        #[serde(flatten)]
        base: BaseEvent,
    },
}
