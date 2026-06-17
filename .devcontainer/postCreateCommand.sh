sudo apt update
sudo apt upgrade -y
sudo apt install -y rustup qemu-system-riscv64 u-boot-qemu dosfstools mtools u-boot-tools parted

rustup default stable 
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools
cargo install cargo-binutils

echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc