#![no_main]
#![no_std]

extern crate alloc;

use crate::{
    drivers::block_device::{self, BlockDevice},
    print::println,
};

mod context;
mod drivers;
mod ecall;
mod print;
mod timer_interrupt;
mod trap_handler;

unsafe extern "C" {
    static _kernel_end: u8;
    static _stack_top: u8;
    static mut _bss_start: u8;
    static mut _bss_end: u8;
}

pub const TIME_QUANTA: u64 = 1_000_000;

const SDIO_BASE_ADDRESS: usize = 0x4310000;

#[global_allocator]
pub static HEAP: buddy_system_allocator::LockedHeap<32> = buddy_system_allocator::LockedHeap::<32>::empty();

core::arch::global_asm!(
    r#"
    .section .text.entry
    .globl _start
    _start:
        la sp, _stack_top
        j kmain
    "#
);

#[unsafe(no_mangle)]
extern "C" fn kmain(_hart_id: usize, fdt_address: usize) -> ! {
    // Clear .bss section
    {
        let bss_start = &raw mut _bss_start;
        let bss_end = &raw mut _bss_end;
        let bss_size = (bss_end as usize) - (bss_start as usize);
        unsafe { core::ptr::write_bytes(bss_start, 0, bss_size) };
    }
    // Get flattened device tree
    let fdt = {
        let device_tree_binary_header = unsafe { core::slice::from_raw_parts(fdt_address as *const u32, 40) };
        let total_size = device_tree_binary_header.get(1).unwrap();
        let device_tree_binary_data = unsafe { core::slice::from_raw_parts(fdt_address as *const u8, *total_size as usize) };
        fdt::Fdt::new(device_tree_binary_data).expect("Failed to parse full FDT")
    };
    // Initialize heap
    {
        let memory = fdt.memory().regions().next().unwrap();
        let memory_base = memory.starting_address as usize;
        let memory_size = memory.size.unwrap();
        let heap_start = &raw const _kernel_end as usize;
        let heap_end = memory_base + memory_size;
        unsafe { HEAP.lock().add_to_heap(heap_start, heap_end) };
        println!("heap initialized");
        println!("start: {:#x}", heap_start);
        println!("end: {:#x}", heap_end);
        println!("size: {:#x}", heap_end - heap_start);
    }
    // Setup timer and externel interrupts
    {
        timer_interrupt::set_time_quanta(TIME_QUANTA);
        unsafe {
            use riscv::{
                interrupt,
                register::{self, stvec},
            };
            register::stvec::write(riscv::register::stvec::Stvec::new(trap_handler::entry::trap_handler_entry as *const u8 as usize, stvec::TrapMode::Direct));
            interrupt::enable();
            interrupt::enable_interrupt(interrupt::Interrupt::SupervisorTimer);
            interrupt::enable_interrupt(interrupt::Interrupt::SupervisorExternal);
        }
    }
    {
        let block_device = alloc::rc::Rc::new(drivers::block_device::sdmmc::SdmmcBlockDevice::new(SDIO_BASE_ADDRESS));
        let mbr = {
            let mut mbr_data = [0u8; 512];
            block_device.read(0, &mut mbr_data);
            mbrs::Mbr::try_from_bytes(&mbr_data).unwrap()
        };
        for entry in mbr.partition_table.entries {
            if let Some(partition) = entry {
                let start_block = partition.start_sector_lba() as usize;
                let block_count = partition.sector_count_lba() as usize;
                let ext4_partition = drivers::filesystem::ext4::Ext4Partition::new(start_block, block_count, block_device);
                let _ext4_filesystem = drivers::filesystem::ext4::Ext4FileSystem::new(ext4_partition);
                break;
            }
        }
    }
    println!("Kernel initialized");
    loop {
        riscv::asm::wfi();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    let _ = sbi::system_reset::system_reset(sbi::system_reset::ResetType::ColdReboot, sbi::system_reset::ResetReason::SystemFailure);
    loop {
        riscv::asm::wfi();
    }
}
