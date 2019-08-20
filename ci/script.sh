#!/bin/sh

set -eu

make update-all-metadata generate-makefile
make -C gen gen-all cargo-all ARGS=check -k || {
    res=$?
    make show-all-errors
    exit $res
}
