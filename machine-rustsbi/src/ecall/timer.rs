use super::SbiRet;

const FUNCTION_TIMER_SET_TIMER: usize = 0x0;

#[inline]
pub fn handle_ecall_timer(function: usize, param0: usize) -> SbiRet {
    match function {
        FUNCTION_TIMER_SET_TIMER => set_timer(param0),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
fn set_timer(time_value: usize) -> SbiRet {
    if crate::timer::set_timer(time_value as u64) { // todo!!!!
        SbiRet::ok(0)
    } else {
        // should be probed with probe_extension
        SbiRet::not_supported()
    }
}
