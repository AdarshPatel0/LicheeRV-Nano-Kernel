#/bin/bash
export PATH="$PATH:/home/vscode/.cargo/bin/"

set -e

cargo build

rust-objcopy --strip-all -O binary \
  target/riscv64gc-unknown-none-elf/debug/kernel \
  kernel.bin

sudo stty -F /dev/ttyUSB0 115200

sudo sh -c "printf 'loady 0x80200000\r' > /dev/ttyUSB0"

sudo sh -c "sb -Y kernel.bin < /dev/ttyUSB0 > /dev/ttyUSB0"

sudo sh -c "printf 'go 0x80200000 \${fdtcontroladdr}\r' > /dev/ttyUSB0"

sudo picocom -b 115200 /dev/ttyUSB0