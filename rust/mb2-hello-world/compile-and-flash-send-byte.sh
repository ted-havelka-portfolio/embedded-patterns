#!/bin/bash
RUSTFLAGS="-C link-arg=-Tlink.x" cargo embed --example send-byte --chip nrf52833_xxAA --target thumbv7em-none-eabihf
exit $?
