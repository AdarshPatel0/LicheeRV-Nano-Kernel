#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

rm -f sg2002-licheerv-nano-b.dtb.xz
rm -f kernel.bin.xz

dtc -I dts -O dtb -q -o sg2002-licheerv-nano-b.dtb sg2002-licheerv-nano-b.dts

rust-objcopy --strip-all -O binary \
  target/riscv64gc-unknown-none-elf/debug/kernel \
  kernel.bin

xz -9 sg2002-licheerv-nano-b.dtb
xz -9 kernel.bin

mkimage -f image.its image.itb

sudo stty -F /dev/ttyUSB0 115200

sudo sh -c "printf 'loady \${loadaddr}\r' > /dev/ttyUSB0"

sleep 1

sudo sh -c "sb -Y image.itb < /dev/ttyUSB0 > /dev/ttyUSB0"

sudo sh -c "printf 'bootm \${loadaddr}\r' > /dev/ttyUSB0"

sudo picocom -b 115200 /dev/ttyUSB0