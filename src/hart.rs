use core::{alloc::Layout, arch::naked_asm};

use riscv::{
    interrupt,
    register::stvec::{self, Stvec, TrapMode},
};

use crate::{HART_STACK_SIZE, TIME_QUANTA, print::println, trap_handler::trap_handler_entry};
use alloc::alloc::alloc;

#[repr(C)]
pub struct HartInfo {
    pub hart_id: usize,
    pub current_thread_id: Option<usize>,
}

#[unsafe(naked)]
pub extern "C" fn hart_startup_entry() {
    naked_asm!(
        "
        csrw sscratch, a1
        mv sp, a1
        mv a0, a1
        call hart_startup
        "
    )
}

#[unsafe(no_mangle)]
extern "C" fn hart_startup(hart_id: usize) -> ! {
    unsafe {
        stvec::write(Stvec::new(trap_handler_entry as *const u8 as usize, TrapMode::Direct));
        interrupt::enable();
        sbi::timer::set_timer(riscv::register::time::read64() + TIME_QUANTA).unwrap();
        interrupt::enable_interrupt(interrupt::Interrupt::SupervisorTimer);
        interrupt::enable_interrupt(interrupt::Interrupt::SupervisorExternal);
    }
    println!("{}", hart_id);
    loop {
        riscv::asm::wfi();
    }
}

pub fn create_hart_stack(hart_id: usize) -> usize {
    unsafe {
        let hart_stack_ptr = alloc(Layout::from_size_align(HART_STACK_SIZE, 16).unwrap());
        let hart_stack_top = hart_stack_ptr as usize + HART_STACK_SIZE - size_of::<HartInfo>();
        let hart_info_ptr = hart_stack_top as *mut HartInfo;
        *hart_info_ptr = HartInfo { hart_id, current_thread_id: None };
        return hart_stack_top;
    }
}

pub fn get_hart_info() -> &'static mut HartInfo {
    let hart_info_address = riscv::register::sscratch::read();
    unsafe { &mut *(hart_info_address as *mut HartInfo) }
}
