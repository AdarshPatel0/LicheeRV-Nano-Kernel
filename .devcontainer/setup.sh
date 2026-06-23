sudo apt update
sudo apt upgrade -y
sudo apt install -y rustup picocom lrzsz u-boot-tools

rustup default stable
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools
cargo install cargo-binutils

echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc