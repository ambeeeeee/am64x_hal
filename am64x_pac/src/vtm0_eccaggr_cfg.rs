#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    __eccaggr_cfg__regs_aggr_revision: _EccaggrCfg_RegsAggrRevision,
    _reserved1: [u8; 0x04],
    __eccaggr_cfg__regs_ecc_vector: _EccaggrCfg_RegsEccVector,
    __eccaggr_cfg__regs_misc_status: _EccaggrCfg_RegsMiscStatus,
    __eccaggr_cfg__regs_reserved_svbus: _EccaggrCfg_RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    __eccaggr_cfg__regs_sec_eoi_reg: _EccaggrCfg_RegsSecEoiReg,
    __eccaggr_cfg__regs_sec_status_reg0: _EccaggrCfg_RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    __eccaggr_cfg__regs_sec_enable_set_reg0: _EccaggrCfg_RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    __eccaggr_cfg__regs_sec_enable_clr_reg0: _EccaggrCfg_RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    __eccaggr_cfg__regs_ded_eoi_reg: _EccaggrCfg_RegsDedEoiReg,
    __eccaggr_cfg__regs_ded_status_reg0: _EccaggrCfg_RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    __eccaggr_cfg__regs_ded_enable_set_reg0: _EccaggrCfg_RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    __eccaggr_cfg__regs_ded_enable_clr_reg0: _EccaggrCfg_RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    __eccaggr_cfg__regs_aggr_enable_set: _EccaggrCfg_RegsAggrEnableSet,
    __eccaggr_cfg__regs_aggr_enable_clr: _EccaggrCfg_RegsAggrEnableClr,
    __eccaggr_cfg__regs_aggr_status_set: _EccaggrCfg_RegsAggrStatusSet,
    __eccaggr_cfg__regs_aggr_status_clr: _EccaggrCfg_RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_aggr_revision(&self) -> &_EccaggrCfg_RegsAggrRevision {
        &self.__eccaggr_cfg__regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_ecc_vector(&self) -> &_EccaggrCfg_RegsEccVector {
        &self.__eccaggr_cfg__regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_misc_status(&self) -> &_EccaggrCfg_RegsMiscStatus {
        &self.__eccaggr_cfg__regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_reserved_svbus(&self) -> &_EccaggrCfg_RegsReservedSvbus {
        &self.__eccaggr_cfg__regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_sec_eoi_reg(&self) -> &_EccaggrCfg_RegsSecEoiReg {
        &self.__eccaggr_cfg__regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_sec_status_reg0(&self) -> &_EccaggrCfg_RegsSecStatusReg0 {
        &self.__eccaggr_cfg__regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_sec_enable_set_reg0(
        &self,
    ) -> &_EccaggrCfg_RegsSecEnableSetReg0 {
        &self.__eccaggr_cfg__regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_sec_enable_clr_reg0(
        &self,
    ) -> &_EccaggrCfg_RegsSecEnableClrReg0 {
        &self.__eccaggr_cfg__regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_ded_eoi_reg(&self) -> &_EccaggrCfg_RegsDedEoiReg {
        &self.__eccaggr_cfg__regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_ded_status_reg0(&self) -> &_EccaggrCfg_RegsDedStatusReg0 {
        &self.__eccaggr_cfg__regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_ded_enable_set_reg0(
        &self,
    ) -> &_EccaggrCfg_RegsDedEnableSetReg0 {
        &self.__eccaggr_cfg__regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_ded_enable_clr_reg0(
        &self,
    ) -> &_EccaggrCfg_RegsDedEnableClrReg0 {
        &self.__eccaggr_cfg__regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_aggr_enable_set(&self) -> &_EccaggrCfg_RegsAggrEnableSet {
        &self.__eccaggr_cfg__regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_aggr_enable_clr(&self) -> &_EccaggrCfg_RegsAggrEnableClr {
        &self.__eccaggr_cfg__regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_aggr_status_set(&self) -> &_EccaggrCfg_RegsAggrStatusSet {
        &self.__eccaggr_cfg__regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn __eccaggr_cfg__regs_aggr_status_clr(&self) -> &_EccaggrCfg_RegsAggrStatusClr {
        &self.__eccaggr_cfg__regs_aggr_status_clr
    }
}
#[doc = "__ECCAGGR_CFG__REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_aggr_revision`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_aggr_revision")]
pub type _EccaggrCfg_RegsAggrRevision =
    crate::Reg<__eccaggr_cfg__regs_aggr_revision::_EccaggrCfg_RegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod __eccaggr_cfg__regs_aggr_revision;
#[doc = "__ECCAGGR_CFG__REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_ecc_vector`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_ecc_vector")]
pub type _EccaggrCfg_RegsEccVector =
    crate::Reg<__eccaggr_cfg__regs_ecc_vector::_EccaggrCfg_RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod __eccaggr_cfg__regs_ecc_vector;
#[doc = "__ECCAGGR_CFG__REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_misc_status`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_misc_status")]
pub type _EccaggrCfg_RegsMiscStatus =
    crate::Reg<__eccaggr_cfg__regs_misc_status::_EccaggrCfg_RegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod __eccaggr_cfg__regs_misc_status;
#[doc = "__ECCAGGR_CFG__REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_reserved_svbus`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_reserved_svbus")]
pub type _EccaggrCfg_RegsReservedSvbus =
    crate::Reg<__eccaggr_cfg__regs_reserved_svbus::_EccaggrCfg_RegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod __eccaggr_cfg__regs_reserved_svbus;
#[doc = "__ECCAGGR_CFG__REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_sec_eoi_reg`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_sec_eoi_reg")]
pub type _EccaggrCfg_RegsSecEoiReg =
    crate::Reg<__eccaggr_cfg__regs_sec_eoi_reg::_EccaggrCfg_RegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod __eccaggr_cfg__regs_sec_eoi_reg;
#[doc = "__ECCAGGR_CFG__REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_sec_status_reg0`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_sec_status_reg0")]
pub type _EccaggrCfg_RegsSecStatusReg0 =
    crate::Reg<__eccaggr_cfg__regs_sec_status_reg0::_EccaggrCfg_RegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod __eccaggr_cfg__regs_sec_status_reg0;
#[doc = "__ECCAGGR_CFG__REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_sec_enable_set_reg0")]
pub type _EccaggrCfg_RegsSecEnableSetReg0 =
    crate::Reg<__eccaggr_cfg__regs_sec_enable_set_reg0::_EccaggrCfg_RegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod __eccaggr_cfg__regs_sec_enable_set_reg0;
#[doc = "__ECCAGGR_CFG__REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_sec_enable_clr_reg0")]
pub type _EccaggrCfg_RegsSecEnableClrReg0 =
    crate::Reg<__eccaggr_cfg__regs_sec_enable_clr_reg0::_EccaggrCfg_RegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod __eccaggr_cfg__regs_sec_enable_clr_reg0;
#[doc = "__ECCAGGR_CFG__REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_ded_eoi_reg`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_ded_eoi_reg")]
pub type _EccaggrCfg_RegsDedEoiReg =
    crate::Reg<__eccaggr_cfg__regs_ded_eoi_reg::_EccaggrCfg_RegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod __eccaggr_cfg__regs_ded_eoi_reg;
#[doc = "__ECCAGGR_CFG__REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_ded_status_reg0`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_ded_status_reg0")]
pub type _EccaggrCfg_RegsDedStatusReg0 =
    crate::Reg<__eccaggr_cfg__regs_ded_status_reg0::_EccaggrCfg_RegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod __eccaggr_cfg__regs_ded_status_reg0;
#[doc = "__ECCAGGR_CFG__REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_ded_enable_set_reg0")]
pub type _EccaggrCfg_RegsDedEnableSetReg0 =
    crate::Reg<__eccaggr_cfg__regs_ded_enable_set_reg0::_EccaggrCfg_RegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod __eccaggr_cfg__regs_ded_enable_set_reg0;
#[doc = "__ECCAGGR_CFG__REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_ded_enable_clr_reg0")]
pub type _EccaggrCfg_RegsDedEnableClrReg0 =
    crate::Reg<__eccaggr_cfg__regs_ded_enable_clr_reg0::_EccaggrCfg_RegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod __eccaggr_cfg__regs_ded_enable_clr_reg0;
#[doc = "__ECCAGGR_CFG__REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_aggr_enable_set`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_aggr_enable_set")]
pub type _EccaggrCfg_RegsAggrEnableSet =
    crate::Reg<__eccaggr_cfg__regs_aggr_enable_set::_EccaggrCfg_RegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod __eccaggr_cfg__regs_aggr_enable_set;
#[doc = "__ECCAGGR_CFG__REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_aggr_enable_clr`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_aggr_enable_clr")]
pub type _EccaggrCfg_RegsAggrEnableClr =
    crate::Reg<__eccaggr_cfg__regs_aggr_enable_clr::_EccaggrCfg_RegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod __eccaggr_cfg__regs_aggr_enable_clr;
#[doc = "__ECCAGGR_CFG__REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_aggr_status_set`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_aggr_status_set")]
pub type _EccaggrCfg_RegsAggrStatusSet =
    crate::Reg<__eccaggr_cfg__regs_aggr_status_set::_EccaggrCfg_RegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod __eccaggr_cfg__regs_aggr_status_set;
#[doc = "__ECCAGGR_CFG__REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@__eccaggr_cfg__regs_aggr_status_clr`]
module"]
#[doc(alias = "__ECCAGGR_CFG__REGS_aggr_status_clr")]
pub type _EccaggrCfg_RegsAggrStatusClr =
    crate::Reg<__eccaggr_cfg__regs_aggr_status_clr::_EccaggrCfg_RegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod __eccaggr_cfg__regs_aggr_status_clr;
