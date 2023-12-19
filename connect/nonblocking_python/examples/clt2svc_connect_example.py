import logging
from time import sleep
from ouch_connect_nonblocking_python import (
    CltManual,
    SvcManual,
    LoggerCallback,
)

logging.basicConfig(
    format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
)
logging.getLogger().setLevel(logging.INFO)
log = logging.getLogger(__name__)

callback = LoggerCallback(logging.NOTSET)

svc = SvcManual("127.0.0.1:8080", callback, timeout=1.0, name="svc-ouch")
clt = CltManual("127.0.0.1:8080", callback, timeout=1.0, name="clt-ouch")
assert clt.is_connected() and svc.is_connected()

log.info(f"svc: {svc}")
log.info(f"clt: {clt}")

# help("ouch_connect_nonblocking_python")
# log.info(ouch_connect_nonblocking_python)

clt.send(
    {
        "Login": {
            "username": "dummy",
            "password": "dummy",
            "session_id": "session #1",
            "sequence_number": "1",
            "hbeat_timeout_ms": "1000",
        }
    }
)
svc.send({"LoginAccepted": {"session_id": "session #1", "sequence_number": "1"}})

sleep(0.5)
