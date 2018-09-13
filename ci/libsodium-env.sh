#!/usr/bin/env bash

export PKG_CONFIG_PATH=$HOME/lib/libsodium/lib/pkgconfig:$PKG_CONFIG_PATH
export LD_LIBRARY_PATH=$HOME/lib/libsodium/lib:$LD_LIBRARY_PATH

export SODIUM_STATIC=true
export SODIUM_LIB_DIR=$HOME/lib/libsodium/src/libsodium/.libs
export SODIUM_INC_DIR=$HOME/lib/libsodium/src/libsodium/include