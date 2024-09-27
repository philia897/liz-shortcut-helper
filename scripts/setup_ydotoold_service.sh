#!/bin/bash
set -e

# Step 1: Create the group if it doesn’t exist
if ! getent group ydotoolgroup > /dev/null; then
    echo "Creating group 'ydotoolgroup'..."
    sudo groupadd ydotoolgroup
else
    echo "Group 'ydotoolgroup' already exists."
fi

# Step 2: Add the current user to the group
echo "Adding $(whoami) to the 'ydotoolgroup'..."
sudo usermod -aG ydotoolgroup $(whoami)


# Check if need to further creating the service
SERVICE_FILE="/etc/systemd/system/ydotoold.service"

if [ -f "$SERVICE_FILE" ]; then
    echo "The service file $SERVICE_FILE already exists."
    read -p "Do you want to overwrite it? (y/N): " choice
    choice=${choice:-N}  # Default to 'N' if no input is provided
    case "$choice" in 
        [yY][eE][sS]|[yY]) 
            echo "Disabling and stopping the existing ydotoold.service..."
            sudo systemctl disable ydotoold.service
            sudo systemctl stop ydotoold.service
            echo "Overwriting $SERVICE_FILE..."
            ;;
        *) 
            echo "Exiting without making changes."
            return 0
            ;;
    esac
else
    echo "Creating the systemd service file..."
fi


# Step 3: Remove the old socket file if it exists
SOCKET_FILE="/tmp/.ydotool_socket"
if [ -e "$SOCKET_FILE" ]; then
    echo "Removing old socket file: $SOCKET_FILE..."
    sudo rm -f $SOCKET_FILE
else
    echo "No old socket file found at $SOCKET_FILE."
fi

# Step 4: Edit the systemd service file
echo "Creating or updating the systemd service file..."

sudo tee $SERVICE_FILE > /dev/null << EOL
# /etc/systemd/system/ydotoold.service
[Unit]
Description=Starts ydotoold service
# After=network.target

[Service]
Type=simple
Restart=always
ExecStart=/usr/bin/ydotoold --socket-path="$SOCKET_FILE"
ExecStartPost=/bin/sh -c 'chown root:ydotoolgroup $SOCKET_FILE && chmod 660 $SOCKET_FILE'
ExecReload=/usr/bin/kill -HUP \$MAINPID
KillMode=process
TimeoutSec=180

[Install]
WantedBy=default.target
EOL

# Step 5: Reload systemd
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Step 6: Enable and start the ydotoold service
echo "Enabling and starting ydotoold.service..."
sudo systemctl enable ydotoold.service
sudo systemctl start ydotoold.service

# Step 7: Verify the service status
echo "Checking ydotoold.service status..."
sudo systemctl status ydotoold.service --no-pager
