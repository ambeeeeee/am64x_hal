#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pktdma_tchanrt_trt_ctl: PktdmaTchanrtTrtCtl,
    _reserved1: [u8; 0x3c],
    pktdma_tchanrt_trt_status0: PktdmaTchanrtTrtStatus0,
    pktdma_tchanrt_trt_status1: PktdmaTchanrtTrtStatus1,
    _reserved3: [u8; 0x38],
    pktdma_tchanrt_trt_stdata: PktdmaTchanrtTrtStdata,
    _reserved4: [u8; 0x017c],
    pktdma_tchanrt_trt_peer0: PktdmaTchanrtTrtPeer0,
    pktdma_tchanrt_trt_peer1: PktdmaTchanrtTrtPeer1,
    pktdma_tchanrt_trt_peer2: PktdmaTchanrtTrtPeer2,
    pktdma_tchanrt_trt_peer3: PktdmaTchanrtTrtPeer3,
    pktdma_tchanrt_trt_peer4: PktdmaTchanrtTrtPeer4,
    pktdma_tchanrt_trt_peer5: PktdmaTchanrtTrtPeer5,
    pktdma_tchanrt_trt_peer6: PktdmaTchanrtTrtPeer6,
    pktdma_tchanrt_trt_peer7: PktdmaTchanrtTrtPeer7,
    pktdma_tchanrt_trt_peer8: PktdmaTchanrtTrtPeer8,
    pktdma_tchanrt_trt_peer9: PktdmaTchanrtTrtPeer9,
    pktdma_tchanrt_trt_peer10: PktdmaTchanrtTrtPeer10,
    pktdma_tchanrt_trt_peer11: PktdmaTchanrtTrtPeer11,
    pktdma_tchanrt_trt_peer12: PktdmaTchanrtTrtPeer12,
    pktdma_tchanrt_trt_peer13: PktdmaTchanrtTrtPeer13,
    pktdma_tchanrt_trt_peer14: PktdmaTchanrtTrtPeer14,
    pktdma_tchanrt_trt_peer15: PktdmaTchanrtTrtPeer15,
    _reserved20: [u8; 0x01c0],
    pktdma_tchanrt_trt_pcnt: PktdmaTchanrtTrtPcnt,
    _reserved21: [u8; 0x04],
    pktdma_tchanrt_trt_bcnt: PktdmaTchanrtTrtBcnt,
    _reserved22: [u8; 0x04],
    pktdma_tchanrt_trt_sbcnt: PktdmaTchanrtTrtSbcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_ctl(&self) -> &PktdmaTchanrtTrtCtl {
        &self.pktdma_tchanrt_trt_ctl
    }
    #[doc = "0x40 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_status0(&self) -> &PktdmaTchanrtTrtStatus0 {
        &self.pktdma_tchanrt_trt_status0
    }
    #[doc = "0x44 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_status1(&self) -> &PktdmaTchanrtTrtStatus1 {
        &self.pktdma_tchanrt_trt_status1
    }
    #[doc = "0x80 - The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the PKTDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_stdata(&self) -> &PktdmaTchanrtTrtStdata {
        &self.pktdma_tchanrt_trt_stdata
    }
    #[doc = "0x200 - This register provides access to the remote peer's realtime register at 0x400."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer0(&self) -> &PktdmaTchanrtTrtPeer0 {
        &self.pktdma_tchanrt_trt_peer0
    }
    #[doc = "0x204 - This register provides access to the remote peer's realtime register at 0x401."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer1(&self) -> &PktdmaTchanrtTrtPeer1 {
        &self.pktdma_tchanrt_trt_peer1
    }
    #[doc = "0x208 - This register provides access to the remote peer's realtime register at 0x402."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer2(&self) -> &PktdmaTchanrtTrtPeer2 {
        &self.pktdma_tchanrt_trt_peer2
    }
    #[doc = "0x20c - This register provides access to the remote peer's realtime register at 0x403."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer3(&self) -> &PktdmaTchanrtTrtPeer3 {
        &self.pktdma_tchanrt_trt_peer3
    }
    #[doc = "0x210 - This register provides access to the remote peer's realtime register at 0x404."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer4(&self) -> &PktdmaTchanrtTrtPeer4 {
        &self.pktdma_tchanrt_trt_peer4
    }
    #[doc = "0x214 - This register provides access to the remote peer's realtime register at 0x405."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer5(&self) -> &PktdmaTchanrtTrtPeer5 {
        &self.pktdma_tchanrt_trt_peer5
    }
    #[doc = "0x218 - This register provides access to the remote peer's realtime register at 0x406."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer6(&self) -> &PktdmaTchanrtTrtPeer6 {
        &self.pktdma_tchanrt_trt_peer6
    }
    #[doc = "0x21c - This register provides access to the remote peer's realtime register at 0x407."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer7(&self) -> &PktdmaTchanrtTrtPeer7 {
        &self.pktdma_tchanrt_trt_peer7
    }
    #[doc = "0x220 - This register provides access to the remote peer's realtime register at 0x408."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer8(&self) -> &PktdmaTchanrtTrtPeer8 {
        &self.pktdma_tchanrt_trt_peer8
    }
    #[doc = "0x224 - This register provides access to the remote peer's realtime register at 0x409."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer9(&self) -> &PktdmaTchanrtTrtPeer9 {
        &self.pktdma_tchanrt_trt_peer9
    }
    #[doc = "0x228 - This register provides access to the remote peer's realtime register at 0x40A."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer10(&self) -> &PktdmaTchanrtTrtPeer10 {
        &self.pktdma_tchanrt_trt_peer10
    }
    #[doc = "0x22c - This register provides access to the remote peer's realtime register at 0x40B."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer11(&self) -> &PktdmaTchanrtTrtPeer11 {
        &self.pktdma_tchanrt_trt_peer11
    }
    #[doc = "0x230 - This register provides access to the remote peer's realtime register at 0x40C."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer12(&self) -> &PktdmaTchanrtTrtPeer12 {
        &self.pktdma_tchanrt_trt_peer12
    }
    #[doc = "0x234 - This register provides access to the remote peer's realtime register at 0x40D."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer13(&self) -> &PktdmaTchanrtTrtPeer13 {
        &self.pktdma_tchanrt_trt_peer13
    }
    #[doc = "0x238 - This register provides access to the remote peer's realtime register at 0x40E."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer14(&self) -> &PktdmaTchanrtTrtPeer14 {
        &self.pktdma_tchanrt_trt_peer14
    }
    #[doc = "0x23c - This register provides access to the remote peer's realtime register at 0x40F."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_peer15(&self) -> &PktdmaTchanrtTrtPeer15 {
        &self.pktdma_tchanrt_trt_peer15
    }
    #[doc = "0x400 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_pcnt(&self) -> &PktdmaTchanrtTrtPcnt {
        &self.pktdma_tchanrt_trt_pcnt
    }
    #[doc = "0x408 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_bcnt(&self) -> &PktdmaTchanrtTrtBcnt {
        &self.pktdma_tchanrt_trt_bcnt
    }
    #[doc = "0x410 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn pktdma_tchanrt_trt_sbcnt(&self) -> &PktdmaTchanrtTrtSbcnt {
        &self.pktdma_tchanrt_trt_sbcnt
    }
}
#[doc = "PKTDMA_TCHANRT_TRT_CTL (rw) register accessor: The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_ctl`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_CTL")]
pub type PktdmaTchanrtTrtCtl = crate::Reg<pktdma_tchanrt_trt_ctl::PktdmaTchanrtTrtCtlSpec>;
#[doc = "The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
pub mod pktdma_tchanrt_trt_ctl;
#[doc = "PKTDMA_TCHANRT_TRT_STATUS0 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_status0`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_STATUS0")]
pub type PktdmaTchanrtTrtStatus0 =
    crate::Reg<pktdma_tchanrt_trt_status0::PktdmaTchanrtTrtStatus0Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod pktdma_tchanrt_trt_status0;
