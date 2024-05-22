#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mmr__mmrvbp__mcanss_regs_mcanss_pid: Mmr_Mmrvbp_McanssRegsMcanssPid,
    mmr__mmrvbp__mcanss_regs_mcanss_ctrl: Mmr_Mmrvbp_McanssRegsMcanssCtrl,
    mmr__mmrvbp__mcanss_regs_mcanss_stat: Mmr_Mmrvbp_McanssRegsMcanssStat,
    mmr__mmrvbp__mcanss_regs_mcanss_ics: Mmr_Mmrvbp_McanssRegsMcanssIcs,
    mmr__mmrvbp__mcanss_regs_mcanss_irs: Mmr_Mmrvbp_McanssRegsMcanssIrs,
    mmr__mmrvbp__mcanss_regs_mcanss_iecs: Mmr_Mmrvbp_McanssRegsMcanssIecs,
    mmr__mmrvbp__mcanss_regs_mcanss_ie: Mmr_Mmrvbp_McanssRegsMcanssIe,
    mmr__mmrvbp__mcanss_regs_mcanss_ies: Mmr_Mmrvbp_McanssRegsMcanssIes,
    mmr__mmrvbp__mcanss_regs_mcanss_eoi: Mmr_Mmrvbp_McanssRegsMcanssEoi,
    mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler: Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescaler,
    mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr:
        Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntr,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_pid(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssPid {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_pid
    }
    #[doc = "0x04 - The Control Register contains general control bits for the MCANSS"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_ctrl(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssCtrl {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_ctrl
    }
    #[doc = "0x08 - The Status register provide general status bits for the MCANSS"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_stat(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssStat {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_stat
    }
    #[doc = "0x0c - Write to clear interrupt bits"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_ics(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssIcs {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_ics
    }
    #[doc = "0x10 - Read raw interrupt status. Write '1' to set interrupt bits."]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_irs(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssIrs {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_irs
    }
    #[doc = "0x14 - Write to clear interrupt enable bits"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_iecs(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssIecs {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_iecs
    }
    #[doc = "0x18 - Read interrupt Enable"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_ie(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssIe {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_ie
    }
    #[doc = "0x1c - Read Enabled Interrupts"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_ies(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssIes {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_ies
    }
    #[doc = "0x20 - End of Interrupt Register"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_eoi(&self) -> &Mmr_Mmrvbp_McanssRegsMcanssEoi {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_eoi
    }
    #[doc = "0x24 - External TImeStamp PreScaler"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler(
        &self,
    ) -> &Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescaler {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler
    }
    #[doc = "0x28 - External TImeStamp Unserviced Interrupts Counter"]
    #[inline(always)]
    pub const fn mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr(
        &self,
    ) -> &Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntr {
        &self.mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr
    }
}
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_PID (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_pid`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_PID")]
pub type Mmr_Mmrvbp_McanssRegsMcanssPid =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_pid::Mmr_Mmrvbp_McanssRegsMcanssPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_pid;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_CTRL (rw) register accessor: The Control Register contains general control bits for the MCANSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_ctrl`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_CTRL")]
pub type Mmr_Mmrvbp_McanssRegsMcanssCtrl =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_ctrl::Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec>;
#[doc = "The Control Register contains general control bits for the MCANSS"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_ctrl;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_STAT (rw) register accessor: The Status register provide general status bits for the MCANSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_stat`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_STAT")]
pub type Mmr_Mmrvbp_McanssRegsMcanssStat =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_stat::Mmr_Mmrvbp_McanssRegsMcanssStatSpec>;
#[doc = "The Status register provide general status bits for the MCANSS"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_stat;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_ICS (rw) register accessor: Write to clear interrupt bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ics::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ics::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_ics`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_ICS")]
pub type Mmr_Mmrvbp_McanssRegsMcanssIcs =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_ics::Mmr_Mmrvbp_McanssRegsMcanssIcsSpec>;
#[doc = "Write to clear interrupt bits"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_ics;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IRS (rw) register accessor: Read raw interrupt status. Write '1' to set interrupt bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_irs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_irs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_irs`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IRS")]
pub type Mmr_Mmrvbp_McanssRegsMcanssIrs =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_irs::Mmr_Mmrvbp_McanssRegsMcanssIrsSpec>;
#[doc = "Read raw interrupt status. Write '1' to set interrupt bits."]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_irs;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IECS (rw) register accessor: Write to clear interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_iecs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_iecs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_iecs`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IECS")]
pub type Mmr_Mmrvbp_McanssRegsMcanssIecs =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_iecs::Mmr_Mmrvbp_McanssRegsMcanssIecsSpec>;
#[doc = "Write to clear interrupt enable bits"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_iecs;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IE (rw) register accessor: Read interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_ie`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IE")]
pub type Mmr_Mmrvbp_McanssRegsMcanssIe =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_ie::Mmr_Mmrvbp_McanssRegsMcanssIeSpec>;
#[doc = "Read interrupt Enable"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_ie;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IES (rw) register accessor: Read Enabled Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ies::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ies::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_ies`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_IES")]
pub type Mmr_Mmrvbp_McanssRegsMcanssIes =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_ies::Mmr_Mmrvbp_McanssRegsMcanssIesSpec>;
#[doc = "Read Enabled Interrupts"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_ies;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_EOI (rw) register accessor: End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_eoi`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_EOI")]
pub type Mmr_Mmrvbp_McanssRegsMcanssEoi =
    crate::Reg<mmr__mmrvbp__mcanss_regs_mcanss_eoi::Mmr_Mmrvbp_McanssRegsMcanssEoiSpec>;
#[doc = "End of Interrupt Register"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_eoi;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_PRESCALER (rw) register accessor: External TImeStamp PreScaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_PRESCALER")]
pub type Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescaler = crate::Reg<
    mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec,
>;
#[doc = "External TImeStamp PreScaler"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler;
#[doc = "MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_UNSERVICED_INTR_CNTR (rw) register accessor: External TImeStamp Unserviced Interrupts Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr`]
module"]
#[doc(alias = "MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_UNSERVICED_INTR_CNTR")]
pub type Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntr = crate :: Reg < mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr :: Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec > ;
#[doc = "External TImeStamp Unserviced Interrupts Counter"]
pub mod mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr;
