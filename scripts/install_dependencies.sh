#!/bin/bash
set -e

# Function to install packages
install_packages() {
    local packages=("$@")
    echo "Installing ${packages[*]}..."
    if [ "$OS" == "Arch" ]; then
        sudo pacman -S "${packages[@]}"
    elif [ "$OS" == "Debian" ]; then
        sudo apt update
        sudo apt install -y "${packages[@]}"
    elif [ "$OS" == "Fedora" ]; then
        sudo dnf install -y "${packages[@]}"
    elif [ "$OS" == "SUSE" ]; then
        sudo zypper install -y "${packages[@]}"
    else
        echo "Unsupported OS: $OS"
        exit 1
    fi
}

echo "Start installing necessary dependencies..."

# Determine the OS
if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "$ID" in
        arch)
            OS="Arch"
            ;;
        ubuntu|debian)
            OS="Debian"
            ;;
        fedora)
            OS="Fedora"
            ;;
        opensuse|suse)
            OS="SUSE"
            ;;
        *)
            echo "Unsupported Linux distribution: $ID"
            exit 1
            ;;
    esac
else
    echo "Could not detect OS."
    exit 1
fi

# Install ydotool and rofi
# Check if running on Wayland
if [ "$XDG_SESSION_TYPE" == "wayland" ]; then
    echo "Running on Wayland. Installing ydotool rofi-wayland."
    install_packages ydotool rofi-wayland
else
    echo "Running on X11. Installing ydotool rofi."
    install_packages ydotool rofi
fi

echo "Installation complete!"