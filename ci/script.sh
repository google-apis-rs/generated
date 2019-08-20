#!/bin/sh

set -eu

make update-all-metadata generate-makefile

# gen + cargo check + cargo doc
cd gen
{
  make gen-all cargo-all ARGS=check -k
  make cargo-all ARGS=doc -k
} || {
    res=$?
    make show-all-errors
    exit $res
}

