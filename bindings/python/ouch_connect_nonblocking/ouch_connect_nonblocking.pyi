from ouch_connect_nonblocking import Callback, MsgDict

class CltManual:
    def __init__(
        self,
        host: str,
        callback: Callback,
        connect_timeout: float | None = None,
        io_timeout: float | None = None,
        name: str | None = None,
    ) -> None: ...
    def send(self, msg: dict, io_timeout: float | None = None): ...
    def is_connected(self, io_timeout: float | None = None): ...

class SvcManual:
    def __init__(
        self,
        host: str,
        callback: Callback,
        max_connections: int | None = None,
        io_timeout: float | None = None,
        name: str | None = None,
    ) -> None: ...
    def send(self, msg: dict, io_timeout: float | None = None): ...
    def is_connected(self, io_timeout: float | None = None): ...

class CltAuto:
    def __init__(
        self,
        host: str,
        callback: Callback,
        usr: str,
        pwd: str,
        session: str,
        sequence: int,
        clt_max_hbeat_interval: float,
        svc_max_hbeat_interval: float,
        connect_timeout: float | None = None,
        io_timeout: float | None = None,
        name: str | None = None,
    ) -> None: ...
    def send(self, msg: dict, io_timeout: float | None = None): ...
    def is_connected(self, io_timeout: float | None = None): ...

class SvcAuto:
    def __init__(
        self,
        host: str,
        callback: Callback,
        usr: str,
        pwd: str,
        session: str,
        svc_max_hbeat_interval: float,
        max_connections: int | None = None,
        io_timeout: float | None = None,
        name: str | None = None,
    ) -> None: ...
    def send(self, msg: dict, io_timeout: float | None = None): ...
    def is_connected(self, io_timeout: float | None = None): ...
