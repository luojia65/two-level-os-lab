#![no_std]

extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub mod legacy_stdio;
mod ecall;

const SBI_SPEC_MAJOR: usize = 0;
const SBI_SPEC_MINOR: usize = 2;

pub use ecall::handle_ecall as ecall;
