import logging
import ouch_connect
from ouch_connect import CltManual, SvcManual

log = logging.getLogger(__name__)


def test_doc():
    log.info(ouch_connect.__doc__)

    log.info(SvcManual.__doc__)
    assert SvcManual.__doc__ is not None

    log.info(SvcManual.is_connected.__doc__)
    assert SvcManual.is_connected.__doc__ is not None

    log.info(CltManual.__doc__)
    assert CltManual.__doc__ is not None

    log.info(CltManual.is_connected.__doc__)
    assert CltManual.is_connected.__doc__ is not None

    log.info(CltManual.send.__doc__)
    assert CltManual.send.__doc__ is not None

    # logging.info(type(CltManual.msg_samples))
    # logging.info("Valid Clt Dict Format\n" + "\n".join(CltManual.msg_samples))
    assert CltManual.msg_samples is not None


if __name__ == "__main__":
    import pytest

    pytest.main([__file__])
