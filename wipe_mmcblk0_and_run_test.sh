#!/bin/bash
# SD Card Wipe and Format Script for F469 Discovery
# WARNING: This will ERASE ALL DATA on /dev/mmcblk0

echo "=== SD Card Wipe and Format Script ==="
echo "Target device: /dev/mmcblk0"
echo ""
read -p "Press Enter to continue or Ctrl+C to abort..."

# 1. Wipe the card
echo "Wiping card..."
sudo wipefs -a /dev/mmcblk0

# 2. Create new partition table
echo "Creating GPT partition table..."
sudo parted /dev/mmcblk0 --script mklabel gpt

# 3. Create FAT32 partition
echo "Creating FAT32 partition..."
sudo parted /dev/mmcblk0 --script mkpart primary fat32 1MiB 100%

# 4. Format as FAT32
echo "Formatting as FAT32..."
sudo mkfs.vfat -F32 /dev/mmcblk0p1

# 5. Verify
echo ""
echo "=== Done ==="
sudo fdisk -l /dev/mmcblk0
echo ""
echo "Card is ready. Now run:"
echo "  cd /tmp && . ~/.cargo/env && probe-rs run --chip STM32F469NI soak.elf"
