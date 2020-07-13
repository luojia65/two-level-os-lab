#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

mod hal;

use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;
use core::alloc::Layout;
use spin::Mutex;

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

lazy_static::lazy_static! {
    static ref SERIAL: Mutex<hal::Ns16550a> = Mutex::new(
        hal::Ns16550a::new(0x10000000, 0, 11_059_200, 115200)
    );
}

#[export_name = "_start"]
#[link_section = ".text.entry"] // this is stable
#[naked]
extern fn entry() -> ! {
    extern { 
        static _sheap: u8;
        static _heap_size: usize;
    }
    unsafe {
        ALLOCATOR.lock().init(&_sheap as *const _ as usize, _heap_size);
    }

    use machine_rustsbi::legacy_stdio::{init_legacy_stdio, EmbeddedHalSerial};
    
    let serial = EmbeddedHalSerial::new(&*SERIAL.lock());
    init_legacy_stdio(serial);
    
    loop {}
}
