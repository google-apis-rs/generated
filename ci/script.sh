#!/bin/sh

set -eu

make update-all-metadata generate-makefile
make -f Makefile.generated gen-all -k || {
    res=$?
    make show-all-errors
    exit $res
}
