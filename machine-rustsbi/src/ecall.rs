//! 这个模块将会处理所有的SBI调用陷入
// 你应该在riscv-rt或其它中断处理函数里，调用这个模块的内容

mod base;
mod ipi;
mod legacy;
mod timer;

pub const EXTENSION_BASE: usize = 0x10;
pub const EXTENSION_TIMER: usize = 0x54494D45;
pub const EXTENSION_IPI: usize = 0x735049;
// const EXTENSION_RFENCE: usize = 0x52464E43;
// const EXTENSION_HSM: usize = 0x48534D;

const LEGACY_SET_TIMER: usize = 0x0;
const LEGACY_CONSOLE_PUTCHAR: usize = 0x01;
const LEGACY_CONSOLE_GETCHAR: usize = 0x02;
// const LEGACY_CLEAR_IPI: usize = 0x03;
const LEGACY_SEND_IPI: usize = 0x04;
// const LEGACY_REMOTE_FENCE_I: usize = 0x05;
// const LEGACY_REMOTE_SFENCE_VMA: usize = 0x06;
// const LEGACY_REMOTE_SFENCE_VMA_ASID: usize = 0x07;
// const LEGACY_SHUTDOWN: usize = 0x08;

/// Supervisor environment call handler function
///
/// You should call this function in your runtime's exception handler.
/// If the incoming exception is caused by supervisor `ecall`,
/// call this function with parameters extracted from trap frame.
/// After this function returns, store the return value into `a0` and `a1` parameters.
///
/// This function also adapts to the legacy functions.
/// If the supervisor called any of legacy function, the `a0` and `a1` parameter
/// is transferred to `SbiRet`'s error and value respectively.
/// So you should store the result into `a0` and `a1` in any function calls including legacy functions.
///
/// # Example
///
/// A typical usage:
///
/// ```no_run
/// #[exception]
/// fn handle_exception(ctx: &mut TrapFrame) {
///     if mcause::read().cause() == Trap::Exception(Exception::SupervisorEnvCall) {
///         let params = [ctx.a0, ctx.a1, ctx.a2, ctx.a3];
///         let ans = rustsbi::ecall(ctx.a7, ctx.a6, params);
///         ctx.a0 = ans.error;
///         ctx.a1 = ans.value;
///         mepc::write(mepc::read().wrapping_add(4));
///     }
///     // other conditions..
/// }
/// ```
///
/// Do not forget to advance `mepc` by 4 after an ecall is handled.
/// This skips the `ecall` instruction itself which is 4-byte long in all conditions.
#[inline]
pub fn handle_ecall(extension: usize, function: usize, param: [usize; 4]) -> SbiRet {
    match extension {
        EXTENSION_BASE => base::handle_ecall_base(function, param[0]),
        EXTENSION_TIMER => match () {
            #[cfg(target_pointer_width = "64")]
            () => timer::handle_ecall_timer_64(function, param[0]),
            #[cfg(target_pointer_width = "32")]
            () => timer::handle_ecall_timer_32(function, param[0], param[1]),
        },
        EXTENSION_IPI => ipi::handle_ecall_ipi(function, param[0], param[1]),
        LEGACY_SET_TIMER => match () {
            #[cfg(target_pointer_width = "64")]
            () => legacy::set_timer_64(param[0]),
            #[cfg(target_pointer_width = "32")]
            () => legacy::set_timer_32(param[0], param[1]),
        }
        .legacy_void(param[0], param[1]),
        LEGACY_CONSOLE_PUTCHAR => legacy::console_putchar(param[0]).legacy_void(param[0], param[1]),
        LEGACY_CONSOLE_GETCHAR => legacy::console_getchar().legacy_return(param[1]),
        LEGACY_SEND_IPI => legacy::send_ipi(param[0]).legacy_void(param[0], param[1]),
        _ => SbiRet::not_supported(),
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

const SBI_SUCCESS: usize = 0;
// const SBI_ERR_FAILED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-1));
const SBI_ERR_NOT_SUPPORTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-2));
// const SBI_ERR_INVALID_PARAM: usize = usize::from_ne_bytes(isize::to_ne_bytes(-3));
// const SBI_ERR_DENIED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-4));
// const SBI_ERR_INVALID_ADDRESS: usize = usize::from_ne_bytes(isize::to_ne_bytes(-5));
// const SBI_ERR_ALREADY_AVAILABLE: usize = usize::from_ne_bytes(isize::to_ne_bytes(-6));

impl SbiRet {
    pub(crate) fn ok(value: usize) -> SbiRet {
        SbiRet {
            error: SBI_SUCCESS,
            value,
        }
    }
    pub(crate) fn not_supported() -> SbiRet {
        SbiRet {
            error: SBI_ERR_NOT_SUPPORTED,
            value: 0,
        }
    }
    // only used for legacy where a0, a1 return value is not modified
    pub(crate) fn legacy_void(self, a0: usize, a1: usize) -> SbiRet {
        SbiRet {
            error: a0,
            value: a1,
        }
    }
    pub(crate) fn legacy_return(self, a1: usize) -> SbiRet {
        SbiRet {
            error: self.error,
            value: a1,
        }
    }
}
