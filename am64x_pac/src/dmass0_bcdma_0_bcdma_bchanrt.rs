#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_bchanrt_trt_ctl: BcdmaBchanrtTrtCtl,
    _reserved1: [u8; 0x04],
    bcdma_bchanrt_trt_swtrig: BcdmaBchanrtTrtSwtrig,
    _reserved2: [u8; 0x34],
    bcdma_bchanrt_trt_status0: BcdmaBchanrtTrtStatus0,
    bcdma_bchanrt_trt_status1: BcdmaBchanrtTrtStatus1,
    bcdma_bchanrt_trt_status2: BcdmaBchanrtTrtStatus2,
    bcdma_bchanrt_trt_status3: BcdmaBchanrtTrtStatus3,
    _reserved6: [u8; 0x30],
    bcdma_bchanrt_trt_stdata: BcdmaBchanrtTrtStdata,
    _reserved7: [u8; 0x7c],
    bcdma_bchanrt_rrt_stdata: BcdmaBchanrtRrtStdata,
    _reserved8: [u8; 0x02fc],
    bcdma_bchanrt_trt_pcnt: BcdmaBchanrtTrtPcnt,
    _reserved9: [u8; 0x04],
    bcdma_bchanrt_trt_bcnt: BcdmaBchanrtTrtBcnt,
    _reserved10: [u8; 0x04],
    bcdma_bchanrt_trt_sbcnt: BcdmaBchanrtTrtSbcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_ctl(&self) -> &BcdmaBchanrtTrtCtl {
        &self.bcdma_bchanrt_trt_ctl
    }
    #[doc = "0x08 - The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_swtrig(&self) -> &BcdmaBchanrtTrtSwtrig {
        &self.bcdma_bchanrt_trt_swtrig
    }
    #[doc = "0x40 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_status0(&self) -> &BcdmaBchanrtTrtStatus0 {
        &self.bcdma_bchanrt_trt_status0
    }
    #[doc = "0x44 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_status1(&self) -> &BcdmaBchanrtTrtStatus1 {
        &self.bcdma_bchanrt_trt_status1
    }
    #[doc = "0x48 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_status2(&self) -> &BcdmaBchanrtTrtStatus2 {
        &self.bcdma_bchanrt_trt_status2
    }
    #[doc = "0x4c - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_status3(&self) -> &BcdmaBchanrtTrtStatus3 {
        &self.bcdma_bchanrt_trt_status3
    }
    #[doc = "0x80 - The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_stdata(&self) -> &BcdmaBchanrtTrtStdata {
        &self.bcdma_bchanrt_trt_stdata
    }
    #[doc = "0x100 - The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
    #[inline(always)]
    pub const fn bcdma_bchanrt_rrt_stdata(&self) -> &BcdmaBchanrtRrtStdata {
        &self.bcdma_bchanrt_rrt_stdata
    }
    #[doc = "0x400 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_pcnt(&self) -> &BcdmaBchanrtTrtPcnt {
        &self.bcdma_bchanrt_trt_pcnt
    }
    #[doc = "0x408 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_bcnt(&self) -> &BcdmaBchanrtTrtBcnt {
        &self.bcdma_bchanrt_trt_bcnt
    }
    #[doc = "0x410 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_bchanrt_trt_sbcnt(&self) -> &BcdmaBchanrtTrtSbcnt {
        &self.bcdma_bchanrt_trt_sbcnt
    }
}
#[doc = "BCDMA_BCHANRT_TRT_CTL (rw) register accessor: The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_ctl`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_CTL")]
pub type BcdmaBchanrtTrtCtl = crate::Reg<bcdma_bchanrt_trt_ctl::BcdmaBchanrtTrtCtlSpec>;
#[doc = "The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
pub mod bcdma_bchanrt_trt_ctl;
#[doc = "BCDMA_BCHANRT_TRT_SWTRIG (rw) register accessor: The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_swtrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_swtrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_swtrig`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_SWTRIG")]
pub type BcdmaBchanrtTrtSwtrig = crate::Reg<bcdma_bchanrt_trt_swtrig::BcdmaBchanrtTrtSwtrigSpec>;
#[doc = "The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel."]
pub mod bcdma_bchanrt_trt_swtrig;
#[doc = "BCDMA_BCHANRT_TRT_STATUS0 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_status0`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_STATUS0")]
pub type BcdmaBchanrtTrtStatus0 = crate::Reg<bcdma_bchanrt_trt_status0::BcdmaBchanrtTrtStatus0Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_bchanrt_trt_status0;
#[doc = "BCDMA_BCHANRT_TRT_STATUS1 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_status1`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_STATUS1")]
pub type BcdmaBchanrtTrtStatus1 = crate::Reg<bcdma_bchanrt_trt_status1::BcdmaBchanrtTrtStatus1Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_bchanrt_trt_status1;
#[doc = "BCDMA_BCHANRT_TRT_STATUS2 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_status2`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_STATUS2")]
pub type BcdmaBchanrtTrtStatus2 = crate::Reg<bcdma_bchanrt_trt_status2::BcdmaBchanrtTrtStatus2Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_bchanrt_trt_status2;
#[doc = "BCDMA_BCHANRT_TRT_STATUS3 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_status3`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_STATUS3")]
pub type BcdmaBchanrtTrtStatus3 = crate::Reg<bcdma_bchanrt_trt_status3::BcdmaBchanrtTrtStatus3Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_bchanrt_trt_status3;
#[doc = "BCDMA_BCHANRT_TRT_STDATA (rw) register accessor: The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_stdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_stdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_stdata`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_STDATA")]
pub type BcdmaBchanrtTrtStdata = crate::Reg<bcdma_bchanrt_trt_stdata::BcdmaBchanrtTrtStdataSpec>;
#[doc = "The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
pub mod bcdma_bchanrt_trt_stdata;
#[doc = "BCDMA_BCHANRT_RRT_STDATA (rw) register accessor: The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_rrt_stdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_rrt_stdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_rrt_stdata`]
module"]
#[doc(alias = "BCDMA_BCHANRT_RRT_STDATA")]
pub type BcdmaBchanrtRrtStdata = crate::Reg<bcdma_bchanrt_rrt_stdata::BcdmaBchanrtRrtStdataSpec>;
#[doc = "The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
pub mod bcdma_bchanrt_rrt_stdata;
#[doc = "BCDMA_BCHANRT_TRT_PCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_pcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_pcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_pcnt`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_PCNT")]
pub type BcdmaBchanrtTrtPcnt = crate::Reg<bcdma_bchanrt_trt_pcnt::BcdmaBchanrtTrtPcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_bchanrt_trt_pcnt;
#[doc = "BCDMA_BCHANRT_TRT_BCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_bcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_bcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_bcnt`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_BCNT")]
pub type BcdmaBchanrtTrtBcnt = crate::Reg<bcdma_bchanrt_trt_bcnt::BcdmaBchanrtTrtBcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_bchanrt_trt_bcnt;
#[doc = "BCDMA_BCHANRT_TRT_SBCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_sbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_sbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchanrt_trt_sbcnt`]
module"]
#[doc(alias = "BCDMA_BCHANRT_TRT_SBCNT")]
pub type BcdmaBchanrtTrtSbcnt = crate::Reg<bcdma_bchanrt_trt_sbcnt::BcdmaBchanrtTrtSbcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_bchanrt_trt_sbcnt;
