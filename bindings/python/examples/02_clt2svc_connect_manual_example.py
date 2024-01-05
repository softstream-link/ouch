import logging
from time import sleep
from ouch_connect import (
    CltManual,
    SvcManual,
    LoggerCallback,
)

logging.basicConfig(format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s")
logging.getLogger().setLevel(logging.INFO)
log = logging.getLogger(__name__)

callback = LoggerCallback(sent_level=logging.NOTSET, recv_level=logging.INFO)
addr = "127.0.0.1:8080"
max_connections = 1
io_timeout = 0.01
connect_timeout = 1.0

with (
    SvcManual(addr, callback, max_connections, io_timeout, name="svc-ouch") as svc,
    CltManual("127.0.0.1:8080", callback, connect_timeout, io_timeout, name="clt-ouch") as clt,
):
    assert clt.is_connected() and svc.is_connected()

    log.info(f"svc: {svc}")
    log.info(f"clt: {clt}")

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
    log.info("********** awaiting receipt of HBeat messages **********")
