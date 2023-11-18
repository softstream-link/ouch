from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum, auto
from logging import error
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





class Callback(ABC):
    @abstractmethod
    def on_recv(self, con_id: ConId, msg: str) -> None:
        ...

    @abstractmethod
    def on_sent(self, con_id: ConId, msg: str) -> None:
        ...

    def on_fail(self, con_id: ConId, msg: str, err: str) -> None:
        error(f"on_fail: {con_id} {msg} {err}")
