// 这部分其实是运行时提供的，不应该做到实现库里面

pub struct Clint {
    base: usize,
}

impl Clint {
    pub fn new(base: *mut u8) -> Clint {
        Clint { base: base as usize }
    }

    pub fn setup_leader(&mut self) {
        // Setup mtime
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile(base.offset(0xbff8) as *mut u64, 0);
        }
    }

    pub fn setup(&mut self, hart_id: usize) {
        // Writes timecmp to no timer
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile(
                (base.offset(0x4000) as *mut u64).offset(hart_id as isize),
                core::u64::MAX >> 4, // Fix QEMU timer loop
            );
        }

        // Clears all software interrupts for current HART
        self.clear_soft(hart_id);
    }

    pub fn set_timer(&mut self, hart_id: usize, instant: u64) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile(
                (base.offset(0x4000) as *mut u64).offset(hart_id as isize),
                instant,
            );
        }
    }

    pub fn send_soft(&mut self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base as *mut u32).offset(hart_id as isize), 1);
        }
    }

    pub fn clear_soft(&mut self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base as *mut u32).offset(hart_id as isize), 0);
        }
    }
}

use machine_rustsbi::{Ipi, HartMask};   

impl Ipi for Clint {
    fn max_hart_id(&self) -> usize {
        extern "C" {
            static _max_hart_id: u8;
        }
        unsafe { &_max_hart_id as *const _ as usize }
    }

    fn send_ipi_many(&mut self, hart_mask: HartMask) {
        for i in 0..=self.max_hart_id() {
            if hart_mask.has_bit(i) {
                self.send_soft(i);
            }
        }
    }
}
