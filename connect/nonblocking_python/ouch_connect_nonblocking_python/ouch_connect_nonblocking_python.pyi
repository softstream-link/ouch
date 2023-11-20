from abc import ABC, abstractmethod
from enum import auto, Enum
from dataclasses import dataclass
from ouch_connect_nonblocking_python import Callback, MsgDict

class SendStatus(Enum):
    Ok = auto()
    Busy = auto()

class RecvStatus(Enum):
    Ok = auto()
    Busy = auto()
    def payload(self) -> MsgDict: ...


class AcceptStatus(Enum):
    Ok = auto()
    Busy = auto()

class CltOuchSupervised:
    def __init__(
        self, host: str, callback: Callback, name: str = "CltOuchSupervised"
    ) -> None: ...
    def send(self, msg: MsgDict) -> SendStatus: ...

class SvcOuchSupervised:
    def pool_accept(self) -> AcceptStatus: ...
    def send(self, msg: MsgDict) -> SendStatus: ...
    def recv(self) -> RecvStatus: ...
