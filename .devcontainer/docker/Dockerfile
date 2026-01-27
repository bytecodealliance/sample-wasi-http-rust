ARG TAG="latest"
FROM mcr.microsoft.com/devcontainers/rust:${TAG}

USER vscode

# Install wasmtime
RUN curl https://wasmtime.dev/install.sh -sSf | bash

# Install cargo-component and wkg
RUN cargo install wkg
