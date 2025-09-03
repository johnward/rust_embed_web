# ----------------------------
# Stage 1: Builder (Rust + Node)
# ----------------------------
FROM rust:1.89.0-trixie AS builder

# Install Node.js (for building the Vite frontend) and build deps
RUN apt-get update \
    && apt-get install -y curl ca-certificates build-essential pkg-config libssl-dev \
    && update-ca-certificates \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y vim

# App dir
WORKDIR /app

# (Optional) Set the binary name via build-arg if your package name != folder name
ARG BIN_NAME=rust_embed_web

# Pre-copy manifests for better Docker layer caching (Rust + npm)
# Rust manifests
COPY Cargo.toml Cargo.lock ./
# Create a dummy src to allow `cargo fetch` without compiling your real code
RUN mkdir -p src

# Now copy the real sources
COPY src/main.rs ./src/main.rs

# Cache Rust dependencies
RUN cargo fetch

RUN mkdir -p frontend

ADD frontend/ ./frontend

WORKDIR /app/frontend

# Install frontend deps (cached until package.json/package-lock.json change)
#RUN npm ci --prefix frontend

RUN npm i

# Build the frontend (outputs to frontend/dist, which rust-embed will embed)
RUN npm run build 

WORKDIR /app

# Build the Rust binary (release)
# If you rely on build.rs to build the frontend, it’s fine — we already built it explicitly.
RUN cargo build --release

# ----------------------------
# Stage 2: Runtime (slim)
# ----------------------------
FROM debian:bookworm-slim AS runtime

# Add certs for HTTPS requests if your server or clients need it
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Non-root user for better security
RUN useradd -u 10001 -m appuser

WORKDIR /app

# Bring over the compiled binary
ARG BIN_NAME=rust_embed_web

COPY --from=builder /app/target/release/${BIN_NAME} /usr/local/bin/app

# Expose the port your Axum server listens on
EXPOSE 8080

# Drop privileges
USER appuser

# Run the app
ENTRYPOINT ["/usr/local/bin/app"]