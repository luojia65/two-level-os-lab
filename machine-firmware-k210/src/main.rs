#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

use core::alloc::Layout;
use core::panic::PanicInfo;

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
    loop {}
}
