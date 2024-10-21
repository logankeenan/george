# Use the official Rust image as a base for building
FROM rust:1.81.0 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

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


# Install Firefox directly from Mozilla's archive
RUN wget -O firefox.tar.bz2 'https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US' && \
    tar -xjf firefox.tar.bz2 -C /opt/ && \
    ln -s /opt/firefox/firefox /usr/bin/firefox && \
    rm firefox.tar.bz2


# Set the working directory
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/george-client .

# Expose port 3000 to the host
EXPOSE 3000

# Set environment variables
ENV DISPLAY=:99

CMD ["sh", "-c", "Xvfb :99 -screen 0 1024x768x16 & sleep 2 && firefox --display=:99 & sleep 5 && ./george-client"]
