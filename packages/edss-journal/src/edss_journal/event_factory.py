from typing import TypedDict

import msgspec

from edss_journal.events.base import BaseEvent


class EventPayload(TypedDict):
    event: str


class EventFactory:
    def __init__(self) -> None:
        self._map: dict[str, type[BaseEvent]] = {}

    def register(self, event_type: type[BaseEvent]) -> None:
        self._map[event_type.event] = event_type
    
    def convert(self, event_payload: EventPayload) -> BaseEvent:
        return msgspec.convert(event_payload, type=self._map[event_payload["event"]])
