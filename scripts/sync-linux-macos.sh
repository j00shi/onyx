#!/usr/bin/env bash
rsync -a --no-owner --no-group --delete --exclude='target/' --exclude='node_modules/' --exclude='.git/' /mnt/share/ ~/Code/onyx/
