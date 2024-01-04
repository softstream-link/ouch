import logging
from time import sleep
from ouch_bindings_py import (
    CltManual,
    SvcManual,
    LoggerCallback,
)

logging.basicConfig(
    format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
)
logging.getLogger().setLevel(logging.DEBUG)
log = logging.getLogger(__name__)

# log.info(ouch_connect_nonblocking_python.__doc__)

callback = LoggerCallback(logging.NOTSET)
# callback = LoggerCallback()
svc = SvcManual("127.0.0.1:8080", callback, io_timeout=0.01, name="svc-ouch")
clt = CltManual(
    "127.0.0.1:8080", callback, connect_timeout=1.0, io_timeout=0.01, name="clt-ouch"
)
assert clt.is_connected() and svc.is_connected()

log.info(f"svc: {svc}")
log.info(f"clt: {clt}")

# help("ouch_connect_nonblocking_python")


clt.send(
    {
        "LoginRequest": {
            "username": "dummy",
            "password": "dummy",
            "session_id": "session #1",
            "sequence_number": "1",
            "hbeat_timeout_ms": "1000",
        }
    }
)
svc.send({"LoginAccepted": {"session_id": "session #1", "sequence_number": "1"}})

clt.send({"HBeat": {}})
svc.send({"HBeat": {}})

sleep(0.5)
