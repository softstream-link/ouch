from abc import ABC, abstractmethod
from enum import auto, Enum
from dataclasses import dataclass
from ouch_connect_nonblocking_python import Callback, MsgDict

class AcceptStatus(Enum):
    Ok = auto()
    WouldBlock = auto()

class SendStatus(Enum):
    Ok = auto()
    WouldBlock = auto()

class RecvStatus(Enum):
    Ok = auto()
    WouldBlock = auto()
    def payload(self) -> MsgDict: ...

class CltOuchSupervised:
    def __init__(
        self,
        host: str,
        callback: Callback,
        timeout: float | None = None,
        name: str = "CltOuchSupervised",
    ) -> None: ...
    def send(self, msg: MsgDict, timeout: float | None = None) -> SendStatus: ...
    def recv(self, timeout: float | None = None) -> RecvStatus: ...


class SvcOuchSender:
    def send(self, msg: MsgDict, timeout: float | None = None) -> SendStatus: ...

class SvcOuchSupervised:
    def __init__(
        self,
        host: str,
        callback: Callback,
        max_connections: int = 1,
        timeout: float | None = None,
        name: str = "SvcOuchSupervised",
    ) -> None: ...
    def send(self, msg: MsgDict, timeout: float | None = None) -> SendStatus: ...
    def pool_accept(self, timeout: float | None = None) -> AcceptStatus: ...
    def recv(self, timeout: float | None = None) -> RecvStatus: ...
    def into_sender(self) -> SvcOuchSender: ...