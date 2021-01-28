import socket
import json
import sys

if len(sys.argv) < 4:
    print(f"Usage: {sys.argv[0]} <msg type> <x> <y>")
    sys.exit(1)

path = "/tmp/test.sock"
sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

sock.connect(path)

payload = {
    "x": int(sys.argv[2]), 
    "y": int(sys.argv[3]),
}

payload_encoded = json.dumps(payload, separators=(',', ':'))

header = {
    "len": len(payload_encoded),
    "msg_type": sys.argv[1],
}

header_encoded = json.dumps(header, separators=(',', ':'))

header_len = len(header_encoded)

print(f"Header length: {header_len}")
print(f"Header length encoded: {bytes([header_len])}")
print(f"Header: {header_encoded}")
print(f"Header encoded: {bytes(header_encoded, 'utf-8')}")
print(f"Payload: {payload_encoded}")
print(f"Payload encoded: {bytes(payload_encoded, 'utf-8')}")

sock.send(bytes([header_len]))
sock.send(bytes(header_encoded, 'utf-8'))
sock.send(bytes(payload_encoded, 'utf-8'))

response_data = sock.recv(1024)

response_data = json.loads(response_data.decode('utf-8').strip())

for key in response_data:
    print(key, response_data[key])

sock.close(  )