#![no_std]
#![feature(llvm_asm)]

extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub mod legacy_stdio;
mod ipi;
mod ecall;
mod hart_mask;

const SBI_SPEC_MAJOR: usize = 0;
const SBI_SPEC_MINOR: usize = 2;

const IMPL_ID_RUSTSBI: usize = 4;
const RUSTSBI_VERSION: usize = 1; // todo: read from env!("CARGO_PKG_VERSION")

pub use ecall::handle_ecall as ecall;
pub use hart_mask::HartMask;
pub use ipi::{Ipi, init_ipi};

pub const LOGO: &'static str = include_str!("logo.txt");
