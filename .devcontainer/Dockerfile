# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.166.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# Add target for Wio Terminal
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install --no-install-recommends \
  git \
  minicom \
  libusb-1.0-0-dev \
  libsdl2-dev \
  libssl-dev \
  nodejs \
  && rm -rf /var/lib/apt/lists/*
RUN rustup target add thumbv7em-none-eabihf
RUN cargo install cargo-generate hf2-cli cargo-hf2
ENV DISPLAY=host.docker.internal:0
