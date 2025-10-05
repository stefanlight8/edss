import asyncio
import pathlib
import logging
from datetime import datetime

from edss_journal.aio.reader import JournalReader
from edss_journal.events.base import (
    BaseEvent as JournalBaseEvent,
    PowerplayMerits,
    Bounty,
    CreditsExpense,
    NpcCrewPaidWage,
    FactionKillBond,
    Died,
)
from edss_journal.events.event_factory import EventFactory

from edss_backend.session import GameSession
from edss_backend.events.base_events import BaseEvent as InternalBaseEvent
from edss_backend.events.session_events import SessionUpdateEvent


class GameSessionManager:
    def __init__(self, folder_path: pathlib.Path) -> None:
        self._logger: logging.Logger = logging.getLogger("edss.session.manager")

        self.folder_path = folder_path

        self.queue: asyncio.Queue[InternalBaseEvent] = asyncio.Queue()

        self.reader: JournalReader | None = None
        self.game_session: GameSession | None = None

    def process_event(self, event: JournalBaseEvent) -> None:
        if not self.game_session:
            return
        match event:
            case PowerplayMerits(
                power=power,
                merits_gained=merits_gained,
                total_merits=total_merits,
            ):
                self.game_session.power = power
                self.game_session.merits_earned += merits_gained
                self.game_session.merits_earned_per_hour = (
                    self.game_session.get_merits_earned_per_hour()
                )
                self.game_session.total_merits = total_merits
            case Bounty(total_reward=total_reward):
                self.game_session.bounties_earned += total_reward
                self.game_session.bounties_earned_per_hour = (
                    self.game_session.get_bounties_earned_per_hour()
                )
                self.game_session.credits_earned = (
                    self.game_session.get_total_credits_earned()
                )
                self.game_session.credits_earned_per_hour = (
                    self.game_session.get_total_credits_earned_per_hour()
                )
                self.game_session.ships_destroyed += 1
            case CreditsExpense(cost=cost):
                self.game_session.credits_expenses -= cost
                self.game_session.credits_earned_per_hour = (
                    self.game_session.get_total_credits_earned_per_hour()
                )
            case NpcCrewPaidWage(amount=amount):
                self.game_session.credits_expenses -= amount
                self.game_session.credits_earned_per_hour = (
                    self.game_session.get_total_credits_earned_per_hour()
                )
            case FactionKillBond(reward=reward):
                self.game_session.combat_bonds_earned += reward
                self.game_session.combat_bonds_earned_per_hour = (
                    self.game_session.get_combat_bonds_earned_per_hour()
                )
                self.game_session.credits_earned += reward
                self.game_session.credits_earned_per_hour = (
                    self.game_session.get_total_credits_earned_per_hour()
                )
                self.game_session.ships_destroyed += 1
            case Died():
                self.game_session.deaths += 1

    async def dispatch_session_update(
        self, journal_file: pathlib.Path | None = None
    ) -> None:
        if journal_file is not None:
            self.queue.put_nowait(SessionUpdateEvent(journal_file.name))
        else:
            if not self.reader:
                raise RuntimeError("Manager wasn't started")
            if not self.reader.journal_file:
                self._logger.info(
                    "tried to dispatch session update event, but there's no journal file"
                )
            self.queue.put_nowait(SessionUpdateEvent(self.reader.journal_file.name))  # type: ignore

    async def poll_events(self) -> None:
        if not self.reader:
            raise RuntimeError("Manager wasn't started")
        async for raw_event in self.reader.tail_lines():
            if isinstance(raw_event, pathlib.Path):
                self.game_session = GameSession(started_at=datetime.now())
                await self.dispatch_session_update(raw_event)
                self._logger.debug("new session")
            else:
                event: JournalBaseEvent | None = EventFactory.create(raw_event)
                if event:
                    self._logger.debug("received event %r", event)
                    try:
                        self.process_event(event)
                    except Exception as exc:
                        self._logger.error(
                            "failed to process event: %s", exc, exc_info=exc
                        )
                    await self.dispatch_session_update()

    async def start(self) -> None:
        self.reader = JournalReader(self.folder_path)
