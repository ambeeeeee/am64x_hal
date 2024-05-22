#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_rchan_rcfg: BcdmaRchanRcfg,
    _reserved1: [u8; 0x60],
    bcdma_rchan_rpri_ctrl: BcdmaRchanRpriCtrl,
    bcdma_rchan_rthrd_id: BcdmaRchanRthrdId,
    _reserved3: [u8; 0x14],
    bcdma_rchan_rst_sched: BcdmaRchanRstSched,
}
impl RegisterBlock {
    #[doc = "0x00 - The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0)."]
    #[inline(always)]
    pub const fn bcdma_rchan_rcfg(&self) -> &BcdmaRchanRcfg {
        &self.bcdma_rchan_rcfg
    }
    #[doc = "0x64 - The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
    #[inline(always)]
    pub const fn bcdma_rchan_rpri_ctrl(&self) -> &BcdmaRchanRpriCtrl {
        &self.bcdma_rchan_rpri_ctrl
    }
    #[doc = "0x68 - The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
    #[inline(always)]
    pub const fn bcdma_rchan_rthrd_id(&self) -> &BcdmaRchanRthrdId {
        &self.bcdma_rchan_rthrd_id
    }
    #[doc = "0x80 - The Rx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
    #[inline(always)]
    pub const fn bcdma_rchan_rst_sched(&self) -> &BcdmaRchanRstSched {
        &self.bcdma_rchan_rst_sched
    }
}
#[doc = "BCDMA_RCHAN_RCFG (rw) register accessor: The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchan_rcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchan_rcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchan_rcfg`]
module"]
#[doc(alias = "BCDMA_RCHAN_RCFG")]
pub type BcdmaRchanRcfg = crate::Reg<bcdma_rchan_rcfg::BcdmaRchanRcfgSpec>;
#[doc = "The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0)."]
pub mod bcdma_rchan_rcfg;
#[doc = "BCDMA_RCHAN_RPRI_CTRL (rw) register accessor: The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchan_rpri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchan_rpri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchan_rpri_ctrl`]
module"]
#[doc(alias = "BCDMA_RCHAN_RPRI_CTRL")]
pub type BcdmaRchanRpriCtrl = crate::Reg<bcdma_rchan_rpri_ctrl::BcdmaRchanRpriCtrlSpec>;
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
pub mod bcdma_rchan_rpri_ctrl;
#[doc = "BCDMA_RCHAN_RTHRD_ID (rw) register accessor: The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchan_rthrd_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchan_rthrd_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchan_rthrd_id`]
module"]
#[doc(alias = "BCDMA_RCHAN_RTHRD_ID")]
pub type BcdmaRchanRthrdId = crate::Reg<bcdma_rchan_rthrd_id::BcdmaRchanRthrdIdSpec>;
#[doc = "The thread ID mapping register is used to pair the Rx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
pub mod bcdma_rchan_rthrd_id;
#[doc = "BCDMA_RCHAN_RST_SCHED (rw) register accessor: The Rx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchan_rst_sched::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchan_rst_sched::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_rchan_rst_sched`]
module"]
#[doc(alias = "BCDMA_RCHAN_RST_SCHED")]
pub type BcdmaRchanRstSched = crate::Reg<bcdma_rchan_rst_sched::BcdmaRchanRstSchedSpec>;
#[doc = "The Rx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
pub mod bcdma_rchan_rst_sched;
