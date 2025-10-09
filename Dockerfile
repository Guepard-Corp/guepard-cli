# ---- Stage 1: Build Environment ----
# Use an even newer Rust image to support the project's latest dependencies.
FROM rust:1.82-slim AS builder

# Install necessary dependencies for your project's crates (like keyring on Linux).
# build-essential provides tools like a C compiler, and libdbus-1-dev is for the secret-service feature.
RUN apt-get update && apt-get install -y build-essential libdbus-1-dev pkg-config

# Set the working directory within the container.
WORKDIR /usr/src/guepard-cli

# Copy the Cargo manifest files.
# This is done first to leverage Docker's layer caching.
# Dependencies will only be rebuilt if these files change.
COPY Cargo.toml Cargo.lock ./

# Create a dummy project to build only the dependencies.
# This caches the expensive dependency compilation step.
RUN mkdir src && \
    echo "fn main() {}" > src/cli.rs && \
    cargo build --release --locked && \
    rm -rf src

# Copy your actual source code.
COPY src ./src

# Build the application binary, leveraging the cached dependencies.
# Using --locked ensures the build uses the exact dependency versions from Cargo.lock.
RUN cargo build --release --locked

# ---- Stage 2: Final Production Image ----
# Use a minimal base image from Google's distroless project for a small and secure final image.
FROM gcr.io/distroless/cc-debian12

# Set the working directory.
WORKDIR /usr/local/bin/

# Copy the compiled binary from the builder stage to the final image.
COPY --from=builder /usr/src/guepard-cli/target/release/guepard .

# Set the entrypoint for the container. When the container runs, it will execute your application.
ENTRYPOINT ["./guepard"]
