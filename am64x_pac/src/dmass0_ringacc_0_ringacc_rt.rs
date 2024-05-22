#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    ringacc_rt_rt_db: RingaccRtRtDb,
    _reserved1: [u8; 0x04],
    ringacc_rt_rt_occ: RingaccRtRtOcc,
    ringacc_rt_rt_indx: RingaccRtRtIndx,
    ringacc_rt_rt_hwocc: RingaccRtRtHwocc,
    ringacc_rt_rt_hwindx: RingaccRtRtHwindx,
}
impl RegisterBlock {
    #[doc = "0x10 - The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
    #[inline(always)]
    pub const fn ringacc_rt_rt_db(&self) -> &RingaccRtRtDb {
        &self.ringacc_rt_rt_db
    }
    #[doc = "0x18 - The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
    #[inline(always)]
    pub const fn ringacc_rt_rt_occ(&self) -> &RingaccRtRtOcc {
        &self.ringacc_rt_rt_occ
    }
    #[doc = "0x1c - The Ring N Current Index Register can be read by software for debug purposes to determine the current SW read index for the Ring for the channel."]
    #[inline(always)]
    pub const fn ringacc_rt_rt_indx(&self) -> &RingaccRtRtIndx {
        &self.ringacc_rt_rt_indx
    }
    #[doc = "0x20 - The Ring N Hardware Occupancy Register contains the early increment/decrement version of the the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
    #[inline(always)]
    pub const fn ringacc_rt_rt_hwocc(&self) -> &RingaccRtRtHwocc {
        &self.ringacc_rt_rt_hwocc
    }
    #[doc = "0x24 - The Ring N Current Index Register can be read by software for debug purposes to determine the current HW read index for the Ring for the channel."]
    #[inline(always)]
    pub const fn ringacc_rt_rt_hwindx(&self) -> &RingaccRtRtHwindx {
        &self.ringacc_rt_rt_hwindx
    }
}
#[doc = "RINGACC_RT_RT_DB (rw) register accessor: The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_db::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_db::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_rt_rt_db`]
module"]
#[doc(alias = "RINGACC_RT_RT_DB")]
pub type RingaccRtRtDb = crate::Reg<ringacc_rt_rt_db::RingaccRtRtDbSpec>;
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation."]
pub mod ringacc_rt_rt_db;
#[doc = "RINGACC_RT_RT_OCC (rw) register accessor: The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_occ::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_occ::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_rt_rt_occ`]
module"]
#[doc(alias = "RINGACC_RT_RT_OCC")]
pub type RingaccRtRtOcc = crate::Reg<ringacc_rt_rt_occ::RingaccRtRtOccSpec>;
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
pub mod ringacc_rt_rt_occ;
#[doc = "RINGACC_RT_RT_INDX (rw) register accessor: The Ring N Current Index Register can be read by software for debug purposes to determine the current SW read index for the Ring for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_indx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_indx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_rt_rt_indx`]
module"]
#[doc(alias = "RINGACC_RT_RT_INDX")]
pub type RingaccRtRtIndx = crate::Reg<ringacc_rt_rt_indx::RingaccRtRtIndxSpec>;
#[doc = "The Ring N Current Index Register can be read by software for debug purposes to determine the current SW read index for the Ring for the channel."]
pub mod ringacc_rt_rt_indx;
#[doc = "RINGACC_RT_RT_HWOCC (rw) register accessor: The Ring N Hardware Occupancy Register contains the early increment/decrement version of the the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_hwocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_hwocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_rt_rt_hwocc`]
module"]
#[doc(alias = "RINGACC_RT_RT_HWOCC")]
pub type RingaccRtRtHwocc = crate::Reg<ringacc_rt_rt_hwocc::RingaccRtRtHwoccSpec>;
#[doc = "The Ring N Hardware Occupancy Register contains the early increment/decrement version of the the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host."]
pub mod ringacc_rt_rt_hwocc;
#[doc = "RINGACC_RT_RT_HWINDX (rw) register accessor: The Ring N Current Index Register can be read by software for debug purposes to determine the current HW read index for the Ring for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_hwindx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_hwindx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_rt_rt_hwindx`]
module"]
#[doc(alias = "RINGACC_RT_RT_HWINDX")]
pub type RingaccRtRtHwindx = crate::Reg<ringacc_rt_rt_hwindx::RingaccRtRtHwindxSpec>;
#[doc = "The Ring N Current Index Register can be read by software for debug purposes to determine the current HW read index for the Ring for the channel."]
pub mod ringacc_rt_rt_hwindx;
