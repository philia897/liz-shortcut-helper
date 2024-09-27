#!/bin/bash
set -e

# Check if the EXEC_START parameter is provided
# if [ -z "$1" ]; then
#     echo "Error: No ExecStart path provided."
#     echo "Usage: sudo ./setup_bluebird_service.sh /path/to/bluebird"
#     exit 1
# fi

# Variables
SERVICE_NAME="bluebird"
SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
BIN_PATH=${1:-"/usr/local/bin"}
USER=$(whoami)  # Automatically get the current user

# Function to upgrade the executable bluebird and liz
upgrading() {

    echo "Upgrading liz and bluebird"

    echo "Move binaries to $BIN_PATH"
    sudo cp liz $BIN_PATH
    sudo cp bluebird $BIN_PATH

    echo "Make binaries executable"
    sudo chmod +x $BIN_PATH/liz
    sudo chmod +x $BIN_PATH/bluebird
}

# Check if need to further running
if [ -f "$SERVICE_FILE" ]; then

    echo "Disabling and stopping the existing $SERVICE_NAME.service..."
    # sudo systemctl disable $SERVICE_NAME.service
    sudo systemctl stop $SERVICE_NAME.service

    upgrading

    echo "The service file $SERVICE_FILE already exists."
    read -p "Do you want to overwrite it? (Y/n): " choice
    choice=${choice:-Y}  # Default to 'N' if no input is provided
    case "$choice" in 
        [yY][eE][sS]|[yY]) 
            echo "Overwriting $SERVICE_FILE..."
            ;;
        *) 
            # Restart the service
            echo "Starting and enabling the services..."
            sudo systemctl start ${SERVICE_NAME}.service
            # sudo systemctl enable ${SERVICE_NAME}.service
            echo "Exiting without changing the service"
            return 0
            ;;
    esac
else
    echo "Creating the systemd service file..."
fi

# Create or overwrite the service file
echo "Creating systemd service file at ${SERVICE_FILE}..."

cat <<EOL | sudo tee ${SERVICE_FILE} > /dev/null
[Unit]
Description=Bluebird Service for Liz
# After=ydotoold.service
# Requires=ydotoold.service

[Service]
ExecStart=${BIN_PATH}/bluebird
Restart=on-failure
User=${USER}
# WorkingDirectory=/home/${USER}/.config/liz
Environment=RUST_LOG=info
TimeoutSec=60
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=default.target
EOL

# Reload systemd configuration
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Start and enable the services
echo "Starting and enabling the services..."
sudo systemctl start ${SERVICE_NAME}.service
sudo systemctl enable ${SERVICE_NAME}.service

# Status check
echo "Checking the status of ${SERVICE_NAME} service..."
sudo systemctl status ${SERVICE_NAME}.service --no-pager