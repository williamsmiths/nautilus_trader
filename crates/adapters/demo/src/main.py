import sys
import signal
import os

def signal_handler(sig, frame):
    print('Received Ctrl+C, exiting...')
    os._exit(1)

signal.signal(signal.SIGINT, signal_handler)

sys.path.append("/home/twitu/Code/nautilus_trader/")

from nautilus_trader.core.nautilus_pyo3 import main

main("/home/twitu/Code/nautilus_trader/crates/adapters/demo/src/big_brain_actor.py")
