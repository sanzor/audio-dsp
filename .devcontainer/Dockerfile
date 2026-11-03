FROM node:22-bookworm AS node
# Matches backend/rust-toolchain.toml
FROM rust:1.90.0-bookworm

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=1000

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libasound2-dev alsa-utils libssl-dev libudev-dev \
    libx11-dev libxcb1-dev libxrandr-dev libxi-dev postgresql-client \
    sudo git less \
    && rm -rf /var/lib/apt/lists/*

COPY --from=node /usr/local/bin/node /usr/local/bin/node
COPY --from=node /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
    && ln -s /usr/local/lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx \
    && ln -s /usr/local/lib/node_modules/corepack/dist/corepack.js /usr/local/bin/corepack \
    && corepack enable \
    && corepack prepare pnpm@10.15.0 --activate

RUN rustup component add rustfmt clippy \
    && rustup target add wasm32-unknown-unknown \
    && cargo install sqlx-cli --version 0.7.4 --locked --no-default-features --features rustls,postgres \
    && cargo install cargo-watch --locked \
    && rm -rf /usr/local/cargo/registry

# Non-root user matching the host UID so files written to the bind-mounted
# repo (target/, node_modules/) stay owned by you on the host.
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m -s /bin/bash $USERNAME \
    && echo "$USERNAME ALL=(root) NOPASSWD:ALL" > /etc/sudoers.d/$USERNAME \
    && mkdir -p /usr/local/cargo/registry \
    && chown -R $USER_UID:$USER_GID /usr/local/cargo /usr/local/rustup \
    # Login shells reset PATH via /etc/profile; keep the toolchain visible
    && printf 'export RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo\nexport PATH=/usr/local/cargo/bin:$PATH\n' > /etc/profile.d/rust.sh

# Claude Code CLI (rebuild the container to update; the autoupdater can't
# write to the root-owned global prefix)
RUN npm install -g @anthropic-ai/claude-code

USER $USERNAME
RUN corepack prepare pnpm@10.15.0 --activate \
    # Pre-create so the named volume mounted here is owned by $USERNAME
    && mkdir -p /home/$USERNAME/.claude
