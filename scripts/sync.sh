#!/usr/bin/env bash
# Script for syncing shared directory in VM using rsync
# Checks every 2 seconds for updates
echo "Watching /mnt/share..."
while true; do
  rsync -a --no-owner --no-group --delete \
    --exclude='target/' \
    --exclude='node_modules/' \
    /mnt/share/ ~/Code/onyx/
  sleep 2
done
