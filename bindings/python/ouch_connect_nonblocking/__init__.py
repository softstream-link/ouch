"""
This module provides a Python interface to the `ouch_connect_nonblocking` library.

Below contains a list of valid message formats that both Clt & Svc can send. Examples include a full form for each message, however many of the 
fields are optional and can be omitted.

"""


from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum, auto

# from typing import Any
import logging
from .ouch_connect_nonblocking import *


class ConType(Enum):
    Initiator = auto()
    Acceptor = auto()


@dataclass
class ConId(Enum):
    con_type: ConType
    name: str
    local: str
    peer: str


MsgDict = dict[str, str | int | float | bool | dict | list]  # Any?


class Callback(ABC):
    @abstractmethod
    def on_recv(self, con_id: ConId, msg: MsgDict) -> None:
        ...

    @abstractmethod
    def on_sent(self, con_id: ConId, msg: MsgDict) -> None:
        ...


class LoggerCallback(Callback):
    def __init__(self, sent_level=logging.INFO, recv_level=logging.INFO) -> None:
        super().__init__()
        self.sent_level = sent_level
        self.recv_level = recv_level

    def on_sent(self, con_id: ConId, msg: MsgDict):
        logging.getLogger(__name__).log(
            self.sent_level, f"on_sent: {con_id} {type(msg).__name__}({msg})"
        )

    def on_recv(self, con_id: ConId, msg: MsgDict):
        logging.getLogger(__name__).log(
            self.recv_level, f"on_recv: {con_id} {type(msg).__name__}({msg})"
        )
