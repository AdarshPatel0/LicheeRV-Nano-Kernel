#![allow(unused)]

use core::alloc::Layout;

use alloc::{
    alloc::{alloc, dealloc},
    collections::vec_deque::VecDeque,
};
use slab::Slab;
use spin::Mutex;

use crate::{context::Context, hart::get_hart_info};

static THREADS: Mutex<Slab<Thread>> = Mutex::new(Slab::new());
static QUEUE: Mutex<VecDeque<usize>> = Mutex::new(VecDeque::new());

struct Thread {
    pub context: Context,
    pub stack_address: usize,
    pub stack_size: usize,
    pub status: ThreadStatus,
}

#[derive(PartialEq, Eq)]
pub enum ThreadStatus {
    Ready,
    Running,
    Blocking,
    Dead,
}

pub fn create_thread(entry: usize, privileged: bool, stack_size: usize, arguments: &[u8]) -> usize {
    assert!(stack_size.is_power_of_two());
    assert!(stack_size > 256);
    let mut threads = THREADS.lock();
    let mut queue = QUEUE.lock();

    let stack_address = unsafe { alloc(Layout::from_size_align(stack_size, 16).unwrap()) as usize };
    let stack_top = stack_address + stack_size - arguments.len();

    unsafe {
        core::ptr::copy(arguments.as_ptr(), stack_top as *mut u8, arguments.len());
    }

    let mut context = Context::default();
    context.sp = stack_top;
    context.a[0] = stack_top;
    context.sepc = entry;

    let mut sstatus = riscv::register::sstatus::read();

    if privileged {
        sstatus.set_spp(riscv::register::sstatus::SPP::Supervisor);
    } else {
        sstatus.set_spp(riscv::register::sstatus::SPP::User);
    }

    sstatus.set_spie(true);

    context.sstatus = sstatus.bits();

    let thread = Thread { context, stack_address, stack_size, status: ThreadStatus::Ready };

    let id = threads.insert(thread);
    queue.push_back(id);
    return id;
}

pub fn kill_thread(id: usize) -> bool {
    let mut threads = THREADS.lock();
    if let Some(thread) = threads.get_mut(id) {
        thread.status = ThreadStatus::Dead;
        return true;
    }
    return false;
}

pub fn cleanup_thread(id: usize) -> bool {
    let mut threads = THREADS.lock();
    match threads.get(id) {
        Some(thread) => {
            if thread.status == ThreadStatus::Dead {
                unsafe { dealloc(thread.stack_address as *mut u8, Layout::from_size_align(thread.stack_size, 16).unwrap()) }
                threads.remove(id);
                return true;
            }
            return false;
        }
        None => return false,
    }
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wait() {
    core::arch::naked_asm!(
        "
        wait_start:
            wfi
            j   wait_start
        "
    );
}

pub fn schedule(context: &mut Context) {
    let mut threads = THREADS.lock();
    let mut queue = QUEUE.lock();
    let hart_info = get_hart_info();
    if let Some(current_thread_id) = hart_info.current_thread_id {
        if let Some(current_thread) = threads.get_mut(current_thread_id) {
            queue.push_back(current_thread_id);
            current_thread.context = *context;
            if current_thread.status == ThreadStatus::Running {
                current_thread.status = ThreadStatus::Ready;
            }
        }
    }
    loop {
        if let Some(new_thread_id) = queue.pop_front() {
            if let Some(new_thread) = threads.get_mut(new_thread_id) {
                if new_thread.status == ThreadStatus::Ready {
                    *context = new_thread.context;
                    hart_info.current_thread_id = Some(new_thread_id);
                    new_thread.status = ThreadStatus::Running;
                    return;
                }
            }
        } else {
            hart_info.current_thread_id = None;
            context.sepc = wait as *const u8 as usize;
            let mut sstatus = riscv::register::sstatus::read();
            sstatus.set_spie(true);
            sstatus.set_spp(riscv::register::sstatus::SPP::Supervisor);
            context.sstatus = sstatus.bits();
            return;
        }
    }
}
