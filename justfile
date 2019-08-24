generator_dir := "generator"
MCP := generator_dir + "/target/release/mcp"
API_INDEX_JSON := "etc/api-index.v1.json"
API_INDEX_MAPPED_JSON := "etc/api-index-mapped.v1.json"
OUTPUT_DIR := "gen"
SPEC_DIR := "etc/api"  # keep in sync with Standard::config_dir: TODO: can we write an .env file instead?
MAKEFILE_TPL := ""
CARGO_TOML_TPL := ""
GEN_MAKEFILE := OUTPUT_DIR + "/Makefile"
GEN_CARGO_TOML := OUTPUT_DIR + "/Cargo.toml"
ERRORS_FILE_SUFFIX := "-errors.log"
SKIP_MCP := "no"

# display an overview
help:
    #!/bin/sh
    cat <<EOF
    Run 'just --list' for more details, here is an overview
    -- Used most often... ---------------------------------------------------------------------------)
    'refresh-pruned-specs' and 'refresh-all'
    'gen/all' and 'gen/cargo check'
    -- The source of it all -- first mentioned serve as input for these mentioned later -------------)
    'refresh-google-api-index'  and 'fetch-api-specs-pruned' and 'fetch-api-specs-google'
    -- Drive the generator and update its inputs ----------------------------------------------------)
    'update-drivers'
    -- Operations on the gen/ directory  ------------------------------------------------------------)
    'gen-all' and 'gen-check' and 'gen-doc' and 'gen-cargo <+arguments>' and 'gen-make <target> <+arguments>'
    -- Developer Targets ----------------------------------------------------------------------------)
    'mcp' and 'show-errors' and 'clear-errors' and 
    EOF

# Fetch the latest API index from Googles discovery service
refresh-api-index:
    curl -S https://www.googleapis.com/discovery/v1/apis > {{API_INDEX_JSON}}

# build or update the MCP tool, used for generation and much more
mcp:
    #!/usr/bin/env bash 
    set -eux -o pipefail
    [[ "{{SKIP_MCP}}" = "yes" ]] && exit 0
    if [ ! -d {{generator_dir}} ]; then
        git clone --depth=1 https://github.com/google-apis-rs/generator {{generator_dir}}
    else
        (cd {{generator_dir}} && git pull --ff-only)
    fi
    cd {{generator_dir}} && cargo build --release

# Using the API index and known errors, generate a pruned API index containing only working APIs
_map-api-index: mcp
    {{MCP}} map-api-index {{API_INDEX_JSON}} {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}} {{OUTPUT_DIR}}

# update files we can use to run the tooling
update-drivers: _map-api-index mcp
    cat {{API_INDEX_MAPPED_JSON}} | {{MCP}} substitute \
        templates/Makefile.liquid:{{GEN_MAKEFILE}} \
        templates/Cargo.toml.liquid:{{GEN_CARGO_TOML}}

# fetch API specifications from Google's discovery service, based on the list of ones we know work
fetch-api-specs-pruned: _map-api-index mcp
    {{MCP}} fetch-api-specs {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}} {{OUTPUT_DIR}}

# fetch API specifications from Google's discovery service, based the entire index of available APIs, and regenerate code
fetch-api-specs-google: refresh-api-index
    {{MCP}} fetch-api-specs {{API_INDEX_JSON}} {{SPEC_DIR}} {{OUTPUT_DIR}}

# fetch latest API specifications known to be working, just make it work!
refresh-pruned-specs: fetch-api-specs-pruned update-drivers

# clear errors, fetch latest index from google, and fetch all specs
refresh-with-force:
    just clear-errors generator
    just clear-errors cargo
    just fetch-api-specs-google
    just update-drivers

# clear errors, fetch latest index from google, and fetch all specs, run cargo check and doc
refresh-all: refresh-with-force collect-errors

any_error := "*"
# valid prefixes: generator or cargo
clear-errors prefix=any_error: 
    @echo Clearing all errors...
    find {{OUTPUT_DIR}} -name '{{prefix}}{{ERRORS_FILE_SUFFIX}}' -exec rm -v '{}' \;
    just update-drivers


# valid prefixes: generator or cargo or '*'
show-errors prefix=any_error:
    #!/usr/bin/env sh
    set -eu
    find {{OUTPUT_DIR}} -name '{{prefix}}{{ERRORS_FILE_SUFFIX}}' | while read -r fp; do 
        echo $"\n---> $fp <---\n"
        cat "$fp"
    done

# Best after 'refresh-all', it generates all code and runs cargo against it, collecting errors
collect-errors:
    just mcp 
    just gen-cargo-errors check
    just update-drivers
    just gen-cargo-errors doc
    just update-drivers

# generate as many APIs as possible, in parallel, and one by one (use it if generator changed)
gen-all:
    make -C {{OUTPUT_DIR}} gen-all -kj8

check := "check"
# Run cargo on the workspace with all projects
gen-cargo +arguments=check:
    cd {{OUTPUT_DIR}} && cargo {{arguments}}

# Run MCP to control Cargo to efficiently run it and store errors accordingly
gen-cargo-errors +arguments=check:
    {{MCP}} cargo-errors {{API_INDEX_MAPPED_JSON}} {{GEN_CARGO_TOML}} {{OUTPUT_DIR}} {{arguments}}

# Run make on the given target
gen-make target +arguments=check:
    make -C {{OUTPUT_DIR}} {{target}}-cargo ARGS={{arguments}}

# Run cargo via Make, one by one, and collect errors when checking
gen-check:
    make -C {{OUTPUT_DIR}} cargo-all ARGS=check  -k

# Run cargo via Make, one by one, and collect errors when making docs
gen-doc:
    make -C {{OUTPUT_DIR}} cargo-all ARGS=doc  -k