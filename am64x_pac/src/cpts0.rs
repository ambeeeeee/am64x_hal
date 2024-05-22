#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpts_vbusp_cpts_idver_reg: CptsVbuspCptsIdverReg,
    cpts_vbusp_control_reg: CptsVbuspControlReg,
    cpts_vbusp_rftclk_sel_reg: CptsVbuspRftclkSelReg,
    cpts_vbusp_ts_push_reg: CptsVbuspTsPushReg,
    cpts_vbusp_ts_load_low_val_reg: CptsVbuspTsLoadLowValReg,
    cpts_vbusp_ts_load_en_reg: CptsVbuspTsLoadEnReg,
    cpts_vbusp_ts_comp_low_val_reg: CptsVbuspTsCompLowValReg,
    cpts_vbusp_ts_comp_len_reg: CptsVbuspTsCompLenReg,
    cpts_vbusp_intstat_raw_reg: CptsVbuspIntstatRawReg,
    cpts_vbusp_intstat_masked_reg: CptsVbuspIntstatMaskedReg,
    cpts_vbusp_int_enable_reg: CptsVbuspIntEnableReg,
    cpts_vbusp_ts_comp_nudge_reg: CptsVbuspTsCompNudgeReg,
    cpts_vbusp_event_pop_reg: CptsVbuspEventPopReg,
    cpts_vbusp_event_0_reg: CptsVbuspEvent0Reg,
    cpts_vbusp_event_1_reg: CptsVbuspEvent1Reg,
    cpts_vbusp_event_2_reg: CptsVbuspEvent2Reg,
    cpts_vbusp_event_3_reg: CptsVbuspEvent3Reg,
    cpts_vbusp_ts_load_high_val_reg: CptsVbuspTsLoadHighValReg,
    cpts_vbusp_ts_comp_high_val_reg: CptsVbuspTsCompHighValReg,
    cpts_vbusp_ts_add_val_reg: CptsVbuspTsAddValReg,
    cpts_vbusp_ts_ppm_low_val_reg: CptsVbuspTsPpmLowValReg,
    cpts_vbusp_ts_ppm_high_val_reg: CptsVbuspTsPpmHighValReg,
    cpts_vbusp_ts_nudge_val_reg: CptsVbuspTsNudgeValReg,
    _reserved23: [u8; 0x74],
    cpts_vbusp_ts_config: CptsVbuspTsConfig,
    _reserved24: [u8; 0x0c],
    cpts_vbusp_comp_low_reg: CptsVbuspCompLowReg,
    cpts_vbusp_comp_high_reg: CptsVbuspCompHighReg,
    cpts_vbusp_control_reg_: CptsVbuspControlReg_,
    cpts_vbusp_length_reg: CptsVbuspLengthReg,
    cpts_vbusp_ppm_low_reg: CptsVbuspPpmLowReg,
    cpts_vbusp_ppm_high_reg: CptsVbuspPpmHighReg,
    cpts_vbusp_nudge_reg: CptsVbuspNudgeReg,
    _reserved31: [u8; 0x0104],
    cpts_vbusp_comp_low_reg_: CptsVbuspCompLowReg_,
    cpts_vbusp_comp_high_reg_: CptsVbuspCompHighReg_,
    cpts_vbusp_control_reg__: CptsVbuspControlReg__,
    cpts_vbusp_length_reg_: CptsVbuspLengthReg_,
    cpts_vbusp_ppm_low_reg_: CptsVbuspPpmLowReg_,
    cpts_vbusp_ppm_high_reg_: CptsVbuspPpmHighReg_,
    cpts_vbusp_nudge_reg_: CptsVbuspNudgeReg_,
}
impl RegisterBlock {
    #[doc = "0x00 - Identification and Version Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_cpts_idver_reg(&self) -> &CptsVbuspCptsIdverReg {
        &self.cpts_vbusp_cpts_idver_reg
    }
    #[doc = "0x04 - Time Sync Control Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_control_reg(&self) -> &CptsVbuspControlReg {
        &self.cpts_vbusp_control_reg
    }
    #[doc = "0x08 - RFTCLK Select Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_rftclk_sel_reg(&self) -> &CptsVbuspRftclkSelReg {
        &self.cpts_vbusp_rftclk_sel_reg
    }
    #[doc = "0x0c - Time Stamp Event Push Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_push_reg(&self) -> &CptsVbuspTsPushReg {
        &self.cpts_vbusp_ts_push_reg
    }
    #[doc = "0x10 - Time Stamp Load Low Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_load_low_val_reg(&self) -> &CptsVbuspTsLoadLowValReg {
        &self.cpts_vbusp_ts_load_low_val_reg
    }
    #[doc = "0x14 - Time Stamp Load Enable Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_load_en_reg(&self) -> &CptsVbuspTsLoadEnReg {
        &self.cpts_vbusp_ts_load_en_reg
    }
    #[doc = "0x18 - Time Stamp Comparison Low Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_comp_low_val_reg(&self) -> &CptsVbuspTsCompLowValReg {
        &self.cpts_vbusp_ts_comp_low_val_reg
    }
    #[doc = "0x1c - Time Stamp Comparison Length Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_comp_len_reg(&self) -> &CptsVbuspTsCompLenReg {
        &self.cpts_vbusp_ts_comp_len_reg
    }
    #[doc = "0x20 - Interrupt Status Register Raw"]
    #[inline(always)]
    pub const fn cpts_vbusp_intstat_raw_reg(&self) -> &CptsVbuspIntstatRawReg {
        &self.cpts_vbusp_intstat_raw_reg
    }
    #[doc = "0x24 - Interrupt Status Register Masked"]
    #[inline(always)]
    pub const fn cpts_vbusp_intstat_masked_reg(&self) -> &CptsVbuspIntstatMaskedReg {
        &self.cpts_vbusp_intstat_masked_reg
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_int_enable_reg(&self) -> &CptsVbuspIntEnableReg {
        &self.cpts_vbusp_int_enable_reg
    }
    #[doc = "0x2c - Time Stamp Comparison Nudge Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_comp_nudge_reg(&self) -> &CptsVbuspTsCompNudgeReg {
        &self.cpts_vbusp_ts_comp_nudge_reg
    }
    #[doc = "0x30 - Event Pop Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_event_pop_reg(&self) -> &CptsVbuspEventPopReg {
        &self.cpts_vbusp_event_pop_reg
    }
    #[doc = "0x34 - Event 0 Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_event_0_reg(&self) -> &CptsVbuspEvent0Reg {
        &self.cpts_vbusp_event_0_reg
    }
    #[doc = "0x38 - Event 1 Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_event_1_reg(&self) -> &CptsVbuspEvent1Reg {
        &self.cpts_vbusp_event_1_reg
    }
    #[doc = "0x3c - Event 2 Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_event_2_reg(&self) -> &CptsVbuspEvent2Reg {
        &self.cpts_vbusp_event_2_reg
    }
    #[doc = "0x40 - Event 3 Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_event_3_reg(&self) -> &CptsVbuspEvent3Reg {
        &self.cpts_vbusp_event_3_reg
    }
    #[doc = "0x44 - Time Stamp Load High Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_load_high_val_reg(&self) -> &CptsVbuspTsLoadHighValReg {
        &self.cpts_vbusp_ts_load_high_val_reg
    }
    #[doc = "0x48 - Time Stamp Comparison High Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_comp_high_val_reg(&self) -> &CptsVbuspTsCompHighValReg {
        &self.cpts_vbusp_ts_comp_high_val_reg
    }
    #[doc = "0x4c - TS Add Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_add_val_reg(&self) -> &CptsVbuspTsAddValReg {
        &self.cpts_vbusp_ts_add_val_reg
    }
    #[doc = "0x50 - Time Stamp PPM Low Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_ppm_low_val_reg(&self) -> &CptsVbuspTsPpmLowValReg {
        &self.cpts_vbusp_ts_ppm_low_val_reg
    }
    #[doc = "0x54 - Time Stamp PPM High Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_ppm_high_val_reg(&self) -> &CptsVbuspTsPpmHighValReg {
        &self.cpts_vbusp_ts_ppm_high_val_reg
    }
    #[doc = "0x58 - Time Stamp Nudge Value Register"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_nudge_val_reg(&self) -> &CptsVbuspTsNudgeValReg {
        &self.cpts_vbusp_ts_nudge_val_reg
    }
    #[doc = "0xd0 - Time Stamp Configuration Read"]
    #[inline(always)]
    pub const fn cpts_vbusp_ts_config(&self) -> &CptsVbuspTsConfig {
        &self.cpts_vbusp_ts_config
    }
    #[doc = "0xe0 - Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_comp_low_reg(&self) -> &CptsVbuspCompLowReg {
        &self.cpts_vbusp_comp_low_reg
    }
    #[doc = "0xe4 - Time Stamp Generate Function Comparison high Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_comp_high_reg(&self) -> &CptsVbuspCompHighReg {
        &self.cpts_vbusp_comp_high_reg
    }
    #[doc = "0xe8 - Time Stamp Generate Function Control"]
    #[inline(always)]
    pub const fn cpts_vbusp_control_reg_(&self) -> &CptsVbuspControlReg_ {
        &self.cpts_vbusp_control_reg_
    }
    #[doc = "0xec - Time Stamp Generate Function Length Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_length_reg(&self) -> &CptsVbuspLengthReg {
        &self.cpts_vbusp_length_reg
    }
    #[doc = "0xf0 - Time Stamp Generate Function PPM Low Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_ppm_low_reg(&self) -> &CptsVbuspPpmLowReg {
        &self.cpts_vbusp_ppm_low_reg
    }
    #[doc = "0xf4 - Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_ppm_high_reg(&self) -> &CptsVbuspPpmHighReg {
        &self.cpts_vbusp_ppm_high_reg
    }
    #[doc = "0xf8 - Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_nudge_reg(&self) -> &CptsVbuspNudgeReg {
        &self.cpts_vbusp_nudge_reg
    }
    #[doc = "0x200 - Time Stamp ESTF Generate Function Comparison Low Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_comp_low_reg_(&self) -> &CptsVbuspCompLowReg_ {
        &self.cpts_vbusp_comp_low_reg_
    }
    #[doc = "0x204 - Time Stamp ESTF Generate Function Comparison high Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_comp_high_reg_(&self) -> &CptsVbuspCompHighReg_ {
        &self.cpts_vbusp_comp_high_reg_
    }
    #[doc = "0x208 - Time Stamp ESTF Generate Function Control"]
    #[inline(always)]
    pub const fn cpts_vbusp_control_reg__(&self) -> &CptsVbuspControlReg__ {
        &self.cpts_vbusp_control_reg__
    }
    #[doc = "0x20c - Time Stamp ESTF Generate Function Length Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_length_reg_(&self) -> &CptsVbuspLengthReg_ {
        &self.cpts_vbusp_length_reg_
    }
    #[doc = "0x210 - Time Stamp ESTF Generate Function PPM Low Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_ppm_low_reg_(&self) -> &CptsVbuspPpmLowReg_ {
        &self.cpts_vbusp_ppm_low_reg_
    }
    #[doc = "0x214 - Time Stamp ESTF Generate Function PPM High Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_ppm_high_reg_(&self) -> &CptsVbuspPpmHighReg_ {
        &self.cpts_vbusp_ppm_high_reg_
    }
    #[doc = "0x218 - Time Stamp ESTF Generate Function Nudge Value"]
    #[inline(always)]
    pub const fn cpts_vbusp_nudge_reg_(&self) -> &CptsVbuspNudgeReg_ {
        &self.cpts_vbusp_nudge_reg_
    }
}
#[doc = "CPTS_VBUSP_CPTS_IDVER_REG (rw) register accessor: Identification and Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_cpts_idver_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_cpts_idver_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_cpts_idver_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_CPTS_IDVER_REG")]
pub type CptsVbuspCptsIdverReg = crate::Reg<cpts_vbusp_cpts_idver_reg::CptsVbuspCptsIdverRegSpec>;
#[doc = "Identification and Version Register"]
pub mod cpts_vbusp_cpts_idver_reg;
#[doc = "CPTS_VBUSP_CONTROL_REG (rw) register accessor: Time Sync Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_control_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_CONTROL_REG")]
pub type CptsVbuspControlReg = crate::Reg<cpts_vbusp_control_reg::CptsVbuspControlRegSpec>;
#[doc = "Time Sync Control Register"]
pub mod cpts_vbusp_control_reg;
#[doc = "CPTS_VBUSP_RFTCLK_SEL_REG (rw) register accessor: RFTCLK Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_rftclk_sel_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_rftclk_sel_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_rftclk_sel_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_RFTCLK_SEL_REG")]
pub type CptsVbuspRftclkSelReg = crate::Reg<cpts_vbusp_rftclk_sel_reg::CptsVbuspRftclkSelRegSpec>;
#[doc = "RFTCLK Select Register"]
pub mod cpts_vbusp_rftclk_sel_reg;
#[doc = "CPTS_VBUSP_TS_PUSH_REG (rw) register accessor: Time Stamp Event Push Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_push_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_push_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_push_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_PUSH_REG")]
pub type CptsVbuspTsPushReg = crate::Reg<cpts_vbusp_ts_push_reg::CptsVbuspTsPushRegSpec>;
#[doc = "Time Stamp Event Push Register"]
pub mod cpts_vbusp_ts_push_reg;
#[doc = "CPTS_VBUSP_TS_LOAD_LOW_VAL_REG (rw) register accessor: Time Stamp Load Low Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_load_low_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_load_low_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_load_low_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_LOAD_LOW_VAL_REG")]
pub type CptsVbuspTsLoadLowValReg =
    crate::Reg<cpts_vbusp_ts_load_low_val_reg::CptsVbuspTsLoadLowValRegSpec>;
