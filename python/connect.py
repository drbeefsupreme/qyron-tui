#!/usr/bin/env python3

#RPC library
from simple_rpc import Interface
#web app library
#from flask import Flask, request
import time, os
#from config import Config

VERBOSE = True
SERIAL_PORT = '/dev/ttyACM0'
READ_TIMEOUT = 5

#app = Flask(__name__)
#app.config.from_object(Config)
interface = Interface(SERIAL_PORT)
#import processing


#if __name__ == '__main__':
def run():
    options = {
        'serial_port': SERIAL_PORT,
        'read_timeout': READ_TIMEOUT,
        'verbose': VERBOSE
    }


    if interface is None:
        raise Exception('[-] connect_to_serial_port failed')

    print("connection successful")


    time.sleep(1)

    interface.scrollingLayer1_start(b'testtttt', -1)

    #app.run(host='0.0.0.0', port=80, debug=True)
