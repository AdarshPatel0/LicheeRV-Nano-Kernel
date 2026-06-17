#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

cargo build 

qemu-img create -f raw disk.img 1G && mkfs.vfat -F 32 disk.img

mcopy -i disk.img target/riscv64gc-unknown-none-elf/debug/kernel ::