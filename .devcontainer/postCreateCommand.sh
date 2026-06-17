sudo apt update
sudo apt upgrade -y
sudo apt install -y rustup qemu-system-riscv64 u-boot-qemu dosfstools mtools u-boot-tools

rustup default stable 
rustup target add riscv64gc-unknown-none-elf

qemu-img create -f raw disk.img 1G
mkfs.vfat -F 32 disk.img