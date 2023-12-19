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

callback = LoggerCallback(logging.NOTSET)

svc = SvcManual("127.0.0.1:8080", callback, 1, 1.0, name="svc-ouch")
log.info(f"svc: {svc}")


clt = CltManual("127.0.0.1:8080", callback, timeout=1.0, name="clt-ouch")
log.info(f"clt: {clt}")
assert clt.is_connected() and svc.is_connected()

log.info(f"svc: {svc}")



clt.send({"Login":{"username":"dummy","password":"dummy","session_id":"session #1","sequence_number":"1","hbeat_timeout_ms":"1000"}})
svc.send({"LoginAccepted":{"session_id":"session #1","sequence_number":"1"}})
# log.info(f"svc: {svc}")
sleep(.5)

