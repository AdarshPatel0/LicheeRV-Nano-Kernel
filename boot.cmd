load ${devtype} ${devnum}:${distro_bootpart} 0x80200000 kernel.bin
go 0x80200000 ${fdt_addr}