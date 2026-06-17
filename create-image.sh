#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

cargo build 

rust-objcopy --strip-all -O binary \
  target/riscv64gc-unknown-none-elf/debug/kernel \
  target/riscv64gc-unknown-none-elf/debug/kernel.bin

qemu-img create -f raw disk.img 64M