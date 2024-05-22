#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid: Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsPid,
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl: Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrl,
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat: Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStat,
    _reserved3: [u8; 0x14],
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi: Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoi,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsPid {
        &self.ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid
    }
    #[doc = "0x04 - The Control Register contains general control bits for the ospi"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrl {
        &self.ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl
    }
    #[doc = "0x08 - The Status register provide general status bits for the ospi"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStat {
        &self.ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat
    }
    #[doc = "0x20 - End of Interrupt Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoi {
        &self.ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi
    }
}
#[doc = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_PID (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_PID")]
pub type Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsPid = crate::Reg<
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid::Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsPidSpec,
>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_pid;
#[doc = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_CTRL (rw) register accessor: The Control Register contains general control bits for the ospi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_CTRL")]
pub type Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrl = crate::Reg<
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl::Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsCtrlSpec,
>;
#[doc = "The Control Register contains general control bits for the ospi"]
pub mod ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_ctrl;
#[doc = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_STAT (rw) register accessor: The Status register provide general status bits for the ospi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_STAT")]
pub type Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStat = crate::Reg<
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat::Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsStatSpec,
>;
#[doc = "The Status register provide general status bits for the ospi"]
pub mod ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_stat;
#[doc = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_EOI (rw) register accessor: End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__MMR__MMRVBP__REGS_EOI")]
pub type Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoi = crate::Reg<
    ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi::Ospi0_OspiCfgVbusp_Mmr_Mmrvbp_RegsEoiSpec,
>;
#[doc = "End of Interrupt Register"]
pub mod ospi0__ospi_cfg_vbusp__mmr__mmrvbp__regs_eoi;
