# Sample: `wasi:http` in Rust

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/yoshuawuyts/rust-wasi-hello)

An example project showing how to build a spec-compliant
[`wasi:http/proxy`][wasi-http] server for WASI 0.2 written in Rust. This sample
includes several [routes](#routes) that showcase different behavior. This sample
can be run by any spec-compliant `wasi:http/proxy` server.

Each release of this sample is packaged up as a [Wasm OCI image][wasm-oci-image]
and published to the [GitHub Packages Registry][gh-pkg]. See the ["Deploying
published artifacts"][using-arifacts] section for more on how to fetch and run
the published artifacts.

## Routes

The following HTTP routes are available from the component:

```text
/               # Hello world
/wait           # Sleep for one second
/echo           # Echo the HTTP body
/echo-headers   # Echo the HTTP headers
/echo-trailers  # Echo the HTTP trailers
```

## Installation

The easiest way to try this project is by opening it in a GitHub Codespace. This
will create a VS Code instance with all dependencies installed. If instead you
would prefer to run this locally, you can run the following commands:

```bash
$ curl https://wasmtime.dev/install.sh -sSf | bash # install wasm runtime
$ cargo install cargo-component                    # install build tooling
$ cargo install wkg                                # install wasm OCI tooling
```

## Local Development

The HTTP server uses the `wasi:http/proxy` world. You can run it locally in a
`wasmtime` instance by using the following [cargo-component] command:

```rust
$ cargo component serve
```
### Note on Debugging
There are launch and task configuration files if you want to use VSCode for debugging in an IDE; however, if you prefer using GDB or LLDB directly the configuration files should be enough to get you up and running. Note that the [GDB configuration requires an absolute path](https://github.com/bytecodealliance/sample-wasi-http-rust/blob/fe47fc9f6c87d09575f6683a26f9a67e3e71aa26/.vscode/launch.json#L28), so that configuration in VSCode you will need to modify for your computer.

## Deploying published artifacts

This project automatically published compiled Wasm Components as OCI to GitHub
Artifacts. You can pull the artifact with any OCI-compliant tooling and run it
in any Wasm runtime that supports the `wasi:http/proxy` world. To fetch the
latest published version from GitHub releases using [wkg][wkg] and run it in a
local [`wasmtime` instance][wasmtime] you can run the following commands:

```bash
$ wkg oci pull ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:latest
$ wasmtime serve sample-wasi-http-rust.wasm
```

For production workloads however, you may want to use other runtimes or
integrations which provide their own OCI integrations. Deployment will vary
depending on you providers, though at their core they will tend to be variations
on the pull + serve pattern we've shown here.

## See Also

**Hosts**
- [sample-wasi-http-aks-wasmcloud](https://github.com/yoshuawuyts/sample-wasi-http-aks-wasmcloud) - A `wasi:http` example host environment running on AKS using the WasmCloud runtime

## License

Apache-2.0 with LLVM Exception

[cargo-component]: https://github.com/bytecodealliance/cargo-component
[wasm-oci-image]: https://tag-runtime.cncf.io/wgs/wasm/deliverables/wasm-oci-artifact/
[gh-pkg]: https://github.com/bytecodealliance/sample-wasi-http-rust/pkgs/container/sample-wasi-http-rust%2Fsample-wasi-http-rust
[using-arifacts]: #working-with-deployment-artifacts
[wasi-http]: https://github.com/WebAssembly/wasi-http
[wkg]: https://github.com/bytecodealliance/wasm-pkg-tools/tree/main/crates/wkg
[wasmtime]: https://wasmtime.dev
