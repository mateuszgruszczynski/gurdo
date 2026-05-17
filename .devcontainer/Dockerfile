FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

# Rust toolchain (2024 edition supported by stable >= 1.85)
USER vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable \
    && echo 'source $HOME/.cargo/env' >> /home/vscode/.bashrc
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# Native deps needed to build the project's GUI / TLS / image / sqlite crates
# even though the GUI itself runs on the host (hybrid mode).
USER root
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libx11-dev \
    libxcb1-dev \
    libxkbcommon-dev \
    libwayland-dev \
    && rm -rf /var/lib/apt/lists/*

RUN npm install -g @anthropic-ai/claude-code 2>/dev/null || \
    (curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
     apt-get install -y nodejs && \
     npm install -g @anthropic-ai/claude-code)

# Container-only Claude Code settings — broader permissions that NEVER touch
# the host's ~/.claude/settings.json. The container is sandboxed, so allowing
# any Bash command and any directory is acceptable here even though it would
# not be on the host.
USER vscode
RUN mkdir -p /home/vscode/.claude && cat > /home/vscode/.claude/settings.json <<'JSON'
{
  "permissions": {
    "allow": ["Bash(*)", "Read(/**)", "Write(/**)", "Edit(/**)"],
    "additionalDirectories": ["/"]
  },
  "extraKnownMarketplaces": {
    "agile-dev": {
      "source": { "source": "github", "repo": "mateuszgruszczynski/agile-dev-pipeline" }
    }
  },
  "enabledPlugins": {
    "agile-dev@agile-dev": true
  }
}
JSON
USER root
