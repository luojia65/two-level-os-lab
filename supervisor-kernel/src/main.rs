#![no_std]
#![no_main]
#![feature(global_asm)]

extern crate alloc;
use linked_list_allocator::LockedHeap;
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
const HEAP_SIZE: usize = 512 * 1024;

const INTERVAL: u64 = 10_000_000;

use riscv::register::{sie, sip, sstatus, time};
use riscv_sbi::{base, legacy, println, HartMask};
use riscv_sbi_rt::{heap_start, max_hart_id};

#[cfg(target_pointer_width = "64")]
riscv_sbi_rt::boot_page_sv39! {
    (0xffffffff_80000000 => 0x00000000_80000000, rwx);
    (0xffffffff_00000000 => 0x00000000_00000000, rwx);
    (0x00000000_80000000 => 0x00000000_80000000, rwx);
}

#[export_name = "_mp_hook"]
fn mp_hook(hartid: usize, _dtb_pa: usize) -> bool {
    if hartid == 0 {
        true
    } else {
        unsafe {
            sie::set_ssoft();
            loop {
                riscv::asm::wfi();
                if sip::read().ssoft() {
                    break;
                }
            }
            sie::clear_ssoft();
        }
        false
    }
}

#[riscv_sbi_rt::entry]
fn main(hartid: usize, dtb_pa: usize) {
    if hartid == 0 {
        println!("[Kernel] hartid: {}, dtb_pa: 0x{:x}", hartid, dtb_pa);
        println!("spec_version = {:?}", base::get_spec_version());
        println!("impl_id      = {:?}", base::get_impl_id());
        println!("impl_version = {:?}", base::get_impl_version());
        println!("mvendorid    = {:?}", base::get_mvendorid());
        println!("marchid      = {:?}", base::get_marchid());
        println!("mimpid       = {:?}", base::get_mimpid());
        unsafe {
            HEAP_ALLOCATOR.lock().init(heap_start() as usize, HEAP_SIZE);
        }
        let mut hart_mask = HartMask::all(max_hart_id());
        hart_mask.clear(0);
        legacy::send_ipi(hart_mask);
    }
    unsafe {
        sie::set_stimer();
        sstatus::set_sie();
    }
    println!("[Kernal] Hart {} is running!", hartid);
    legacy::set_timer(time::read64().wrapping_add(INTERVAL));
    loop {}
}

use riscv_sbi_rt::TrapFrame;

#[export_name = "SupervisorTimer"]
unsafe extern "C" fn supervisor_timer(context: &mut TrapFrame) -> *mut TrapFrame {
    static mut TICKS: usize = 0;
    legacy::set_timer(time::read64().wrapping_add(INTERVAL));
    TICKS += 1;
    println!("1 tick~");
    context as *mut _
}
