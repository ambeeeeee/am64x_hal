#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pktdma_rchan_rcfg: PktdmaRchanRcfg,
    _reserved1: [u8; 0x60],
    pktdma_rchan_rpri_ctrl: PktdmaRchanRpriCtrl,
    pktdma_rchan_thrd_id: PktdmaRchanThrdId,
    _reserved3: [u8; 0x14],
    pktdma_rchan_rst_sched: PktdmaRchanRstSched,
}
impl RegisterBlock {
    #[doc = "0x00 - The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0)."]
    #[inline(always)]
    pub const fn pktdma_rchan_rcfg(&self) -> &PktdmaRchanRcfg {
        &self.pktdma_rchan_rcfg
    }
    #[doc = "0x64 - The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
    #[inline(always)]
    pub const fn pktdma_rchan_rpri_ctrl(&self) -> &PktdmaRchanRpriCtrl {
        &self.pktdma_rchan_rpri_ctrl
    }
    #[doc = "0x68 - The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
    #[inline(always)]
    pub const fn pktdma_rchan_thrd_id(&self) -> &PktdmaRchanThrdId {
        &self.pktdma_rchan_thrd_id
    }
    #[doc = "0x80 - The Rx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
    #[inline(always)]
    pub const fn pktdma_rchan_rst_sched(&self) -> &PktdmaRchanRstSched {
        &self.pktdma_rchan_rst_sched
    }
}
#[doc = "PKTDMA_RCHAN_RCFG (rw) register accessor: The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchan_rcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchan_rcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchan_rcfg`]
module"]
#[doc(alias = "PKTDMA_RCHAN_RCFG")]
pub type PktdmaRchanRcfg = crate::Reg<pktdma_rchan_rcfg::PktdmaRchanRcfgSpec>;
#[doc = "The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0)."]
pub mod pktdma_rchan_rcfg;
#[doc = "PKTDMA_RCHAN_RPRI_CTRL (rw) register accessor: The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchan_rpri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchan_rpri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchan_rpri_ctrl`]
module"]
#[doc(alias = "PKTDMA_RCHAN_RPRI_CTRL")]
pub type PktdmaRchanRpriCtrl = crate::Reg<pktdma_rchan_rpri_ctrl::PktdmaRchanRpriCtrlSpec>;
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
pub mod pktdma_rchan_rpri_ctrl;
#[doc = "PKTDMA_RCHAN_THRD_ID (rw) register accessor: The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchan_thrd_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchan_thrd_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchan_thrd_id`]
module"]
#[doc(alias = "PKTDMA_RCHAN_THRD_ID")]
pub type PktdmaRchanThrdId = crate::Reg<pktdma_rchan_thrd_id::PktdmaRchanThrdIdSpec>;
#[doc = "The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
pub mod pktdma_rchan_thrd_id;
#[doc = "PKTDMA_RCHAN_RST_SCHED (rw) register accessor: The Rx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchan_rst_sched::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchan_rst_sched::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rchan_rst_sched`]
module"]
#[doc(alias = "PKTDMA_RCHAN_RST_SCHED")]
pub type PktdmaRchanRstSched = crate::Reg<pktdma_rchan_rst_sched::PktdmaRchanRstSchedSpec>;
#[doc = "The Rx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
pub mod pktdma_rchan_rst_sched;
