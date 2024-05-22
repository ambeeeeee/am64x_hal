#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pktdma_rchanrt_rrt_ctl: PktdmaRchanrtRrtCtl,
    _reserved1: [u8; 0x3c],
    pktdma_rchanrt_rrt_status0: PktdmaRchanrtRrtStatus0,
    pktdma_rchanrt_rrt_status1: PktdmaRchanrtRrtStatus1,
    _reserved3: [u8; 0x38],
    pktdma_rchanrt_rrt_stdata: PktdmaRchanrtRrtStdata,
    _reserved4: [u8; 0x017c],
    pktdma_rchanrt_rrt_peer0: PktdmaRchanrtRrtPeer0,
    pktdma_rchanrt_rrt_peer1: PktdmaRchanrtRrtPeer1,
    pktdma_rchanrt_rrt_peer2: PktdmaRchanrtRrtPeer2,
    pktdma_rchanrt_rrt_peer3: PktdmaRchanrtRrtPeer3,
    pktdma_rchanrt_rrt_peer4: PktdmaRchanrtRrtPeer4,
    pktdma_rchanrt_rrt_peer5: PktdmaRchanrtRrtPeer5,
    pktdma_rchanrt_rrt_peer6: PktdmaRchanrtRrtPeer6,
    pktdma_rchanrt_rrt_peer7: PktdmaRchanrtRrtPeer7,
    pktdma_rchanrt_rrt_peer8: PktdmaRchanrtRrtPeer8,
    pktdma_rchanrt_rrt_peer9: PktdmaRchanrtRrtPeer9,
    pktdma_rchanrt_rrt_peer10: PktdmaRchanrtRrtPeer10,
    pktdma_rchanrt_rrt_peer11: PktdmaRchanrtRrtPeer11,
    pktdma_rchanrt_rrt_peer12: PktdmaRchanrtRrtPeer12,
    pktdma_rchanrt_rrt_peer13: PktdmaRchanrtRrtPeer13,
    pktdma_rchanrt_rrt_peer14: PktdmaRchanrtRrtPeer14,
    pktdma_rchanrt_rrt_peer15: PktdmaRchanrtRrtPeer15,
    _reserved20: [u8; 0x01c0],
    pktdma_rchanrt_rrt_pcnt: PktdmaRchanrtRrtPcnt,
    pktdma_rchanrt_rrt_dcnt: PktdmaRchanrtRrtDcnt,
    pktdma_rchanrt_rrt_bcnt: PktdmaRchanrtRrtBcnt,
    _reserved23: [u8; 0x04],
    pktdma_rchanrt_rrt_sbcnt: PktdmaRchanrtRrtSbcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_ctl(&self) -> &PktdmaRchanrtRrtCtl {
        &self.pktdma_rchanrt_rrt_ctl
    }
    #[doc = "0x40 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_status0(&self) -> &PktdmaRchanrtRrtStatus0 {
        &self.pktdma_rchanrt_rrt_status0
    }
    #[doc = "0x44 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_status1(&self) -> &PktdmaRchanrtRrtStatus1 {
        &self.pktdma_rchanrt_rrt_status1
    }
    #[doc = "0x80 - The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the PKTDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_stdata(&self) -> &PktdmaRchanrtRrtStdata {
        &self.pktdma_rchanrt_rrt_stdata
    }
    #[doc = "0x200 - This register provides access to the remote peer's realtime register at 0x400."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer0(&self) -> &PktdmaRchanrtRrtPeer0 {
        &self.pktdma_rchanrt_rrt_peer0
    }
    #[doc = "0x204 - This register provides access to the remote peer's realtime register at 0x401."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer1(&self) -> &PktdmaRchanrtRrtPeer1 {
        &self.pktdma_rchanrt_rrt_peer1
    }
    #[doc = "0x208 - This register provides access to the remote peer's realtime register at 0x402."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer2(&self) -> &PktdmaRchanrtRrtPeer2 {
        &self.pktdma_rchanrt_rrt_peer2
    }
    #[doc = "0x20c - This register provides access to the remote peer's realtime register at 0x403."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer3(&self) -> &PktdmaRchanrtRrtPeer3 {
        &self.pktdma_rchanrt_rrt_peer3
    }
    #[doc = "0x210 - This register provides access to the remote peer's realtime register at 0x404."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer4(&self) -> &PktdmaRchanrtRrtPeer4 {
        &self.pktdma_rchanrt_rrt_peer4
    }
    #[doc = "0x214 - This register provides access to the remote peer's realtime register at 0x405."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer5(&self) -> &PktdmaRchanrtRrtPeer5 {
        &self.pktdma_rchanrt_rrt_peer5
    }
    #[doc = "0x218 - This register provides access to the remote peer's realtime register at 0x406."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer6(&self) -> &PktdmaRchanrtRrtPeer6 {
        &self.pktdma_rchanrt_rrt_peer6
    }
    #[doc = "0x21c - This register provides access to the remote peer's realtime register at 0x407."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer7(&self) -> &PktdmaRchanrtRrtPeer7 {
        &self.pktdma_rchanrt_rrt_peer7
    }
    #[doc = "0x220 - This register provides access to the remote peer's realtime register at 0x408."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer8(&self) -> &PktdmaRchanrtRrtPeer8 {
        &self.pktdma_rchanrt_rrt_peer8
    }
    #[doc = "0x224 - This register provides access to the remote peer's realtime register at 0x409."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer9(&self) -> &PktdmaRchanrtRrtPeer9 {
        &self.pktdma_rchanrt_rrt_peer9
    }
    #[doc = "0x228 - This register provides access to the remote peer's realtime register at 0x40A."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer10(&self) -> &PktdmaRchanrtRrtPeer10 {
        &self.pktdma_rchanrt_rrt_peer10
    }
    #[doc = "0x22c - This register provides access to the remote peer's realtime register at 0x40B."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer11(&self) -> &PktdmaRchanrtRrtPeer11 {
        &self.pktdma_rchanrt_rrt_peer11
    }
    #[doc = "0x230 - This register provides access to the remote peer's realtime register at 0x40C."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer12(&self) -> &PktdmaRchanrtRrtPeer12 {
        &self.pktdma_rchanrt_rrt_peer12
    }
    #[doc = "0x234 - This register provides access to the remote peer's realtime register at 0x40D."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer13(&self) -> &PktdmaRchanrtRrtPeer13 {
        &self.pktdma_rchanrt_rrt_peer13
    }
    #[doc = "0x238 - This register provides access to the remote peer's realtime register at 0x40E."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer14(&self) -> &PktdmaRchanrtRrtPeer14 {
        &self.pktdma_rchanrt_rrt_peer14
    }
    #[doc = "0x23c - This register provides access to the remote peer's realtime register at 0x40F."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_peer15(&self) -> &PktdmaRchanrtRrtPeer15 {
        &self.pktdma_rchanrt_rrt_peer15
    }
    #[doc = "0x400 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_pcnt(&self) -> &PktdmaRchanrtRrtPcnt {
        &self.pktdma_rchanrt_rrt_pcnt
    }
    #[doc = "0x404 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_dcnt(&self) -> &PktdmaRchanrtRrtDcnt {
        &self.pktdma_rchanrt_rrt_dcnt
    }
    #[doc = "0x408 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_bcnt(&self) -> &PktdmaRchanrtRrtBcnt {
        &self.pktdma_rchanrt_rrt_bcnt
    }
    #[doc = "0x410 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_rchanrt_rrt_sbcnt(&self) -> &PktdmaRchanrtRrtSbcnt {
        &self.pktdma_rchanrt_rrt_sbcnt
    }
}
#[doc = "PKTDMA_RCHANRT_RRT_CTL (rw) register accessor: The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_ctl`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_CTL")]
pub type PktdmaRchanrtRrtCtl = crate::Reg<pktdma_rchanrt_rrt_ctl::PktdmaRchanrtRrtCtlSpec>;
#[doc = "The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
pub mod pktdma_rchanrt_rrt_ctl;
#[doc = "PKTDMA_RCHANRT_RRT_STATUS0 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_status0`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_STATUS0")]
pub type PktdmaRchanrtRrtStatus0 =
    crate::Reg<pktdma_rchanrt_rrt_status0::PktdmaRchanrtRrtStatus0Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod pktdma_rchanrt_rrt_status0;
