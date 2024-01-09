This package is python extension module for rust crate [ouch_connect_nonblocking](./../../connect/nonblocking/readme.md)

## Installation

```shell
micromamba create --name ouch_pypi_env --yes python=3.10
micromamba run --name ouch_pypi_env pip install --index-url https://test.pypi.org/simple/ ouch-connect==5.0.1rc1
micromamba run --name ouch_pypi_env pip install markdown-code-runner
micromamba run --name ouch_pypi_env markdown-code-runner ./bindings/python/readme.md
```

## Usage
```python markdown-code-runner
import logging
from time import sleep
from ouch_connect import (
    CltAuto,
    SvcAuto,
    LoggerCallback,
)


logging.basicConfig(format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s")
logging.getLogger().setLevel(logging.INFO)
log = logging.getLogger(__name__)

callback = LoggerCallback(logging.NOTSET)
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
