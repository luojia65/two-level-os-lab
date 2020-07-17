use super::SbiRet;
use crate::hart_mask::HartMask;
use crate::ipi::{max_hart_id, send_ipi_many};
use crate::legacy_stdio::{legacy_stdio_getchar, legacy_stdio_putchar}; 
use riscv::register::{mip, mie};

#[inline]
pub fn console_putchar(param0: usize) -> SbiRet {
    let ch = (param0 & 0xff) as u8;
    legacy_stdio_putchar(ch);
    SbiRet::ok(0) // todo
}

#[inline]
pub fn console_getchar() -> SbiRet {
    let ch = legacy_stdio_getchar();
    SbiRet::ok(ch as usize)
}

#[inline]
pub fn send_ipi(hart_mask_addr: usize) -> SbiRet {
    // note(unsafe): if any load fault, should be handled by user or supervisor
    let hart_mask = unsafe { HartMask::from_addr(hart_mask_addr, max_hart_id()) };
    send_ipi_many(hart_mask);
    SbiRet::ok(0)
}

#[inline]
pub fn set_timer(time_value: usize) -> SbiRet {   
    crate::timer::set_timer(time_value as u64); // todo: 32 bit

    let mtip = mip::read().mtimer();
    if mtip {
        unsafe {
            mie::clear_mtimer();
            mip::set_stimer();
        }
    } else {
        unsafe {
            mie::set_mtimer();
            mip::clear_stimer();
        }
    }
    SbiRet::ok(0)
}

