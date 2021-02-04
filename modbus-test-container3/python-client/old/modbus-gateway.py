#!/usr/bin/env python

from pymodbus.server.asynchronous import StartTcpServer
from pymodbus.device import ModbusDeviceIdentification
from pymodbus.datastore import ModbusSequentialDataBlock
from pymodbus.datastore import ModbusSlaveContext, ModbusServerContext
from pymodbus.transaction import ModbusRtuFramer, ModbusAsciiFramer

from twisted.internet.task import LoopingCall

import random
import socket
import json

import logging
logging.basicConfig()
log = logging.getLogger()
log.setLevel(logging.DEBUG)

def updating_writer(a):
    """ A worker process that runs every so often and
    updates live values of the context. It should be noted
    that there is a race condition for the update.

    :param arguments: The input arguments to the call
    """
    log.debug("updating the context")
    context = a[0]
    register = 4
    slave_id = 0x00
    address = 0x01

    ## read some values
    read_values = context[slave_id].getValues(3, 0, count=2)
    log.debug(f"{read_values}")

    # Update the values some way or another, for example from another server
   
    # find out how to put this into another file
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(("127.0.0.1", 9977))

    payload = {
        "x": read_values[0], 
        "y": read_values[1],  
    }
    payload_encoded = json.dumps(payload, separators=(',', ':'))    
    header = {
        "len": len(payload_encoded),
        "msg_type": "get_info",
    }
    header_encoded = json.dumps(header, separators=(',', ':'))
    header_len = len(header_encoded)

    sock.send(bytes([header_len]))
    sock.send(bytes(header_encoded, 'utf-8'))
    sock.send(bytes(payload_encoded, 'utf-8'))

    response_data = sock.recv(1024)

    response_data = json.loads(response_data.decode('utf-8').strip())

    for key in response_data:
        log.debug(f"{key}: {response_data[key]}")

    sock.close()
    values = [1,2,3,4,5]
    #log.debug(values)
    values = [random.randint(1, 5) for v in values]
    values = [response_data["value"]]
    log.debug("new values: " + str(values))
    context[slave_id].setValues(register, address, values)


def run_updating_server():
    # ----------------------------------------------------------------------- # 
    # initialize your data store
    # ----------------------------------------------------------------------- # 
    
    store = ModbusSlaveContext(
        di=ModbusSequentialDataBlock(0, [16]*100),
        co=ModbusSequentialDataBlock(0, [17]*100),
        hr=ModbusSequentialDataBlock(0, [18]*100),
        ir=ModbusSequentialDataBlock(0, [19]*100))
    context = ModbusServerContext(slaves=store, single=True)
    # log.debug(context[0])
    
    # ----------------------------------------------------------------------- # 
    # initialize the server information
    # ----------------------------------------------------------------------- # 
    identity = ModbusDeviceIdentification()
    identity.VendorName = 'pymodbus'
    identity.ProductCode = 'PM'
    identity.VendorUrl = 'http://github.com/bashwork/pymodbus/'
    identity.ProductName = 'pymodbus Server'
    identity.ModelName = 'pymodbus Server'
    identity.MajorMinorRevision = '2.3.0'
    
    # ----------------------------------------------------------------------- # 
    # run the server you want
    # ----------------------------------------------------------------------- # 
    time = 1  # 1 seconds delay
    loop = LoopingCall(f=updating_writer, a=(context,))
    loop.start(time, now=False) # initially delay by time
    StartTcpServer(context, identity=identity, address=("localhost", 5020))


if __name__ == "__main__":
    run_updating_server()