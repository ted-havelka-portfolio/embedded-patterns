#!/bin/bash
# RUSTFLAGS="-C link-arg=-Tlink.x" cargo embed --example $1 --chip nrf52833_xxAA --target thumbv7em-none-eabihf
RUSTFLAGS="-C link-arg=-Tlink.x" cargo flash --chip nrf52833_xxAA --target thumbv7em-none-eabihf
exit $?
