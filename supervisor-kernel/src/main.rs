#![no_std]
#![no_main]
#![feature(global_asm)]

extern crate alloc;
use linked_list_allocator::LockedHeap;
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
const HEAP_SIZE: usize = 0x100_0000;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

use riscv_sbi::println;

#[cfg(target_pointer_width = "64")]
riscv_sbi_rt::boot_page_sv39! {
    (0xffffffff_80000000 => 0x00000000_80000000, rwx);
    (0xffffffff_00000000 => 0x00000000_00000000, rwx);
    (0x00000000_80000000 => 0x00000000_80000000, rwx);
}

#[riscv_sbi_rt::entry]
fn main(hartid: usize, dtb_pa: usize) {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, HEAP_SIZE);
    }
    println!("[Kernel] hartid: {}, dtb_pa: 0x{:x}", hartid, dtb_pa);
    println!("操作系统启动！")
}
