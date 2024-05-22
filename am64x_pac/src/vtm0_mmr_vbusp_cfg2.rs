#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    mmr__vbusp__cfg2_vtm_clk_ctrl: Mmr_Vbusp_Cfg2VtmClkCtrl,
    mmr__vbusp__cfg2_vtm_misc_ctrl: Mmr_Vbusp_Cfg2VtmMiscCtrl,
    mmr__vbusp__cfg2_vtm_misc_ctrl2: Mmr_Vbusp_Cfg2VtmMiscCtrl2,
    _reserved3: [u8; 0x0c],
    mmr__vbusp__cfg2_vtm_sample_ctrl: Mmr_Vbusp_Cfg2VtmSampleCtrl,
    _reserved4: [u8; 0x02dc],
    mmr__vbusp__cfg2_ctrl: Mmr_Vbusp_Cfg2Ctrl,
    mmr__vbusp__cfg2_trim: Mmr_Vbusp_Cfg2Trim,
}
impl RegisterBlock {
    #[doc = "0x08 - VTM clock related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_tsens_clk_src_div. The tsens_clks_src_div field is Device specific."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg2_vtm_clk_ctrl(&self) -> &Mmr_Vbusp_Cfg2VtmClkCtrl {
        &self.mmr__vbusp__cfg2_vtm_clk_ctrl
    }
    #[doc = "0x0c - VTM miscellaneous control bits."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg2_vtm_misc_ctrl(&self) -> &Mmr_Vbusp_Cfg2VtmMiscCtrl {
        &self.mmr__vbusp__cfg2_vtm_misc_ctrl
    }
    #[doc = "0x10 - VTM miscellaneous control bits."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg2_vtm_misc_ctrl2(&self) -> &Mmr_Vbusp_Cfg2VtmMiscCtrl2 {
        &self.mmr__vbusp__cfg2_vtm_misc_ctrl2
    }
    #[doc = "0x20 - VTM sample related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_sample_clk_cnt. The sample_clk_cnt field is Device specific."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg2_vtm_sample_ctrl(&self) -> &Mmr_Vbusp_Cfg2VtmSampleCtrl {
        &self.mmr__vbusp__cfg2_vtm_sample_ctrl
    }
    #[doc = "0x300 - Temperature Sensor Band-gap control register for sensor a."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg2_ctrl(&self) -> &Mmr_Vbusp_Cfg2Ctrl {
        &self.mmr__vbusp__cfg2_ctrl
    }
    #[doc = "0x304 - Temperature Sensor Band-gap trim values register for sensor a. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary."]
    #[inline(always)]
    pub const fn mmr__vbusp__cfg2_trim(&self) -> &Mmr_Vbusp_Cfg2Trim {
        &self.mmr__vbusp__cfg2_trim
    }
}
#[doc = "MMR__VBUSP__CFG2_VTM_CLK_CTRL (rw) register accessor: VTM clock related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_tsens_clk_src_div. The tsens_clks_src_div field is Device specific.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg2_vtm_clk_ctrl`]
module"]
#[doc(alias = "MMR__VBUSP__CFG2_VTM_CLK_CTRL")]
pub type Mmr_Vbusp_Cfg2VtmClkCtrl =
    crate::Reg<mmr__vbusp__cfg2_vtm_clk_ctrl::Mmr_Vbusp_Cfg2VtmClkCtrlSpec>;
#[doc = "VTM clock related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_tsens_clk_src_div. The tsens_clks_src_div field is Device specific."]
pub mod mmr__vbusp__cfg2_vtm_clk_ctrl;
#[doc = "MMR__VBUSP__CFG2_VTM_MISC_CTRL (rw) register accessor: VTM miscellaneous control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_misc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_misc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg2_vtm_misc_ctrl`]
module"]
#[doc(alias = "MMR__VBUSP__CFG2_VTM_MISC_CTRL")]
pub type Mmr_Vbusp_Cfg2VtmMiscCtrl =
    crate::Reg<mmr__vbusp__cfg2_vtm_misc_ctrl::Mmr_Vbusp_Cfg2VtmMiscCtrlSpec>;
#[doc = "VTM miscellaneous control bits."]
pub mod mmr__vbusp__cfg2_vtm_misc_ctrl;
#[doc = "MMR__VBUSP__CFG2_VTM_MISC_CTRL2 (rw) register accessor: VTM miscellaneous control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_misc_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_misc_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg2_vtm_misc_ctrl2`]
module"]
#[doc(alias = "MMR__VBUSP__CFG2_VTM_MISC_CTRL2")]
pub type Mmr_Vbusp_Cfg2VtmMiscCtrl2 =
    crate::Reg<mmr__vbusp__cfg2_vtm_misc_ctrl2::Mmr_Vbusp_Cfg2VtmMiscCtrl2Spec>;
#[doc = "VTM miscellaneous control bits."]
pub mod mmr__vbusp__cfg2_vtm_misc_ctrl2;
#[doc = "MMR__VBUSP__CFG2_VTM_SAMPLE_CTRL (rw) register accessor: VTM sample related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_sample_clk_cnt. The sample_clk_cnt field is Device specific.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_sample_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_sample_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg2_vtm_sample_ctrl`]
module"]
#[doc(alias = "MMR__VBUSP__CFG2_VTM_SAMPLE_CTRL")]
pub type Mmr_Vbusp_Cfg2VtmSampleCtrl =
    crate::Reg<mmr__vbusp__cfg2_vtm_sample_ctrl::Mmr_Vbusp_Cfg2VtmSampleCtrlSpec>;
#[doc = "VTM sample related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_sample_clk_cnt. The sample_clk_cnt field is Device specific."]
pub mod mmr__vbusp__cfg2_vtm_sample_ctrl;
#[doc = "MMR__VBUSP__CFG2_CTRL (rw) register accessor: Temperature Sensor Band-gap control register for sensor a.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg2_ctrl`]
module"]
#[doc(alias = "MMR__VBUSP__CFG2_CTRL")]
pub type Mmr_Vbusp_Cfg2Ctrl = crate::Reg<mmr__vbusp__cfg2_ctrl::Mmr_Vbusp_Cfg2CtrlSpec>;
#[doc = "Temperature Sensor Band-gap control register for sensor a."]
pub mod mmr__vbusp__cfg2_ctrl;
#[doc = "MMR__VBUSP__CFG2_TRIM (rw) register accessor: Temperature Sensor Band-gap trim values register for sensor a. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmr__vbusp__cfg2_trim`]
module"]
#[doc(alias = "MMR__VBUSP__CFG2_TRIM")]
pub type Mmr_Vbusp_Cfg2Trim = crate::Reg<mmr__vbusp__cfg2_trim::Mmr_Vbusp_Cfg2TrimSpec>;
#[doc = "Temperature Sensor Band-gap trim values register for sensor a. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary."]
pub mod mmr__vbusp__cfg2_trim;