#[doc = "Time Stamp Load Low Value Register"]
pub mod cpts_vbusp_ts_load_low_val_reg;
#[doc = "CPTS_VBUSP_TS_LOAD_EN_REG (rw) register accessor: Time Stamp Load Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_load_en_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_load_en_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_load_en_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_LOAD_EN_REG")]
pub type CptsVbuspTsLoadEnReg = crate::Reg<cpts_vbusp_ts_load_en_reg::CptsVbuspTsLoadEnRegSpec>;
#[doc = "Time Stamp Load Enable Register"]
pub mod cpts_vbusp_ts_load_en_reg;
#[doc = "CPTS_VBUSP_TS_COMP_LOW_VAL_REG (rw) register accessor: Time Stamp Comparison Low Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_low_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_low_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_comp_low_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_COMP_LOW_VAL_REG")]
pub type CptsVbuspTsCompLowValReg =
    crate::Reg<cpts_vbusp_ts_comp_low_val_reg::CptsVbuspTsCompLowValRegSpec>;
#[doc = "Time Stamp Comparison Low Value Register"]
pub mod cpts_vbusp_ts_comp_low_val_reg;
#[doc = "CPTS_VBUSP_TS_COMP_LEN_REG (rw) register accessor: Time Stamp Comparison Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_len_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_len_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_comp_len_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_COMP_LEN_REG")]
pub type CptsVbuspTsCompLenReg = crate::Reg<cpts_vbusp_ts_comp_len_reg::CptsVbuspTsCompLenRegSpec>;
#[doc = "Time Stamp Comparison Length Register"]
pub mod cpts_vbusp_ts_comp_len_reg;
#[doc = "CPTS_VBUSP_INTSTAT_RAW_REG (rw) register accessor: Interrupt Status Register Raw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_intstat_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_intstat_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_intstat_raw_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_INTSTAT_RAW_REG")]
pub type CptsVbuspIntstatRawReg =
    crate::Reg<cpts_vbusp_intstat_raw_reg::CptsVbuspIntstatRawRegSpec>;
