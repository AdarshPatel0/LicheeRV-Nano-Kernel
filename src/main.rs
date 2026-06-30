#![no_main]
#![no_std]

use crate::print::println;

mod context;
mod ecall;
mod filesystem;
mod print;
mod sdmmc;
mod timer_interrupt;
mod trap_handler;

unsafe extern "C" {
    static _kernel_end: u8;
    static _stack_top: u8;
    static mut _bss_start: u8;
    static mut _bss_end: u8;
}

pub const TIME_QUANTA: u64 = 1_000_000;

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
        println!("start: {:#x}",heap_start);
        println!("end: {:#x}",heap_end);
        println!("size: {:#x}",heap_end - heap_start);
    }
    // Initialize sd card and scan for paritions
    {
        let card_info = sdmmc::initialize_card();
        println!("total blocks: {}", card_info.capacity_blocks.unwrap());
        let mut mbr_raw = [0u8; 512];
        sdmmc::read_blocks(0, &mut mbr_raw);
        let mbr = mbrs::Mbr::try_from_bytes(&mbr_raw).unwrap();
        for entry in mbr.partition_table.entries {
            if let Some(partition) = entry {
                if partition.part_type() == &mbrs::PartType::ext4() {
                    println!("partition start sector: {}", partition.start_sector_lba());
                    println!("partition sectors: {}", partition.sector_count_lba());
                    filesystem::initialize_filesystem(partition.start_sector_lba(), partition.sector_count_lba() as u32);
                    break;
                }
            }
        }
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
    println!("Kernel initialized");
    loop {
        riscv::asm::wfi();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    let _ = sbi::system_reset::system_reset(sbi::system_reset::ResetType::ColdReboot, sbi::system_reset::ResetReason::SystemFailure);
    loop {riscv::asm::wfi();}
}
