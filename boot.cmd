if fatload virtio 0:1 0x80200000 kernel.bin; then
    echo "Loaded kernel from virtio (QEMU)"
else
    echo "virtio failed, trying mmc..."
    fatload mmc 0:1 0x80200000 kernel.bin
fi

go 0x80200000