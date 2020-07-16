use super::SbiRet;

const FUNCTION_TIMER_SET_TIMER: usize = 0x0;

#[inline]
pub fn handle_ecall_timer(function: usize, param0: usize) -> SbiRet {
    match function {
        FUNCTION_TIMER_SET_TIMER => set_timer(param0),
        _ => unimplemented!(),
    }
}

#[inline]
fn set_timer(_time_value: usize) -> SbiRet {
    // todo: set memory mapped control register mtimecmp
    SbiRet::ok(0)
}
