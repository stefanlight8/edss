import datetime

import msgspec


class GameSession(msgspec.Struct):
    """Session in the game.

    Each session have own journal and we must read last journal to get session information.

    Journal creates → we create session.
    """

    started_at: datetime.datetime

    power: str | None = None
    total_merits: int = 0

    deaths: int = 0
    ships_lost: int = 0
    rebuy_cost_total: int = 0

    kills: int = 0
    ships_destroyed: int = 0

    bounties_earned: int = 0
    bounties_earned_per_hour: float = 0
    """This is placeholder for calculated value e.g., from `*_per_hour`."""

    merits_earned: int = 0
    merits_earned_per_hour: float = 0
    """This is placeholder for calculated value e.g., from `*_per_hour`."""

    combat_bonds_earned: int = 0
    combat_bonds_earned_per_hour: float = 0
    """This is placeholder for calculated value e.g., from `*_per_hour`."""

    credits_earned: int = 0
    """This is placeholder for calculated value from `get_total_credits_earned`."""
    credits_earned_per_hour: float = 0
    """This is placeholder for calculated value e.g., from `*_per_hour`."""

    def hours_played(self) -> float:
        now = datetime.datetime.now()
        delta = now - self.started_at
        return max(delta.total_seconds() / 3600, 1e-6)

    def get_total_credits_earned(self) -> int:
        return self.bounties_earned + self.combat_bonds_earned

    def get_kills_per_hour(self) -> float:
        return self.kills / self.hours_played()

    def get_combat_bonds_earned_per_hour(self) -> float:
        return self.combat_bonds_earned / self.hours_played()

    def get_merits_earned_per_hour(self) -> float:
        return self.merits_earned / self.hours_played()

    def get_bounties_earned_per_hour(self) -> float:
        return self.bounties_earned / self.hours_played()

    def get_total_credits_earned_per_hour(self) -> float:
        return self.get_total_credits_earned() / self.hours_played()
