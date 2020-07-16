use super::SbiRet;
use crate::legacy_stdio::{legacy_stdio_putchar, legacy_stdio_getchar};

pub fn console_putchar(param0: usize) -> SbiRet {
    let ch = (param0 & 0xff) as u8;
    legacy_stdio_putchar(ch);
    SbiRet::ok(0) // todo
}

pub fn console_getchar() -> SbiRet {
    let ch = legacy_stdio_getchar();
    SbiRet::ok(ch as usize)
}

pub fn send_ipi(hart_mask: usize) -> SbiRet {
    // send ipi to other harts
    unsafe {
        // core::ptr::write_volatile(
        //     (0x2000000 as *mut u32).offset(0), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(1), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(2), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(3), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(4), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(5), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(6), 1);
        core::ptr::write_volatile(
            (0x2000000 as *mut u32).offset(7), 1);
    };
    SbiRet::ok(0)
}
