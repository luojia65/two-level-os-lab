#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::alloc::Layout;
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

use riscv::register::{mhartid};

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

#[export_name = "_mp_hook"]
extern fn mp_hook() -> bool {
    mhartid::read() == 0
}

#[riscv_rt::entry]
fn main() -> ! {
    if mhartid::read() == 0 {
        extern "C" {
            static mut _sheap: u8;
            static _heap_size: u8;
        }
        let sheap = unsafe { &mut _sheap } as *mut _ as usize;
        let heap_size = unsafe { &_heap_size } as *const u8 as usize;
        unsafe {
            ALLOCATOR.lock().init(sheap, heap_size);
        }
        let p = pac::Peripherals::take().unwrap();
    
        let mut sysctl = p.SYSCTL.constrain();
        let fpioa = p.FPIOA.split(&mut sysctl.apb0);
        let gpiohs = p.GPIOHS.split();
        fpioa.io16.into_function(fpioa::GPIOHS0);
        let mut boot = gpiohs.gpiohs0.into_pull_up_input();
    
        // Configure clocks (TODO)
        let clocks = k210_hal::clock::Clocks::new();
    
        // Configure UART
        let serial = p.UARTHS.configure(115_200.bps(), &clocks);
        use machine_rustsbi::legacy_stdio::init_legacy_stdio_embedded_hal;
        init_legacy_stdio_embedded_hal(serial);
        
        println!("[rustsbi] Version 0.1.0");

        println!("{}", machine_rustsbi::LOGO);
        println!("[rustsbi] Kernel entry: 0x80200000");
    }
    loop {}
}