#[doc = "Interrupt Status Register Raw"]
pub mod cpts_vbusp_intstat_raw_reg;
#[doc = "CPTS_VBUSP_INTSTAT_MASKED_REG (rw) register accessor: Interrupt Status Register Masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_intstat_masked_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_intstat_masked_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_intstat_masked_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_INTSTAT_MASKED_REG")]
pub type CptsVbuspIntstatMaskedReg =
    crate::Reg<cpts_vbusp_intstat_masked_reg::CptsVbuspIntstatMaskedRegSpec>;
#[doc = "Interrupt Status Register Masked"]
pub mod cpts_vbusp_intstat_masked_reg;
#[doc = "CPTS_VBUSP_INT_ENABLE_REG (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_int_enable_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_int_enable_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_int_enable_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_INT_ENABLE_REG")]
pub type CptsVbuspIntEnableReg = crate::Reg<cpts_vbusp_int_enable_reg::CptsVbuspIntEnableRegSpec>;
#[doc = "Interrupt Enable Register"]
pub mod cpts_vbusp_int_enable_reg;
#[doc = "CPTS_VBUSP_TS_COMP_NUDGE_REG (rw) register accessor: Time Stamp Comparison Nudge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_nudge_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_nudge_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_comp_nudge_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_COMP_NUDGE_REG")]
pub type CptsVbuspTsCompNudgeReg =
    crate::Reg<cpts_vbusp_ts_comp_nudge_reg::CptsVbuspTsCompNudgeRegSpec>;
