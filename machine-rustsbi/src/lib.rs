#![no_std]

extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub mod legacy_stdio;
pub mod ecall;

const SBI_SPEC_MAJOR: usize = 0;
const SBI_SPEC_MINOR: usize = 2;
