# Rust as the base image
FROM rust:1.57 as build

# RUN rustup toolchain install nightly

# Create a new empty shell project
RUN USER=root cargo new --bin khaos-requestor
WORKDIR /khaos-requestor

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# RUN rustup override set nightly

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/khaos-requestor*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /khaos-requestor/target/release/khaos-requestor /usr/src/khaos-requestor

EXPOSE 8000

# Run the binary
CMD ["/usr/src/khaos-requestor"]
