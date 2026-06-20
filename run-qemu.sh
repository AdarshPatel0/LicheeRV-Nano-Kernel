#!/bin/sh
exec qemu-system-riscv64 \
  -m 256M \
  -machine virt \
  -smp 1 \
  -nographic \
  -s -S \
  -drive file=disk.img,format=raw,id=hd0,if=none \
  -device virtio-blk-device,drive=hd0 \
  -kernel /usr/lib/u-boot/qemu-riscv64_smode/uboot.elf