pub trait Timer: Send {
    fn set_timer(&mut self, time_value: u64);
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref TIMER: Mutex<Option<Box<dyn Timer>>> = Mutex::new(None);
}

#[doc(hidden)] // use through a macro
pub fn init_timer<T: Timer + Send + 'static>(ipi: T) {
    *TIMER.lock() = Some(Box::new(ipi));
}

#[inline]
pub fn probe_timer() -> bool {
    TIMER.lock().as_ref().is_none()
}

#[inline]
pub(crate) fn set_timer(time_value: u64) -> bool {
    if let Some(timer) = TIMER.lock().as_mut() {
        timer.set_timer(time_value);
        true
    } else {
        false
    }
}
