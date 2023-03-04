#!/usr/bin/env python3

#RPC library
from simple_rpc import Interface
import time, os

VERBOSE = True
SERIAL_PORT = '/dev/ttyACM0'
READ_TIMEOUT = 5

interface = Interface(SERIAL_PORT)

def connect():
    options = {
        'serial_port': SERIAL_PORT,
        'read_timeout': READ_TIMEOUT,
        'verbose': VERBOSE
    }

    if interface is None:
        raise Exception('[-] connect_to_serial_port failed')

    print("connection successful")

    time.sleep(1)

    test()

def caw():
    interface.scrollingLayer1_start(b'CAWCAWCAW', -1)

def dopamine():
    interface.scrollingLayer2_start(b'DOPAMINE DOPAMINE DOPAMINE', -1)

def clear():
    interface.scrollingLayer1_start(b'', -1)
    interface.scrollingLayer2_start(b'', -1)

def pixels():
    interface.drawRandomPixels()

def shapes():
    interface.drawRandomShapes()
