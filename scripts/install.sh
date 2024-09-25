#!/bin/bash
set -e

echo "Move binaries to /usr/local/bin"
sudo cp liz /usr/local/bin/
sudo cp bluebird /usr/local/bin/

echo "Make binaries executable"
sudo chmod +x /usr/local/bin/liz
sudo chmod +x /usr/local/bin/bluebird

source copy_to_config_liz.sh
source install_dependencies.sh
source setup_ydotoold_service.sh
source setup_bluebird_service.sh /usr/local/bin/bluebird

echo "Install complete!"