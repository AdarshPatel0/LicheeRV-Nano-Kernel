sudo apt update
sudo apt upgrade -y
sudo apt install -y rustup qemu-system-riscv64 u-boot-qemu

rustup default stable 
rustup target add riscv64gc-unknown-none-elf