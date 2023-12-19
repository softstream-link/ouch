from abc import ABC, abstractmethod
from enum import auto, Enum
from dataclasses import dataclass
from ouch_connect_nonblocking_python import Callback, MsgDict

class CltManual:
    def __init__(
        self,
        host: str,
        callback: Callback,
        timeout: float | None = None,
        name: str = "CltManual",
    ) -> None: ...
    def send(self, msg: MsgDict, timeout: float | None = None): ...
    def is_connected(self, timeout: float | None = None): ...
    
class SvcManual:
    def __init__(
        self,
        host: str,
        callback: Callback,
        max_connections: int = 1,
        timeout: float | None = None,
        name: str = "SvcManual",
    ) -> None: ...
    def send(self, msg: MsgDict, timeout: float | None = None): ...
    def is_connected(self, timeout: float | None = None): ...
