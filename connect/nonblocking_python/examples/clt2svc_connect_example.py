import logging
from json import dumps, loads
from time import sleep
from ouch_connect_nonblocking_python import (
    CltOuchSupervised,
    SvcOuchSupervised,
    LoggerCallback,
    SendStatus,
    RecvStatus,
)
from ouch_connect_nonblocking_python.ouch_connect_nonblocking_python import (
    AcceptStatus,
    RecvStatus,
)

FORMAT = "%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

log = logging.getLogger(__name__)


svc = SvcOuchSupervised("127.0.0.1:8080", LoggerCallback(), 1, 1.0, name="svc/ouch")
log.info(f"svc: {svc}")


clt = CltOuchSupervised(
    "127.0.0.1:8080", LoggerCallback(), timeout=1.0, name="clt/ouch"
)
log.info(f"clt1: {clt}")


# assert clt.send({"HBeat": {}}) == SendStatus.Ok
# status = svc.recv()
# assert status == RecvStatus.Ok
# log.info(f"payload: {status.payload()}")

assert svc.send({"HBeat": {}}) == SendStatus.Ok
status = clt.recv()
assert status == RecvStatus.Ok
log.info(f"clt.recv payload: {status.payload()}")

svc = svc.into_sender()
log.info(f"svc: {svc}")
sleep(.5)
assert clt.send({"HBeat": {}}) == SendStatus.Ok
sleep(.5)



# clt2 = CltOuchSupervised("127.0.0.1:8080", LoggerCallback(), name="clt2")
# log.info(f"clt2: {clt2}")
# assert clt2.send({"HBeat": {}}) == SendStatus.Ok

# svc.send({"HBeat": {}})
# log.info(f"svc: {svc}")


sleep(0.1)
