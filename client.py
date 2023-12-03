#!/usr/bin/env python3
from socket import socket, AF_INET, SOCK_STREAM

HOST = "0.0.0.0"
PORT = 1024

PAYLOAD = """Here is some text to send to the server!\nNewlines are pretty fly!
we can also add them like this, what happens then?\n\n"""

PAYLOAD = "Simpler!\n"

with socket(AF_INET, SOCK_STREAM) as s:
    s.connect((HOST, PORT))

    s.sendall(PAYLOAD.encode('utf-8'))

    print(s.recv(1024))

