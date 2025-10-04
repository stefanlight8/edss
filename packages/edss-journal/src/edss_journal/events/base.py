from __future__ import annotations

import datetime

import msgspec

__all__: tuple[str, ...] = (
    "BaseEvent",
    "PowerplayMerits",
    "Bounty",
    "CreditsExpense",
    "NpcCrewPaidWage",
    "FactionKillBond",
)


class BaseEvent(msgspec.Struct):
    timestamp: datetime.datetime
    event: str


class PowerplayMerits(BaseEvent, rename="pascal"):
    power: str
    merits_gained: int
    total_merits: int


class Bounty(BaseEvent, rename="pascal"):
    total_reward: int


class CreditsExpense(BaseEvent, rename="pascal"):
    cost: int


class NpcCrewPaidWage(BaseEvent, rename="pascal"):
    amount: int


class FactionKillBond(BaseEvent, rename="pascal"):
    reward: int


class Died(BaseEvent): ...
