#!/bin/sh
#
#
#


BIN_PATH=$(dirname $(readlink -f $0))

docker build -f $BIN_PATH/rust.image.Dockerfile -t lorust:1 $BIN_PATH
