import logging
from json import dumps, loads
from re import DEBUG
from time import sleep
from ouch_connect_nonblocking_python import (
    CltManual,
    SvcManual,
    LoggerCallback,
)

FORMAT = "%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

log = logging.getLogger(__name__)

callback = LoggerCallback(logging.DEBUG)

svc = SvcOuchManual("127.0.0.1:8080", callback, 1, 1.0, name="svc-ouch")
log.info(f"svc: {svc}")


clt = CltOuchManual(
    "127.0.0.1:8080", callback, timeout=1.0, name="clt-ouch"
)
log.info(f"clt: {clt}")
sleep(1)
# log.info(f"svc.is_connected: {svc.is_next_connected(2.0)}")
log.info(f"svc: {svc}")


clt.send({"Login":{"username":"dummy","password":"dummy","session_id":"session #1","sequence_number":"1","hbeat_timeout_ms":"1000"}})
svc.send({"LoginAccepted":{"session_id":"session #1","sequence_number":"1"}})
sleep(1)

# # status = svc.recv()
# # assert status == RecvStatus.Ok
# # log.info(f"payload: {status.payload()}")

# assert svc.send({"HBeat": {}}) == SendStatus.Ok
# status = clt.recv()
# assert status == RecvStatus.Ok
# log.info(f"clt.recv payload: {status.payload()}")

# svc = svc.into_sender()
# log.info(f"svc: {svc}")
# sleep(.5)
# assert clt.send({"HBeat": {}}) == SendStatus.Ok
# sleep(.5)



# # clt2 = CltOuchSupervised("127.0.0.1:8080", LoggerCallback(), name="clt2")
# # log.info(f"clt2: {clt2}")
# # assert clt2.send({"HBeat": {}}) == SendStatus.Ok

# # svc.send({"HBeat": {}})
# # log.info(f"svc: {svc}")


# sleep(0.1)
