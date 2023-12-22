import logging
from time import sleep

# import ouch_connect_nonblocking_python

from ouch_connect_nonblocking_python import (
    CltAuto,
    SvcAuto,
    LoggerCallback,
)
from ouch_connect_nonblocking_python.ouch_connect_nonblocking_python import (
    CltAuto,
    SvcAuto,
)

logging.basicConfig(
    format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
)
logging.getLogger().setLevel(logging.DEBUG)
log = logging.getLogger(__name__)

# log.info(ouch_connect_nonblocking_python.__doc__)

callback = LoggerCallback(logging.NOTSET)

usr = "dummy"
pwd = "dummy"
session = ""
sequence = 0
clt_max_hbeat_interval = 2.5
svc_max_hbeat_interval = 2.5
io_timeout = 0.1

svc = SvcAuto(
    "127.0.0.1:8080",
    callback,
    usr,
    pwd,
    session,
    svc_max_hbeat_interval,
    max_connections=1,
    io_timeout=io_timeout,
    name="svc-ouch",
)

clt = CltAuto(
    "127.0.0.1:8080",
    callback,
    usr,
    pwd,
    session,
    sequence,
    clt_max_hbeat_interval,
    svc_max_hbeat_interval,
    connect_timeout=1.0,
    io_timeout=io_timeout,
    name="clt-ouch",
)
assert clt.is_connected() and svc.is_connected()

log.info(f"svc: {svc}")
log.info(f"clt: {clt}")

# help("ouch_connect_nonblocking_python")


clt.send({"Dbg": {"text": "Hello from Clt"}})
svc.send({"Dbg": {"text": "Hello from Svc"}})
# svc.send({"LoginAccepted": {"session_id": "session #1", "sequence_number": "1"}})

# sleep(1)

