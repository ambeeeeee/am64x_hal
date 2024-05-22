#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    bcdma_ring_ba_lo: BcdmaRingBaLo,
    bcdma_ring_ba_hi: BcdmaRingBaHi,
    bcdma_ring_size: BcdmaRingSize,
}
impl RegisterBlock {
    #[doc = "0x40 - The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn bcdma_ring_ba_lo(&self) -> &BcdmaRingBaLo {
        &self.bcdma_ring_ba_lo
    }
    #[doc = "0x44 - The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn bcdma_ring_ba_hi(&self) -> &BcdmaRingBaHi {
        &self.bcdma_ring_ba_hi
    }
    #[doc = "0x48 - The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn bcdma_ring_size(&self) -> &BcdmaRingSize {
        &self.bcdma_ring_size
    }
}
#[doc = "BCDMA_RING_BA_LO (rw) register accessor: The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ring_ba_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ring_ba_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ring_ba_lo`]
module"]
#[doc(alias = "BCDMA_RING_BA_LO")]
pub type BcdmaRingBaLo = crate::Reg<bcdma_ring_ba_lo::BcdmaRingBaLoSpec>;
#[doc = "The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod bcdma_ring_ba_lo;
#[doc = "BCDMA_RING_BA_HI (rw) register accessor: The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ring_ba_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ring_ba_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ring_ba_hi`]
module"]
#[doc(alias = "BCDMA_RING_BA_HI")]
pub type BcdmaRingBaHi = crate::Reg<bcdma_ring_ba_hi::BcdmaRingBaHiSpec>;
#[doc = "The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod bcdma_ring_ba_hi;
#[doc = "BCDMA_RING_SIZE (rw) register accessor: The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ring_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ring_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ring_size`]
module"]
#[doc(alias = "BCDMA_RING_SIZE")]
pub type BcdmaRingSize = crate::Reg<bcdma_ring_size::BcdmaRingSizeSpec>;
#[doc = "The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod bcdma_ring_size;
