#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_tchanrt_trt_ctl: BcdmaTchanrtTrtCtl,
    _reserved1: [u8; 0x04],
    bcdma_tchanrt_trt_swtrig: BcdmaTchanrtTrtSwtrig,
    _reserved2: [u8; 0x34],
    bcdma_tchanrt_trt_status0: BcdmaTchanrtTrtStatus0,
    bcdma_tchanrt_trt_status1: BcdmaTchanrtTrtStatus1,
    _reserved4: [u8; 0x38],
    bcdma_tchanrt_trt_stdata: BcdmaTchanrtTrtStdata,
    _reserved5: [u8; 0x017c],
    bcdma_tchanrt_trt_peer0: BcdmaTchanrtTrtPeer0,
    bcdma_tchanrt_trt_peer1: BcdmaTchanrtTrtPeer1,
    bcdma_tchanrt_trt_peer2: BcdmaTchanrtTrtPeer2,
    bcdma_tchanrt_trt_peer3: BcdmaTchanrtTrtPeer3,
    bcdma_tchanrt_trt_peer4: BcdmaTchanrtTrtPeer4,
    bcdma_tchanrt_trt_peer5: BcdmaTchanrtTrtPeer5,
    bcdma_tchanrt_trt_peer6: BcdmaTchanrtTrtPeer6,
    bcdma_tchanrt_trt_peer7: BcdmaTchanrtTrtPeer7,
    bcdma_tchanrt_trt_peer8: BcdmaTchanrtTrtPeer8,
    bcdma_tchanrt_trt_peer9: BcdmaTchanrtTrtPeer9,
    bcdma_tchanrt_trt_peer10: BcdmaTchanrtTrtPeer10,
    bcdma_tchanrt_trt_peer11: BcdmaTchanrtTrtPeer11,
    bcdma_tchanrt_trt_peer12: BcdmaTchanrtTrtPeer12,
    bcdma_tchanrt_trt_peer13: BcdmaTchanrtTrtPeer13,
    bcdma_tchanrt_trt_peer14: BcdmaTchanrtTrtPeer14,
    bcdma_tchanrt_trt_peer15: BcdmaTchanrtTrtPeer15,
    _reserved21: [u8; 0x01c0],
    bcdma_tchanrt_trt_pcnt: BcdmaTchanrtTrtPcnt,
    _reserved22: [u8; 0x04],
    bcdma_tchanrt_trt_bcnt: BcdmaTchanrtTrtBcnt,
    _reserved23: [u8; 0x04],
    bcdma_tchanrt_trt_sbcnt: BcdmaTchanrtTrtSbcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_ctl(&self) -> &BcdmaTchanrtTrtCtl {
        &self.bcdma_tchanrt_trt_ctl
    }
    #[doc = "0x08 - The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_swtrig(&self) -> &BcdmaTchanrtTrtSwtrig {
        &self.bcdma_tchanrt_trt_swtrig
    }
    #[doc = "0x40 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_status0(&self) -> &BcdmaTchanrtTrtStatus0 {
        &self.bcdma_tchanrt_trt_status0
    }
    #[doc = "0x44 - The Status Register provides a read only view of channel status bits."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_status1(&self) -> &BcdmaTchanrtTrtStatus1 {
        &self.bcdma_tchanrt_trt_status1
    }
    #[doc = "0x80 - The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_stdata(&self) -> &BcdmaTchanrtTrtStdata {
        &self.bcdma_tchanrt_trt_stdata
    }
    #[doc = "0x200 - This register provides access to the remote peer's realtime register at 0x400."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer0(&self) -> &BcdmaTchanrtTrtPeer0 {
        &self.bcdma_tchanrt_trt_peer0
    }
    #[doc = "0x204 - This register provides access to the remote peer's realtime register at 0x401."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer1(&self) -> &BcdmaTchanrtTrtPeer1 {
        &self.bcdma_tchanrt_trt_peer1
    }
    #[doc = "0x208 - This register provides access to the remote peer's realtime register at 0x402."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer2(&self) -> &BcdmaTchanrtTrtPeer2 {
        &self.bcdma_tchanrt_trt_peer2
    }
    #[doc = "0x20c - This register provides access to the remote peer's realtime register at 0x403."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer3(&self) -> &BcdmaTchanrtTrtPeer3 {
        &self.bcdma_tchanrt_trt_peer3
    }
    #[doc = "0x210 - This register provides access to the remote peer's realtime register at 0x404."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer4(&self) -> &BcdmaTchanrtTrtPeer4 {
        &self.bcdma_tchanrt_trt_peer4
    }
    #[doc = "0x214 - This register provides access to the remote peer's realtime register at 0x405."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer5(&self) -> &BcdmaTchanrtTrtPeer5 {
        &self.bcdma_tchanrt_trt_peer5
    }
    #[doc = "0x218 - This register provides access to the remote peer's realtime register at 0x406."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer6(&self) -> &BcdmaTchanrtTrtPeer6 {
        &self.bcdma_tchanrt_trt_peer6
    }
    #[doc = "0x21c - This register provides access to the remote peer's realtime register at 0x407."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer7(&self) -> &BcdmaTchanrtTrtPeer7 {
        &self.bcdma_tchanrt_trt_peer7
    }
    #[doc = "0x220 - This register provides access to the remote peer's realtime register at 0x408."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer8(&self) -> &BcdmaTchanrtTrtPeer8 {
        &self.bcdma_tchanrt_trt_peer8
    }
    #[doc = "0x224 - This register provides access to the remote peer's realtime register at 0x409."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer9(&self) -> &BcdmaTchanrtTrtPeer9 {
        &self.bcdma_tchanrt_trt_peer9
    }
    #[doc = "0x228 - This register provides access to the remote peer's realtime register at 0x40A."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer10(&self) -> &BcdmaTchanrtTrtPeer10 {
        &self.bcdma_tchanrt_trt_peer10
    }
    #[doc = "0x22c - This register provides access to the remote peer's realtime register at 0x40B."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer11(&self) -> &BcdmaTchanrtTrtPeer11 {
        &self.bcdma_tchanrt_trt_peer11
    }
    #[doc = "0x230 - This register provides access to the remote peer's realtime register at 0x40C."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer12(&self) -> &BcdmaTchanrtTrtPeer12 {
        &self.bcdma_tchanrt_trt_peer12
    }
    #[doc = "0x234 - This register provides access to the remote peer's realtime register at 0x40D."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer13(&self) -> &BcdmaTchanrtTrtPeer13 {
        &self.bcdma_tchanrt_trt_peer13
    }
    #[doc = "0x238 - This register provides access to the remote peer's realtime register at 0x40E."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer14(&self) -> &BcdmaTchanrtTrtPeer14 {
        &self.bcdma_tchanrt_trt_peer14
    }
    #[doc = "0x23c - This register provides access to the remote peer's realtime register at 0x40F."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_peer15(&self) -> &BcdmaTchanrtTrtPeer15 {
        &self.bcdma_tchanrt_trt_peer15
    }
    #[doc = "0x400 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_pcnt(&self) -> &BcdmaTchanrtTrtPcnt {
        &self.bcdma_tchanrt_trt_pcnt
    }
    #[doc = "0x408 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_bcnt(&self) -> &BcdmaTchanrtTrtBcnt {
        &self.bcdma_tchanrt_trt_bcnt
    }
    #[doc = "0x410 - The statistics registers are supplied to give software applications operational progress status for the channel."]
    #[inline(always)]
    pub const fn bcdma_tchanrt_trt_sbcnt(&self) -> &BcdmaTchanrtTrtSbcnt {
        &self.bcdma_tchanrt_trt_sbcnt
    }
}
#[doc = "BCDMA_TCHANRT_TRT_CTL (rw) register accessor: The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_ctl`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_CTL")]
pub type BcdmaTchanrtTrtCtl = crate::Reg<bcdma_tchanrt_trt_ctl::BcdmaTchanrtTrtCtlSpec>;
#[doc = "The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation."]
pub mod bcdma_tchanrt_trt_ctl;
#[doc = "BCDMA_TCHANRT_TRT_SWTRIG (rw) register accessor: The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_swtrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_swtrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_swtrig`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_SWTRIG")]
pub type BcdmaTchanrtTrtSwtrig = crate::Reg<bcdma_tchanrt_trt_swtrig::BcdmaTchanrtTrtSwtrigSpec>;
#[doc = "The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel."]
pub mod bcdma_tchanrt_trt_swtrig;
#[doc = "BCDMA_TCHANRT_TRT_STATUS0 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_status0`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_STATUS0")]
pub type BcdmaTchanrtTrtStatus0 = crate::Reg<bcdma_tchanrt_trt_status0::BcdmaTchanrtTrtStatus0Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_tchanrt_trt_status0;
#[doc = "BCDMA_TCHANRT_TRT_STATUS1 (rw) register accessor: The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_status1`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_STATUS1")]
pub type BcdmaTchanrtTrtStatus1 = crate::Reg<bcdma_tchanrt_trt_status1::BcdmaTchanrtTrtStatus1Spec>;
#[doc = "The Status Register provides a read only view of channel status bits."]
pub mod bcdma_tchanrt_trt_status1;
#[doc = "BCDMA_TCHANRT_TRT_STDATA (rw) register accessor: The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_stdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_stdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_stdata`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_STDATA")]
pub type BcdmaTchanrtTrtStdata = crate::Reg<bcdma_tchanrt_trt_stdata::BcdmaTchanrtTrtStdataSpec>;
#[doc = "The State Data Registers contain the current working state of the Tx DMA channel. These registers are provided so that the Host can determine the potential cause of an error or exception condition which was reported by the channel. These registers should not be accessed without reason while the BCDMA is operating as accesses will cause performance to decrease as these MMRs are just providing a window into the actual state RAM"]
pub mod bcdma_tchanrt_trt_stdata;
#[doc = "BCDMA_TCHANRT_TRT_PEER0 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x400.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer0`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER0")]
pub type BcdmaTchanrtTrtPeer0 = crate::Reg<bcdma_tchanrt_trt_peer0::BcdmaTchanrtTrtPeer0Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x400."]
pub mod bcdma_tchanrt_trt_peer0;
#[doc = "BCDMA_TCHANRT_TRT_PEER1 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x401.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer1`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER1")]
pub type BcdmaTchanrtTrtPeer1 = crate::Reg<bcdma_tchanrt_trt_peer1::BcdmaTchanrtTrtPeer1Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x401."]
pub mod bcdma_tchanrt_trt_peer1;
#[doc = "BCDMA_TCHANRT_TRT_PEER2 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x402.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer2`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER2")]
pub type BcdmaTchanrtTrtPeer2 = crate::Reg<bcdma_tchanrt_trt_peer2::BcdmaTchanrtTrtPeer2Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x402."]
pub mod bcdma_tchanrt_trt_peer2;
#[doc = "BCDMA_TCHANRT_TRT_PEER3 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x403.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer3`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER3")]
pub type BcdmaTchanrtTrtPeer3 = crate::Reg<bcdma_tchanrt_trt_peer3::BcdmaTchanrtTrtPeer3Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x403."]
pub mod bcdma_tchanrt_trt_peer3;
#[doc = "BCDMA_TCHANRT_TRT_PEER4 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x404.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer4`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER4")]
pub type BcdmaTchanrtTrtPeer4 = crate::Reg<bcdma_tchanrt_trt_peer4::BcdmaTchanrtTrtPeer4Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x404."]
pub mod bcdma_tchanrt_trt_peer4;
#[doc = "BCDMA_TCHANRT_TRT_PEER5 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x405.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer5`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER5")]
pub type BcdmaTchanrtTrtPeer5 = crate::Reg<bcdma_tchanrt_trt_peer5::BcdmaTchanrtTrtPeer5Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x405."]
pub mod bcdma_tchanrt_trt_peer5;
#[doc = "BCDMA_TCHANRT_TRT_PEER6 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x406.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer6`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER6")]
pub type BcdmaTchanrtTrtPeer6 = crate::Reg<bcdma_tchanrt_trt_peer6::BcdmaTchanrtTrtPeer6Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x406."]
pub mod bcdma_tchanrt_trt_peer6;
#[doc = "BCDMA_TCHANRT_TRT_PEER7 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x407.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer7`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER7")]
pub type BcdmaTchanrtTrtPeer7 = crate::Reg<bcdma_tchanrt_trt_peer7::BcdmaTchanrtTrtPeer7Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x407."]
pub mod bcdma_tchanrt_trt_peer7;
#[doc = "BCDMA_TCHANRT_TRT_PEER8 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x408.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer8`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER8")]
pub type BcdmaTchanrtTrtPeer8 = crate::Reg<bcdma_tchanrt_trt_peer8::BcdmaTchanrtTrtPeer8Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x408."]
pub mod bcdma_tchanrt_trt_peer8;
#[doc = "BCDMA_TCHANRT_TRT_PEER9 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x409.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer9`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER9")]
pub type BcdmaTchanrtTrtPeer9 = crate::Reg<bcdma_tchanrt_trt_peer9::BcdmaTchanrtTrtPeer9Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x409."]
pub mod bcdma_tchanrt_trt_peer9;
#[doc = "BCDMA_TCHANRT_TRT_PEER10 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer10`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER10")]
pub type BcdmaTchanrtTrtPeer10 = crate::Reg<bcdma_tchanrt_trt_peer10::BcdmaTchanrtTrtPeer10Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40A."]
pub mod bcdma_tchanrt_trt_peer10;
#[doc = "BCDMA_TCHANRT_TRT_PEER11 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer11`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER11")]
pub type BcdmaTchanrtTrtPeer11 = crate::Reg<bcdma_tchanrt_trt_peer11::BcdmaTchanrtTrtPeer11Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40B."]
pub mod bcdma_tchanrt_trt_peer11;
#[doc = "BCDMA_TCHANRT_TRT_PEER12 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer12`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER12")]
pub type BcdmaTchanrtTrtPeer12 = crate::Reg<bcdma_tchanrt_trt_peer12::BcdmaTchanrtTrtPeer12Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40C."]
pub mod bcdma_tchanrt_trt_peer12;
#[doc = "BCDMA_TCHANRT_TRT_PEER13 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer13`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER13")]
pub type BcdmaTchanrtTrtPeer13 = crate::Reg<bcdma_tchanrt_trt_peer13::BcdmaTchanrtTrtPeer13Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40D."]
pub mod bcdma_tchanrt_trt_peer13;
#[doc = "BCDMA_TCHANRT_TRT_PEER14 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40E.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer14`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER14")]
pub type BcdmaTchanrtTrtPeer14 = crate::Reg<bcdma_tchanrt_trt_peer14::BcdmaTchanrtTrtPeer14Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40E."]
pub mod bcdma_tchanrt_trt_peer14;
#[doc = "BCDMA_TCHANRT_TRT_PEER15 (rw) register accessor: This register provides access to the remote peer's realtime register at 0x40F.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_peer15`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PEER15")]
pub type BcdmaTchanrtTrtPeer15 = crate::Reg<bcdma_tchanrt_trt_peer15::BcdmaTchanrtTrtPeer15Spec>;
#[doc = "This register provides access to the remote peer's realtime register at 0x40F."]
pub mod bcdma_tchanrt_trt_peer15;
#[doc = "BCDMA_TCHANRT_TRT_PCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_pcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_pcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_pcnt`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_PCNT")]
pub type BcdmaTchanrtTrtPcnt = crate::Reg<bcdma_tchanrt_trt_pcnt::BcdmaTchanrtTrtPcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_tchanrt_trt_pcnt;
#[doc = "BCDMA_TCHANRT_TRT_BCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_bcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_bcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_bcnt`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_BCNT")]
pub type BcdmaTchanrtTrtBcnt = crate::Reg<bcdma_tchanrt_trt_bcnt::BcdmaTchanrtTrtBcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_tchanrt_trt_bcnt;
#[doc = "BCDMA_TCHANRT_TRT_SBCNT (rw) register accessor: The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_sbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_sbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchanrt_trt_sbcnt`]
module"]
#[doc(alias = "BCDMA_TCHANRT_TRT_SBCNT")]
pub type BcdmaTchanrtTrtSbcnt = crate::Reg<bcdma_tchanrt_trt_sbcnt::BcdmaTchanrtTrtSbcntSpec>;
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel."]
pub mod bcdma_tchanrt_trt_sbcnt;
