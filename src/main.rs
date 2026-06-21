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
extern "C" fn kmain(_argc: usize, argv: *const *const core::ffi::c_char) -> ! {
    let dtb_address_str = unsafe { core::ffi::CStr::from_ptr(*argv.add(1)) }.to_str().unwrap();

    let fdt_address = usize::from_str_radix(dtb_address_str, 16).unwrap();

    println!("Flattened device address: {:#x}", fdt_address);

    let fdt = unsafe {
        let device_tree_binary_header = core::slice::from_raw_parts(fdt_address as *const u32, 40);
        let total_size = device_tree_binary_header.get(1).unwrap();
        let device_tree_binary_data = core::slice::from_raw_parts(fdt_address as *const u8, *total_size as usize);
        fdt::Fdt::new(device_tree_binary_data).expect("Failed to parse full FDT")
    };

    let kernel_end_address = core::ptr::addr_of!(_kernel_end) as usize;
    let memory_region = fdt.memory().regions().next().unwrap();
    let base_address = memory_region.starting_address as usize;
    let size = memory_region.size.unwrap();

    unsafe {
        HEAP.force_unlock();
        HEAP.lock().add_to_heap(kernel_end_address, base_address + size);
    };

    println!("Heap initialized:\n\tstart: {:#x}\n\tend: {:#x}", kernel_end_address, base_address + size);

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
    timer_interrupt::update_timer();
    let sdmmc = {
        use sg200x_bsp::sdmmc::Sdmmc;
        use sg200x_bsp::soc::{SD_DRIVER_BASE, TOP_BASE};

        let sdmmc = unsafe { Sdmmc::new(SD_DRIVER_BASE, TOP_BASE) };
        sdmmc.init().unwrap();
        sdmmc
    };
    println!("sdmmc capacity: \n\tbytes: {}\n\tblocks: {}", sdmmc.card_capacity_bytes(), sdmmc.card_capacity_blocks());
    let mut mbr_data: [u8; 512] = [0; 512];
    println!("Reading MBR data...");
    match sdmmc.read_block_single(0, &mut mbr_data) {
        Ok(_) => {},
        Err(_) => {
            println!("Error: Cannot read block 0");
        },
    }
    println!("Getting MBR partitions...");
    let mbr = mbrs::Mbr::try_from_bytes(&mbr_data).unwrap();
    let partition_table = mbr.partition_table;
    for entry in partition_table.entries {
        if let Some(partition_information) = entry {
            println!("{:?}", partition_information.part_type());
        }
    }
    println!("Kernel initialization complete.");
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
