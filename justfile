generator_dir := "generator"
MCP := generator_dir + "/target/release/mcp"
API_INDEX_JSON := "etc/api-index.v1.json"
API_INDEX_MAPPED_JSON := "etc/api-index-mapped.v1.json"
OUTPUT_DIR := "gen"
SPEC_DIR := "etc/api"  # keep in sync with Standard::config_dir: TODO: can we write an .env file instead?
MAKEFILE_TPL := "templates/Makefile.liquid"
CARGO_TOML_TPL := "templates/Cargo.toml.liquid"
GEN_MAKEFILE := OUTPUT_DIR + "/Makefile"
GEN_CARGO_TOML := OUTPUT_DIR + "/Cargo.toml"
ERRORS_FILE_SUFFIX := "-errors.log"

# display an overview
help:
    #!/bin/sh
    cat <<EOF
    Run 'just --list' for mor details, here is an overview
    -- Used most often... ---------------------------------------------------------------------------)
    'refresh-pruned-specs' and 'refresh-with-force'
    -- The source of it all -- first mentioned serve as input for these mentioned later -------------)
    'refresh-google-api-index'  and 'fetch-api-specs-pruned' and 'fetch-api-specs-google'
    -- Drive the generator and update its inputs ----------------------------------------------------)
    'update-drivers'
    -- Developer Targets ----------------------------------------------------------------------------)
    'mcp' and 'show-errors' and 'clear-errors' and 
    EOF

# Fetch the latest API index from Googles discovery service
refresh-api-index:
    curl -S https://www.googleapis.com/discovery/v1/apis > {{API_INDEX_JSON}}

# build or update the MCP tool, used for generation and much more
mcp:
    #!/usr/bin/env bash -eux -o pipefail
    if [ ! -d {{generator_dir}} ]; then
        git clone --depth=1 https://github.com/google-apis-rs/generator {{generator_dir}}
    else
        (cd {{generator_dir}} && git pull --ff-only)
    fi
    cd {{generator_dir}} && cargo build --release

# Using the API index and known errors, generate a pruned API index containing only working APIs
map-api-index: mcp
    {{MCP}} map-api-index {{API_INDEX_JSON}} {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}} {{OUTPUT_DIR}}

# update files we can use to run the tooling
update-drivers: map-api-index mcp
    {{MCP}} substitute {{MAKEFILE_TPL}}:{{GEN_MAKEFILE}} {{CARGO_TOML_TPL}}:{{GEN_CARGO_TOML}} < {{API_INDEX_MAPPED_JSON}}

# fetch API specifications from Google's discovery service, based on the list of ones we know work
fetch-api-specs-pruned: map-api-index mcp
    {{MCP}} fetch-api-specs {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}}

# fetch API specifications from Google's discovery service, based the entire index of available APIs
fetch-api-specs-google: refresh-api-index
    {{MCP}} fetch-api-specs {{API_INDEX_JSON}} {{SPEC_DIR}}

# fetch latest API specifications known to be working, just make it work!
refresh-pruned-specs: fetch-api-specs-pruned update-drivers

# clear errors, fetch latest index from google, and fetch all specs
refresh-with-force:
    just clear-errors generator
    just clear-errors cargo
    just fetch-api-specs-google
    just update-drivers


any_error := "*"
# valid prefixes: generator or cargo
clear-errors prefix=any_error: 
    @echo Clearing all errors...
    find {{OUTPUT_DIR}} -name '{{prefix}}{{ERRORS_FILE_SUFFIX}}' -exec rm -v '{}' \;
    just update-drivers


# valid prefixes: generator or cargo or '*'
show-errors prefix=any_error:
    #!/usr/bin/env bash -eu
    find {{OUTPUT_DIR}} -name '{{prefix}}{{ERRORS_FILE_SUFFIX}}' | while read -r fp; do 
        echo $$"\n---> $fp <---\n"; cat "$fp"
    done