#[doc = "Time Stamp Comparison Nudge Register"]
pub mod cpts_vbusp_ts_comp_nudge_reg;
#[doc = "CPTS_VBUSP_EVENT_POP_REG (rw) register accessor: Event Pop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_pop_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_pop_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_event_pop_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_EVENT_POP_REG")]
pub type CptsVbuspEventPopReg = crate::Reg<cpts_vbusp_event_pop_reg::CptsVbuspEventPopRegSpec>;
#[doc = "Event Pop Register"]
pub mod cpts_vbusp_event_pop_reg;
#[doc = "CPTS_VBUSP_EVENT_0_REG (rw) register accessor: Event 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_event_0_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_EVENT_0_REG")]
pub type CptsVbuspEvent0Reg = crate::Reg<cpts_vbusp_event_0_reg::CptsVbuspEvent0RegSpec>;
#[doc = "Event 0 Register"]
pub mod cpts_vbusp_event_0_reg;
#[doc = "CPTS_VBUSP_EVENT_1_REG (rw) register accessor: Event 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_event_1_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_EVENT_1_REG")]
pub type CptsVbuspEvent1Reg = crate::Reg<cpts_vbusp_event_1_reg::CptsVbuspEvent1RegSpec>;
#[doc = "Event 1 Register"]
pub mod cpts_vbusp_event_1_reg;
#[doc = "CPTS_VBUSP_EVENT_2_REG (rw) register accessor: Event 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_event_2_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_EVENT_2_REG")]
pub type CptsVbuspEvent2Reg = crate::Reg<cpts_vbusp_event_2_reg::CptsVbuspEvent2RegSpec>;
#[doc = "Event 2 Register"]
pub mod cpts_vbusp_event_2_reg;
#[doc = "CPTS_VBUSP_EVENT_3_REG (rw) register accessor: Event 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_event_3_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_EVENT_3_REG")]
pub type CptsVbuspEvent3Reg = crate::Reg<cpts_vbusp_event_3_reg::CptsVbuspEvent3RegSpec>;
#[doc = "Event 3 Register"]
pub mod cpts_vbusp_event_3_reg;
#[doc = "CPTS_VBUSP_TS_LOAD_HIGH_VAL_REG (rw) register accessor: Time Stamp Load High Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_load_high_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_load_high_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_load_high_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_LOAD_HIGH_VAL_REG")]
pub type CptsVbuspTsLoadHighValReg =
    crate::Reg<cpts_vbusp_ts_load_high_val_reg::CptsVbuspTsLoadHighValRegSpec>;
