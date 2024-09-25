#!/bin/bash
set -e

# Function to uninstall a package
uninstall_package() {
    local package=$1
    if command -v pacman &> /dev/null; then
        sudo pacman -Rns --noconfirm "$package"
    elif command -v apt &> /dev/null; then
        sudo apt remove --purge -y "$package"
    elif command -v dnf &> /dev/null; then
        sudo dnf remove -y "$package"
    elif command -v zypper &> /dev/null; then
        sudo zypper rm -y "$package"
    else
        echo "Unsupported package manager or package not found."
        exit 1
    fi
}

# Function to stop and disable a systemd service
stop_and_disable_service() {
    local service=$1
    sudo systemctl stop "$service"
    sudo systemctl disable "$service"
}

# Stop and disable the services
echo "Stopping and disabling services..."
stop_and_disable_service bluebird
stop_and_disable_service ydotoold

sudo rm -rf /etc/systemd/system/ydotoold.service
sudo rm -rf /etc/systemd/system/bluebird.service

# Remove the specified files and directories
echo "Removing files and directories..."
sudo rm -rf /usr/local/bin/bluebird
sudo rm -rf /usr/local/bin/liz

# uncomment this if you want all the data to be deleted
# rm -rf ~/.config/liz/

echo "Files and directories removed."

# Uninstall the packages
echo "Uninstalling packages..."
uninstall_package ydotool
if [ "$XDG_SESSION_TYPE" == "wayland" ]; then
    uninstall_package rofi-wayland
else
    uninstall_package rofi
fi

echo "Uninstallation complete!"
