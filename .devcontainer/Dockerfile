# Claude Code Devcontainer for polkadot-cookbook
FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG TZ
ENV TZ="$TZ"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Install system packages
RUN apt-get update && apt-get install -y --no-install-recommends \
  python3 \
  python3-setuptools \
  # Sandboxing support for Claude Code
  bubblewrap \
  socat \
  # Modern CLI tools
  fd-find \
  ripgrep \
  tmux \
  zsh \
  # Build tools
  build-essential \
  libssl-dev \
  pkg-config \
  # Utilities
  jq \
  nano \
  unzip \
  vim \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install git-delta
ARG GIT_DELTA_VERSION=0.18.2
RUN ARCH=$(dpkg --print-architecture) && \
  curl -fsSL "https://github.com/dandavison/delta/releases/download/${GIT_DELTA_VERSION}/git-delta_${GIT_DELTA_VERSION}_${ARCH}.deb" -o /tmp/git-delta.deb && \
  dpkg -i /tmp/git-delta.deb && \
  rm /tmp/git-delta.deb

# Install fzf
ARG FZF_VERSION=0.67.0
RUN ARCH=$(dpkg --print-architecture) && \
  case "${ARCH}" in \
    amd64) FZF_ARCH="linux_amd64" ;; \
    arm64) FZF_ARCH="linux_arm64" ;; \
    *) echo "Unsupported architecture: ${ARCH}" && exit 1 ;; \
  esac && \
  curl -fsSL "https://github.com/junegunn/fzf/releases/download/v${FZF_VERSION}/fzf-${FZF_VERSION}-${FZF_ARCH}.tar.gz" | tar -xz -C /usr/local/bin

# Create directories and set ownership
RUN mkdir -p /commandhistory /workspace /home/vscode/.claude /opt && \
  touch /commandhistory/.bash_history && \
  touch /commandhistory/.zsh_history && \
  chown -R vscode:vscode /commandhistory /workspace /home/vscode/.claude /opt

ENV DEVCONTAINER=true
ENV SHELL=/bin/zsh
ENV EDITOR=nano
ENV VISUAL=nano

# Use bash instead of dash
RUN echo "dash dash/sh boolean false" | debconf-set-selections \
  && dpkg-reconfigure -f noninteractive dash

WORKDIR /workspace

USER vscode

ENV PATH="/home/vscode/.local/bin:$PATH"

RUN mkdir -p /home/vscode/.local/bin && ln -s /usr/bin/bash /home/vscode/.local/bin/sh

# Install Claude Code natively with marketplace plugins
RUN curl -fsSL https://claude.ai/install.sh | bash && \
  claude plugin marketplace add anthropics/skills && \
  claude plugin marketplace add trailofbits/skills && \
  claude plugin marketplace add trailofbits/skills-curated

# Install fnm (Fast Node Manager) and Node 22
ARG NODE_VERSION=22
ENV FNM_DIR="/home/vscode/.fnm"
RUN curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir "$FNM_DIR" --skip-shell && \
  export PATH="$FNM_DIR:$PATH" && \
  eval "$(fnm env)" && \
  fnm install ${NODE_VERSION} && \
  fnm default ${NODE_VERSION}

# Install Rust via rustup
# The exact toolchain version is resolved from rust-toolchain.toml at first build in /workspace.
# We install a minimal 'stable' here just for cargo-llvm-cov; rustup will auto-switch in the workspace.
ENV RUSTUP_HOME="/home/vscode/.rustup"
ENV CARGO_HOME="/home/vscode/.cargo"
ENV PATH="/home/vscode/.cargo/bin:$PATH"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal && \
  . /home/vscode/.cargo/env && \
  cargo install cargo-llvm-cov

# Install Oh My Zsh
ARG ZSH_IN_DOCKER_VERSION=1.2.1
RUN sh -c "$(curl -fsSL https://github.com/deluan/zsh-in-docker/releases/download/v${ZSH_IN_DOCKER_VERSION}/zsh-in-docker.sh)" -- \
  -p git \
  -x

# Configure shell: fnm + cargo in PATH
RUN echo 'export FNM_DIR="/home/vscode/.fnm"' >> /home/vscode/.zshrc && \
  echo 'export PATH="$FNM_DIR:$PATH"' >> /home/vscode/.zshrc && \
  echo 'eval "$(fnm env --use-on-cd)"' >> /home/vscode/.zshrc && \
  echo 'source "$HOME/.cargo/env"' >> /home/vscode/.zshrc

# Copy post_install script
COPY --chown=vscode:vscode post_install.py /opt/post_install.py
