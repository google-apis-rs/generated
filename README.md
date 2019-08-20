[![Build Status](https://travis-ci.org/google-apis-rs/generated.svg?branch=master)](https://travis-ci.org/google-apis-rs/generated)

A repository to contain all files generated by tools with the [APIs repository][gen-repo].

[gen-repo](https://github.com/google-apis-rs/apis)

## Development

To easily alter MCP code without having to commit, run `make` with an overridden `MCP` environment variable, pointing
to one that you just build. Typical invocations would look like this:

```
(cd ../generator && cargo build) && MCP=../generator/target/debug/mcp make -f Makefile.generated
```

