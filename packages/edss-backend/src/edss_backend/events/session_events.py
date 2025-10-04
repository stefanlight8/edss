from edss_backend.events.base_events import BaseEvent


class SessionUpdateEvent(BaseEvent):
    journal_name: str
