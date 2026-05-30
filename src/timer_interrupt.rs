static mut TIME_QUANTA: u64 = 0;

pub fn update_timer() {
    let time = riscv::register::time::read64();
    sbi::timer::set_timer(time + unsafe { TIME_QUANTA }).unwrap();
    return;
}

pub fn set_time_quanta(time_quanta: u64) {
    unsafe {
        TIME_QUANTA = time_quanta;
    }
}