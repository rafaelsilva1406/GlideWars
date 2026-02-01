# Multi-stage build for Glide Wars
# Stage 1: Build the WASM binary
FROM rust:1.75 as builder

WORKDIR /app

# Install wasm target and dependencies
RUN rustup target add wasm32-unknown-unknown && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets

# Build for WebAssembly
RUN cargo build --release --target wasm32-unknown-unknown

# Stage 2: Runtime with nginx
FROM nginx:alpine

# Copy built WASM files and web assets
COPY --from=builder /app/target/wasm32-unknown-unknown/release/glidewars.wasm /usr/share/nginx/html/
COPY index.html /usr/share/nginx/html/
COPY nginx.conf /etc/nginx/nginx.conf
COPY assets /usr/share/nginx/html/assets

# Expose port 80
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --quiet --tries=1 --spider http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]
