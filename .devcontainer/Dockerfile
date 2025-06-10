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

# Install Python and Poetry
ENV POETRY_HOME=/opt/poetry
ENV POETRY_VERSION=2.1.3
ENV PATH="/opt/poetry/bin:$PATH"
RUN curl -sSL https://install.python-poetry.org | python3 - \
    && poetry config virtualenvs.create false

# Install Rust components
RUN rustup component add rustfmt clippy \
    && cargo install cargo-watch \
    && cargo install just

# Set environment variables
ENV PATH="/home/vscode/.local/bin:${PATH}"
ENV PYTHONPATH="/workspaces/connector-x:${PYTHONPATH}"
