//! 这个模块将会处理所有的SBI调用陷入
// 你应该在riscv-rt或其它中断处理函数里，调用这个模块的内容

mod base;
mod legacy;

const EXTENSION_BASE: usize = 0x10;

// const LEGACY_SET_TIMER: usize = 0x0;
const LEGACY_CONSOLE_PUTCHAR: usize = 0x01;
const LEGACY_CONSOLE_GETCHAR: usize = 0x02;
// const LEGACY_CLEAR_IPI: usize = 0x03;
// const LEGACY_SEND_IPI: usize = 0x04;
// const LEGACY_REMOTE_FENCE_I: usize = 0x05;
// const LEGACY_REMOTE_SFENCE_VMA: usize = 0x06;
// const LEGACY_REMOTE_SFENCE_VMA_ASID: usize = 0x07;
// const LEGACY_SHUTDOWN: usize = 0x08;

/// You should call this function in your runtime's exception handler.
/// If the incoming exception is caused by `ecall`,
/// call this function with parameters extracted from trap frame.
#[inline]
pub fn handle_ecall(extension: usize, function: usize, param: [usize; 4]) -> SbiRet {
    match extension {
        EXTENSION_BASE => base::handle_ecall_base(function, param[0]),
        LEGACY_CONSOLE_PUTCHAR => legacy::console_putchar(param[0]),
        LEGACY_CONSOLE_GETCHAR => legacy::console_getchar(),
        _ => todo!(),
    }
}

/// Returned by handle_ecall function
/// After `handle_ecall` finished, you should save returned `error` in `a0`, and `value` in `a1`.
#[repr(C)]
pub struct SbiRet {
    /// Error number
    pub error: usize,
    /// Result value
    pub value: usize,
}

impl SbiRet {
    pub(crate) fn ok(value: usize) -> SbiRet {
        SbiRet { error: 0, value }
    }
}
