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
extern "C" fn kmain(_argc: usize, argv: *const *const core::ffi::c_char) -> ! {
    let dtb_address_str = unsafe { core::ffi::CStr::from_ptr(*argv.add(1)) }.to_str().unwrap();

    let fdt_address = usize::from_str_radix(dtb_address_str, 16).unwrap();

    println!("Flattened device address: {:#x}", fdt_address);

    let fdt = unsafe {
        let device_tree_binary_header = core::slice::from_raw_parts(fdt_address as *const u32, 40);
        let total_size = device_tree_binary_header.get(1).unwrap();
        let device_tree_binary_data = core::slice::from_raw_parts(fdt_address as *const u8, *total_size as usize);
        fdt::Fdt::new(device_tree_binary_data).expect("Failed to parse full FDT")
    };

    let kernel_end_address = core::ptr::addr_of!(_kernel_end) as usize;
    let memory_region = fdt.memory().regions().next().unwrap();
    let base_address = memory_region.starting_address as usize;
    let size = memory_region.size.unwrap();

    unsafe {
        HEAP.force_unlock();
        HEAP.lock().add_to_heap(kernel_end_address, base_address + size);
    };

    println!("Heap initialized:");
    println!("\tStart: {:#x}", kernel_end_address);
    println!("\tEnd: {:#x}", base_address + size);

    timer_interrupt::set_time_quanta(1_000_000);
    unsafe {
        use riscv::{
            interrupt,
            register::{self, stvec},
        };
        register::stvec::write(riscv::register::stvec::Stvec::new(trap_handler::entry::trap_handler_entry as *const u8 as usize, stvec::TrapMode::Direct));
        println!("Supervisor trap: Direct@{:#x}", trap_handler::entry::trap_handler_entry as *const u8 as usize);
        interrupt::enable();
        println!("Interrupts enabled.");
        interrupt::enable_interrupt(interrupt::Interrupt::SupervisorTimer);
        println!("Supervisor timer interrupt enabled.");
    }
    timer_interrupt::update_timer();
    loop {
        riscv::asm::wfi();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print::print!("{}", info);
    let _ = sbi::system_reset::system_reset(sbi::system_reset::ResetType::Shutdown, sbi::system_reset::ResetReason::SystemFailure);
    loop {}
}
