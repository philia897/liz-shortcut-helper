#!/bin/bash

# Get the current username
USER=$(whoami)

# Define the paths with the current user
LIZ_PATH="/home/$USER/.config/liz/"
MUSIC_SHEET_PATH="/home/$USER/Workroom/liz-shortcut-helper/data/music_sheet.lock"
USER_SHEETS_PATH="/home/$USER/Workroom/liz-shortcut-helper/data/sheets"
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
