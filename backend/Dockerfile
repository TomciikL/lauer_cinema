FROM rust:latest

WORKDIR /app

# Instalace nástrojů pro vývoj
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev curl

# Instalace cargo-watch (live reload)
RUN cargo install cargo-watch loco-cli

# Překopíruj Cargo.toml a src pro optimalizaci build layerů
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

# Pre-build (volitelné)
RUN cargo build

CMD ["cargo", "watch", "-x", "loco start"]
