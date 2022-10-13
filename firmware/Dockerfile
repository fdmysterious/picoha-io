FROM ubuntu:22.04

# Install Packages
RUN apt-get update && DEBIAN_FRONTEND=noninteractive TZ=Europe/Paris \
    apt-get -y install \
    build-essential \
    curl \
    pkg-config \
    libudev-dev

# Create a tmp setup dir
WORKDIR /setup

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update path to end rust installation
ENV PATH="/root/.cargo/bin:${PATH}"

# Install target specifics
RUN rustup target install thumbv6m-none-eabi
RUN cargo install elf2uf2-rs

# Create the workspace
WORKDIR /work
