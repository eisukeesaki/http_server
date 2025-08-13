#!/usr/bin/env bash
rustc tcp_server.rs -o server
./server 127.0.0.1 4242