#[doc = "PKTDMA_RCHANRT_RRT_STATUS1 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_status1`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_STATUS1")]
pub type PktdmaRchanrtRrtStatus1 =
    crate::Reg<pktdma_rchanrt_rrt_status1::PktdmaRchanrtRrtStatus1Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod pktdma_rchanrt_rrt_status1;
#[doc = "PKTDMA_RCHANRT_RRT_STDATA (rw) register accessor: The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the PKTDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_stdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_stdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_stdata`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_STDATA")]
pub type PktdmaRchanrtRrtStdata = crate::Reg<pktdma_rchanrt_rrt_stdata::PktdmaRchanrtRrtStdataSpec>;
#[doc = "The State Data Registers contain the current working state of the Rx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the PKTDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
pub mod pktdma_rchanrt_rrt_stdata;
#[doc = "PKTDMA_RCHANRT_RRT_PEER0 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x400.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer0`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER0")]
pub type PktdmaRchanrtRrtPeer0 = crate::Reg<pktdma_rchanrt_rrt_peer0::PktdmaRchanrtRrtPeer0Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x400."]
pub mod pktdma_rchanrt_rrt_peer0;
#[doc = "PKTDMA_RCHANRT_RRT_PEER1 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x401.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer1`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER1")]
pub type PktdmaRchanrtRrtPeer1 = crate::Reg<pktdma_rchanrt_rrt_peer1::PktdmaRchanrtRrtPeer1Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x401."]
pub mod pktdma_rchanrt_rrt_peer1;
#[doc = "PKTDMA_RCHANRT_RRT_PEER2 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x402.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer2`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER2")]
pub type PktdmaRchanrtRrtPeer2 = crate::Reg<pktdma_rchanrt_rrt_peer2::PktdmaRchanrtRrtPeer2Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x402."]
pub mod pktdma_rchanrt_rrt_peer2;
#[doc = "PKTDMA_RCHANRT_RRT_PEER3 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x403.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer3`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER3")]
pub type PktdmaRchanrtRrtPeer3 = crate::Reg<pktdma_rchanrt_rrt_peer3::PktdmaRchanrtRrtPeer3Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x403."]
pub mod pktdma_rchanrt_rrt_peer3;
#[doc = "PKTDMA_RCHANRT_RRT_PEER4 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x404.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer4`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER4")]
pub type PktdmaRchanrtRrtPeer4 = crate::Reg<pktdma_rchanrt_rrt_peer4::PktdmaRchanrtRrtPeer4Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x404."]
pub mod pktdma_rchanrt_rrt_peer4;
#[doc = "PKTDMA_RCHANRT_RRT_PEER5 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x405.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer5`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER5")]
pub type PktdmaRchanrtRrtPeer5 = crate::Reg<pktdma_rchanrt_rrt_peer5::PktdmaRchanrtRrtPeer5Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x405."]
pub mod pktdma_rchanrt_rrt_peer5;
#[doc = "PKTDMA_RCHANRT_RRT_PEER6 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x406.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer6`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER6")]
pub type PktdmaRchanrtRrtPeer6 = crate::Reg<pktdma_rchanrt_rrt_peer6::PktdmaRchanrtRrtPeer6Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x406."]
pub mod pktdma_rchanrt_rrt_peer6;
#[doc = "PKTDMA_RCHANRT_RRT_PEER7 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x407.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer7`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER7")]
pub type PktdmaRchanrtRrtPeer7 = crate::Reg<pktdma_rchanrt_rrt_peer7::PktdmaRchanrtRrtPeer7Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x407."]
pub mod pktdma_rchanrt_rrt_peer7;
#[doc = "PKTDMA_RCHANRT_RRT_PEER8 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x408.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer8`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER8")]
pub type PktdmaRchanrtRrtPeer8 = crate::Reg<pktdma_rchanrt_rrt_peer8::PktdmaRchanrtRrtPeer8Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x408."]
pub mod pktdma_rchanrt_rrt_peer8;
#[doc = "PKTDMA_RCHANRT_RRT_PEER9 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x409.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer9`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER9")]
pub type PktdmaRchanrtRrtPeer9 = crate::Reg<pktdma_rchanrt_rrt_peer9::PktdmaRchanrtRrtPeer9Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x409."]
pub mod pktdma_rchanrt_rrt_peer9;
#[doc = "PKTDMA_RCHANRT_RRT_PEER10 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer10`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER10")]
pub type PktdmaRchanrtRrtPeer10 = crate::Reg<pktdma_rchanrt_rrt_peer10::PktdmaRchanrtRrtPeer10Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40A."]
pub mod pktdma_rchanrt_rrt_peer10;
#[doc = "PKTDMA_RCHANRT_RRT_PEER11 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer11`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER11")]
pub type PktdmaRchanrtRrtPeer11 = crate::Reg<pktdma_rchanrt_rrt_peer11::PktdmaRchanrtRrtPeer11Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40B."]
pub mod pktdma_rchanrt_rrt_peer11;
#[doc = "PKTDMA_RCHANRT_RRT_PEER12 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer12`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER12")]
pub type PktdmaRchanrtRrtPeer12 = crate::Reg<pktdma_rchanrt_rrt_peer12::PktdmaRchanrtRrtPeer12Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40C."]
pub mod pktdma_rchanrt_rrt_peer12;
#[doc = "PKTDMA_RCHANRT_RRT_PEER13 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer13`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER13")]
pub type PktdmaRchanrtRrtPeer13 = crate::Reg<pktdma_rchanrt_rrt_peer13::PktdmaRchanrtRrtPeer13Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40D."]
pub mod pktdma_rchanrt_rrt_peer13;
#[doc = "PKTDMA_RCHANRT_RRT_PEER14 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40E.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer14`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER14")]
pub type PktdmaRchanrtRrtPeer14 = crate::Reg<pktdma_rchanrt_rrt_peer14::PktdmaRchanrtRrtPeer14Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40E."]
pub mod pktdma_rchanrt_rrt_peer14;
#[doc = "PKTDMA_RCHANRT_RRT_PEER15 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40F.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_peer15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_peer15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_peer15`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PEER15")]
pub type PktdmaRchanrtRrtPeer15 = crate::Reg<pktdma_rchanrt_rrt_peer15::PktdmaRchanrtRrtPeer15Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40F."]
pub mod pktdma_rchanrt_rrt_peer15;
#[doc = "PKTDMA_RCHANRT_RRT_PCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_pcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_pcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_pcnt`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_PCNT")]
pub type PktdmaRchanrtRrtPcnt = crate::Reg<pktdma_rchanrt_rrt_pcnt::PktdmaRchanrtRrtPcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_rchanrt_rrt_pcnt;
#[doc = "PKTDMA_RCHANRT_RRT_DCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_dcnt`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_DCNT")]
pub type PktdmaRchanrtRrtDcnt = crate::Reg<pktdma_rchanrt_rrt_dcnt::PktdmaRchanrtRrtDcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_rchanrt_rrt_dcnt;
#[doc = "PKTDMA_RCHANRT_RRT_BCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_bcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_bcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_bcnt`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_BCNT")]
pub type PktdmaRchanrtRrtBcnt = crate::Reg<pktdma_rchanrt_rrt_bcnt::PktdmaRchanrtRrtBcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_rchanrt_rrt_bcnt;
#[doc = "PKTDMA_RCHANRT_RRT_SBCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_sbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_sbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchanrt_rrt_sbcnt`]
module"]
#[doc(alias = "PKTDMA_RCHANRT_RRT_SBCNT")]
pub type PktdmaRchanrtRrtSbcnt = crate::Reg<pktdma_rchanrt_rrt_sbcnt::PktdmaRchanrtRrtSbcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_rchanrt_rrt_sbcnt;
