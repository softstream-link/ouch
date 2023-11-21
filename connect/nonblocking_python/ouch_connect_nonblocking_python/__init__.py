from abc import ABC, abstractmethod
from concurrent.futures import thread
from dataclasses import dataclass
from enum import Enum, auto
from logging import error, info
from threading import current_thread
from typing import Any
import logging
from .ouch_connect_nonblocking_python import *


class ConType(Enum):
    Initiator = auto()
    Acceptor = auto()


@dataclass
class ConId(Enum):
    con_type: ConType
    name: str
    local: str
    peer: str


type MsgDict = dict[str, str | int | float | bool | Any]


class Callback(ABC):

    @abstractmethod
    def on_recv(self, con_id: ConId, msg: MsgDict) -> None:
        ...

    @abstractmethod
    def on_sent(self, con_id: ConId, msg: MsgDict) -> None:
        ...

    def on_fail(self, con_id: ConId, msg: MsgDict, err: str) -> None:
        error(f"on_fail: {con_id} {type(msg).__name__}({msg}) {err}")

    def on_send(self, con_id: ConId, msg: MsgDict) -> None:
        pass


class LoggerCallback(Callback):
    def __init__(self, sent_level=logging.INFO, recv_level=logging.INFO) -> None:
        super().__init__()
        self.sent_level = sent_level
        self.recv_level = recv_level

    def on_sent(self, con_id: ConId, msg: MsgDict):
        logging.getLogger(__name__).log(self.sent_level, f"on_sent: {con_id} {type(msg).__name__}({msg})")

    def on_recv(self, con_id: ConId, msg: MsgDict):
        logging.getLogger(__name__).log(self.recv_level, f"on_recv: {con_id} {type(msg).__name__}({msg})")
