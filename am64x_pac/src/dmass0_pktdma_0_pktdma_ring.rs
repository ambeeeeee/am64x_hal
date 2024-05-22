#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    pktdma_ring_ba_lo: PktdmaRingBaLo,
    pktdma_ring_ba_hi: PktdmaRingBaHi,
    pktdma_ring_size: PktdmaRingSize,
}
impl RegisterBlock {
    #[doc = "0x40 - The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn pktdma_ring_ba_lo(&self) -> &PktdmaRingBaLo {
        &self.pktdma_ring_ba_lo
    }
    #[doc = "0x44 - The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn pktdma_ring_ba_hi(&self) -> &PktdmaRingBaHi {
        &self.pktdma_ring_ba_hi
    }
    #[doc = "0x48 - The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
    #[inline(always)]
    pub const fn pktdma_ring_size(&self) -> &PktdmaRingSize {
        &self.pktdma_ring_size
    }
}
#[doc = "PKTDMA_RING_BA_LO (rw) register accessor: The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ring_ba_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ring_ba_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ring_ba_lo`]
module"]
#[doc(alias = "PKTDMA_RING_BA_LO")]
pub type PktdmaRingBaLo = crate::Reg<pktdma_ring_ba_lo::PktdmaRingBaLoSpec>;
#[doc = "The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod pktdma_ring_ba_lo;
#[doc = "PKTDMA_RING_BA_HI (rw) register accessor: The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ring_ba_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ring_ba_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ring_ba_hi`]
module"]
#[doc(alias = "PKTDMA_RING_BA_HI")]
pub type PktdmaRingBaHi = crate::Reg<pktdma_ring_ba_hi::PktdmaRingBaHiSpec>;
#[doc = "The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod pktdma_ring_ba_hi;
#[doc = "PKTDMA_RING_SIZE (rw) register accessor: The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ring_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ring_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ring_size`]
module"]
#[doc(alias = "PKTDMA_RING_SIZE")]
pub type PktdmaRingSize = crate::Reg<pktdma_ring_size::PktdmaRingSizeSpec>;
#[doc = "The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers."]
pub mod pktdma_ring_size;
