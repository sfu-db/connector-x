FROM mcr.microsoft.com/devcontainers/rust:1

# Install system dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    sqlite3 \
    pkg-config \
    libclang-dev \
    postgresql-client \
    default-mysql-client \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install uv
COPY --from=ghcr.io/astral-sh/uv:0.11.28 /uv /uvx /usr/local/bin/

# Install Rust components
RUN rustup component add rustfmt clippy \
    && cargo install cargo-watch \
    && cargo install just

# Set environment variables
ENV PATH="/home/vscode/.local/bin:${PATH}"
ENV PYTHONPATH="/workspaces/connector-x:${PYTHONPATH}"
