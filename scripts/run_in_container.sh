#!/bin/sh
#
#



BIN_PATH=$(dirname $(readlink -f $0))
SOURCE_PATH=$BIN_PATH/../


cd $SOURCE_PATH

cargo run --package telegram-client --example tclient

