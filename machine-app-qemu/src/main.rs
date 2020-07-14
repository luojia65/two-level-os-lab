#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]

mod hal;

use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;
use core::alloc::Layout;

use machine_rustsbi::legacy_stdio::init_legacy_stdio_embedded_hal;
use machine_rustsbi::println;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    loop {}
}

#[export_name = "_start"]
#[link_section = ".text.entry"] // this is stable
#[naked]
fn main() -> ! {
    unsafe { llvm_asm!("
        la sp, _stack_start
    ") };
    extern { 
        static mut _sheap: u8;
        static _heap_size: u8;
    }
    let sheap = unsafe { &mut _sheap } as *mut _ as usize;
    let heap_size = unsafe { &_heap_size } as *const u8 as usize;
    unsafe {
        ALLOCATOR.lock().init(sheap, heap_size);
    }

    let serial = hal::Ns16550a::new(0x10000000, 0, 11_059_200, 115200);

    unsafe { init_legacy_stdio_embedded_hal(serial); }

    println!("正在启动RustSBI……");
    
    unsafe { llvm_asm!("
    .option push
    .option norelax
1:
    auipc ra, %pcrel_hi(1f)
    ld ra, %pcrel_lo(1b)(ra)
    jr ra
    .align  3
1:
    .dword 0x80200000
.option pop
    ") };

    loop {}
}
