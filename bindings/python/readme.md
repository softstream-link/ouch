This package is python extension module for rust crate [ouch_connect_nonblocking](./../../connect/nonblocking/readme.md)

## Installation & Test
* `pytest` expects `ouch_connect` source to be checked out from github. Test files are located in `./bindings/python/tests` directory.

```shell
micromamba create --name ouch_pypi_env --yes python
micromamba run --name ouch_pypi_env pip install "ouch-connect[test]>=5.0.0,<5.1.0"
micromamba run --name ouch_pypi_env pytest
```


## Basic Usage Example
```python
import logging
from time import sleep
from ouch_connect import CltAuto, SvcAuto
from links_connect.callbacks import LoggerCallback, DecoratorDriver, on_recv, on_sent, MemoryStoreCallback


logging.basicConfig(format="%(asctime)-15s [%(threadName)10s|%(levelname)8s] %(message)s \t%(filename)s:%(lineno)d")
logging.getLogger().setLevel(logging.INFO)
log = logging.getLogger(__name__)
addr = "127.0.0.1:8080"

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

store = MemoryStoreCallback()
clt_clbk = LoggerCallback(sent_level=logging.NOTSET) + store
svc_clbk = SimulatorExample() + store
with (
    SvcAuto(addr, svc_clbk, **dict(name="svc-ouch")) as svc,
    CltAuto(addr, clt_clbk, **dict(name="clt-ouch")) as clt,
):
    assert clt.is_connected() and svc.is_connected()

    log.info(f"svc: {svc}")
    log.info(f"clt: {clt}")

    clt.send({"Dbg": {"text": "Hello from Clt"}})

    found = store.find_recv(name="svc-ouch", filter={"Dbg":{}})
    assert found is not None and found.msg["Dbg"]["text"] == "Hello from Clt"
    log.info(f"found: {found}")

    found = store.find_recv(name="clt-ouch", filter={"Dbg":{}})
    assert found is not None and found.msg["Dbg"]["text"] == "Hello from Simulator"
    log.info(f"found: {found}")
```
