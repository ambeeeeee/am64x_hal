#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    pktdma_ringrt_rt_fdb: PktdmaRingrtRtFdb,
    _reserved1: [u8; 0x04],
    pktdma_ringrt_rt_focc: PktdmaRingrtRtFocc,
    _reserved2: [u8; 0x0ff4],
    pktdma_ringrt_rt_rdb: PktdmaRingrtRtRdb,
    _reserved3: [u8; 0x04],
    pktdma_ringrt_rt_rocc: PktdmaRingrtRtRocc,
}
impl RegisterBlock {
    #[doc = "0x10 - The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
    #[inline(always)]
    pub const fn pktdma_ringrt_rt_fdb(&self) -> &PktdmaRingrtRtFdb {
        &self.pktdma_ringrt_rt_fdb
    }
    #[doc = "0x18 - The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
    #[inline(always)]
    pub const fn pktdma_ringrt_rt_focc(&self) -> &PktdmaRingrtRtFocc {
        &self.pktdma_ringrt_rt_focc
    }
    #[doc = "0x1010 - The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
    #[inline(always)]
    pub const fn pktdma_ringrt_rt_rdb(&self) -> &PktdmaRingrtRtRdb {
        &self.pktdma_ringrt_rt_rdb
    }
    #[doc = "0x1018 - The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
    #[inline(always)]
    pub const fn pktdma_ringrt_rt_rocc(&self) -> &PktdmaRingrtRtRocc {
        &self.pktdma_ringrt_rt_rocc
    }
}
#[doc = "PKTDMA_RINGRT_RT_FDB (rw) register accessor: The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ringrt_rt_fdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ringrt_rt_fdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ringrt_rt_fdb`]
module"]
#[doc(alias = "PKTDMA_RINGRT_RT_FDB")]
pub type PktdmaRingrtRtFdb = crate::Reg<pktdma_ringrt_rt_fdb::PktdmaRingrtRtFdbSpec>;
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
pub mod pktdma_ringrt_rt_fdb;
#[doc = "PKTDMA_RINGRT_RT_FOCC (rw) register accessor: The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ringrt_rt_focc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ringrt_rt_focc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ringrt_rt_focc`]
module"]
#[doc(alias = "PKTDMA_RINGRT_RT_FOCC")]
pub type PktdmaRingrtRtFocc = crate::Reg<pktdma_ringrt_rt_focc::PktdmaRingrtRtFoccSpec>;
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
pub mod pktdma_ringrt_rt_focc;
#[doc = "PKTDMA_RINGRT_RT_RDB (rw) register accessor: The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ringrt_rt_rdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ringrt_rt_rdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ringrt_rt_rdb`]
module"]
#[doc(alias = "PKTDMA_RINGRT_RT_RDB")]
pub type PktdmaRingrtRtRdb = crate::Reg<pktdma_ringrt_rt_rdb::PktdmaRingrtRtRdbSpec>;
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
pub mod pktdma_ringrt_rt_rdb;
#[doc = "PKTDMA_RINGRT_RT_ROCC (rw) register accessor: The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ringrt_rt_rocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ringrt_rt_rocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_ringrt_rt_rocc`]
module"]
#[doc(alias = "PKTDMA_RINGRT_RT_ROCC")]
pub type PktdmaRingrtRtRocc = crate::Reg<pktdma_ringrt_rt_rocc::PktdmaRingrtRtRoccSpec>;
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
pub mod pktdma_ringrt_rt_rocc;
