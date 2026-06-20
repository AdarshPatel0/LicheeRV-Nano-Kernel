pub mod entry;

use crate::{context, ecall, print::println, timer_interrupt};

#[unsafe(no_mangle)]
extern "C" fn trap_handler(context: &mut context::Context) {
    let raw_trap = riscv::register::scause::read().cause();
    let trap: riscv::interrupt::Trap<riscv::interrupt::Interrupt, riscv::interrupt::Exception> = raw_trap.try_into().unwrap();
    match trap {
        riscv::interrupt::Trap::Interrupt(interrupt) => match interrupt {
            riscv::interrupt::Interrupt::SupervisorSoft => todo!(),
            riscv::interrupt::Interrupt::SupervisorTimer => {
                println!("Timer interrupt.");
                timer_interrupt::update_timer();
            }
            riscv::interrupt::Interrupt::SupervisorExternal => todo!(),
        },
        riscv::interrupt::Trap::Exception(exception) => match exception {
            riscv::interrupt::Exception::InstructionMisaligned => todo!(),
            riscv::interrupt::Exception::InstructionFault => todo!(),
            riscv::interrupt::Exception::IllegalInstruction => todo!(),
            riscv::interrupt::Exception::Breakpoint => todo!(),
            riscv::interrupt::Exception::LoadMisaligned => todo!(),
            riscv::interrupt::Exception::LoadFault => todo!(),
            riscv::interrupt::Exception::StoreMisaligned => todo!(),
            riscv::interrupt::Exception::StoreFault => todo!(),
            riscv::interrupt::Exception::UserEnvCall => {
                ecall::call(context);
            }
            riscv::interrupt::Exception::SupervisorEnvCall => {
                ecall::call(context);
            }
            riscv::interrupt::Exception::InstructionPageFault => todo!(),
            riscv::interrupt::Exception::LoadPageFault => todo!(),
            riscv::interrupt::Exception::StorePageFault => todo!(),
        },
    }
    return;
}