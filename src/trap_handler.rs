use crate::{context, ecall};

#[unsafe(no_mangle)]
extern "C" fn trap_handler(context: &mut context::Context) {
    let raw_trap = riscv::register::scause::read().cause();
    let trap: riscv::interrupt::Trap<riscv::interrupt::Interrupt, riscv::interrupt::Exception> = raw_trap.try_into().unwrap();
    match trap {
        riscv::interrupt::Trap::Interrupt(interrupt) => match interrupt {
            riscv::interrupt::Interrupt::SupervisorSoft => todo!(),
            riscv::interrupt::Interrupt::SupervisorTimer => {
                let time = riscv::register::time::read64();
                sbi::timer::set_timer(time + crate::TIME_QUANTA).unwrap();
                return;
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

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trap_handler_entry() {
    core::arch::naked_asm!(
        r#"
        addi    sp,     sp,     -528

        sd      ra,     0*8(sp)
        sd      sp,     1*8(sp)
        sd      gp,     2*8(sp)
        sd      tp,     3*8(sp)

        sd      t0,     4*8(sp)
        sd      t1,     5*8(sp)
        sd      t2,     6*8(sp)
        sd      t3,     7*8(sp)
        sd      t4,     8*8(sp)
        sd      t5,     9*8(sp)
        sd      t6,     10*8(sp)

        sd      a0,     11*8(sp)
        sd      a1,     12*8(sp)
        sd      a2,     13*8(sp)
        sd      a3,     14*8(sp)
        sd      a4,     15*8(sp)
        sd      a5,     16*8(sp)
        sd      a6,     17*8(sp)
        sd      a7,     18*8(sp)

        sd      s0,     19*8(sp)
        sd      s1,     20*8(sp)
        sd      s2,     21*8(sp)
        sd      s3,     22*8(sp)
        sd      s4,     23*8(sp)
        sd      s5,     24*8(sp)
        sd      s6,     25*8(sp)
        sd      s7,     26*8(sp)
        sd      s8,     27*8(sp)
        sd      s9,     28*8(sp)
        sd      s10,    29*8(sp)
        sd      s11,    30*8(sp)

        csrr    t0,     sstatus
        sd      t0,     31*8(sp)
        csrr    t0,     sepc
        sd      t0,     32*8(sp)

        addi    t0,     sp,     528
        sd      t0,     1*8(sp)

        call    trap_handler

        ld      t0,     31*8(sp)
        csrw    sstatus,    t0

        ld      t0,     32*8(sp)
        csrw    sepc,   t0

        ld      ra,     0*8(sp)
        ld      gp,     2*8(sp)
        ld      tp,     3*8(sp)

        ld      t0,     4*8(sp)
        ld      t1,     5*8(sp)
        ld      t2,     6*8(sp)
        ld      t3,     7*8(sp)
        ld      t4,     8*8(sp)
        ld      t5,     9*8(sp)
        ld      t6,     10*8(sp)

        ld      a0,     11*8(sp)
        ld      a1,     12*8(sp)
        ld      a2,     13*8(sp)
        ld      a3,     14*8(sp)
        ld      a4,     15*8(sp)
        ld      a5,     16*8(sp)
        ld      a6,     17*8(sp)
        ld      a7,     18*8(sp)

        ld      s0,     19*8(sp)
        ld      s1,     20*8(sp)
        ld      s2,     21*8(sp)
        ld      s3,     22*8(sp)
        ld      s4,     23*8(sp)
        ld      s5,     24*8(sp)
        ld      s6,     25*8(sp)
        ld      s7,     26*8(sp)
        ld      s8,     27*8(sp)
        ld      s9,     28*8(sp)
        ld      s10,    29*8(sp)
        ld      s11,    30*8(sp)

        ld      sp,     1*8(sp)

        sret
        "#
    );
}
