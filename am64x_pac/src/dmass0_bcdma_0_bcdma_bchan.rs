#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_bchan_cfg: BcdmaBchanCfg,
    _reserved1: [u8; 0x60],
    bcdma_bchan_pri_ctrl: BcdmaBchanPriCtrl,
    _reserved2: [u8; 0x18],
    bcdma_bchan_st_sched: BcdmaBchanStSched,
}
impl RegisterBlock {
    #[doc = "0x00 - The Channel Configuration Register is used to initialize static mode settings for the Block Copy DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0)."]
    #[inline(always)]
    pub const fn bcdma_bchan_cfg(&self) -> &BcdmaBchanCfg {
        &self.bcdma_bchan_cfg
    }
    #[doc = "0x64 - The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
    #[inline(always)]
    pub const fn bcdma_bchan_pri_ctrl(&self) -> &BcdmaBchanPriCtrl {
        &self.bcdma_bchan_pri_ctrl
    }
    #[doc = "0x80 - The Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
    #[inline(always)]
    pub const fn bcdma_bchan_st_sched(&self) -> &BcdmaBchanStSched {
        &self.bcdma_bchan_st_sched
    }
}
#[doc = "BCDMA_BCHAN_CFG (rw) register accessor: The Channel Configuration Register is used to initialize static mode settings for the Block Copy DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchan_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchan_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchan_cfg`]
module"]
#[doc(alias = "BCDMA_BCHAN_CFG")]
pub type BcdmaBchanCfg = crate::Reg<bcdma_bchan_cfg::BcdmaBchanCfgSpec>;
#[doc = "The Channel Configuration Register is used to initialize static mode settings for the Block Copy DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0)."]
pub mod bcdma_bchan_cfg;
#[doc = "BCDMA_BCHAN_PRI_CTRL (rw) register accessor: The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchan_pri_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchan_pri_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchan_pri_ctrl`]
module"]
#[doc(alias = "BCDMA_BCHAN_PRI_CTRL")]
pub type BcdmaBchanPriCtrl = crate::Reg<bcdma_bchan_pri_ctrl::BcdmaBchanPriCtrlSpec>;
#[doc = "The priority control register is used to control the priority of the transactions which the DMA generates on it's master interface."]
pub mod bcdma_bchan_pri_ctrl;
#[doc = "BCDMA_BCHAN_ST_SCHED (rw) register accessor: The Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchan_st_sched::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchan_st_sched::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_bchan_st_sched`]
module"]
#[doc(alias = "BCDMA_BCHAN_ST_SCHED")]
pub type BcdmaBchanStSched = crate::Reg<bcdma_bchan_st_sched::BcdmaBchanStSchedSpec>;
#[doc = "The Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:"]
pub mod bcdma_bchan_st_sched;
