from __future__ import annotations

from collections.abc import Mapping
import typing

import aiofiles
from aiofiles.threadpool.text import AsyncTextIOWrapper
import msgspec

__all__: tuple[str, ...] = ()


class JournalReader:
    @classmethod
    async def open(cls, path: str) -> JournalReader:
        file_handle: AsyncTextIOWrapper = await aiofiles.open(
            path, "r", encoding="UTF-8"
        ).__aenter__()
        return cls(path, file_handle)

    def __init__(self, path: str, file_handle: AsyncTextIOWrapper) -> None:
        self.path = path
        self.file_handle = file_handle

    async def read_last_event(self) -> Mapping[str, typing.Any] | None:
        await self.file_handle.seek(0, 2)
        line = await self.file_handle.readline()
        if line.strip():
            return msgspec.json.decode(line)

    async def close(self):
        await self.file_handle.close()
