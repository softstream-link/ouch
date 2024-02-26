import logging
from time import sleep
from random import randint
from ouch_connect import CltManual, SvcManual
from links_connect.callbacks import LoggerCallback, MemoryStoreCallback

log = logging.getLogger(__name__)

store = MemoryStoreCallback(default_find_timeout=0.2)
callback = LoggerCallback(sent_level=logging.NOTSET) + store
addr = f"127.0.0.1:{randint(1_000, 65_000)}"


def test_ouch_manual_connect():
    for i in range(1, 6):
        log.info(f"{'*'*60} Start {i} {'*'*60}")
        store.clear()
        assert len(store) == 0
        with (
            SvcManual(addr, callback, **dict(name="svc-ouch")) as svc,
            CltManual(addr, callback, **dict(name="clt-ouch")) as clt,
        ):
            assert clt.is_connected() and svc.is_connected()

            log.info(f"svc: {svc}")
            log.info(f"clt: {clt}")

            clt.send({"LoginRequest": {"username": "dummy", "password": "dummy", "session_id": "session #1", "sequence_number": "1"}})
            svc.send({"LoginAccepted": {"session_id": "session #1", "sequence_number": "1"}})

            clt.send({"HBeat": {}})
            svc.send({"HBeat": {}})

            found = store.find_recv("svc-ouch", {"LoginRequest": {}})
            assert found is not None and found.msg["LoginRequest"]["username"] == "dummy"

            found = store.find_recv("clt-ouch", {"LoginAccepted": {}})
            assert found is not None and found.msg["LoginAccepted"]["sequence_number"] == "1"

            log.info(f"{store}")
            assert len(store) >= 6  # LoginReq*2 + LoginAccept*2 + HbeatSent*2 + Might Get HbeatRecv*2

        sleep(0.1)  # OSError: Address already in use (os error 48)


if __name__ == "__main__":
    import pytest

    pytest.main([__file__])
