import logging
from math import log

from ouch_connect.ouch_connect import SvcManual


logging.basicConfig(format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s")
logging.getLogger().setLevel(logging.INFO)
from ouch_connect import ConType, CltManual


def test_con_type():
    con_ty = ConType.Initiator
    logging.info(con_ty)

    logging.info(CltManual.__doc__)
    logging.info(SvcManual.__doc__)


if __name__ == "__main__":
    test_con_type()
