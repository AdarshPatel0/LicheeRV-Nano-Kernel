#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

rust-objcopy --strip-all -O binary \
  target/riscv64gc-unknown-none-elf/debug/kernel \
  kernel.bin

mkimage -f kernel.its kernel.itb

sudo stty -F /dev/ttyUSB0 115200

sudo sh -c "printf 'loady \${loadaddr}\r' > /dev/ttyUSB0"

sleep 1

sudo sh -c "sb -Y kernel.itb < /dev/ttyUSB0 > /dev/ttyUSB0"

sudo sh -c "printf 'bootm \${loadaddr}\r' > /dev/ttyUSB0"

sudo picocom -b 115200 /dev/ttyUSB0