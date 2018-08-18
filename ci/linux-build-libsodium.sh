#!/usr/bin/env bash

mkdir -p $HOME/lib
wget https://github.com/jedisct1/libsodium/releases/download/1.0.15/libsodium-1.0.15.tar.gz
tar xvfz libsodium-1.0.15.tar.gz
cd libsodium-1.0.15 && ./configure --prefix=$HOME/lib/libsodium && make && make install && cd ..
