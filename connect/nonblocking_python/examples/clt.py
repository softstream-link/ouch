import logging
from json import dumps, loads
from ouch_connect_nonblocking_python import (
    CltOuchSupervised,
    SvcOuchSupervised,
    Callback,
    ConId,
    SendStatus,
    AcceptStatus,
    RecvStatus,
    MsgDict,
)

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(threadName)s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

log = logging.getLogger(__name__)


class LoggerCallback(Callback):
    def on_sent(self, con_id: ConId, msg: MsgDict):
        log.info(f"on_sent: {con_id} {msg} {type(msg)}")

    def on_recv(self, con_id: ConId, msg: MsgDict):
        log.info(f"on_recv: {con_id} {msg} {type(msg)}")


svc = SvcOuchSupervised("127.0.0.1:8080", LoggerCallback(), 1, "test")

clt = CltOuchSupervised("127.0.0.1:8080", LoggerCallback())


assert svc.pool_accept() == AcceptStatus.Ok
assert svc.pool_accept() == AcceptStatus.Busy

assert clt.send({"HBeat": {}}) == SendStatus.Ok

status = svc.recv()
assert status == RecvStatus.Ok
print(status.payload(), type(status.payload()))
