import logging
from time import sleep
from random import randint
from ouch_connect import CltAuto, SvcAuto
from links_connect.callbacks import LoggerCallback

log = logging.getLogger(__name__)


callback = LoggerCallback(logging.INFO, logging.DEBUG)
# addr = "127.0.0.1:8081"
addr = f"127.0.0.1:{randint(1_000, 65_000)}"
usr = "dummy"
pwd = "dummy"
session = ""
sequence = 0
clt_max_hbeat_interval = 2.5
svc_max_hbeat_interval = 2.5
max_connections = 1
connect_timeout = 1.0
io_timeout = 1.0


def test_ouch_auto_connect():
    with (
        SvcAuto(
            addr, callback, usr, pwd, session, clt_max_hbeat_interval, svc_max_hbeat_interval, max_connections, io_timeout, name="svc-ouch"
        ) as svc,
        CltAuto(
            addr, callback, usr, pwd, session, sequence, clt_max_hbeat_interval, svc_max_hbeat_interval, connect_timeout, io_timeout, name="clt-ouch"
        ) as clt,
    ):
        assert clt.is_connected() and svc.is_connected()

        log.info(f"svc: {svc}")
        log.info(f"clt: {clt}")

        clt.send({"Dbg": {"text": "Hello from Clt"}})
        svc.send({"Dbg": {"text": "Hello from Svc"}})

        sleep(0.5)
        log.info("********** awaiting receipt of Dbg messages **********")


if __name__ == "__main__":
    import pytest

    pytest.main([__file__])
