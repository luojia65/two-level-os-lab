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
