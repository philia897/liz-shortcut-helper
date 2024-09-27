#!/bin/bash
set -e

# Function to check if a package is installed
is_installed() {
    local package="$1"
    if [ "$OS" == "Arch" ]; then
        pacman -Qi "$package" &> /dev/null
    elif [ "$OS" == "Debian" ]; then
        dpkg -l "$package" &> /dev/null
    elif [ "$OS" == "Fedora" ]; then
        rpm -q "$package" &> /dev/null
    elif [ "$OS" == "SUSE" ]; then
        rpm -q "$package" &> /dev/null
    else
        echo "Unsupported OS: $OS"
        return 1
    fi
}

# Function to install packages
install_packages() {
    local packages=("$@")
    local to_install=()

    for package in "${packages[@]}"; do
        if is_installed "$package"; then
            echo "$package is already installed. Skipping."
        else
            to_install+=("$package")
        fi
    done

    if [ ${#to_install[@]} -eq 0 ]; then
        echo "All packages are already installed. Exiting."
        return 0
    fi

    echo "Installing ${packages[*]}..."
    if [ "$OS" == "Arch" ]; then
        sudo pacman -S "${packages[@]}"
    elif [ "$OS" == "Debian" ]; then
        sudo apt update
        sudo apt install "${packages[@]}"
    elif [ "$OS" == "Fedora" ]; then
        sudo dnf install "${packages[@]}"
    elif [ "$OS" == "SUSE" ]; then
        sudo zypper install "${packages[@]}"
    else
        echo "Unsupported OS: $OS"
        return 1
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
            return 1
            ;;
    esac
else
    echo "Could not detect OS."
    return 1
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

echo "All packages are installed!"