#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

cargo build 

qemu-img create -f raw disk.img 1G

mkfs.vfat -F 32 disk.img

rust-objcopy --strip-all -O binary \
  target/riscv64gc-unknown-none-elf/debug/kernel \
  target/riscv64gc-unknown-none-elf/debug/kernel.bin

mcopy -i disk.img target/riscv64gc-unknown-none-elf/debug/kernel.bin ::kernel.bin