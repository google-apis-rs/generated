GENERATOR_DIR = _generator
MCP ?= $(GENERATOR_DIR)/target/release/mcp
API_INDEX_JSON = etc/api-index.v1.json
API_INDEX_MAPPED_JSON = etc/api-index-mapped.v1.json
OUTPUT_DIR = gen
MAKEFILE_TPL = templates/Makefile.liquid
GEN_MAKEFILE = $(OUTPUT_DIR)/Makefile

help:
	$(info -- Targets for files we depend on ----------------------------------------------------)
	$(info update-all-metadata        | invalidate all specifications from google and fetch the latest versions)
	$(info update-mapped-index        | invalidate the mapped index and regenerate it)
	$(info fetch-api-specs            | fetch all apis our local discovery document knows, and store)
	$(info generate-makefile          | a makefile containing useful targets to build and test generated crates)
	$(info update-mcp                 | pull latest code and build the mcp program)
	$(info show-all-errors            | display all error files currently present)
	$(info --------------------------------------------------------------------------------------)
	$(info -- `make -C gen` is used to interact with the generator and generate code)
	$(info --------------------------------------------------------------------------------------)

$(MCP): $(GENERATOR_DIR)
	cd $(GENERATOR_DIR) && cargo build --release

$(API_INDEX_JSON):
	curl -S https://www.googleapis.com/discovery/v1/apis > $@

$(API_INDEX_MAPPED_JSON): $(API_INDEX_JSON) $(MCP) 
	$(MCP) map-api-index $< $@ $(OUTPUT_DIR)

$(GENERATOR_DIR):
	git clone --depth=1 https://github.com/google-apis-rs/generator $@

update-mcp: $(GENERATOR_DIR)
	cd $(GENERATOR_DIR) && git pull --ff-only && cargo build --release

update-all-metadata:
	-rm $(API_INDEX_JSON)
	$(MAKE) fetch-api-specs update-mapped-index

update-mapped-index:
	-rm $(API_INDEX_MAPPED_JSON)
	$(MAKE) $(API_INDEX_MAPPED_JSON)

api-index: $(API_INDEX_JSON) $(GEN_MAKEFILE)

fetch-api-specs: api-index $(MCP) 
	$(MCP) fetch-api-specs $(API_INDEX_MAPPED_JSON) $(OUTPUT_DIR)

$(GEN_MAKEFILE): $(API_INDEX_MAPPED_JSON) $(MCP) $(MAKEFILE_TPL) 
	$(MCP) substitute $(MAKEFILE_TPL):$@ < $< 
	
generate-makefile: $(GEN_MAKEFILE)

show-all-errors:
	find . -name generator-errors.log | while read -r fp; do echo $$"\n---> $$fp <---\n"; cat "$$fp"; done