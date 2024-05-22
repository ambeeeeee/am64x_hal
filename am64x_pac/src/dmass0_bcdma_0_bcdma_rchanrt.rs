#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_rchanrt_rrt_ctl: BcdmaRchanrtRrtCtl,
    _reserved1: [u8; 0x04],
    bcdma_rchanrt_rrt_swtrig: BcdmaRchanrtRrtSwtrig,
    _reserved2: [u8; 0x34],
    bcdma_rchanrt_rrt_status0: BcdmaRchanrtRrtStatus0,
    bcdma_rchanrt_rrt_status1: BcdmaRchanrtRrtStatus1,
    _reserved4: [u8; 0x38],
    bcdma_rchanrt_rrt_stdata: BcdmaRchanrtRrtStdata,
    _reserved5: [u8; 0x017c],
    bcdma_rchanrt_rrt_peer0: BcdmaRchanrtRrtPeer0,
    bcdma_rchanrt_rrt_peer1: BcdmaRchanrtRrtPeer1,
    bcdma_rchanrt_rrt_peer2: BcdmaRchanrtRrtPeer2,
    bcdma_rchanrt_rrt_peer3: BcdmaRchanrtRrtPeer3,
    bcdma_rchanrt_rrt_peer4: BcdmaRchanrtRrtPeer4,
    bcdma_rchanrt_rrt_peer5: BcdmaRchanrtRrtPeer5,
    bcdma_rchanrt_rrt_peer6: BcdmaRchanrtRrtPeer6,
    bcdma_rchanrt_rrt_peer7: BcdmaRchanrtRrtPeer7,
    bcdma_rchanrt_rrt_peer8: BcdmaRchanrtRrtPeer8,
    bcdma_rchanrt_rrt_peer9: BcdmaRchanrtRrtPeer9,
    bcdma_rchanrt_rrt_peer10: BcdmaRchanrtRrtPeer10,
    bcdma_rchanrt_rrt_peer11: BcdmaRchanrtRrtPeer11,
    bcdma_rchanrt_rrt_peer12: BcdmaRchanrtRrtPeer12,
    bcdma_rchanrt_rrt_peer13: BcdmaRchanrtRrtPeer13,
    bcdma_rchanrt_rrt_peer14: BcdmaRchanrtRrtPeer14,
    bcdma_rchanrt_rrt_peer15: BcdmaRchanrtRrtPeer15,
    _reserved21: [u8; 0x01c0],
    bcdma_rchanrt_rrt_pcnt: BcdmaRchanrtRrtPcnt,
    _reserved22: [u8; 0x04],
    bcdma_rchanrt_rrt_bcnt: BcdmaRchanrtRrtBcnt,
    _reserved23: [u8; 0x04],
    bcdma_rchanrt_rrt_sbcnt: BcdmaRchanrtRrtSbcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_ctl(&self) -> &BcdmaRchanrtRrtCtl {
        &self.bcdma_rchanrt_rrt_ctl
    }
    #[doc = "0x08 - The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_swtrig(&self) -> &BcdmaRchanrtRrtSwtrig {
        &self.bcdma_rchanrt_rrt_swtrig
    }
    #[doc = "0x40 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_status0(&self) -> &BcdmaRchanrtRrtStatus0 {
        &self.bcdma_rchanrt_rrt_status0
    }
    #[doc = "0x44 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_status1(&self) -> &BcdmaRchanrtRrtStatus1 {
        &self.bcdma_rchanrt_rrt_status1
    }
    #[doc = "0x80 - The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_stdata(&self) -> &BcdmaRchanrtRrtStdata {
        &self.bcdma_rchanrt_rrt_stdata
    }
    #[doc = "0x200 - This register provides access to the remote peer's realtime register at 0x400."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer0(&self) -> &BcdmaRchanrtRrtPeer0 {
        &self.bcdma_rchanrt_rrt_peer0
    }
    #[doc = "0x204 - This register provides access to the remote peer's realtime register at 0x401."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer1(&self) -> &BcdmaRchanrtRrtPeer1 {
        &self.bcdma_rchanrt_rrt_peer1
    }
    #[doc = "0x208 - This register provides access to the remote peer's realtime register at 0x402."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer2(&self) -> &BcdmaRchanrtRrtPeer2 {
        &self.bcdma_rchanrt_rrt_peer2
    }
    #[doc = "0x20c - This register provides access to the remote peer's realtime register at 0x403."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer3(&self) -> &BcdmaRchanrtRrtPeer3 {
        &self.bcdma_rchanrt_rrt_peer3
    }
    #[doc = "0x210 - This register provides access to the remote peer's realtime register at 0x404."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer4(&self) -> &BcdmaRchanrtRrtPeer4 {
        &self.bcdma_rchanrt_rrt_peer4
    }
    #[doc = "0x214 - This register provides access to the remote peer's realtime register at 0x405."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer5(&self) -> &BcdmaRchanrtRrtPeer5 {
        &self.bcdma_rchanrt_rrt_peer5
    }
    #[doc = "0x218 - This register provides access to the remote peer's realtime register at 0x406."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer6(&self) -> &BcdmaRchanrtRrtPeer6 {
        &self.bcdma_rchanrt_rrt_peer6
    }
    #[doc = "0x21c - This register provides access to the remote peer's realtime register at 0x407."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer7(&self) -> &BcdmaRchanrtRrtPeer7 {
        &self.bcdma_rchanrt_rrt_peer7
    }
    #[doc = "0x220 - This register provides access to the remote peer's realtime register at 0x408."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer8(&self) -> &BcdmaRchanrtRrtPeer8 {
        &self.bcdma_rchanrt_rrt_peer8
    }
    #[doc = "0x224 - This register provides access to the remote peer's realtime register at 0x409."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer9(&self) -> &BcdmaRchanrtRrtPeer9 {
        &self.bcdma_rchanrt_rrt_peer9
    }
    #[doc = "0x228 - This register provides access to the remote peer's realtime register at 0x40A."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer10(&self) -> &BcdmaRchanrtRrtPeer10 {
        &self.bcdma_rchanrt_rrt_peer10
    }
    #[doc = "0x22c - This register provides access to the remote peer's realtime register at 0x40B."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer11(&self) -> &BcdmaRchanrtRrtPeer11 {
        &self.bcdma_rchanrt_rrt_peer11
    }
    #[doc = "0x230 - This register provides access to the remote peer's realtime register at 0x40C."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer12(&self) -> &BcdmaRchanrtRrtPeer12 {
        &self.bcdma_rchanrt_rrt_peer12
    }
    #[doc = "0x234 - This register provides access to the remote peer's realtime register at 0x40D."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer13(&self) -> &BcdmaRchanrtRrtPeer13 {
        &self.bcdma_rchanrt_rrt_peer13
    }
    #[doc = "0x238 - This register provides access to the remote peer's realtime register at 0x40E."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer14(&self) -> &BcdmaRchanrtRrtPeer14 {
        &self.bcdma_rchanrt_rrt_peer14
    }
    #[doc = "0x23c - This register provides access to the remote peer's realtime register at 0x40F."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_peer15(&self) -> &BcdmaRchanrtRrtPeer15 {
        &self.bcdma_rchanrt_rrt_peer15
    }
    #[doc = "0x400 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_pcnt(&self) -> &BcdmaRchanrtRrtPcnt {
        &self.bcdma_rchanrt_rrt_pcnt
    }
    #[doc = "0x408 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_bcnt(&self) -> &BcdmaRchanrtRrtBcnt {
        &self.bcdma_rchanrt_rrt_bcnt
    }
    #[doc = "0x410 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_rchanrt_rrt_sbcnt(&self) -> &BcdmaRchanrtRrtSbcnt {
        &self.bcdma_rchanrt_rrt_sbcnt
    }
}
#[doc = "BCDMA_RCHANRT_RRT_CTL (rw) register accessor: The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_ctl`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_CTL")]
pub type BcdmaRchanrtRrtCtl = crate::Reg<bcdma_rchanrt_rrt_ctl::BcdmaRchanrtRrtCtlSpec>;
#[doc = "The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
pub mod bcdma_rchanrt_rrt_ctl;
#[doc = "BCDMA_RCHANRT_RRT_SWTRIG (rw) register accessor: The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_swtrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_swtrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_swtrig`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_SWTRIG")]
pub type BcdmaRchanrtRrtSwtrig = crate::Reg<bcdma_rchanrt_rrt_swtrig::BcdmaRchanrtRrtSwtrigSpec>;
#[doc = "The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel."]
pub mod bcdma_rchanrt_rrt_swtrig;
#[doc = "BCDMA_RCHANRT_RRT_STATUS0 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_status0`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_STATUS0")]
pub type BcdmaRchanrtRrtStatus0 = crate::Reg<bcdma_rchanrt_rrt_status0::BcdmaRchanrtRrtStatus0Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_rchanrt_rrt_status0;
#[doc = "BCDMA_RCHANRT_RRT_STATUS1 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_status1`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_STATUS1")]
pub type BcdmaRchanrtRrtStatus1 = crate::Reg<bcdma_rchanrt_rrt_status1::BcdmaRchanrtRrtStatus1Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_rchanrt_rrt_status1;
#[doc = "BCDMA_RCHANRT_RRT_STDATA (rw) register accessor: The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_stdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_stdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_stdata`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_STDATA")]
pub type BcdmaRchanrtRrtStdata = crate::Reg<bcdma_rchanrt_rrt_stdata::BcdmaRchanrtRrtStdataSpec>;
#[doc = "The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
pub mod bcdma_rchanrt_rrt_stdata;
#[doc = "BCDMA_RCHANRT_RRT_PEER0 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x400.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer0`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER0")]
pub type BcdmaRchanrtRrtPeer0 = crate::Reg<bcdma_rchanrt_rrt_peer0::BcdmaRchanrtRrtPeer0Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x400."]
pub mod bcdma_rchanrt_rrt_peer0;
#[doc = "BCDMA_RCHANRT_RRT_PEER1 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x401.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer1`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER1")]
pub type BcdmaRchanrtRrtPeer1 = crate::Reg<bcdma_rchanrt_rrt_peer1::BcdmaRchanrtRrtPeer1Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x401."]
pub mod bcdma_rchanrt_rrt_peer1;
#[doc = "BCDMA_RCHANRT_RRT_PEER2 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x402.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer2`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER2")]
pub type BcdmaRchanrtRrtPeer2 = crate::Reg<bcdma_rchanrt_rrt_peer2::BcdmaRchanrtRrtPeer2Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x402."]
pub mod bcdma_rchanrt_rrt_peer2;
#[doc = "BCDMA_RCHANRT_RRT_PEER3 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x403.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer3`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER3")]
pub type BcdmaRchanrtRrtPeer3 = crate::Reg<bcdma_rchanrt_rrt_peer3::BcdmaRchanrtRrtPeer3Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x403."]
pub mod bcdma_rchanrt_rrt_peer3;
#[doc = "BCDMA_RCHANRT_RRT_PEER4 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x404.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer4`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER4")]
pub type BcdmaRchanrtRrtPeer4 = crate::Reg<bcdma_rchanrt_rrt_peer4::BcdmaRchanrtRrtPeer4Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x404."]
pub mod bcdma_rchanrt_rrt_peer4;
#[doc = "BCDMA_RCHANRT_RRT_PEER5 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x405.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer5`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER5")]
pub type BcdmaRchanrtRrtPeer5 = crate::Reg<bcdma_rchanrt_rrt_peer5::BcdmaRchanrtRrtPeer5Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x405."]
pub mod bcdma_rchanrt_rrt_peer5;
#[doc = "BCDMA_RCHANRT_RRT_PEER6 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x406.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer6`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER6")]
pub type BcdmaRchanrtRrtPeer6 = crate::Reg<bcdma_rchanrt_rrt_peer6::BcdmaRchanrtRrtPeer6Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x406."]
pub mod bcdma_rchanrt_rrt_peer6;
#[doc = "BCDMA_RCHANRT_RRT_PEER7 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x407.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer7`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER7")]
pub type BcdmaRchanrtRrtPeer7 = crate::Reg<bcdma_rchanrt_rrt_peer7::BcdmaRchanrtRrtPeer7Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x407."]
pub mod bcdma_rchanrt_rrt_peer7;
#[doc = "BCDMA_RCHANRT_RRT_PEER8 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x408.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer8`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER8")]
pub type BcdmaRchanrtRrtPeer8 = crate::Reg<bcdma_rchanrt_rrt_peer8::BcdmaRchanrtRrtPeer8Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x408."]
pub mod bcdma_rchanrt_rrt_peer8;
#[doc = "BCDMA_RCHANRT_RRT_PEER9 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x409.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer9`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER9")]
pub type BcdmaRchanrtRrtPeer9 = crate::Reg<bcdma_rchanrt_rrt_peer9::BcdmaRchanrtRrtPeer9Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x409."]
pub mod bcdma_rchanrt_rrt_peer9;
#[doc = "BCDMA_RCHANRT_RRT_PEER10 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer10`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER10")]
pub type BcdmaRchanrtRrtPeer10 = crate::Reg<bcdma_rchanrt_rrt_peer10::BcdmaRchanrtRrtPeer10Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40A."]
pub mod bcdma_rchanrt_rrt_peer10;
#[doc = "BCDMA_RCHANRT_RRT_PEER11 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer11`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER11")]
pub type BcdmaRchanrtRrtPeer11 = crate::Reg<bcdma_rchanrt_rrt_peer11::BcdmaRchanrtRrtPeer11Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40B."]
pub mod bcdma_rchanrt_rrt_peer11;
#[doc = "BCDMA_RCHANRT_RRT_PEER12 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer12`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER12")]
pub type BcdmaRchanrtRrtPeer12 = crate::Reg<bcdma_rchanrt_rrt_peer12::BcdmaRchanrtRrtPeer12Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40C."]
pub mod bcdma_rchanrt_rrt_peer12;
#[doc = "BCDMA_RCHANRT_RRT_PEER13 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer13`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER13")]
pub type BcdmaRchanrtRrtPeer13 = crate::Reg<bcdma_rchanrt_rrt_peer13::BcdmaRchanrtRrtPeer13Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40D."]
pub mod bcdma_rchanrt_rrt_peer13;
#[doc = "BCDMA_RCHANRT_RRT_PEER14 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40E.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer14`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER14")]
pub type BcdmaRchanrtRrtPeer14 = crate::Reg<bcdma_rchanrt_rrt_peer14::BcdmaRchanrtRrtPeer14Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40E."]
pub mod bcdma_rchanrt_rrt_peer14;
#[doc = "BCDMA_RCHANRT_RRT_PEER15 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40F.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_peer15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_peer15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_peer15`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PEER15")]
pub type BcdmaRchanrtRrtPeer15 = crate::Reg<bcdma_rchanrt_rrt_peer15::BcdmaRchanrtRrtPeer15Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40F."]
pub mod bcdma_rchanrt_rrt_peer15;
#[doc = "BCDMA_RCHANRT_RRT_PCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_pcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_pcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_pcnt`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_PCNT")]
pub type BcdmaRchanrtRrtPcnt = crate::Reg<bcdma_rchanrt_rrt_pcnt::BcdmaRchanrtRrtPcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_rchanrt_rrt_pcnt;
#[doc = "BCDMA_RCHANRT_RRT_BCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_bcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_bcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_bcnt`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_BCNT")]
pub type BcdmaRchanrtRrtBcnt = crate::Reg<bcdma_rchanrt_rrt_bcnt::BcdmaRchanrtRrtBcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_rchanrt_rrt_bcnt;
#[doc = "BCDMA_RCHANRT_RRT_SBCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_sbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_sbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchanrt_rrt_sbcnt`]
module"]
#[doc(alias = "BCDMA_RCHANRT_RRT_SBCNT")]
pub type BcdmaRchanrtRrtSbcnt = crate::Reg<bcdma_rchanrt_rrt_sbcnt::BcdmaRchanrtRrtSbcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_rchanrt_rrt_sbcnt;
