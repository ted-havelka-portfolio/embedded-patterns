#!/bin/bash

cargo flash --release --chip STM32F401RE --log=INFO --connect-under-reset

exit $?
