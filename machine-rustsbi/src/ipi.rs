use crate::hart_mask::HartMask;

/// Inter-processor interrupt support
pub trait Ipi: Send {
    /// Get the maximum hart id available by this IPI support module
    fn max_hart_id(&self) -> usize;
    /// Send an inter-processor interrupt to all the harts defined in `hart_mask`.
    /// 
    /// Interprocessor interrupts manifest at the receiving harts as the supervisor software interrupts.
    fn send_ipi_many(&mut self, hart_mask: HartMask);
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref IPI: Mutex<Option<Box<dyn Ipi>>> = Mutex::new(None);
}

#[doc(hidden)] // use through a macro
pub fn init_ipi<T: Ipi + Send + 'static>(ipi: T) {
    *IPI.lock() = Some(Box::new(ipi));
}

#[inline]
pub(crate) fn probe_ipi() -> bool {
    IPI.lock().as_ref().is_none()
}

pub(crate) fn send_ipi_many(hart_mask: HartMask) {
    if let Some(ipi) = IPI.lock().as_mut() {
        ipi.send_ipi_many(hart_mask)
    }
}

pub(crate) fn max_hart_id() -> usize {
    loop {
        if let Some(ipi) = IPI.lock().as_ref() {
            return ipi.max_hart_id();
        }
    }
}
