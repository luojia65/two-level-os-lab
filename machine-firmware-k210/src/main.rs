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

#[riscv_rt::entry]
fn main() -> ! {
    loop {}
}