#[doc = "Time Stamp Load High Value Register"]
pub mod cpts_vbusp_ts_load_high_val_reg;
#[doc = "CPTS_VBUSP_TS_COMP_HIGH_VAL_REG (rw) register accessor: Time Stamp Comparison High Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_high_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_high_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_comp_high_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_COMP_HIGH_VAL_REG")]
pub type CptsVbuspTsCompHighValReg =
    crate::Reg<cpts_vbusp_ts_comp_high_val_reg::CptsVbuspTsCompHighValRegSpec>;
#[doc = "Time Stamp Comparison High Value Register"]
pub mod cpts_vbusp_ts_comp_high_val_reg;
#[doc = "CPTS_VBUSP_TS_ADD_VAL_REG (rw) register accessor: TS Add Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_add_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_add_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_add_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_ADD_VAL_REG")]
pub type CptsVbuspTsAddValReg = crate::Reg<cpts_vbusp_ts_add_val_reg::CptsVbuspTsAddValRegSpec>;
#[doc = "TS Add Value Register"]
pub mod cpts_vbusp_ts_add_val_reg;
#[doc = "CPTS_VBUSP_TS_PPM_LOW_VAL_REG (rw) register accessor: Time Stamp PPM Low Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_ppm_low_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_ppm_low_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_ppm_low_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_PPM_LOW_VAL_REG")]
pub type CptsVbuspTsPpmLowValReg =
    crate::Reg<cpts_vbusp_ts_ppm_low_val_reg::CptsVbuspTsPpmLowValRegSpec>;
