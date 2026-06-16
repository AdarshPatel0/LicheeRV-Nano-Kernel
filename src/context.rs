#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Context {
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,

    pub t: [usize; 7],
    pub a: [usize; 8],
    pub s: [usize; 12],

    pub sstatus: usize,
    pub sepc: usize,
}