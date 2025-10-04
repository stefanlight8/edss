from __future__ import annotations

import asyncio
import os
import pathlib
import logging
from collections.abc import AsyncGenerator

from edss_journal.fs.exceptions import DirectoryNotExists

import aiofiles

__all__: tuple[str, ...] = ("JournalReader",)


class JournalReader:
    __slots__: tuple[str, ...] = (
        "_logger",
        "folder_path",
        "poll_interval",
        "journal_file",
    )

    def __init__(self, folder_path: pathlib.Path, poll_interval: float = 0.5) -> None:
        self._logger: logging.Logger = logging.getLogger("edss.journal.reader")

        self.folder_path = folder_path
        self.poll_interval = poll_interval

        self.journal_file: pathlib.Path | None = None

    def get_last_journal_file(self) -> pathlib.Path:
        if not self.folder_path.exists():
            raise DirectoryNotExists(self.folder_path)
        return max(
            self.folder_path.glob("Journal.*.log"), key=lambda journal: journal.name
        )

    async def get_last_line(self, file_path: pathlib.Path) -> bytes | None:
        try:
            async with aiofiles.open(file_path, "rb") as stream:
                await stream.seek(0, os.SEEK_END)
                file_size = await stream.tell()
                if file_size == 0:
                    return None
                buffer = b""
                chunk_size = 1024
                offset = file_size
                while offset > 0:
                    seek_to = max(0, offset - chunk_size)
                    await stream.seek(seek_to)
                    chunk = await stream.read(offset - seek_to)
                    buffer = chunk + buffer
                    if buffer.count(b"\n") >= 2 or seek_to == 0:
                        break
                    offset = seek_to
                lines = buffer.strip().split(b"\n")
                if lines and lines[-1]:
                    return lines[-1]
                return None
        except (FileNotFoundError, IOError):
            return None

    async def tail_lines(self) -> AsyncGenerator[pathlib.Path | bytes, None]:
        reader: aiofiles.threadpool.binary.AsyncBufferedReader | None = None
        while True:
            try:
                last_file = self.get_last_journal_file()
                if last_file != self.journal_file:
                    if reader:
                        await reader.close()
                    self.journal_file = last_file
                    if not self.journal_file:
                        await asyncio.sleep(self.poll_interval)
                        continue

                    reader = await aiofiles.open(self.journal_file, "rb")
                    await reader.seek(0, 2)
                    yield self.journal_file

                    self._logger.debug("new session: %s", self.journal_file)
                if not reader:
                    await asyncio.sleep(self.poll_interval)
                    continue

                line = await reader.readline()
                if line:
                    stripped_line = line.strip()
                    if stripped_line:
                        yield stripped_line
                else:
                    await asyncio.sleep(self.poll_interval)
            except FileNotFoundError:
                if reader:
                    await reader.close()
                    reader = None
                self.journal_file = None
                await asyncio.sleep(self.poll_interval)
