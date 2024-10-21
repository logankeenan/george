#!/bin/bash
# This only needs to be run once on the GH action runner so that it can take screenshots
# in the tests

# Create the systemd service file
sudo bash -c 'cat > /etc/systemd/system/xvfb.service' <<EOL
[Unit]
Description=Virtual Framebuffer Service for GitHub Actions
After=network.target

[Service]
Type=simple
ExecStart=Xvfb :99 -screen 0 1024x768x24
Restart=always
User=$(whoami)
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

[Install]
WantedBy=multi-user.target
EOL

sudo systemctl daemon-reload
sudo systemctl enable xvfb
sudo systemctl start xvfb