#[doc = "Time Stamp PPM Low Value Register"]
pub mod cpts_vbusp_ts_ppm_low_val_reg;
#[doc = "CPTS_VBUSP_TS_PPM_HIGH_VAL_REG (rw) register accessor: Time Stamp PPM High Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_ppm_high_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_ppm_high_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_ppm_high_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_PPM_HIGH_VAL_REG")]
pub type CptsVbuspTsPpmHighValReg =
    crate::Reg<cpts_vbusp_ts_ppm_high_val_reg::CptsVbuspTsPpmHighValRegSpec>;
#[doc = "Time Stamp PPM High Value Register"]
pub mod cpts_vbusp_ts_ppm_high_val_reg;
#[doc = "CPTS_VBUSP_TS_NUDGE_VAL_REG (rw) register accessor: Time Stamp Nudge Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_nudge_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_nudge_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_nudge_val_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_NUDGE_VAL_REG")]
pub type CptsVbuspTsNudgeValReg =
    crate::Reg<cpts_vbusp_ts_nudge_val_reg::CptsVbuspTsNudgeValRegSpec>;
#[doc = "Time Stamp Nudge Value Register"]
pub mod cpts_vbusp_ts_nudge_val_reg;
#[doc = "CPTS_VBUSP_TS_CONFIG (rw) register accessor: Time Stamp Configuration Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ts_config`]
module"]
#[doc(alias = "CPTS_VBUSP_TS_CONFIG")]
pub type CptsVbuspTsConfig = crate::Reg<cpts_vbusp_ts_config::CptsVbuspTsConfigSpec>;
#[doc = "Time Stamp Configuration Read"]
pub mod cpts_vbusp_ts_config;
#[doc = "CPTS_VBUSP_COMP_LOW_REG (rw) register accessor: Time Stamp Generate Function Comparison Low Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_comp_low_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_comp_low_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_comp_low_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_COMP_LOW_REG")]
pub type CptsVbuspCompLowReg = crate::Reg<cpts_vbusp_comp_low_reg::CptsVbuspCompLowRegSpec>;
#[doc = "Time Stamp Generate Function Comparison Low Value"]
pub mod cpts_vbusp_comp_low_reg;
#[doc = "CPTS_VBUSP_COMP_HIGH_REG (rw) register accessor: Time Stamp Generate Function Comparison high Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_comp_high_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_comp_high_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_comp_high_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_COMP_HIGH_REG")]
pub type CptsVbuspCompHighReg = crate::Reg<cpts_vbusp_comp_high_reg::CptsVbuspCompHighRegSpec>;
#[doc = "Time Stamp Generate Function Comparison high Value"]
pub mod cpts_vbusp_comp_high_reg;
#[doc = "CPTS_VBUSP_CONTROL_REG_ (rw) register accessor: Time Stamp Generate Function Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_control_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_control_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_control_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_CONTROL_REG_")]
pub type CptsVbuspControlReg_ = crate::Reg<cpts_vbusp_control_reg_::CptsVbuspControlReg_Spec>;
#[doc = "Time Stamp Generate Function Control"]
pub mod cpts_vbusp_control_reg_;
#[doc = "CPTS_VBUSP_LENGTH_REG (rw) register accessor: Time Stamp Generate Function Length Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_length_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_length_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_length_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_LENGTH_REG")]
pub type CptsVbuspLengthReg = crate::Reg<cpts_vbusp_length_reg::CptsVbuspLengthRegSpec>;
#[doc = "Time Stamp Generate Function Length Value"]
pub mod cpts_vbusp_length_reg;
#[doc = "CPTS_VBUSP_PPM_LOW_REG (rw) register accessor: Time Stamp Generate Function PPM Low Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_low_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_low_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ppm_low_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_PPM_LOW_REG")]
pub type CptsVbuspPpmLowReg = crate::Reg<cpts_vbusp_ppm_low_reg::CptsVbuspPpmLowRegSpec>;
#[doc = "Time Stamp Generate Function PPM Low Value"]
pub mod cpts_vbusp_ppm_low_reg;
#[doc = "CPTS_VBUSP_PPM_HIGH_REG (rw) register accessor: Time Stamp Generate Function PPM High Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_high_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_high_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ppm_high_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_PPM_HIGH_REG")]
pub type CptsVbuspPpmHighReg = crate::Reg<cpts_vbusp_ppm_high_reg::CptsVbuspPpmHighRegSpec>;
#[doc = "Time Stamp Generate Function PPM High Value"]
pub mod cpts_vbusp_ppm_high_reg;
#[doc = "CPTS_VBUSP_NUDGE_REG (rw) register accessor: Time Stamp Generate Function Nudge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_nudge_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_nudge_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_nudge_reg`]
module"]
#[doc(alias = "CPTS_VBUSP_NUDGE_REG")]
pub type CptsVbuspNudgeReg = crate::Reg<cpts_vbusp_nudge_reg::CptsVbuspNudgeRegSpec>;
#[doc = "Time Stamp Generate Function Nudge Value"]
pub mod cpts_vbusp_nudge_reg;
#[doc = "CPTS_VBUSP_COMP_LOW_REG_ (rw) register accessor: Time Stamp ESTF Generate Function Comparison Low Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_comp_low_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_comp_low_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_comp_low_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_COMP_LOW_REG_")]
pub type CptsVbuspCompLowReg_ = crate::Reg<cpts_vbusp_comp_low_reg_::CptsVbuspCompLowReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function Comparison Low Value"]
pub mod cpts_vbusp_comp_low_reg_;
#[doc = "CPTS_VBUSP_COMP_HIGH_REG_ (rw) register accessor: Time Stamp ESTF Generate Function Comparison high Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_comp_high_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_comp_high_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_comp_high_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_COMP_HIGH_REG_")]
pub type CptsVbuspCompHighReg_ = crate::Reg<cpts_vbusp_comp_high_reg_::CptsVbuspCompHighReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function Comparison high Value"]
pub mod cpts_vbusp_comp_high_reg_;
#[doc = "CPTS_VBUSP_CONTROL_REG__ (rw) register accessor: Time Stamp ESTF Generate Function Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_control_reg__::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_control_reg__::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_control_reg__`]
module"]
#[doc(alias = "CPTS_VBUSP_CONTROL_REG__")]
pub type CptsVbuspControlReg__ = crate::Reg<cpts_vbusp_control_reg__::CptsVbuspControlReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function Control"]
pub mod cpts_vbusp_control_reg__;
#[doc = "CPTS_VBUSP_LENGTH_REG_ (rw) register accessor: Time Stamp ESTF Generate Function Length Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_length_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_length_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_length_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_LENGTH_REG_")]
pub type CptsVbuspLengthReg_ = crate::Reg<cpts_vbusp_length_reg_::CptsVbuspLengthReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function Length Value"]
pub mod cpts_vbusp_length_reg_;
#[doc = "CPTS_VBUSP_PPM_LOW_REG_ (rw) register accessor: Time Stamp ESTF Generate Function PPM Low Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_low_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_low_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ppm_low_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_PPM_LOW_REG_")]
pub type CptsVbuspPpmLowReg_ = crate::Reg<cpts_vbusp_ppm_low_reg_::CptsVbuspPpmLowReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function PPM Low Value"]
pub mod cpts_vbusp_ppm_low_reg_;
#[doc = "CPTS_VBUSP_PPM_HIGH_REG_ (rw) register accessor: Time Stamp ESTF Generate Function PPM High Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_high_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_high_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_ppm_high_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_PPM_HIGH_REG_")]
pub type CptsVbuspPpmHighReg_ = crate::Reg<cpts_vbusp_ppm_high_reg_::CptsVbuspPpmHighReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function PPM High Value"]
pub mod cpts_vbusp_ppm_high_reg_;
#[doc = "CPTS_VBUSP_NUDGE_REG_ (rw) register accessor: Time Stamp ESTF Generate Function Nudge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_nudge_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_nudge_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpts_vbusp_nudge_reg_`]
module"]
#[doc(alias = "CPTS_VBUSP_NUDGE_REG_")]
pub type CptsVbuspNudgeReg_ = crate::Reg<cpts_vbusp_nudge_reg_::CptsVbuspNudgeReg_Spec>;
#[doc = "Time Stamp ESTF Generate Function Nudge Value"]
pub mod cpts_vbusp_nudge_reg_;
