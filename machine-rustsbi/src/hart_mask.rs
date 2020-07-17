use core::mem::size_of;

pub struct HartMask {
    bit_vector: *const usize,
    max_hart_id: usize,
}

impl HartMask {
    // note(unsafe): must ensure all usize values in the bit vector is accessible
    pub unsafe fn from_addr(vaddr: usize, max_hart_id: usize) -> HartMask {
        HartMask {
            bit_vector: vaddr as *const usize,
            max_hart_id,
        }
    }

    pub fn has_bit(&self, hart_id: usize) -> bool {
        assert!(hart_id <= self.max_hart_id);
        let (i, j) = split_index_usize(hart_id);
        let cur_vector = unsafe { get_vaddr_usize(self.bit_vector.offset(i as isize)) };
        cur_vector & (1 << j) != 0
    }
}

#[inline]
fn split_index_usize(index: usize) -> (usize, usize) {
    let bits_in_usize = size_of::<usize>() * 8;
    (index / bits_in_usize, index % bits_in_usize)
}

#[inline]
unsafe fn get_vaddr_usize(vaddr_ptr: *const usize) -> usize {
    let mut ans: usize;
    llvm_asm!("
        li      t0, (1 << 17)
        mv      t1, $1
        csrrs   t0, mstatus, t0
        lw      t1, 0(t1)
        csrw    mstatus, t0
        mv      $0, t1
    "
        :"=r"(ans) 
        :"r"(vaddr_ptr)
        :"t0", "t1");
    ans
}
