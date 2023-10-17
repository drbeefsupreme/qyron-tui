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

def caw():
    interface.scrollingLayer1_start(b'CAWCAWCAW', -1)

def text1(text):
    interface.scrollingLayer1_start(bytes(text), -1)
def text2(text):
    interface.scrollingLayer2_start(bytes(text), -1)
def text3(text):
    interface.scrollingLayer3_start(bytes(text), -1)
def text4(text):
    interface.scrollingLayer4_start(bytes(text), -1)
def text5(text):
    interface.scrollingLayer5_start(bytes(text), -1)

def dopamine():
    interface.scrollingLayer2_start(b'DOPAMINE DOPAMINE DOPAMINE', -1)

def clear():
    interface.scrollingLayer1_start(b'', -1)
    interface.scrollingLayer2_start(b'', -1)
    interface.scrollingLayer3_start(b'', -1)
    interface.scrollingLayer4_start(b'', -1)
    interface.scrollingLayer5_start(b'', -1)
    interface.noGif()
    interface.clearLoops()
    interface.disableGifsLoop()

def enableGifsLoop():
    interface.enableGifsLoop()

def disableGifsLoop():
    interface.disableGifsLoop()

def gifK():
    interface.gifK()
def gifO():
    interface.gifO()
def gifF():
    interface.gifF()
def gifD():
    interface.gifD()
def gifJ():
    interface.gifJ()
def gifS():
    interface.gifS()
def gifB():
    interface.gifB()
def gifT():
    interface.gifT()
def gifA():
    interface.gifA()

def pixels():
    interface.drawRandomPixels()

def shapes():
    interface.drawRandomShapes()

def nextGif():
    interface.nextGif()

def noGif():
    interface.noGif()

def hitShapes():
    interface.hitShapes()

def hitPixels():
    interface.hitShapes()
