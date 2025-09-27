from __future__ import annotations

from collections.abc import Mapping
import pathlib
import asyncio
from typing import Any, AsyncGenerator

from watchdog.observers import Observer
from watchdog.events import (
    DirCreatedEvent,
    DirModifiedEvent,
    FileCreatedEvent,
    FileModifiedEvent,
    FileSystemEventHandler,
)
from watchdog.observers.api import BaseObserver

from edss_journal.aio.reader import JournalReader
from edss_journal.events.base import BaseEvent

__all__: tuple[str, ...] = ()


class DirectoryNotExists(Exception):
    ...


class JournalEventHandler(FileSystemEventHandler):
    def __init__(self, queue: asyncio.Queue[str], loop: asyncio.AbstractEventLoop) -> None:
        self._file_queue = queue
        self._loop = loop

    def put_file_in_queue(self, src_path: bytes | str) -> None:
        file_path: str
        if isinstance(src_path, str):
            file_path = src_path
        elif isinstance(src_path, memoryview):
            file_path = src_path.tobytes().decode("utf-8")
        else:
            file_path = src_path.decode("utf-8")
        self._loop.call_soon_threadsafe(self._file_queue.put_nowait, file_path)

    def on_created(self, event: DirCreatedEvent | FileCreatedEvent) -> None:
        if not event.is_directory:
            self.put_file_in_queue(event.src_path)

    def on_modified(self, event: DirModifiedEvent | FileModifiedEvent) -> None:
        if not event.is_directory:
            self.put_file_in_queue(event.src_path)


class JournalObserver:
    def __init__(self, folder: pathlib.Path, loop: asyncio.AbstractEventLoop) -> None:
        self._base_folder = folder

        self._file_queue: asyncio.Queue[str] = asyncio.Queue()

        self._observer: BaseObserver = Observer()
        self._event_handler: JournalEventHandler = JournalEventHandler(self._file_queue, loop)

        self._reader: JournalReader | None = None
    
    def start(self) -> None:
        if not self._base_folder.exists():
            raise DirectoryNotExists(f"{self._base_folder} is not exists")
        self._observer.schedule(self._event_handler, str(self._base_folder))
        self._observer.start()

    async def get_events(self) -> AsyncGenerator[BaseEvent]:
        while True:
            src_path = await self._file_queue.get()
            if not self._reader:
                self._reader = await JournalReader.open(src_path)
            elif self._reader.path is not src_path:
                await self._reader.close()
                self._reader = await JournalReader.open(src_path)
            last_event_payload: Mapping[str, Any] | None = await self._reader.read_last_event()
            if last_event_payload is None:
                pass
            # now we need event factory that will convert event payload to event object from name
            # :b
            yield BaseEvent()
            self._file_queue.task_done()
