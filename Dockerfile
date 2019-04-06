FROM rust:1.32.0
ADD . "/usr/src/acheron"
WORKDIR "/usr/src/acheron"
RUN ["rustup", "default", "stable"]
RUN ["cargo", "install", "--force", "cargo-make"]
RUN ["rustup", "component", "add", "rustfmt"]
RUN ["rustup", "component", "add", "clippy"]
RUN ["cargo", "make"]
