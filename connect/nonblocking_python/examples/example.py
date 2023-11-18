import logging
from json import dumps
from ouch_connect_nonblocking_python import sum_as_string

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(threadName)s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

log = logging.getLogger(__name__)

log.info("Starting example.py")

x = sum_as_string(1,2)
log.info(f'x = {x}')


