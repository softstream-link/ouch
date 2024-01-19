import logging
from time import sleep
from random import randint
from ouch_connect import (
    CltManual,
    SvcManual,
    LoggerCallback,
)

logging.basicConfig(format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s")
logging.getLogger().setLevel(logging.INFO)
log = logging.getLogger(__name__)

callback = LoggerCallback(sent_level=logging.NOTSET, recv_level=logging.INFO)
addr = f"127.0.0.1:{randint(1_000, 65_000)}"
max_connections = 1
io_timeout = 0.5
connect_timeout = 1.0


def test_ouch_manual_connect():
    with (
        SvcManual(addr, callback, max_connections, io_timeout, name="svc-ouch") as svc,
        CltManual(addr, callback, connect_timeout, io_timeout, name="clt-ouch") as clt,
    ):
        assert clt.is_connected() and svc.is_connected()

        log.info(f"svc: {svc}")
        log.info(f"clt: {clt}")

        clt.send({"LoginRequest": {"username": "dummy", "password": "dummy", "session_id": "session #1", "sequence_number": "1"}})
        svc.send({"LoginAccepted": {"session_id": "session #1", "sequence_number": "1"}})

        clt.send({"HBeat": {}})
        svc.send({"HBeat": {}})

        sleep(0.5)
        log.info("********** awaiting receipt of HBeat messages **********")


if __name__ == "__main__":
    test_ouch_manual_connect()
