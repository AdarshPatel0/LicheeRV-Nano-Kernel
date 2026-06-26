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

pub const TIME_QUANTA: u64 = 1_000_000;

#[global_allocator]
pub static HEAP: buddy_system_allocator::LockedHeap<32> = buddy_system_allocator::LockedHeap::<32>::empty();

pub static FDT: spin::Once<fdt::Fdt> = spin::Once::new();

core::arch::global_asm!(
    r#"
    .section .text.entry
    .globl _start
    _start:
        la sp, _stack_top
        j kmain
    "#
);

fn clear_bss() {
    let bss_start = &raw mut _bss_start;
    let bss_end = &raw mut _bss_end;
    let bss_size = (bss_end as usize) - (bss_start as usize);
    unsafe { core::ptr::write_bytes(bss_start, 0, bss_size) };
}

fn setup_interrupts() {
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

fn set_fdt(fdt_address: usize) -> &'static fdt::Fdt<'static> {
    FDT.call_once(|| unsafe {
        let device_tree_binary_header = core::slice::from_raw_parts(fdt_address as *const u32, 40);
        let total_size = device_tree_binary_header.get(1).unwrap();
        let device_tree_binary_data = core::slice::from_raw_parts(fdt_address as *const u8, *total_size as usize);
        fdt::Fdt::new(device_tree_binary_data).expect("Failed to parse full FDT")
    })
}

fn initialize_heap(fdt: &fdt::Fdt) -> (usize, usize) {
    let memory = fdt.memory().regions().next().unwrap();
    let memory_base = memory.starting_address as usize;
    let memory_size = memory.size.unwrap();
    let heap_start = &raw const _kernel_end as usize;
    let heap_end = memory_base + memory_size;
    unsafe {
        HEAP.lock().add_to_heap(heap_start, heap_end);
    }
    (heap_start, heap_end)
}

#[unsafe(no_mangle)]
extern "C" fn kmain(_hart_id: usize, fdt_address: usize) -> ! {
    clear_bss();
    setup_interrupts();
    let fdt = set_fdt(fdt_address);
    let (heap_start, heap_end) = initialize_heap(fdt);
    println!("kernel started");
    println!("system heap size: {} bytes", heap_end - heap_start);
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
