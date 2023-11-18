import logging
from json import dumps
from ouch_connect_nonblocking_python import (
    CltOuchSupervised,
    SvcOuchSupervised,
    Callback,
    ConId,
    Status,
)

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(threadName)s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

log = logging.getLogger(__name__)


class LoggerCallback(Callback):
    def on_sent(self, con_id: ConId, msg: str):
        log.info(f"on_sent: {con_id} {msg}")

    def on_recv(self, con_id: ConId, msg: str):
        log.info(f"on_recv: {con_id} {msg}")


svc = SvcOuchSupervised("127.0.0.1:8080", LoggerCallback(), 1, "test")

clt = CltOuchSupervised("127.0.0.1:8080", LoggerCallback())


assert svc.accept() == Status.Ok
assert svc.accept() == Status.Busy

msg = {"HBeat": {}}
clt.send(msg)

# clt.send(dumps("{HBeat:{}}"))
