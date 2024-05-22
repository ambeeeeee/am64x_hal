#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecc__cfg__regs_aggr_revision: Ecc_Cfg_RegsAggrRevision,
    _reserved1: [u8; 0x04],
    ecc__cfg__regs_ecc_vector: Ecc_Cfg_RegsEccVector,
    ecc__cfg__regs_misc_status: Ecc_Cfg_RegsMiscStatus,
    ecc__cfg__regs_reserved_svbus: Ecc_Cfg_RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    ecc__cfg__regs_sec_eoi_reg: Ecc_Cfg_RegsSecEoiReg,
    ecc__cfg__regs_sec_status_reg0: Ecc_Cfg_RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ecc__cfg__regs_sec_enable_set_reg0: Ecc_Cfg_RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ecc__cfg__regs_sec_enable_clr_reg0: Ecc_Cfg_RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ecc__cfg__regs_ded_eoi_reg: Ecc_Cfg_RegsDedEoiReg,
    ecc__cfg__regs_ded_status_reg0: Ecc_Cfg_RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ecc__cfg__regs_ded_enable_set_reg0: Ecc_Cfg_RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ecc__cfg__regs_ded_enable_clr_reg0: Ecc_Cfg_RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ecc__cfg__regs_aggr_enable_set: Ecc_Cfg_RegsAggrEnableSet,
    ecc__cfg__regs_aggr_enable_clr: Ecc_Cfg_RegsAggrEnableClr,
    ecc__cfg__regs_aggr_status_set: Ecc_Cfg_RegsAggrStatusSet,
    ecc__cfg__regs_aggr_status_clr: Ecc_Cfg_RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_aggr_revision(&self) -> &Ecc_Cfg_RegsAggrRevision {
        &self.ecc__cfg__regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_ecc_vector(&self) -> &Ecc_Cfg_RegsEccVector {
        &self.ecc__cfg__regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_misc_status(&self) -> &Ecc_Cfg_RegsMiscStatus {
        &self.ecc__cfg__regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc__cfg__regs_reserved_svbus(&self) -> &Ecc_Cfg_RegsReservedSvbus {
        &self.ecc__cfg__regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_sec_eoi_reg(&self) -> &Ecc_Cfg_RegsSecEoiReg {
        &self.ecc__cfg__regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_sec_status_reg0(&self) -> &Ecc_Cfg_RegsSecStatusReg0 {
        &self.ecc__cfg__regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_sec_enable_set_reg0(&self) -> &Ecc_Cfg_RegsSecEnableSetReg0 {
        &self.ecc__cfg__regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_sec_enable_clr_reg0(&self) -> &Ecc_Cfg_RegsSecEnableClrReg0 {
        &self.ecc__cfg__regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_ded_eoi_reg(&self) -> &Ecc_Cfg_RegsDedEoiReg {
        &self.ecc__cfg__regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_ded_status_reg0(&self) -> &Ecc_Cfg_RegsDedStatusReg0 {
        &self.ecc__cfg__regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_ded_enable_set_reg0(&self) -> &Ecc_Cfg_RegsDedEnableSetReg0 {
        &self.ecc__cfg__regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_ded_enable_clr_reg0(&self) -> &Ecc_Cfg_RegsDedEnableClrReg0 {
        &self.ecc__cfg__regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_aggr_enable_set(&self) -> &Ecc_Cfg_RegsAggrEnableSet {
        &self.ecc__cfg__regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_aggr_enable_clr(&self) -> &Ecc_Cfg_RegsAggrEnableClr {
        &self.ecc__cfg__regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_aggr_status_set(&self) -> &Ecc_Cfg_RegsAggrStatusSet {
        &self.ecc__cfg__regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ecc__cfg__regs_aggr_status_clr(&self) -> &Ecc_Cfg_RegsAggrStatusClr {
        &self.ecc__cfg__regs_aggr_status_clr
    }
}
#[doc = "ECC__CFG__REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_aggr_revision`]
module"]
#[doc(alias = "ECC__CFG__REGS_aggr_revision")]
pub type Ecc_Cfg_RegsAggrRevision =
    crate::Reg<ecc__cfg__regs_aggr_revision::Ecc_Cfg_RegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc__cfg__regs_aggr_revision;
#[doc = "ECC__CFG__REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_ecc_vector`]
module"]
#[doc(alias = "ECC__CFG__REGS_ecc_vector")]
pub type Ecc_Cfg_RegsEccVector = crate::Reg<ecc__cfg__regs_ecc_vector::Ecc_Cfg_RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc__cfg__regs_ecc_vector;
#[doc = "ECC__CFG__REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_misc_status`]
module"]
#[doc(alias = "ECC__CFG__REGS_misc_status")]
pub type Ecc_Cfg_RegsMiscStatus =
    crate::Reg<ecc__cfg__regs_misc_status::Ecc_Cfg_RegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod ecc__cfg__regs_misc_status;
#[doc = "ECC__CFG__REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_reserved_svbus`]
module"]
#[doc(alias = "ECC__CFG__REGS_reserved_svbus")]
pub type Ecc_Cfg_RegsReservedSvbus =
    crate::Reg<ecc__cfg__regs_reserved_svbus::Ecc_Cfg_RegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc__cfg__regs_reserved_svbus;
#[doc = "ECC__CFG__REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_sec_eoi_reg`]
module"]
#[doc(alias = "ECC__CFG__REGS_sec_eoi_reg")]
pub type Ecc_Cfg_RegsSecEoiReg = crate::Reg<ecc__cfg__regs_sec_eoi_reg::Ecc_Cfg_RegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc__cfg__regs_sec_eoi_reg;
#[doc = "ECC__CFG__REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_sec_status_reg0`]
module"]
#[doc(alias = "ECC__CFG__REGS_sec_status_reg0")]
pub type Ecc_Cfg_RegsSecStatusReg0 =
    crate::Reg<ecc__cfg__regs_sec_status_reg0::Ecc_Cfg_RegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc__cfg__regs_sec_status_reg0;
#[doc = "ECC__CFG__REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC__CFG__REGS_sec_enable_set_reg0")]
pub type Ecc_Cfg_RegsSecEnableSetReg0 =
    crate::Reg<ecc__cfg__regs_sec_enable_set_reg0::Ecc_Cfg_RegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc__cfg__regs_sec_enable_set_reg0;
#[doc = "ECC__CFG__REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC__CFG__REGS_sec_enable_clr_reg0")]
pub type Ecc_Cfg_RegsSecEnableClrReg0 =
    crate::Reg<ecc__cfg__regs_sec_enable_clr_reg0::Ecc_Cfg_RegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc__cfg__regs_sec_enable_clr_reg0;
#[doc = "ECC__CFG__REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_ded_eoi_reg`]
module"]
#[doc(alias = "ECC__CFG__REGS_ded_eoi_reg")]
pub type Ecc_Cfg_RegsDedEoiReg = crate::Reg<ecc__cfg__regs_ded_eoi_reg::Ecc_Cfg_RegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc__cfg__regs_ded_eoi_reg;
#[doc = "ECC__CFG__REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_ded_status_reg0`]
module"]
#[doc(alias = "ECC__CFG__REGS_ded_status_reg0")]
pub type Ecc_Cfg_RegsDedStatusReg0 =
    crate::Reg<ecc__cfg__regs_ded_status_reg0::Ecc_Cfg_RegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc__cfg__regs_ded_status_reg0;
#[doc = "ECC__CFG__REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC__CFG__REGS_ded_enable_set_reg0")]
pub type Ecc_Cfg_RegsDedEnableSetReg0 =
    crate::Reg<ecc__cfg__regs_ded_enable_set_reg0::Ecc_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc__cfg__regs_ded_enable_set_reg0;
#[doc = "ECC__CFG__REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC__CFG__REGS_ded_enable_clr_reg0")]
pub type Ecc_Cfg_RegsDedEnableClrReg0 =
    crate::Reg<ecc__cfg__regs_ded_enable_clr_reg0::Ecc_Cfg_RegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc__cfg__regs_ded_enable_clr_reg0;
#[doc = "ECC__CFG__REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_aggr_enable_set`]
module"]
#[doc(alias = "ECC__CFG__REGS_aggr_enable_set")]
pub type Ecc_Cfg_RegsAggrEnableSet =
    crate::Reg<ecc__cfg__regs_aggr_enable_set::Ecc_Cfg_RegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod ecc__cfg__regs_aggr_enable_set;
#[doc = "ECC__CFG__REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_aggr_enable_clr`]
module"]
#[doc(alias = "ECC__CFG__REGS_aggr_enable_clr")]
pub type Ecc_Cfg_RegsAggrEnableClr =
    crate::Reg<ecc__cfg__regs_aggr_enable_clr::Ecc_Cfg_RegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ecc__cfg__regs_aggr_enable_clr;
#[doc = "ECC__CFG__REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_aggr_status_set`]
module"]
#[doc(alias = "ECC__CFG__REGS_aggr_status_set")]
pub type Ecc_Cfg_RegsAggrStatusSet =
    crate::Reg<ecc__cfg__regs_aggr_status_set::Ecc_Cfg_RegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod ecc__cfg__regs_aggr_status_set;
#[doc = "ECC__CFG__REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc__cfg__regs_aggr_status_clr`]
module"]
#[doc(alias = "ECC__CFG__REGS_aggr_status_clr")]
pub type Ecc_Cfg_RegsAggrStatusClr =
    crate::Reg<ecc__cfg__regs_aggr_status_clr::Ecc_Cfg_RegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod ecc__cfg__regs_aggr_status_clr;
