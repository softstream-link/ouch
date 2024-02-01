This package is python extension module for rust crate [ouch_connect_nonblocking](./../../connect/nonblocking/readme.md)

## Installation

```shell
if [ -d ./ouch_connect ] ; then PREFIX="./../.." ; else PREFIX="." fi
micromamba create --name ouch_pypi_env --yes python=3.11
micromamba run --name ouch_pypi_env --cwd ${PREFIX} pip install ouch-connect links-connect
micromamba run --name ouch_pypi_env --cwd ${PREFIX} pip install markdown-code-runner
micromamba run --name ouch_pypi_env --cwd ${PREFIX} markdown-code-runner ./bindings/python/readme.md
```

## Usage
```python markdown-code-runner
import logging
from time import sleep
from ouch_connect import (
    CltAuto,
    SvcAuto,
)
from links_connect.callbacks import LoggerCallback


logging.basicConfig(format="%(asctime)-15s [%(threadName)10s %(levelname)8s] %(message)s \t%(filename)s:%(lineno)d")
logging.getLogger().setLevel(logging.INFO)
log = logging.getLogger(__name__)

callback = LoggerCallback(sent_level=logging.NOTSET)
addr = "127.0.0.1:8081"
usr = "dummy"
pwd = "dummy"
session = ""
sequence = 0
clt_max_hbeat_interval = 2.5
svc_max_hbeat_interval = 2.5
max_connections = 1
connect_timeout = 1.0
io_timeout = 0.1

with (
    SvcAuto(
        addr,
        callback,
        usr,
        pwd,
        session,
        clt_max_hbeat_interval,
        svc_max_hbeat_interval,
        max_connections,
        io_timeout,
        name="svc-ouch",
    ) as svc,
    CltAuto(
        addr,
        callback,
        usr,
        pwd,
        session,
        sequence,
        clt_max_hbeat_interval,
        svc_max_hbeat_interval,
        connect_timeout,
        io_timeout,
        name="clt-ouch",
    ) as clt,
):
    assert clt.is_connected() and svc.is_connected()

    log.info(f"svc: {svc}")
    log.info(f"clt: {clt}")

    clt.send({"Dbg": {"text": "Hello from Clt"}})
    svc.send({"Dbg": {"text": "Hello from Svc"}})

    sleep(0.5)
    log.info("********** awaiting receipt of Dbg messages **********")

```
