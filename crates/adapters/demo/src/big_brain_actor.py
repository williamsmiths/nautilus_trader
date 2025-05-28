import sys

sys.path.append("/home/twitu/Code/nautilus_trader/")

from nautilus_trader.core.nautilus_pyo3.model import DataType
from nautilus_trader.core.nautilus_pyo3.common import register_response_handler
from nautilus_trader.core.nautilus_pyo3.common import send_request
from nautilus_trader.core.nautilus_pyo3.common import register
from nautilus_trader.core.nautilus_pyo3.common import RequestCustomData
from nautilus_trader.core.nautilus_pyo3.common import init_logging
from nautilus_trader.core.nautilus_pyo3.common import LogLevel
from nautilus_trader.core.nautilus_pyo3.model import TraderId
from nautilus_trader.core.nautilus_pyo3.core import UUID4


class BigBrainActor:
    def __init__(self):
        self.pos_val = 0
        self.neg_val = 0
        self.guard = init_logging(TraderId("Test-trader"), UUID4(), level_stdout=LogLevel.DEBUG)

    def register_handlers(self):
        register("negative_stream", self.negative_handler)

    def negative_handler(self, val):
        self.neg_val = val
        print(f"Received negative value: {self.neg_val}")

        correlation_id = UUID4()
        register_response_handler(correlation_id, self.positive_handler)

        data_type = None
        if self.neg_val == -3:
            data_type = DataType("skip", None)
        else:
            data_type = DataType("get", None)

        request = RequestCustomData(
            client_id="mock_data_client",
            data_type=data_type,
            request_id=correlation_id,
            ts_init=0,
            params=None,
        )

        send_request("data_engine", request)

    def positive_handler(self, val):
        self.pos_val = val
        print(f"Received positive value: {self.pos_val}")

        correlation_id = UUID4()
        register_response_handler(correlation_id, self.negative_handler)

        data_type = None
        if self.pos_val == 3:
            data_type = DataType("skip", None)
        else:
            data_type = DataType("get", None)

        request = RequestCustomData(
            client_id="mock_data_client",
            data_type=data_type,
            request_id=correlation_id,
            ts_init=0,
            params=None,
        )

        send_request("data_engine", request)


if __name__ == "__main__":
    big_brain_actor = BigBrainActor()
    big_brain_actor.register_handlers()
    big_brain_actor.negative_handler(-2)
    big_brain_actor.positive_handler(2)
    print("hello world")
