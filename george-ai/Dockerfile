FROM ubuntu:22.04

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
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

RUN wget -q https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb \
    && apt-get update \
    && apt-get install -y ./google-chrome-stable_current_amd64.deb \
    && rm google-chrome-stable_current_amd64.deb \
    && rm -rf /var/lib/apt/lists/*

# local development
#COPY george-daemon .
#RUN chmod +x george-daemon

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/logankeenan/george/releases/download/george-daemon/0.1.0/george-daemon-installer.sh | sh
RUN cp ./root/.cargo/bin/george-daemon /