//! 这个模块的两个宏应该公开
//! 如果制造实例的时候，给定了stdout，那么就会打印到这个stdout里面
use embedded_hal::serial::{Read, Write};
use nb::block;

/// Legacy standard input/output
pub trait LegacyStdio: Send {
    /// Get a character from legacy stdin
    fn getchar(&mut self) -> u8;
    /// Put a character into legacy stdout
    fn putchar(&mut self, ch: u8);
}

/// Use serial in `embedded-hal` as legacy standard input/output
struct EmbeddedHalSerial<T> {
    inner: T,
}

impl<T> EmbeddedHalSerial<T> {
    /// Create a wrapper with a value
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<'a, T: Send> LegacyStdio for EmbeddedHalSerial<&'a mut T>
where
    T: Read<u8> + Write<u8>,
{
    fn getchar(&mut self) -> u8 {
        // 直接调用embedded-hal里面的函数
        // 关于unwrap：因为这个是legacy函数，这里没有详细的处理流程，就panic掉
        block!(self.inner.try_read()).ok().unwrap()
    }

    fn putchar(&mut self, ch: u8) {
        // 直接调用函数写一个字节
        block!(self.inner.try_write(ch)).ok();
        // 写一次flush一次，因为是legacy，就不考虑效率了
        block!(self.inner.try_flush()).ok();
    }
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref LEGACY_STDIO: Mutex<Option<Box<dyn LegacyStdio>>> = 
        Mutex::new(None);
}

#[doc(hidden)] // use through a macro
pub unsafe fn init_legacy_stdio_embedded_hal<T: Read<u8> + Write<u8> + Send + 'static>(serial: &mut T) {
    let serial = EmbeddedHalSerial::new(&mut *(serial as *mut _));
    *LEGACY_STDIO.lock() = Some(Box::new(serial));
}

pub(crate) fn legacy_stdio_putchar(ch: u8) {
    if let Some(stdio) = LEGACY_STDIO.lock().as_mut() {
        stdio.putchar(ch)
    }
}

pub(crate) fn legacy_stdio_getchar() -> u8 {
    if let Some(stdio) = LEGACY_STDIO.lock().as_mut() {
        stdio.getchar()
    } else {
        0 // default: always return 0
    }
}
