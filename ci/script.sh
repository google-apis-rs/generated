#!/bin/sh

set -eu

make update-all-metadata generate-makefile
make -C gen gen-all -k || {
    res=$?
    make show-all-errors
    exit $res
}
