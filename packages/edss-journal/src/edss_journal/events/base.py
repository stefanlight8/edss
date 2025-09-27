from typing import ClassVar
import msgspec


class BaseEvent(msgspec.Struct):
    event: ClassVar[str]
