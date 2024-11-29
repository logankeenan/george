FROM rust:1.81.0

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y \
    libxdo-dev \
    && rm -rf /var/lib/apt/lists/*


RUN cargo build --release

