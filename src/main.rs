#![no_main]
#![no_std]

mod context;
mod ecall;
mod print;
mod timer_interrupt;
mod trap_handler;

core::arch::global_asm!(
    r#"
    .section .text.entry
    .globl _start
    _start:
        la sp, _stack_top
        j kmain
    "#
);

unsafe extern "C" {
    static _kernel_end: u8;
    static _stack_top: u8;
}

#[global_allocator]
pub static HEAP: buddy_system_allocator::LockedHeap<32> =
    buddy_system_allocator::LockedHeap::<32>::empty();

#[unsafe(no_mangle)]
extern "C" fn kmain(_hartid: usize, _device_tree_binary: usize) -> ! {
    let kernel_end_address = core::ptr::addr_of!(_kernel_end) as usize;
    unsafe {
        let mut heap = HEAP.lock();
        heap.add_to_heap(kernel_end_address, kernel_end_address + 33554432);
        drop(heap);
    };

    use sg200x_bsp::sdmmc::{PowerLevel, Sdmmc};
    use sg200x_bsp::soc::{SD_DRIVER_BASE, TOP_BASE};
    let sdmmc = unsafe { Sdmmc::new(SD_DRIVER_BASE, TOP_BASE) };
    sdmmc.init().unwrap();
    let buf = "Hello World!".as_bytes();
    sdmmc.write_block(10000000, &buf).unwrap();
    timer_interrupt::set_time_quanta(1_000_000);
    unsafe {
        use riscv::{
            interrupt,
            register::{self, stvec},
        };
        register::stvec::write(riscv::register::stvec::Stvec::new(
            trap_handler::entry::trap_handler_entry as *const u8 as usize,
            stvec::TrapMode::Direct,
        ));
        interrupt::enable();
        interrupt::enable_interrupt(interrupt::Interrupt::SupervisorTimer);
        interrupt::enable_interrupt(interrupt::Interrupt::SupervisorExternal);
    }
    timer_interrupt::update_timer();
    loop {
        riscv::asm::wfi();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print::print!("{}", info);
    let _ = sbi::system_reset::system_reset(
        sbi::system_reset::ResetType::Shutdown,
        sbi::system_reset::ResetReason::SystemFailure,
    );
    loop {}
}
