#!/bin/bash
avrdude -patmega2560 -cwiring -P$1 -b115200 -D -Uflash:w:target/avr-atmega2560/release/rust-avr-test.elf:e

