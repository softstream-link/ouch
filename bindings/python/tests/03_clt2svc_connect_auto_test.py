import logging
from time import sleep
from random import randint
from ouch_connect import CltAuto, SvcAuto
from links_connect.callbacks import LoggerCallback, DecoratorDriver, on_recv, on_sent, MemoryStoreCallback

log = logging.getLogger(__name__)


addr = f"127.0.0.1:{randint(2_000, 65_000)}"


def test_ouch_auto_connect():
    class SimulatorExample(DecoratorDriver):
        @on_recv({"Dbg": {}})
        def on_dbg(self, con_id, msg):
            i = msg["Dbg"]["text"].split("#")[1]
            self.sender.send({"Dbg": {"text": f"Hello from Simulator #{i}"}})

        @on_recv({})
        def on_all_recv(self, con_id, msg):
            pass

        @on_sent({})
        def on_all_sent(self, con_id, msg):
            pass

    store = MemoryStoreCallback(default_find_timeout=0.2)
    sim_clbk = SimulatorExample() + store
    log_clbk = LoggerCallback(logging.INFO, logging.DEBUG) + store
    for i in range(1, 6):
        log.info(f"{'*'*60} Start {i} {'*'*60}")
        store.clear()
        assert len(store) == 0

        with (
            SvcAuto(addr, sim_clbk, **dict(name="svc-ouch")) as svc,
            CltAuto(addr, log_clbk, **dict(name="clt-ouch")) as clt,
        ):
            assert clt.is_connected() and svc.is_connected()

            log.info(f"svc: {svc}")
            log.info(f"clt: {clt}")

            clt.send({"Dbg": {"text": f"Hello from Clt #{i}"}})

            found = store.find_recv(name="svc-ouch", filter={"Dbg": {}})
            log.info(f"found: {found}")
            assert found is not None and "Hello from Clt" in found.msg["Dbg"]["text"]

            found = store.find_recv(name="clt-ouch", filter={"Dbg": {}})
            log.info(f"found: {found}")
            assert found is not None and "Hello from Simulator" in found.msg["Dbg"]["text"]

            log.info(f"{store}")

        sleep(0.1)  # OSError: Address already in use (os error 48)


if __name__ == "__main__":
    import pytest

    pytest.main([__file__])
