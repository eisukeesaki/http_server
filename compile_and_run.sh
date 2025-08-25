#!/usr/bin/env bash

echo "rustc src/main.rs --cfg 'feature=\"debug\"' -o bin/main"
rustc src/main.rs --cfg 'feature="debug"' -o server

# echo 'rustc tcp_server.rs -o server'
# rustc tcp_server.rs -o server

echo "./server 127.0.0.1 4242"
./server 127.0.0.1 4242

