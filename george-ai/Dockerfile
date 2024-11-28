# Use the official Rust image as a base for building
FROM rust:1.81.0 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY ../george-daemon .

RUN apt-get update && apt-get install -y \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the Rust app in release mode
RUN cargo build --release

# Use the official Ubuntu 22.04 as a base image for running
FROM ubuntu:22.04

# Install necessary packages
RUN apt-get update && apt-get install -y \
    wget \
    gnupg \
    libx11-6 \
    bzip2 \
    libxext6 \
    libxrender1 \
    libxtst6 \
    libxi6 \
    libxfixes3 \
    xvfb \
    x11-apps \
    libgtk-3-0 \
    libdbus-glib-1-2 \
    libasound2 \
    libgl1-mesa-glx \
    libegl1-mesa \
    pciutils \
    dbus-x11 \
    libxdo3 \
    && rm -rf /var/lib/apt/lists/*

RUN wget -q https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb \
    && apt-get update \
    && apt-get install -y ./google-chrome-stable_current_amd64.deb \
    && rm google-chrome-stable_current_amd64.deb \
    && rm -rf /var/lib/apt/lists/* \

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/george-daemon .