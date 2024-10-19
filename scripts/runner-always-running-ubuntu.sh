#!/bin/bash

# Create the systemd service file
sudo bash -c 'cat > /etc/systemd/system/actions-runner.service' <<EOL
[Unit]
Description=GitHub Actions Runner
After=network.target

[Service]
User=$(whoami)
WorkingDirectory=$HOME/actions-runner
ExecStart=/bin/bash $HOME/actions-runner/run.sh
Restart=always
RestartSec=5
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

[Install]
WantedBy=multi-user.target
EOL

# Reload systemd, enable, and start the service
sudo systemctl daemon-reload
sudo systemctl enable actions-runner.service
sudo systemctl start actions-runner.service

# Check the service status
sudo systemctl status actions-runner.service --no-pager

echo "Setup complete! The actions-runner service is now running."
