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

help:
    #!/bin/sh
    cat <<EOF
    -- Targets for files we depend on ----------------------------------------------------)
    update-drivers             | update mapped index, generated Makefile and Cargo workspace file)
    update-all-metadata        | invalidate all specifications from google and fetch the latest versions)
    fetch-api-specs            | fetch all apis our local discovery document knows, and store them)
    -- Developer Targets ---------------------------------------------------------------)
    update-mcp                 | pull latest code and build the mcp program)
    force-update-all-metadata  | like update-all-metadata, but will forget all local state beforehand)
    show-all-errors            | display all error files currently present)
    clear-all-errors           | remove all error records)
    -- Precision Tools  ------------------------------------------------------------------)
    force-update-mapped-index  | invalidate the mapped index and regenerate it, useful if there are new errors when generating or building)
    generate-makefile          | a makefile containing useful targets to build and test generated crates)
    EOF

mcp:
    #!/usr/bin/env bash -eux -o pipefail
    if [ ! -d {{generator_dir}} ]; then
        git clone --depth=1 https://github.com/google-apis-rs/generator {{generator_dir}}
    else
        (cd {{generator_dir}} && git pull --ff-only)
    fi
    cd {{generator_dir}} && cargo build --release

api-index:
    curl -S https://www.googleapis.com/discovery/v1/apis > {{API_INDEX_JSON}}

mapped-api-index: mcp api-index
    {{MCP}} map-api-index {{API_INDEX_JSON}} {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}} {{OUTPUT_DIR}}

update-all-metadata: fetch-api-specs update-drivers

update-drivers: mapped-api-index generate-makefile

force-update-all-metadata:
    just clear-errors generator
    just clear-errors cargo
    {{MCP}} fetch-api-specs --use-original-index-at={{API_INDEX_JSON}} {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}}

fetch-api-specs: mapped-api-index mcp
    {{MCP}} fetch-api-specs {{API_INDEX_MAPPED_JSON}} {{SPEC_DIR}}

generate-makefile: mapped-api-index mcp
    {{MCP}} substitute {{MAKEFILE_TPL}}:{{GEN_MAKEFILE}} {{CARGO_TOML_TPL}}:{{GEN_CARGO_TOML}} < {{API_INDEX_MAPPED_JSON}}

# valid prefixes: generator or cargo or '*'
clear-errors prefix: 
    @echo Clearing all errors...
    find {{OUTPUT_DIR}} -name '{{prefix}}{{ERRORS_FILE_SUFFIX}}' -exec rm -v '{}' \;
    just update-drivers


# valid prefixes: generator or cargo or '*'
show-errors prefix:
    #!/usr/bin/env bash -eu
    find {{OUTPUT_DIR}} -name '{{prefix}}{{ERRORS_FILE_SUFFIX}}' | while read -r fp; do 
        echo $$"\n---> $fp <---\n"; cat "$fp"
    done
