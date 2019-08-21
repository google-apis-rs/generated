GENERATOR_DIR = generator
MCP ?= $(GENERATOR_DIR)/target/release/mcp
API_INDEX_JSON = etc/api-index.v1.json
API_INDEX_MAPPED_JSON = etc/api-index-mapped.v1.json
OUTPUT_DIR = gen
SPEC_DIR = etc/api  # keep in sync with Standard::config_dir
MAKEFILE_TPL = templates/Makefile.liquid
CARGO_TOML_TPL = templates/Cargo.toml.liquid
GEN_MAKEFILE = $(OUTPUT_DIR)/Makefile
GEN_CARGO_TOML = $(OUTPUT_DIR)/Cargo.toml
ERRORS_FILE_GLOB = *-errors.log

help:
	$(info -- Targets for files we depend on ----------------------------------------------------)
	$(info update-all-metadata        | invalidate all specifications from google and fetch the latest versions)
	$(info update-drivers             | update mapped index, generated Makefile and Cargo workspace file)
	$(info fetch-api-specs            | fetch all apis our local discovery document knows, and store them)
	$(info -- Developer Targets ---------------------------------------------------------------)
	$(info update-mcp                 | pull latest code and build the mcp program)
	$(info force-update-all-metadata  | like update-all-metadata, but will forget all local state beforehand)
	$(info show-all-errors            | display all error files currently present)
	$(info clear-all-errors           | remove all error records. Then run )
	$(info -- Precision Tools  ------------------------------------------------------------------)
	$(info force-update-mapped-index  | invalidate the mapped index and regenerate it, useful if there are new errors when generating or building)
	$(info generate-makefile          | a makefile containing useful targets to build and test generated crates)
	$(info --------------------------------------------------------------------------------------)
	$(info -- `make -C gen` is used to interact with the generator and generate code)
	$(info --------------------------------------------------------------------------------------)

$(MCP): $(GENERATOR_DIR)
	cd $(GENERATOR_DIR) && cargo build --release

$(API_INDEX_JSON):
	curl -S https://www.googleapis.com/discovery/v1/apis > $@

$(API_INDEX_MAPPED_JSON): $(API_INDEX_JSON) $(MCP) 
	$(MCP) map-api-index $< $@ $(SPEC_DIR) $(OUTPUT_DIR)

$(GENERATOR_DIR):
	git clone --depth=1 https://github.com/google-apis-rs/generator $@

update-mcp: $(GENERATOR_DIR)
	cd $(GENERATOR_DIR) && git pull --ff-only && cargo build --release

update-all-metadata:
	@echo Removing original Google API index
	-rm $(API_INDEX_JSON)
	$(MAKE) fetch-api-specs update-drivers

update-drivers: 
	$(MAKE) force-update-mapped-index generate-makefile

force-update-all-metadata: clear-all-errors
	MCP_FETCH_ARGS=--use-original-index-at=$(API_INDEX_JSON) $(MAKE) update-all-metadata

force-update-mapped-index:
	-rm $(API_INDEX_MAPPED_JSON)
	$(MAKE) $(API_INDEX_MAPPED_JSON)

fetch-api-specs: $(API_INDEX_MAPPED_JSON) $(MCP) 
	$(MCP) fetch-api-specs $(MCP_FETCH_ARGS) $(API_INDEX_MAPPED_JSON) $(SPEC_DIR)

$(GEN_MAKEFILE): $(API_INDEX_MAPPED_JSON) $(MCP) $(MAKEFILE_TPL) 
	$(MCP) substitute $(MAKEFILE_TPL):$@ $(CARGO_TOML_TPL):$(GEN_CARGO_TOML) < $<
	
generate-makefile: $(GEN_MAKEFILE)

clear-all-errors:
	@echo Clearing all errors...
	find $(OUTPUT_DIR) -name '$(ERRORS_FILE_GLOB)' -exec rm -v '{}' \;

show-all-errors:
	find $(OUTPUT_DIR) -name '$(ERRORS_FILE_GLOB)' | while read -r fp; do echo $$"\n---> $$fp <---\n"; cat "$$fp"; done

$(GEN_CARGO_TOML): $(API_INDEX_MAPPED_JSON) $(MCP) $(CARGO_TOML_TPL)
	$(MCP) substitute $(CARGO_TOML_TPL):$@ < $< 
