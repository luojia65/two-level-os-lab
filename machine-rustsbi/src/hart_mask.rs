use core::mem::size_of;

/// Hart mask structure reference
#[derive(Debug, Clone)]
pub struct HartMask {
    bit_vector: *const usize,
    max_hart_id: usize,
}

impl HartMask {
    /// Construct a reference to a hart mask structure.
    /// 
    /// Caller should provide from its address from supervisor level, 
    /// and a maximum hart number for maximum hart limit.
    /// 
    /// # Unsafety
    /// 
    /// Caller must ensure all usize values in the bit vector is accessible.
    pub unsafe fn from_addr(vaddr: usize, max_hart_id: usize) -> HartMask {
        HartMask {
            bit_vector: vaddr as *const usize,
            max_hart_id,
        }
    }

    /// Check if the `hart_id` is included in this hart mask structure.
    pub fn has_bit(&self, hart_id: usize) -> bool {
        assert!(hart_id <= self.max_hart_id);
        let (i, j) = split_index_usize(hart_id);
        let cur_vector = unsafe { get_vaddr_usize(self.bit_vector.add(i)) };
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
    // todo: 32 bit systems
    llvm_asm!("
        li      t0, (1 << 17)
        mv      t1, $1
        csrrs   t0, mstatus, t0
        ld      t1, 0(t1)
        csrw    mstatus, t0
        mv      $0, t1
    "
        :"=r"(ans) 
        :"r"(vaddr_ptr)
        :"t0", "t1");
    ans
}
