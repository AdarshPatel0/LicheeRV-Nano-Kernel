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
    let _fdt = unsafe {
        let device_tree_binary_header = core::slice::from_raw_parts(fdt_address as *const u32, 40);
        let total_size = device_tree_binary_header.get(1).unwrap();
        let device_tree_binary_data = core::slice::from_raw_parts(fdt_address as *const u8, *total_size as usize);
        fdt::Fdt::new(device_tree_binary_data).expect("Failed to parse full FDT")
    };
    println!("Done.");
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
