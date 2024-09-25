#!/bin/bash

# Check if the EXEC_START parameter is provided
if [ -z "$1" ]; then
    echo "Error: No ExecStart path provided."
    echo "Usage: sudo ./setup_bluebird_service.sh /path/to/bluebird"
    exit 1
fi

# Variables
SERVICE_NAME="bluebird"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
EXEC_START="$1"
USER=$(whoami)  # Automatically get the current user

# Create or overwrite the service file
echo "Creating systemd service file at ${SERVICE_FILE}..."

cat <<EOL | sudo tee ${SERVICE_FILE} > /dev/null
[Unit]
Description=Bluebird Service for Liz
After=network.target ydotoold.service
Requires=ydotoold.service

[Service]
ExecStart=${EXEC_START}
Restart=on-failure
User=${USER}  # Use the current user
# WorkingDirectory=/home/${USER}/.config/liz
Environment=RUST_LOG=info  # Optional: Set your logging level or any other environment variables
TimeoutSec=60

[Install]
WantedBy=multi-user.target  # or WantedBy=multi-user.target
EOL

# Reload systemd configuration
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Start and enable the services
echo "Starting and enabling the services..."
sudo systemctl start ydotoold.service
sudo systemctl enable ydotoold.service
sudo systemctl start ${SERVICE_NAME}.service
sudo systemctl enable ${SERVICE_NAME}.service

# Status check
echo "Checking the status of ${SERVICE_NAME} service..."
sudo systemctl status ${SERVICE_NAME}.service
