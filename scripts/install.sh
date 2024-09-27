#!/bin/bash
set -e

# Copy the data/* to the default place where is
# /home/yourname/.config/liz/
# If it already exists, feel free to comment and skip this
source copy_to_config_liz.sh || { echo "Install Failed"; exit 1; }

# Intall ydotool and rofi, if you already installed them
# or you want to install by yourself, comment this
source install_dependencies.sh || { echo "Install Failed"; exit 1; }

# Setup the ydotool deamon service, if youw want to do this
# manually, skip this. Or you can check this to see what it
# does and make sure it is safe in your system.
source setup_ydotoold_service.sh || { echo "Install Failed"; exit 1; }

# Setup the bluebird service, and enable it to automatically
# start on startup. It is mandatory, and should NOT be commented
# Or the bluebird and liz will not be upgraded.
source setup_bluebird_service.sh || { echo "Install Failed"; exit 1; }

echo "Install complete!"