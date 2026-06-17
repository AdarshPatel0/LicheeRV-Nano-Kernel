#!/bin/sh
set -e

KERNEL="$1"

exec qemu-system-riscv64 \
  -m 64M \
  -machine virt \
  -smp 1 \
  -nographic \
  -s -S \
  -drive file=disk.img,format=raw,id=hd0 \
  -device virtio-blk-device,drive=hd0 \
  -kernel /usr/lib/u-boot/qemu-riscv64_smode/uboot.elf