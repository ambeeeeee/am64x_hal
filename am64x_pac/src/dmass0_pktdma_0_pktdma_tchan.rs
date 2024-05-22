#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pktdma_tchan_tcfg: PktdmaTchanTcfg,
    _reserved1: [u8; 0x60],
    pktdma_tchan_tpri_ctrl: PktdmaTchanTpriCtrl,
    pktdma_tchan_thrd_id: PktdmaTchanThrdId,
    _reserved3: [u8; 0x04],
    pktdma_tchan_tfifo_depth: PktdmaTchanTfifoDepth,
    _reserved4: [u8; 0x0c],
    pktdma_tchan_tst_sched: PktdmaTchanTstSched,
}
impl RegisterBlock {
    #[doc = "0x00 - The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0)."]
    #[inline(always)]
    pub const fn pktdma_tchan_tcfg(&self) -> &PktdmaTchanTcfg {
        &self.pktdma_tchan_tcfg
    }
    #[doc = "0x64 - The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
    #[inline(always)]
    pub const fn pktdma_tchan_tpri_ctrl(&self) -> &PktdmaTchanTpriCtrl {
        &self.pktdma_tchan_tpri_ctrl
    }
    #[doc = "0x68 - The thread ID mapping register is used to pair the Tx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
    #[inline(always)]
    pub const fn pktdma_tchan_thrd_id(&self) -> &PktdmaTchanThrdId {
        &self.pktdma_tchan_thrd_id
    }
    #[doc = "0x70 - The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects."]
    #[inline(always)]
    pub const fn pktdma_tchan_tfifo_depth(&self) -> &PktdmaTchanTfifoDepth {
        &self.pktdma_tchan_tfifo_depth
    }
    #[doc = "0x80 - The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
    #[inline(always)]
    pub const fn pktdma_tchan_tst_sched(&self) -> &PktdmaTchanTstSched {
        &self.pktdma_tchan_tst_sched
    }
}
#[doc = "PKTDMA_TCHAN_TCFG (rw) register accessor: The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_tcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_tcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchan_tcfg`]
module"]
#[doc(alias = "PKTDMA_TCHAN_TCFG")]
pub type PktdmaTchanTcfg = crate::Reg<pktdma_tchan_tcfg::PktdmaTchanTcfgSpec>;
#[doc = "The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0)."]
pub mod pktdma_tchan_tcfg;
#[doc = "PKTDMA_TCHAN_TPRI_CTRL (rw) register accessor: The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_tpri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_tpri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchan_tpri_ctrl`]
module"]
#[doc(alias = "PKTDMA_TCHAN_TPRI_CTRL")]
pub type PktdmaTchanTpriCtrl = crate::Reg<pktdma_tchan_tpri_ctrl::PktdmaTchanTpriCtrlSpec>;
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
pub mod pktdma_tchan_tpri_ctrl;
#[doc = "PKTDMA_TCHAN_THRD_ID (rw) register accessor: The thread ID mapping register is used to pair the Tx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_thrd_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_thrd_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchan_thrd_id`]
module"]
#[doc(alias = "PKTDMA_TCHAN_THRD_ID")]
pub type PktdmaTchanThrdId = crate::Reg<pktdma_tchan_thrd_id::PktdmaTchanThrdIdSpec>;
#[doc = "The thread ID mapping register is used to pair the Tx DMA channel to a specific destination thread. All traffic generated from this channel will be sent with a thread_id on the PSI-L interface with the value from this register."]
pub mod pktdma_tchan_thrd_id;
#[doc = "PKTDMA_TCHAN_TFIFO_DEPTH (rw) register accessor: The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_tfifo_depth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_tfifo_depth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchan_tfifo_depth`]
module"]
#[doc(alias = "PKTDMA_TCHAN_TFIFO_DEPTH")]
pub type PktdmaTchanTfifoDepth = crate::Reg<pktdma_tchan_tfifo_depth::PktdmaTchanTfifoDepthSpec>;
#[doc = "The fifo depth register is used to specify how many FIFO data phases deep the Tx per channel FIFO will be for the channel. While the maximum depth of the Tx FIFO is set at design time, the FIFO depth can be artificially reduced in order to control the maximum latency which can be introduced due to buffering effects."]
pub mod pktdma_tchan_tfifo_depth;
#[doc = "PKTDMA_TCHAN_TST_SCHED (rw) register accessor: The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_tst_sched::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_tst_sched::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_tchan_tst_sched`]
module"]
#[doc(alias = "PKTDMA_TCHAN_TST_SCHED")]
pub type PktdmaTchanTstSched = crate::Reg<pktdma_tchan_tst_sched::PktdmaTchanTstSchedSpec>;
#[doc = "The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
pub mod pktdma_tchan_tst_sched;
