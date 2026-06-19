#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

cargo build 

rust-objcopy --strip-all -O binary \
  target/riscv64gc-unknown-none-elf/debug/kernel \
  kernel.bin

mkimage -C none -A riscv -T script -d boot.cmd boot.scr

dd if=/dev/zero of=disk.img bs=4096 count=16384
parted disk.img mklabel msdos
parted disk.img mkpart primary fat32 1MiB 100%
mformat -i disk.img@@1048576 -F -v "BOOT" ::

mcopy -i disk.img@@1048576 -s kernel.bin ::/
mcopy -i disk.img@@1048576 -s boot.scr ::/