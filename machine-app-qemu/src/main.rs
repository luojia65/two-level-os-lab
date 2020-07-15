#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]
#![feature(global_asm)]

mod hal;

use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;
use core::alloc::Layout;

use machine_rustsbi::legacy_stdio::init_legacy_stdio_embedded_hal;
use machine_rustsbi::println;

use riscv::register::{
    mepc, mtvec::{self, TrapMode}, mstatus::{self, MPP}, 
    mcause::{self, Trap, Exception}
};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    loop {}
}

#[export_name = "_start"]
#[link_section = ".text.entry"] // this is stable
#[naked]
fn main() -> ! {
    unsafe { llvm_asm!("
        la sp, _stack_start

        csrr a2, mhartid
        lui t0, %hi(_hart_stack_size)
        add t0, t0, %lo(_hart_stack_size)
        bgtu a2, t0, _start_abort

    .ifdef __riscv_mul
        mul t0, a2, t0
    .else
        beqz a2, 2f  // Jump if single-hart
        mv t1, a2
        mv t2, t0
    1:
        add t0, t0, t2
        addi t1, t1, -1
        bnez t1, 1b
    2:
    .endif
        sub sp, sp, t0

        j _start_success
        
    _start_abort:
        wfi
        j _start_abort
    _start_success:
        
    ") };
    extern {
        fn _start_trap();
    }
    unsafe { 
        mtvec::write(_start_trap as usize, TrapMode::Direct);
    }
    extern { 
        static mut _sheap: u8;
        static _heap_size: u8;
    }
    let mhartid = riscv::register::mhartid::read();
    if mhartid == 0 {
        let sheap = unsafe { &mut _sheap } as *mut _ as usize;
        let heap_size = unsafe { &_heap_size } as *const u8 as usize;
        unsafe {
            ALLOCATOR.lock().init(sheap, heap_size);
        }
    
        let serial = hal::Ns16550a::new(0x10000000, 0, 11_059_200, 115200);
    
        unsafe { init_legacy_stdio_embedded_hal(serial); }

        println!("[rustsbi] Version 0.1.0");
        
        println!(
r#".______       __    __       _______.___________.  _______..______    __
|   _  \     |  |  |  |     /       |           | /       ||   _  \  |  |
|  |_)  |    |  |  |  |    |   (----`---|  |----`|   (----`|  |_)  | |  |
|      /     |  |  |  |     \   \       |  |      \   \    |   _  <  |  |
|  |\  \----.|  `--'  | .----)   |      |  |  .----)   |   |  |_)  | |  |
| _| `._____| \______/  |_______/       |__|  |_______/    |______/  |__|"#);
        // println!("[rustsbi] Kernel entry: 0x80200000");
    }// else wait for software interrupt
    extern {
        fn _s_mode_start();
    }
    unsafe {
        mepc::write(_s_mode_start as usize);
        mstatus::set_mpp(MPP::Supervisor);
    }
    unsafe { llvm_asm!("
        csrr    a0, mhartid
        li      a1, 0x2333333366666666 /* todo */

        call _enter_s_mode
    _s_mode_start:
        .option push
        .option norelax
    1:
        auipc ra, %pcrel_hi(1f)
        ld ra, %pcrel_lo(1b)(ra)
        jr ra
        .align  3
    1:
        .dword 0x80200000
    .option pop
    ") };

    loop {}
}

global_asm!("
    .equ REGBYTES, 8
    .macro STORE reg, offset
        sd  \\reg, \\offset*REGBYTES(sp)
    .endm
    .macro LOAD reg, offset
        ld  \\reg, \\offset*REGBYTES(sp)
    .endm

    .section .text
    .global _start_trap
    .p2align 2
_start_trap:
    csrrw   sp, mscratch, sp
    bnez    sp, _not_from_m_boot
    csrrw   sp, mscratch, zero
_not_from_m_boot:

    addi    sp, sp, -16 * REGBYTES

    STORE   ra, 0
    STORE   t0, 1
    STORE   t1, 2
    STORE   t2, 3
    STORE   t3, 4
    STORE   t4, 5
    STORE   t5, 6
    STORE   t6, 7
    STORE   a0, 8
    STORE   a1, 9
    STORE   a2, 10
    STORE   a3, 11
    STORE   a4, 12
    STORE   a5, 13
    STORE   a6, 14
    STORE   a7, 15

    mv      a0, sp
    call    _start_trap_rust

    LOAD    ra, 0
    LOAD    t0, 1
    LOAD    t1, 2
    LOAD    t2, 3
    LOAD    t3, 4
    LOAD    t4, 5
    LOAD    t5, 6
    LOAD    t6, 7
    LOAD    a0, 8
    LOAD    a1, 9
    LOAD    a2, 10
    LOAD    a3, 11
    LOAD    a4, 12
    LOAD    a5, 13
    LOAD    a6, 14
    LOAD    a7, 15

    addi    sp, sp, 16 * REGBYTES

    .globl _enter_s_mode
_enter_s_mode:
    csrrw   sp, mscratch, sp
    mret
");

#[allow(unused)]
struct TrapFrame {
    ra: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
}

#[export_name = "_start_trap_rust"]
extern fn start_trap_rust(trap_frame: &mut TrapFrame) {
    let cause = mcause::read().cause();
    if cause == Trap::Exception(Exception::SupervisorEnvCall) {
        let params = [trap_frame.a0, trap_frame.a1, trap_frame.a2, trap_frame.a3];
        // 调用rust_sbi库的处理函数
        let ans = machine_rustsbi::ecall(trap_frame.a7, trap_frame.a6, params);
        // 把返回值送还给TrapFrame
        trap_frame.a0 = ans.error;
        trap_frame.a1 = ans.value;
    } else {
        loop {} // cannot handle this
    }
}
