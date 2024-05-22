#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    bcdma_ringrt_rt_fdb: BcdmaRingrtRtFdb,
    _reserved1: [u8; 0x04],
    bcdma_ringrt_rt_focc: BcdmaRingrtRtFocc,
    _reserved2: [u8; 0x0ff4],
    bcdma_ringrt_rt_rdb: BcdmaRingrtRtRdb,
    _reserved3: [u8; 0x04],
    bcdma_ringrt_rt_rocc: BcdmaRingrtRtRocc,
}
impl RegisterBlock {
    #[doc = "0x10 - The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
    #[inline(always)]
    pub const fn bcdma_ringrt_rt_fdb(&self) -> &BcdmaRingrtRtFdb {
        &self.bcdma_ringrt_rt_fdb
    }
    #[doc = "0x18 - The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
    #[inline(always)]
    pub const fn bcdma_ringrt_rt_focc(&self) -> &BcdmaRingrtRtFocc {
        &self.bcdma_ringrt_rt_focc
    }
    #[doc = "0x1010 - The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
    #[inline(always)]
    pub const fn bcdma_ringrt_rt_rdb(&self) -> &BcdmaRingrtRtRdb {
        &self.bcdma_ringrt_rt_rdb
    }
    #[doc = "0x1018 - The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
    #[inline(always)]
    pub const fn bcdma_ringrt_rt_rocc(&self) -> &BcdmaRingrtRtRocc {
        &self.bcdma_ringrt_rt_rocc
    }
}
#[doc = "BCDMA_RINGRT_RT_FDB (rw) register accessor: The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ringrt_rt_fdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ringrt_rt_fdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ringrt_rt_fdb`]
module"]
#[doc(alias = "BCDMA_RINGRT_RT_FDB")]
pub type BcdmaRingrtRtFdb = crate::Reg<bcdma_ringrt_rt_fdb::BcdmaRingrtRtFdbSpec>;
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
pub mod bcdma_ringrt_rt_fdb;
#[doc = "BCDMA_RINGRT_RT_FOCC (rw) register accessor: The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ringrt_rt_focc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ringrt_rt_focc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ringrt_rt_focc`]
module"]
#[doc(alias = "BCDMA_RINGRT_RT_FOCC")]
pub type BcdmaRingrtRtFocc = crate::Reg<bcdma_ringrt_rt_focc::BcdmaRingrtRtFoccSpec>;
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
pub mod bcdma_ringrt_rt_focc;
#[doc = "BCDMA_RINGRT_RT_RDB (rw) register accessor: The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ringrt_rt_rdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ringrt_rt_rdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ringrt_rt_rdb`]
module"]
#[doc(alias = "BCDMA_RINGRT_RT_RDB")]
pub type BcdmaRingrtRtRdb = crate::Reg<bcdma_ringrt_rt_rdb::BcdmaRingrtRtRdbSpec>;
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
pub mod bcdma_ringrt_rt_rdb;
#[doc = "BCDMA_RINGRT_RT_ROCC (rw) register accessor: The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ringrt_rt_rocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ringrt_rt_rocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_ringrt_rt_rocc`]
module"]
#[doc(alias = "BCDMA_RINGRT_RT_ROCC")]
pub type BcdmaRingrtRtRocc = crate::Reg<bcdma_ringrt_rt_rocc::BcdmaRingrtRtRoccSpec>;
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
pub mod bcdma_ringrt_rt_rocc;
