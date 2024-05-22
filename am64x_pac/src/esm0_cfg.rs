#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_pid: CfgPid,
    cfg_info: CfgInfo,
    cfg_en: CfgEn,
    cfg_sft_rst: CfgSftRst,
    cfg_err_raw: CfgErrRaw,
    cfg_err_sts: CfgErrSts,
    cfg_err_en_set: CfgErrEnSet,
    cfg_err_en_clr: CfgErrEnClr,
    cfg_low_pri: CfgLowPri,
    cfg_hi_pri: CfgHiPri,
    cfg_low: CfgLow,
    cfg_hi: CfgHi,
    cfg_eoi: CfgEoi,
    _reserved13: [u8; 0x0c],
    cfg_pin_ctrl: CfgPinCtrl,
    cfg_pin_sts: CfgPinSts,
    cfg_pin_cntr: CfgPinCntr,
    cfg_pin_cntr_pre: CfgPinCntrPre,
    cfg_pwmh_pin_cntr: CfgPwmhPinCntr,
    cfg_pwmh_pin_cntr_pre: CfgPwmhPinCntrPre,
    cfg_pwml_pin_cntr: CfgPwmlPinCntr,
    cfg_pwml_pin_cntr_pre: CfgPwmlPinCntrPre,
    _reserved21: [u8; 0x03a0],
    cfg_raw: CfgRaw,
    cfg_sts: CfgSts,
    cfg_intr_en_set: CfgIntrEnSet,
    cfg_intr_en_clr: CfgIntrEnClr,
    cfg_int_prio: CfgIntPrio,
    cfg_pin_en_set: CfgPinEnSet,
    cfg_pin_en_clr: CfgPinEnClr,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn cfg_pid(&self) -> &CfgPid {
        &self.cfg_pid
    }
    #[doc = "0x04 - The Info Register gives the configuration Inforrmation of this ESM."]
    #[inline(always)]
    pub const fn cfg_info(&self) -> &CfgInfo {
        &self.cfg_info
    }
    #[doc = "0x08 - The Global Enable Register has the master interrupt mask"]
    #[inline(always)]
    pub const fn cfg_en(&self) -> &CfgEn {
        &self.cfg_en
    }
    #[doc = "0x0c - The Global Soft Reset Register controls the global clear for raw status and enables"]
    #[inline(always)]
    pub const fn cfg_sft_rst(&self) -> &CfgSftRst {
        &self.cfg_sft_rst
    }
    #[doc = "0x10 - Raw Status/Set Register for Configuration Errors"]
    #[inline(always)]
    pub const fn cfg_err_raw(&self) -> &CfgErrRaw {
        &self.cfg_err_raw
    }
    #[doc = "0x14 - Config Error Enable and Clear Register"]
    #[inline(always)]
    pub const fn cfg_err_sts(&self) -> &CfgErrSts {
        &self.cfg_err_sts
    }
    #[doc = "0x18 - Config Error Enable Set Register"]
    #[inline(always)]
    pub const fn cfg_err_en_set(&self) -> &CfgErrEnSet {
        &self.cfg_err_en_set
    }
    #[doc = "0x1c - Config Error Interrupt Enabled Clear register"]
    #[inline(always)]
    pub const fn cfg_err_en_clr(&self) -> &CfgErrEnClr {
        &self.cfg_err_en_clr
    }
    #[doc = "0x20 - Shows which is the highest priority outstanding low priority interrupt"]
    #[inline(always)]
    pub const fn cfg_low_pri(&self) -> &CfgLowPri {
        &self.cfg_low_pri
    }
    #[doc = "0x24 - Shows which is the highest priority outstanding high priority interrupt"]
    #[inline(always)]
    pub const fn cfg_hi_pri(&self) -> &CfgHiPri {
        &self.cfg_hi_pri
    }
    #[doc = "0x28 - Shows which groups have oustanding low priority interrupts"]
    #[inline(always)]
    pub const fn cfg_low(&self) -> &CfgLow {
        &self.cfg_low
    }
    #[doc = "0x2c - Shows which groups have oustanding high priority interrupts"]
    #[inline(always)]
    pub const fn cfg_hi(&self) -> &CfgHi {
        &self.cfg_hi
    }
    #[doc = "0x30 - End of Interrupt Register"]
    #[inline(always)]
    pub const fn cfg_eoi(&self) -> &CfgEoi {
        &self.cfg_eoi
    }
    #[doc = "0x40 - This register controls the error_pin_n output"]
    #[inline(always)]
    pub const fn cfg_pin_ctrl(&self) -> &CfgPinCtrl {
        &self.cfg_pin_ctrl
    }
    #[doc = "0x44 - This register reflects the status of the error_pin_n output"]
    #[inline(always)]
    pub const fn cfg_pin_sts(&self) -> &CfgPinSts {
        &self.cfg_pin_sts
    }
    #[doc = "0x48 - This register shows the current value of the error pin counter"]
    #[inline(always)]
    pub const fn cfg_pin_cntr(&self) -> &CfgPinCntr {
        &self.cfg_pin_cntr
    }
    #[doc = "0x4c - This register contains the value that is loaded in to the Error Counter"]
    #[inline(always)]
    pub const fn cfg_pin_cntr_pre(&self) -> &CfgPinCntrPre {
        &self.cfg_pin_cntr_pre
    }
    #[doc = "0x50 - This register shows the current value of the error pin PWM high counter"]
    #[inline(always)]
    pub const fn cfg_pwmh_pin_cntr(&self) -> &CfgPwmhPinCntr {
        &self.cfg_pwmh_pin_cntr
    }
    #[doc = "0x54 - This register contains the value that is loaded in to the Error PWM High Counter"]
    #[inline(always)]
    pub const fn cfg_pwmh_pin_cntr_pre(&self) -> &CfgPwmhPinCntrPre {
        &self.cfg_pwmh_pin_cntr_pre
    }
    #[doc = "0x58 - This register shows the current value of the error pin PWM low counter"]
    #[inline(always)]
    pub const fn cfg_pwml_pin_cntr(&self) -> &CfgPwmlPinCntr {
        &self.cfg_pwml_pin_cntr
    }
    #[doc = "0x5c - This register contains the value that is loaded in to the Error PWM Low Counter"]
    #[inline(always)]
    pub const fn cfg_pwml_pin_cntr_pre(&self) -> &CfgPwmlPinCntrPre {
        &self.cfg_pwml_pin_cntr_pre
    }
    #[doc = "0x400 - Raw Status/Set Register for Group A Errors"]
    #[inline(always)]
    pub const fn cfg_raw(&self) -> &CfgRaw {
        &self.cfg_raw
    }
    #[doc = "0x404 - Error Enable and Clear Register"]
    #[inline(always)]
    pub const fn cfg_sts(&self) -> &CfgSts {
        &self.cfg_sts
    }
    #[doc = "0x408 - Level Error Enable Set Register"]
    #[inline(always)]
    pub const fn cfg_intr_en_set(&self) -> &CfgIntrEnSet {
        &self.cfg_intr_en_set
    }
    #[doc = "0x40c - Level Error Interrupt Enabled Clear register"]
    #[inline(always)]
    pub const fn cfg_intr_en_clr(&self) -> &CfgIntrEnClr {
        &self.cfg_intr_en_clr
    }
    #[doc = "0x410 - Level Error Interrupt Enabled Clear register"]
    #[inline(always)]
    pub const fn cfg_int_prio(&self) -> &CfgIntPrio {
        &self.cfg_int_prio
    }
    #[doc = "0x414 - Level Error Interrupt Enabled Clear register"]
    #[inline(always)]
    pub const fn cfg_pin_en_set(&self) -> &CfgPinEnSet {
        &self.cfg_pin_en_set
    }
    #[doc = "0x418 - Level Error Interrupt Enabled Clear register"]
    #[inline(always)]
    pub const fn cfg_pin_en_clr(&self) -> &CfgPinEnClr {
        &self.cfg_pin_en_clr
    }
}
#[doc = "CFG_PID (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pid`]
module"]
#[doc(alias = "CFG_PID")]
pub type CfgPid = crate::Reg<cfg_pid::CfgPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod cfg_pid;
#[doc = "CFG_INFO (rw) register accessor: The Info Register gives the configuration Inforrmation of this ESM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_info`]
module"]
#[doc(alias = "CFG_INFO")]
pub type CfgInfo = crate::Reg<cfg_info::CfgInfoSpec>;
#[doc = "The Info Register gives the configuration Inforrmation of this ESM."]
pub mod cfg_info;
#[doc = "CFG_EN (rw) register accessor: The Global Enable Register has the master interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_en`]
module"]
#[doc(alias = "CFG_EN")]
pub type CfgEn = crate::Reg<cfg_en::CfgEnSpec>;
#[doc = "The Global Enable Register has the master interrupt mask"]
pub mod cfg_en;
#[doc = "CFG_SFT_RST (rw) register accessor: The Global Soft Reset Register controls the global clear for raw status and enables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sft_rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sft_rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sft_rst`]
module"]
#[doc(alias = "CFG_SFT_RST")]
pub type CfgSftRst = crate::Reg<cfg_sft_rst::CfgSftRstSpec>;
#[doc = "The Global Soft Reset Register controls the global clear for raw status and enables"]
pub mod cfg_sft_rst;
#[doc = "CFG_ERR_RAW (rw) register accessor: Raw Status/Set Register for Configuration Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_raw`]
module"]
#[doc(alias = "CFG_ERR_RAW")]
pub type CfgErrRaw = crate::Reg<cfg_err_raw::CfgErrRawSpec>;
#[doc = "Raw Status/Set Register for Configuration Errors"]
pub mod cfg_err_raw;
#[doc = "CFG_ERR_STS (rw) register accessor: Config Error Enable and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_sts`]
module"]
#[doc(alias = "CFG_ERR_STS")]
pub type CfgErrSts = crate::Reg<cfg_err_sts::CfgErrStsSpec>;
#[doc = "Config Error Enable and Clear Register"]
pub mod cfg_err_sts;
#[doc = "CFG_ERR_EN_SET (rw) register accessor: Config Error Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_en_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_en_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_en_set`]
module"]
#[doc(alias = "CFG_ERR_EN_SET")]
pub type CfgErrEnSet = crate::Reg<cfg_err_en_set::CfgErrEnSetSpec>;
#[doc = "Config Error Enable Set Register"]
pub mod cfg_err_en_set;
#[doc = "CFG_ERR_EN_CLR (rw) register accessor: Config Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_en_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_en_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_en_clr`]
module"]
#[doc(alias = "CFG_ERR_EN_CLR")]
pub type CfgErrEnClr = crate::Reg<cfg_err_en_clr::CfgErrEnClrSpec>;
#[doc = "Config Error Interrupt Enabled Clear register"]
pub mod cfg_err_en_clr;
#[doc = "CFG_LOW_PRI (rw) register accessor: Shows which is the highest priority outstanding low priority interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_low_pri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_low_pri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_low_pri`]
module"]
#[doc(alias = "CFG_LOW_PRI")]
pub type CfgLowPri = crate::Reg<cfg_low_pri::CfgLowPriSpec>;
#[doc = "Shows which is the highest priority outstanding low priority interrupt"]
pub mod cfg_low_pri;
#[doc = "CFG_HI_PRI (rw) register accessor: Shows which is the highest priority outstanding high priority interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hi_pri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hi_pri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hi_pri`]
module"]
#[doc(alias = "CFG_HI_PRI")]
pub type CfgHiPri = crate::Reg<cfg_hi_pri::CfgHiPriSpec>;
#[doc = "Shows which is the highest priority outstanding high priority interrupt"]
pub mod cfg_hi_pri;
#[doc = "CFG_LOW (rw) register accessor: Shows which groups have oustanding low priority interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_low`]
module"]
#[doc(alias = "CFG_LOW")]
pub type CfgLow = crate::Reg<cfg_low::CfgLowSpec>;
#[doc = "Shows which groups have oustanding low priority interrupts"]
pub mod cfg_low;
#[doc = "CFG_HI (rw) register accessor: Shows which groups have oustanding high priority interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hi`]
module"]
#[doc(alias = "CFG_HI")]
pub type CfgHi = crate::Reg<cfg_hi::CfgHiSpec>;
#[doc = "Shows which groups have oustanding high priority interrupts"]
pub mod cfg_hi;
#[doc = "CFG_EOI (rw) register accessor: End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_eoi`]
module"]
#[doc(alias = "CFG_EOI")]
pub type CfgEoi = crate::Reg<cfg_eoi::CfgEoiSpec>;
#[doc = "End of Interrupt Register"]
pub mod cfg_eoi;
#[doc = "CFG_PIN_CTRL (rw) register accessor: This register controls the error_pin_n output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pin_ctrl`]
module"]
#[doc(alias = "CFG_PIN_CTRL")]
pub type CfgPinCtrl = crate::Reg<cfg_pin_ctrl::CfgPinCtrlSpec>;
#[doc = "This register controls the error_pin_n output"]
pub mod cfg_pin_ctrl;
#[doc = "CFG_PIN_STS (rw) register accessor: This register reflects the status of the error_pin_n output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pin_sts`]
module"]
#[doc(alias = "CFG_PIN_STS")]
pub type CfgPinSts = crate::Reg<cfg_pin_sts::CfgPinStsSpec>;
#[doc = "This register reflects the status of the error_pin_n output"]
pub mod cfg_pin_sts;
#[doc = "CFG_PIN_CNTR (rw) register accessor: This register shows the current value of the error pin counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pin_cntr`]
module"]
#[doc(alias = "CFG_PIN_CNTR")]
pub type CfgPinCntr = crate::Reg<cfg_pin_cntr::CfgPinCntrSpec>;
#[doc = "This register shows the current value of the error pin counter"]
pub mod cfg_pin_cntr;
#[doc = "CFG_PIN_CNTR_PRE (rw) register accessor: This register contains the value that is loaded in to the Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_cntr_pre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_cntr_pre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pin_cntr_pre`]
module"]
#[doc(alias = "CFG_PIN_CNTR_PRE")]
pub type CfgPinCntrPre = crate::Reg<cfg_pin_cntr_pre::CfgPinCntrPreSpec>;
#[doc = "This register contains the value that is loaded in to the Error Counter"]
pub mod cfg_pin_cntr_pre;
#[doc = "CFG_PWMH_PIN_CNTR (rw) register accessor: This register shows the current value of the error pin PWM high counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pwmh_pin_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pwmh_pin_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pwmh_pin_cntr`]
module"]
#[doc(alias = "CFG_PWMH_PIN_CNTR")]
pub type CfgPwmhPinCntr = crate::Reg<cfg_pwmh_pin_cntr::CfgPwmhPinCntrSpec>;
#[doc = "This register shows the current value of the error pin PWM high counter"]
pub mod cfg_pwmh_pin_cntr;
#[doc = "CFG_PWMH_PIN_CNTR_PRE (rw) register accessor: This register contains the value that is loaded in to the Error PWM High Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pwmh_pin_cntr_pre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pwmh_pin_cntr_pre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pwmh_pin_cntr_pre`]
module"]
#[doc(alias = "CFG_PWMH_PIN_CNTR_PRE")]
pub type CfgPwmhPinCntrPre = crate::Reg<cfg_pwmh_pin_cntr_pre::CfgPwmhPinCntrPreSpec>;
#[doc = "This register contains the value that is loaded in to the Error PWM High Counter"]
pub mod cfg_pwmh_pin_cntr_pre;
#[doc = "CFG_PWML_PIN_CNTR (rw) register accessor: This register shows the current value of the error pin PWM low counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pwml_pin_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pwml_pin_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pwml_pin_cntr`]
module"]
#[doc(alias = "CFG_PWML_PIN_CNTR")]
pub type CfgPwmlPinCntr = crate::Reg<cfg_pwml_pin_cntr::CfgPwmlPinCntrSpec>;
#[doc = "This register shows the current value of the error pin PWM low counter"]
pub mod cfg_pwml_pin_cntr;
#[doc = "CFG_PWML_PIN_CNTR_PRE (rw) register accessor: This register contains the value that is loaded in to the Error PWM Low Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pwml_pin_cntr_pre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pwml_pin_cntr_pre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pwml_pin_cntr_pre`]
module"]
#[doc(alias = "CFG_PWML_PIN_CNTR_PRE")]
pub type CfgPwmlPinCntrPre = crate::Reg<cfg_pwml_pin_cntr_pre::CfgPwmlPinCntrPreSpec>;
#[doc = "This register contains the value that is loaded in to the Error PWM Low Counter"]
pub mod cfg_pwml_pin_cntr_pre;
#[doc = "CFG_RAW (rw) register accessor: Raw Status/Set Register for Group A Errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_raw`]
module"]
#[doc(alias = "CFG_RAW")]
pub type CfgRaw = crate::Reg<cfg_raw::CfgRawSpec>;
#[doc = "Raw Status/Set Register for Group A Errors"]
pub mod cfg_raw;
#[doc = "CFG_STS (rw) register accessor: Error Enable and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sts`]
module"]
#[doc(alias = "CFG_STS")]
pub type CfgSts = crate::Reg<cfg_sts::CfgStsSpec>;
#[doc = "Error Enable and Clear Register"]
pub mod cfg_sts;
#[doc = "CFG_INTR_EN_SET (rw) register accessor: Level Error Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intr_en_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intr_en_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_intr_en_set`]
module"]
#[doc(alias = "CFG_INTR_EN_SET")]
pub type CfgIntrEnSet = crate::Reg<cfg_intr_en_set::CfgIntrEnSetSpec>;
#[doc = "Level Error Enable Set Register"]
pub mod cfg_intr_en_set;
#[doc = "CFG_INTR_EN_CLR (rw) register accessor: Level Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intr_en_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intr_en_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_intr_en_clr`]
module"]
#[doc(alias = "CFG_INTR_EN_CLR")]
pub type CfgIntrEnClr = crate::Reg<cfg_intr_en_clr::CfgIntrEnClrSpec>;
#[doc = "Level Error Interrupt Enabled Clear register"]
pub mod cfg_intr_en_clr;
#[doc = "CFG_INT_PRIO (rw) register accessor: Level Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_int_prio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_int_prio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_int_prio`]
module"]
#[doc(alias = "CFG_INT_PRIO")]
pub type CfgIntPrio = crate::Reg<cfg_int_prio::CfgIntPrioSpec>;
#[doc = "Level Error Interrupt Enabled Clear register"]
pub mod cfg_int_prio;
#[doc = "CFG_PIN_EN_SET (rw) register accessor: Level Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_en_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_en_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pin_en_set`]
module"]
#[doc(alias = "CFG_PIN_EN_SET")]
pub type CfgPinEnSet = crate::Reg<cfg_pin_en_set::CfgPinEnSetSpec>;
#[doc = "Level Error Interrupt Enabled Clear register"]
pub mod cfg_pin_en_set;
#[doc = "CFG_PIN_EN_CLR (rw) register accessor: Level Error Interrupt Enabled Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_en_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_en_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pin_en_clr`]
module"]
#[doc(alias = "CFG_PIN_EN_CLR")]
pub type CfgPinEnClr = crate::Reg<cfg_pin_en_clr::CfgPinEnClrSpec>;
#[doc = "Level Error Interrupt Enabled Clear register"]
pub mod cfg_pin_en_clr;
