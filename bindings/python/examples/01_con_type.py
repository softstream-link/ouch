import logging

logging.basicConfig(
    format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s"
)
logging.getLogger().setLevel(logging.INFO)
from ouch_connect import ConType

con_ty = ConType.Initiator
logging.info(con_ty)
