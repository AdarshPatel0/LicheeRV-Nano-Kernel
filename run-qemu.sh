qemu-system-riscv64 \
  -m 64M \
  -machine virt \
  -smp 1 \
  -nographic \
  -drive file=disk.img,format=raw,id=hd0 \
  -device virtio-blk-device,drive=hd0 \
  -s -S \
  -kernel /usr/lib/u-boot/qemu-riscv64_smode/uboot.elf