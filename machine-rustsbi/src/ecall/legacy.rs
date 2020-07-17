use super::SbiRet;
use crate::legacy_stdio::{legacy_stdio_getchar, legacy_stdio_putchar};
use crate::hart_mask::HartMask;
use crate::ipi::{send_ipi_many, max_hart_id};

pub fn console_putchar(param0: usize) -> SbiRet {
    let ch = (param0 & 0xff) as u8;
    legacy_stdio_putchar(ch);
    SbiRet::ok(0) // todo
}

pub fn console_getchar() -> SbiRet {
    let ch = legacy_stdio_getchar();
    SbiRet::ok(ch as usize)
}

pub fn send_ipi(hart_mask_addr: usize) -> SbiRet {
    // note(unsafe): if any load fault, should be handled by user or supervisor
    let hart_mask = unsafe {
        HartMask::from_addr(hart_mask_addr, max_hart_id())
    };
    send_ipi_many(hart_mask);
    SbiRet::ok(0)
}
