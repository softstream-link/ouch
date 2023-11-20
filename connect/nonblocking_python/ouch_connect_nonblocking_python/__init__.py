from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum, auto
from logging import error, info
from typing import Any
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
        error(f"on_fail: {con_id} {msg} {err}")

    def on_send(self, con_id: ConId, msg: MsgDict) -> None:
        # info(f"on_send: {con_id} {msg}")
        pass
