// 这部分其实是运行时提供的，不应该做到实现库里面

pub struct Clint {
    base: *mut u8,
    hartid: usize,
}

impl Clint {
    pub fn new(base: *mut u8, hartid: usize) -> Clint {
        Clint { base, hartid }
    }

    pub fn setup_leader(&mut self) {
        // Setup mtime
        unsafe {
            core::ptr::write_volatile(self.base.offset(0xbff8) as *mut u64, 0);
        }
    }

    pub fn setup(&mut self) {
        // Writes timecmp to no timer
        unsafe {
            core::ptr::write_volatile(
                (self.base.offset(0x4000) as *mut u64).offset(self.hartid as isize),
                core::u64::MAX >> 4, // Fix QEMU timer loop
            );
        }

        // Clears all software interrupts for current HART
        self.clear_soft();
    }

    pub fn set_timer(&mut self, instant: u64) {
        unsafe {
            core::ptr::write_volatile(
                (self.base.offset(0x4000) as *mut u64).offset(self.hartid as isize),
                instant,
            );
        }
    }

    pub fn send_soft(&mut self, target: usize) {
        unsafe {
            core::ptr::write_volatile((self.base as *mut u32).offset(target as isize), 1);
        }
    }

    pub fn clear_soft(&mut self) {
        unsafe {
            core::ptr::write_volatile((self.base as *mut u32).offset(self.hartid as isize), 0);
        }
    }
}
