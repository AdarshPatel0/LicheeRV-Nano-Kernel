#![no_main]
#![no_std]

extern crate alloc;

use crate::print::println;

mod context;
mod drivers;
mod ecall;
mod hart;
mod print;
mod thread;
mod trap_handler;
mod shell;

unsafe extern "C" {
    static _kernel_end: u8;
    static _stack_top: u8;
    static mut _bss_start: u8;
    static mut _bss_end: u8;
}

pub const TIME_QUANTA: u64 = 1_000_000;
pub const HART_STACK_SIZE: usize = 16384;

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
extern "C" fn kmain(hart_id: usize, fdt_address: usize) -> ! {
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
    }
    // Create init thread
    {
        thread::create_thread(init as *const u8 as usize, true, 1 << 10, &[]);
    }
    // Initialize all harts.
    {
        for cpu in fdt.cpus() {
            let hart_id = cpu.ids().first();
            let hart_stack_top = hart::create_hart_stack(hart_id);
            if hart_id != hart_id {
                unsafe {
                    sbi::hart_state_management::hart_start(hart_id, sbi::PhysicalAddress::new(hart::hart_startup_entry as *const u8 as usize), hart_stack_top as usize as usize).unwrap();
                }
            }
        }
        let hart_stack_top = hart::create_hart_stack(hart_id);
        unsafe {
            riscv::interrupt::enable();
            riscv::interrupt::enable_interrupt(riscv::interrupt::Interrupt::SupervisorTimer);
            sbi::timer::set_timer(riscv::register::time::read64() + TIME_QUANTA).unwrap();
            sbi::hart_state_management::hart_suspend(sbi::hart_state_management::SuspendType::DefaultNonRetentive { resume_address: sbi::PhysicalAddress::new(hart::hart_startup_entry as *const u8 as usize), opaque: hart_stack_top }).unwrap();
        }
    }
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

pub fn init() {
    println!("Kernel initialized");
}