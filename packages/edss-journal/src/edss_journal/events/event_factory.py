from __future__ import annotations

import typing
from collections.abc import Mapping

from msgspec import json, convert

from edss_journal.events.base import (
    BaseEvent,
    PowerplayMerits,
    Bounty,
    CreditsExpense,
    NpcCrewPaidWage,
    FactionKillBond,
    Died,
)

EVENT_MAP: typing.Final[Mapping[str, type[BaseEvent]]] = {
    "PowerplayMerits": PowerplayMerits,
    "Bounty": Bounty,
    "RepairAll": CreditsExpense,
    "Reammo": CreditsExpense,
    "Refuel": CreditsExpense,
    "Resurrect": CreditsExpense,
    "NpcCrewPaidWage": NpcCrewPaidWage,
    "FactionKillBond": FactionKillBond,
    "Died": Died,
}


class EventFactory:
    @staticmethod
    def create(payload: bytes | bytearray | memoryview[int] | str) -> BaseEvent | None:
        event_payload: Mapping[str, typing.Any] = json.decode(payload)
        if event_name := event_payload.get("event"):
            if event_type := EVENT_MAP.get(event_name):
                return convert(event_payload, type=event_type)
        return None
