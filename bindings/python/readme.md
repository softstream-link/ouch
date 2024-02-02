This package is python extension module for rust crate [ouch_connect_nonblocking](./../../connect/nonblocking/readme.md)

## Installation

```shell
if [ -d ./ouch_connect ] ; then CWD="./../.." ; else CWD="." ; fi
cd ${CWD}
micromamba create --name ouch_pypi_env --yes python
micromamba run --name ouch_pypi_env pip install ouch-connect links-connect
micromamba run --name ouch_pypi_env pip install markdown-code-runner
micromamba run --name ouch_pypi_env markdown-code-runner ./bindings/python/readme.md
```

## Usage
```python markdown-code-runner
import logging
from time import sleep
from ouch_connect import CltAuto, SvcAuto
from links_connect.callbacks import LoggerCallback, DecoratorDriver, on_recv, on_sent



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

log_clbk = LoggerCallback(sent_level=logging.NOTSET)
sim_clbk = SimulatorExample()
with (
    SvcAuto(addr, sim_clbk, **dict(name="svc-ouch")) as svc,
    CltAuto(addr, log_clbk, **dict(name="clt-ouch")) as clt,
):
    assert clt.is_connected() and svc.is_connected()

    log.info(f"svc: {svc}")
    log.info(f"clt: {clt}")

    clt.send({"Dbg": {"text": "Hello from Clt"}})

    sleep(0.5)
    log.info("********** awaiting receipt of Dbg messages **********")

```
