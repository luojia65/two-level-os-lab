use crate::hart_mask::HartMask;

pub trait Ipi: Send {
    fn max_hart_id(&self) -> usize;

    fn send_ipi_many(&mut self, hart_mask: HartMask);
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref LEGACY_STDIO: Mutex<Option<Box<dyn Ipi>>> =
        Mutex::new(None);
}

#[doc(hidden)] // use through a macro
pub fn init_ipi<T: Ipi + Send + 'static>(ipi: T) {
    *LEGACY_STDIO.lock() = Some(Box::new(ipi));
}

pub(crate) fn send_ipi_many(hart_mask: HartMask) {
    if let Some(ipi) = LEGACY_STDIO.lock().as_mut() {
        ipi.send_ipi_many(hart_mask)
    }
}

pub(crate) fn max_hart_id() -> usize {
    loop {
        if let Some(ipi) = LEGACY_STDIO.lock().as_ref() {
            return ipi.max_hart_id();
        }
    }
}
