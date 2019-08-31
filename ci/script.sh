#!/bin/sh

set -eux

curl -LSfs https://japaric.github.io/trust/install.sh | \
  sh -s -- --git casey/just --target x86_64-unknown-linux-musl --force

just refresh-pruned-specs

# gen + cargo check + cargo doc
(
  cd gen
  make gen-all cargo-all ARGS=check -k
  make cargo-all ARGS=doc -k
) || {
    res=$?
    just show-errors
    exit $res
}

(
  cd urlshortener1-cli && cargo check
)

