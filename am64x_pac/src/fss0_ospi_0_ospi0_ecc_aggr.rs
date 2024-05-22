#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrRevision,
    _reserved1: [u8; 0x04],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsEccVector,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatus,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEoiReg,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiReg,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableSet,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableClr,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusSet,
    ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr:
        Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrRevision {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsEccVector {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatus {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsReservedSvbus {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEoiReg {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecStatusReg0 {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableSetReg0 {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0 {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiReg {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedStatusReg0 {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0 {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableClrReg0 {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableSet {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableClr {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusSet {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusClr {
        &self.ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr
    }
}
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_revision")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrRevision = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrRevisionSpec > ;
#[doc = "Revision parameters"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_revision;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ecc_vector")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsEccVector = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsEccVectorSpec > ;
#[doc = "ECC Vector Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ecc_vector;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_misc_status")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatus = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec > ;
#[doc = "Misc Status"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_reserved_svbus")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsReservedSvbus = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsReservedSvbusSpec > ;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_reserved_svbus;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_eoi_reg")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEoiReg = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEoiRegSpec > ;
#[doc = "EOI Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_eoi_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecStatusReg0 = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0 :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecStatusReg0Spec > ;
#[doc = "Interrupt Status Register 0"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_status_reg0;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_set_reg0")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableSetReg0 = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0 :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableSetReg0Spec > ;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0 = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0 :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec > ;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiReg = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec > ;
#[doc = "EOI Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_status_reg0")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedStatusReg0 = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0 :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedStatusReg0Spec > ;
#[doc = "Interrupt Status Register 0"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_status_reg0;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0 = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0 :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec > ;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableClrReg0 = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0 :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableClrReg0Spec > ;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableSet = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableSetSpec > ;
#[doc = "AGGR interrupt enable set Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_set;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_clr")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableClr = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrEnableClrSpec > ;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_enable_clr;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_status_set")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusSet = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusSetSpec > ;
#[doc = "AGGR interrupt status set Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_set;
#[doc = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_aggr_status_clr")]
pub type Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusClr = crate :: Reg < ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr :: Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsAggrStatusClrSpec > ;
#[doc = "AGGR interrupt status clear Register"]
pub mod ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_aggr_status_clr;
