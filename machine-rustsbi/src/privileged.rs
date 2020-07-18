/// Enter lower privilege from M code with given SBI parameters.
/// 
/// Before calling this function, you must write target start address into `mepc` register,
/// and write target privilege into `mstatus` register.
/// Call on all harts after the initialization process is finished.
/// 
/// After this function is called, the stack pointer register `sp` is swapped with `mscratch`,
/// and a `mret` is called to return to `mepc` address with target privilege.
///
/// # Unsafety
/// 
/// This function implictly returns to the program address with the address from `mepc` register.
/// Caller must ensure that the value in `mepc` is a valid program address.
/// Caller should also ensure that `mstatus.MPP` register bits contain valid target privilege level.
///
/// # Example 
///
/// ```rust
/// unsafe {
///     mepc::write(_s_mode_start as usize);
///     mstatus::set_mpp(MPP::Supervisor);
///     enter_privileged(mhartid::read(), 0x2333333366666666);
/// }
/// ```
#[inline]
pub unsafe fn enter_privileged(mhartid: usize, dtb_pa: usize) -> ! {
    llvm_asm!("
        csrrw   sp, mscratch, sp
        mret
    "::"{a0}"(mhartid), "{a1}"(dtb_pa));
    unreachable!()
}
