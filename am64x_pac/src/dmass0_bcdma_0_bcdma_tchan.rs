#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_tchan_tcfg: BcdmaTchanTcfg,
    _reserved1: [u8; 0x60],
    bcdma_tchan_tpri_ctrl: BcdmaTchanTpriCtrl,
    bcdma_tchan_tthrd_id: BcdmaTchanTthrdId,
    _reserved3: [u8; 0x04],
    bcdma_tchan_tfifo_depth: BcdmaTchanTfifoDepth,
    _reserved4: [u8; 0x0c],
    bcdma_tchan_tst_sched: BcdmaTchanTstSched,
}
impl RegisterBlock {
    #[doc = "0x00 - The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0)."]
    #[inline(always)]
    pub const fn bcdma_tchan_tcfg(&self) -> &BcdmaTchanTcfg {
        &self.bcdma_tchan_tcfg
    }
    #[doc = "0x64 - The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
    #[inline(always)]
    pub const fn bcdma_tchan_tpri_ctrl(&self) -> &BcdmaTchanTpriCtrl {
        &self.bcdma_tchan_tpri_ctrl
    }
    #[doc = "0x68 - The thread ID mapping register is used to pair the Tx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
    #[inline(always)]
    pub const fn bcdma_tchan_tthrd_id(&self) -> &BcdmaTchanTthrdId {
        &self.bcdma_tchan_tthrd_id
    }
    #[doc = "0x70 - The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects."]
    #[inline(always)]
    pub const fn bcdma_tchan_tfifo_depth(&self) -> &BcdmaTchanTfifoDepth {
        &self.bcdma_tchan_tfifo_depth
    }
    #[doc = "0x80 - The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
    #[inline(always)]
    pub const fn bcdma_tchan_tst_sched(&self) -> &BcdmaTchanTstSched {
        &self.bcdma_tchan_tst_sched
    }
}
#[doc = "BCDMA_TCHAN_TCFG (rw) register accessor: The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchan_tcfg`]
module"]
#[doc(alias = "BCDMA_TCHAN_TCFG")]
pub type BcdmaTchanTcfg = crate::Reg<bcdma_tchan_tcfg::BcdmaTchanTcfgSpec>;
#[doc = "The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0)."]
pub mod bcdma_tchan_tcfg;
#[doc = "BCDMA_TCHAN_TPRI_CTRL (rw) register accessor: The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tpri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tpri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchan_tpri_ctrl`]
module"]
#[doc(alias = "BCDMA_TCHAN_TPRI_CTRL")]
pub type BcdmaTchanTpriCtrl = crate::Reg<bcdma_tchan_tpri_ctrl::BcdmaTchanTpriCtrlSpec>;
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
pub mod bcdma_tchan_tpri_ctrl;
#[doc = "BCDMA_TCHAN_TTHRD_ID (rw) register accessor: The thread ID mapping register is used to pair the Tx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tthrd_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tthrd_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchan_tthrd_id`]
module"]
#[doc(alias = "BCDMA_TCHAN_TTHRD_ID")]
pub type BcdmaTchanTthrdId = crate::Reg<bcdma_tchan_tthrd_id::BcdmaTchanTthrdIdSpec>;
#[doc = "The thread ID mapping register is used to pair the Tx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
pub mod bcdma_tchan_tthrd_id;
#[doc = "BCDMA_TCHAN_TFIFO_DEPTH (rw) register accessor: The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tfifo_depth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tfifo_depth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchan_tfifo_depth`]
module"]
#[doc(alias = "BCDMA_TCHAN_TFIFO_DEPTH")]
pub type BcdmaTchanTfifoDepth = crate::Reg<bcdma_tchan_tfifo_depth::BcdmaTchanTfifoDepthSpec>;
#[doc = "The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects."]
pub mod bcdma_tchan_tfifo_depth;
#[doc = "BCDMA_TCHAN_TST_SCHED (rw) register accessor: The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tst_sched::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tst_sched::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_tchan_tst_sched`]
module"]
#[doc(alias = "BCDMA_TCHAN_TST_SCHED")]
pub type BcdmaTchanTstSched = crate::Reg<bcdma_tchan_tst_sched::BcdmaTchanTstSchedSpec>;
#[doc = "The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
pub mod bcdma_tchan_tst_sched;
