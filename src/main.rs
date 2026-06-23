#![no_main]
#![no_std]

use crate::print::println;

mod context;
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
    let bss_start = &raw mut _bss_start;
    let bss_end = &raw mut _bss_end;
    let size = (bss_end as usize) - (bss_start as usize);
    unsafe { core::ptr::write_bytes(bss_start, 0, size) };
    timer_interrupt::set_time_quanta(1_000_000);
    unsafe {
        use riscv::{
            interrupt,
            register::{self, stvec},
        };
        register::stvec::write(riscv::register::stvec::Stvec::new(trap_handler::entry::trap_handler_entry as *const u8 as usize, stvec::TrapMode::Direct));
        println!("Supervisor trap entry: {:#x}", trap_handler::entry::trap_handler_entry as *const u8 as usize);
        interrupt::enable();
        interrupt::enable_interrupt(interrupt::Interrupt::SupervisorTimer);
    }
    let fdt = unsafe {
        let device_tree_binary_header = core::slice::from_raw_parts(fdt_address as *const u32, 40);
        let total_size = device_tree_binary_header.get(1).unwrap();
        let device_tree_binary_data = core::slice::from_raw_parts(fdt_address as *const u8, *total_size as usize);
        fdt::Fdt::new(device_tree_binary_data).expect("Failed to parse full FDT")
    };
    let memory = fdt.memory().regions().next().unwrap();
    let memory_base = memory.starting_address as usize;
    let memory_size = memory.size.unwrap();
    let heap_start = &raw const _kernel_end as usize;
    let heap_end = memory_base + memory_size;
    unsafe {
        HEAP.lock().add_to_heap(heap_start, heap_end);
    }
    println!("Heap:\nstart: {:#x}\nend:{:#x}", heap_start, heap_end);
    {
        use sg200x_bsp::sdmmc::Sdmmc;
        use sg200x_bsp::soc::{SD_DRIVER_BASE, TOP_BASE};
        let sdmmc = unsafe { Sdmmc::new(SD_DRIVER_BASE, TOP_BASE) };
        sdmmc.init().unwrap();
        println!("SDMMC initialized.");
        let mut buf = [0u8; 512];
        sdmmc.read_block(0, &mut buf).unwrap();
    }
    println!("Done.");
    loop {
        riscv::asm::wfi();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print::print!("{}", info);
    let _ = sbi::system_reset::system_reset(sbi::system_reset::ResetType::ColdReboot, sbi::system_reset::ResetReason::SystemFailure);
    loop {}
}
