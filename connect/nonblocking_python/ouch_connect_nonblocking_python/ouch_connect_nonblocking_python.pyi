from abc import ABC, abstractmethod
from enum import auto
from enum import Enum
from dataclasses import dataclass
from ouch_connect_nonblocking_python import Callback

class Status(Enum):
    Ok = auto()
    Busy = auto()

class CltOuchSupervised:
    def __init__(
        self, host: str, callback: Callback, name: str = "CltOuchSupervised"
    ) -> None: ...
    def send(self, msg: str | dict) -> Status: ...

class SvcOuchSupervised:
    def accept(self) -> Status: ...
