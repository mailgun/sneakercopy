#!/usr/bin/env bash

LIBSODIUM_VERSION=${LIBSODIUM_VERSION:-1.0.16}

mkdir -p $HOME/lib/libsodium
curl -sSL -olibsodium.tar.gz https://github.com/jedisct1/libsodium/releases/download/${LIBSODIUM_VERSION}/libsodium-${LIBSODIUM_VERSION}.tar.gz
tar xvfz libsodium.tar.gz --strip-components 1 -C $HOME/lib/libsodium
pushd $HOME/lib/libsodium && \
    ./configure \
        --prefix=$HOME/lib/libsodium \
        --disable-debug \
        --disable-dependency-tracking \
        --disable-shared && \
    make && \
    make install && \
    popd