#[doc = "PKTDMA_TCHANRT_TRT_STATUS1 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_status1`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_STATUS1")]
pub type PktdmaTchanrtTrtStatus1 =
    crate::Reg<pktdma_tchanrt_trt_status1::PktdmaTchanrtTrtStatus1Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod pktdma_tchanrt_trt_status1;
#[doc = "PKTDMA_TCHANRT_TRT_STDATA (rw) register accessor: The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the PKTDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_stdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_stdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_stdata`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_STDATA")]
pub type PktdmaTchanrtTrtStdata = crate::Reg<pktdma_tchanrt_trt_stdata::PktdmaTchanrtTrtStdataSpec>;
#[doc = "The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the PKTDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
pub mod pktdma_tchanrt_trt_stdata;
#[doc = "PKTDMA_TCHANRT_TRT_PEER0 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x400.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer0`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER0")]
pub type PktdmaTchanrtTrtPeer0 = crate::Reg<pktdma_tchanrt_trt_peer0::PktdmaTchanrtTrtPeer0Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x400."]
pub mod pktdma_tchanrt_trt_peer0;
#[doc = "PKTDMA_TCHANRT_TRT_PEER1 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x401.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer1`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER1")]
pub type PktdmaTchanrtTrtPeer1 = crate::Reg<pktdma_tchanrt_trt_peer1::PktdmaTchanrtTrtPeer1Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x401."]
pub mod pktdma_tchanrt_trt_peer1;
#[doc = "PKTDMA_TCHANRT_TRT_PEER2 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x402.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer2`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER2")]
pub type PktdmaTchanrtTrtPeer2 = crate::Reg<pktdma_tchanrt_trt_peer2::PktdmaTchanrtTrtPeer2Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x402."]
pub mod pktdma_tchanrt_trt_peer2;
#[doc = "PKTDMA_TCHANRT_TRT_PEER3 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x403.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer3`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER3")]
pub type PktdmaTchanrtTrtPeer3 = crate::Reg<pktdma_tchanrt_trt_peer3::PktdmaTchanrtTrtPeer3Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x403."]
pub mod pktdma_tchanrt_trt_peer3;
#[doc = "PKTDMA_TCHANRT_TRT_PEER4 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x404.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer4`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER4")]
pub type PktdmaTchanrtTrtPeer4 = crate::Reg<pktdma_tchanrt_trt_peer4::PktdmaTchanrtTrtPeer4Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x404."]
pub mod pktdma_tchanrt_trt_peer4;
#[doc = "PKTDMA_TCHANRT_TRT_PEER5 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x405.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer5`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER5")]
pub type PktdmaTchanrtTrtPeer5 = crate::Reg<pktdma_tchanrt_trt_peer5::PktdmaTchanrtTrtPeer5Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x405."]
pub mod pktdma_tchanrt_trt_peer5;
#[doc = "PKTDMA_TCHANRT_TRT_PEER6 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x406.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer6`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER6")]
pub type PktdmaTchanrtTrtPeer6 = crate::Reg<pktdma_tchanrt_trt_peer6::PktdmaTchanrtTrtPeer6Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x406."]
pub mod pktdma_tchanrt_trt_peer6;
#[doc = "PKTDMA_TCHANRT_TRT_PEER7 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x407.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer7`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER7")]
pub type PktdmaTchanrtTrtPeer7 = crate::Reg<pktdma_tchanrt_trt_peer7::PktdmaTchanrtTrtPeer7Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x407."]
pub mod pktdma_tchanrt_trt_peer7;
#[doc = "PKTDMA_TCHANRT_TRT_PEER8 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x408.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer8`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER8")]
pub type PktdmaTchanrtTrtPeer8 = crate::Reg<pktdma_tchanrt_trt_peer8::PktdmaTchanrtTrtPeer8Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x408."]
pub mod pktdma_tchanrt_trt_peer8;
#[doc = "PKTDMA_TCHANRT_TRT_PEER9 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x409.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer9`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER9")]
pub type PktdmaTchanrtTrtPeer9 = crate::Reg<pktdma_tchanrt_trt_peer9::PktdmaTchanrtTrtPeer9Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x409."]
pub mod pktdma_tchanrt_trt_peer9;
#[doc = "PKTDMA_TCHANRT_TRT_PEER10 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer10`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER10")]
pub type PktdmaTchanrtTrtPeer10 = crate::Reg<pktdma_tchanrt_trt_peer10::PktdmaTchanrtTrtPeer10Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40A."]
pub mod pktdma_tchanrt_trt_peer10;
#[doc = "PKTDMA_TCHANRT_TRT_PEER11 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer11`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER11")]
pub type PktdmaTchanrtTrtPeer11 = crate::Reg<pktdma_tchanrt_trt_peer11::PktdmaTchanrtTrtPeer11Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40B."]
pub mod pktdma_tchanrt_trt_peer11;
#[doc = "PKTDMA_TCHANRT_TRT_PEER12 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer12`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER12")]
pub type PktdmaTchanrtTrtPeer12 = crate::Reg<pktdma_tchanrt_trt_peer12::PktdmaTchanrtTrtPeer12Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40C."]
pub mod pktdma_tchanrt_trt_peer12;
#[doc = "PKTDMA_TCHANRT_TRT_PEER13 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer13`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER13")]
pub type PktdmaTchanrtTrtPeer13 = crate::Reg<pktdma_tchanrt_trt_peer13::PktdmaTchanrtTrtPeer13Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40D."]
pub mod pktdma_tchanrt_trt_peer13;
#[doc = "PKTDMA_TCHANRT_TRT_PEER14 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40E.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer14`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER14")]
pub type PktdmaTchanrtTrtPeer14 = crate::Reg<pktdma_tchanrt_trt_peer14::PktdmaTchanrtTrtPeer14Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40E."]
pub mod pktdma_tchanrt_trt_peer14;
#[doc = "PKTDMA_TCHANRT_TRT_PEER15 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40F.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_peer15`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PEER15")]
pub type PktdmaTchanrtTrtPeer15 = crate::Reg<pktdma_tchanrt_trt_peer15::PktdmaTchanrtTrtPeer15Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40F."]
pub mod pktdma_tchanrt_trt_peer15;
#[doc = "PKTDMA_TCHANRT_TRT_PCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_pcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_pcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_pcnt`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_PCNT")]
pub type PktdmaTchanrtTrtPcnt = crate::Reg<pktdma_tchanrt_trt_pcnt::PktdmaTchanrtTrtPcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_tchanrt_trt_pcnt;
#[doc = "PKTDMA_TCHANRT_TRT_BCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_bcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_bcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_bcnt`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_BCNT")]
pub type PktdmaTchanrtTrtBcnt = crate::Reg<pktdma_tchanrt_trt_bcnt::PktdmaTchanrtTrtBcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_tchanrt_trt_bcnt;
#[doc = "PKTDMA_TCHANRT_TRT_SBCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_sbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_sbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchanrt_trt_sbcnt`]
module"]
#[doc(alias = "PKTDMA_TCHANRT_TRT_SBCNT")]
pub type PktdmaTchanrtTrtSbcnt = crate::Reg<pktdma_tchanrt_trt_sbcnt::PktdmaTchanrtTrtSbcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod pktdma_tchanrt_trt_sbcnt;
