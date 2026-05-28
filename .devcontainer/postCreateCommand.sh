sudo apt update
sudo apt upgrade -y
sudo apt install -y \
    rustup \
    qemu-system-riscv64 \

rustup default stable 
rustup target add riscv64gc-unknown-linux-gnu