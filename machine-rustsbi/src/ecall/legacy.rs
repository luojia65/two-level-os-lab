use super::SbiRet;
use crate::legacy_stdio::{legacy_stdio_getchar, legacy_stdio_putchar};

pub fn console_putchar(param0: usize) -> SbiRet {
    let ch = (param0 & 0xff) as u8;
    legacy_stdio_putchar(ch);
    SbiRet::ok(0) // todo
}

pub fn console_getchar() -> SbiRet {
    let ch = legacy_stdio_getchar();
    SbiRet::ok(ch as usize)
}

pub fn send_ipi(hart_mask_ptr: usize) -> SbiRet {
    // todo: wrap
    let mut mask: usize;
    unsafe { llvm_asm!("
        li      t0, (1 << 17)
        mv      t1, $1
        csrrs   t0, mstatus, t0
        lw      t1, 0(t1)
        csrw    mstatus, t0
        mv      $0, t1
    "
        :"=r"(mask) 
        :"r"(hart_mask_ptr)
        :"t0", "t1") 
    };
    for i in 0..64 { 
        if mask & (1 << i) != 0 {
            unsafe { 
                core::ptr::write_volatile((0x2000000 as *mut u32).offset(i), 1);
            }
        }
    }
    SbiRet::ok(0)
}
