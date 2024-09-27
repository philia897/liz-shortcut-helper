#!/bin/bash
set -e

# Define the target directory
TARGET_DIR="$HOME/.config/liz"

# Check if the target directory exists
if [ -d "$TARGET_DIR" ]; then
    echo "Directory $TARGET_DIR already exists."
    read -p "Do you want to overwrite it? (y/N): " choice
    choice=${choice:-N}  # Default to 'N' if no input is provided
    case "$choice" in 
        [yY][eE][sS]|[yY]) 
            echo "Overwriting $TARGET_DIR..."
            rm -rf "$TARGET_DIR"
            mkdir -p "$TARGET_DIR"
            ;;
        *) 
            echo "Exiting without making changes."
            return 0
            ;;
    esac
else
    echo "Directory $TARGET_DIR does not exist. Creating it..."
    mkdir -p "$TARGET_DIR"
fi


# Get the current username
USER=$(whoami)

# Define the paths with the current user
LIZ_PATH="/home/$USER/.config/liz/"
MUSIC_SHEET_PATH="/home/$USER/.config/liz/music_sheet.lock"
USER_SHEETS_PATH="/home/$USER/.config/liz/sheets"
KEYMAP_PATH="/home/$USER/.config/liz/keymap.json"
YDOTTOOL_SOCKET_PATH="/tmp/.ydotool_socket"

# Create the rhythm.toml file with the paths
cat <<EOL > ./data/rhythm.toml
liz_path = "$LIZ_PATH"
music_sheet_path = "$MUSIC_SHEET_PATH"
user_sheets_path = "$USER_SHEETS_PATH"
keymap_path = "$KEYMAP_PATH"
ydotool_socket_path = "$YDOTTOOL_SOCKET_PATH"
EOL

echo "rhythm.toml has been generated with paths for user '$USER'."


# Copy files from ./data/* to ~/.config/liz
echo "Copying files from ./data/* to $TARGET_DIR..."
cp -r ./data/* "$TARGET_DIR"

echo "Files copied successfully!"