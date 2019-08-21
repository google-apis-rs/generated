#!/bin/sh

set -eux

# TODO: wait for https://github.com/casey/just/pull/465 and a new release, then use 
# the following lines instead
# curl -LSfs https://japaric.github.io/trust/install.sh | \
#   sh -s -- --git casey/just --force
cargo install just

just refresh-pruned-specs

# gen + cargo check + cargo doc
cd gen
{
  make gen-all cargo-all ARGS=check -k
  make cargo-all ARGS=doc -k
} || {
    res=$?
    just show-errors
    exit $res
}

