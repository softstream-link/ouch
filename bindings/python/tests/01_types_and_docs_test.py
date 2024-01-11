import logging
from math import log

logging.basicConfig(format="%(levelname)s  %(asctime)-15s %(threadName)s %(name)s %(filename)s:%(lineno)d %(message)s")
logging.getLogger().setLevel(logging.INFO)
import ouch_connect
from ouch_connect import ConType, CltManual, SvcManual


def test_con_type():
    con_ty = ConType.Initiator
    logging.info(con_ty)


def test_doc():
    logging.info(ouch_connect.__doc__)

    # logging.info(SvcManual.__doc__)
    # assert SvcManual.__doc__ is not None

    # logging.info(SvcManual.is_connected.__doc__)
    # assert SvcManual.is_connected.__doc__ is not None

    logging.info(CltManual.__doc__)
    assert CltManual.__doc__ is not None

    logging.info(CltManual.is_connected.__doc__)
    assert CltManual.is_connected.__doc__ is not None

    logging.info(CltManual.send.__doc__)
    assert CltManual.send.__doc__ is not None

    # logging.info(type(CltManual.msg_samples))
    logging.info("Valid Clt Dict Format\n" + "\n".join(CltManual.msg_samples))
    assert CltManual.msg_samples is not None


if __name__ == "__main__":
    test_con_type()
    test_doc()
