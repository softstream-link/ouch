import logging
from time import sleep
from random import randint
from ouch_connect import CltAuto, SvcAuto
from links_connect.callbacks import LoggerCallback, DecoratorDriver, on_recv, on_sent

log = logging.getLogger(__name__)


addr = f"127.0.0.1:{randint(2_000, 65_000)}"


def test_ouch_auto_connect():
    class SimulatorExample(DecoratorDriver):
        @on_recv({"Dbg": {}})
        def on_dbg(self, con_id, msg):
            self.sender.send({"Dbg": {"text": "Hello from Simulator"}})

        @on_recv({})
        def on_all_recv(self, con_id, msg):
            pass

        @on_sent({})
        def on_all_sent(self, con_id, msg):
            pass

    sim_clbk = SimulatorExample()
    log_clbk = LoggerCallback(logging.INFO, logging.DEBUG)
    with (
        SvcAuto(addr, sim_clbk, **dict(name="svc-ouch")) as svc,
        CltAuto(addr, log_clbk, **dict(name="clt-ouch")) as clt,
    ):
        assert clt.is_connected() and svc.is_connected()

        log.info(f"svc: {svc}")
        log.info(f"clt: {clt}")

        clt.send({"Dbg": {"text": "Hello from Clt"}})
        # svc.send({"Dbg": {"text": "Hello from Svc"}})

        sleep(0.5)
        log.info("********** awaiting receipt of Dbg messages **********")


if __name__ == "__main__":
    import pytest

    pytest.main([__file__])
