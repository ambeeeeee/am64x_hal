#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecc_aggr_regsregs_aggr_revision: EccAggrRegsregsAggrRevision,
    _reserved1: [u8; 0x04],
    ecc_aggr_regsregs_ecc_vector: EccAggrRegsregsEccVector,
    ecc_aggr_regsregs_misc_status: EccAggrRegsregsMiscStatus,
    ecc_aggr_regsregs_reserved_svbus: EccAggrRegsregsReservedSvbus,
    _reserved4: [u8; 0x28],
    ecc_aggr_regsregs_sec_eoi_reg: EccAggrRegsregsSecEoiReg,
    ecc_aggr_regsregs_sec_status_reg0: EccAggrRegsregsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ecc_aggr_regsregs_sec_enable_set_reg0: EccAggrRegsregsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ecc_aggr_regsregs_sec_enable_clr_reg0: EccAggrRegsregsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ecc_aggr_regsregs_ded_eoi_reg: EccAggrRegsregsDedEoiReg,
    ecc_aggr_regsregs_ded_status_reg0: EccAggrRegsregsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ecc_aggr_regsregs_ded_enable_set_reg0: EccAggrRegsregsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ecc_aggr_regsregs_ded_enable_clr_reg0: EccAggrRegsregsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ecc_aggr_regsregs_aggr_enable_set: EccAggrRegsregsAggrEnableSet,
    ecc_aggr_regsregs_aggr_enable_clr: EccAggrRegsregsAggrEnableClr,
    ecc_aggr_regsregs_aggr_status_set: EccAggrRegsregsAggrStatusSet,
    ecc_aggr_regsregs_aggr_status_clr: EccAggrRegsregsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_aggr_revision(&self) -> &EccAggrRegsregsAggrRevision {
        &self.ecc_aggr_regsregs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_ecc_vector(&self) -> &EccAggrRegsregsEccVector {
        &self.ecc_aggr_regsregs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_misc_status(&self) -> &EccAggrRegsregsMiscStatus {
        &self.ecc_aggr_regsregs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_reserved_svbus(&self) -> &EccAggrRegsregsReservedSvbus {
        &self.ecc_aggr_regsregs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_sec_eoi_reg(&self) -> &EccAggrRegsregsSecEoiReg {
        &self.ecc_aggr_regsregs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_sec_status_reg0(&self) -> &EccAggrRegsregsSecStatusReg0 {
        &self.ecc_aggr_regsregs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_sec_enable_set_reg0(&self) -> &EccAggrRegsregsSecEnableSetReg0 {
        &self.ecc_aggr_regsregs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_sec_enable_clr_reg0(&self) -> &EccAggrRegsregsSecEnableClrReg0 {
        &self.ecc_aggr_regsregs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_ded_eoi_reg(&self) -> &EccAggrRegsregsDedEoiReg {
        &self.ecc_aggr_regsregs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_ded_status_reg0(&self) -> &EccAggrRegsregsDedStatusReg0 {
        &self.ecc_aggr_regsregs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_ded_enable_set_reg0(&self) -> &EccAggrRegsregsDedEnableSetReg0 {
        &self.ecc_aggr_regsregs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_ded_enable_clr_reg0(&self) -> &EccAggrRegsregsDedEnableClrReg0 {
        &self.ecc_aggr_regsregs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_aggr_enable_set(&self) -> &EccAggrRegsregsAggrEnableSet {
        &self.ecc_aggr_regsregs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_aggr_enable_clr(&self) -> &EccAggrRegsregsAggrEnableClr {
        &self.ecc_aggr_regsregs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_aggr_status_set(&self) -> &EccAggrRegsregsAggrStatusSet {
        &self.ecc_aggr_regsregs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_regsregs_aggr_status_clr(&self) -> &EccAggrRegsregsAggrStatusClr {
        &self.ecc_aggr_regsregs_aggr_status_clr
    }
}
#[doc = "ECC_AGGR_REGSREGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_aggr_revision`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_aggr_revision")]
pub type EccAggrRegsregsAggrRevision =
    crate::Reg<ecc_aggr_regsregs_aggr_revision::EccAggrRegsregsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc_aggr_regsregs_aggr_revision;
#[doc = "ECC_AGGR_REGSREGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_ecc_vector`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_ecc_vector")]
pub type EccAggrRegsregsEccVector =
    crate::Reg<ecc_aggr_regsregs_ecc_vector::EccAggrRegsregsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc_aggr_regsregs_ecc_vector;
#[doc = "ECC_AGGR_REGSREGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_misc_status`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_misc_status")]
pub type EccAggrRegsregsMiscStatus =
    crate::Reg<ecc_aggr_regsregs_misc_status::EccAggrRegsregsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod ecc_aggr_regsregs_misc_status;
#[doc = "ECC_AGGR_REGSREGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_reserved_svbus`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_reserved_svbus")]
pub type EccAggrRegsregsReservedSvbus =
    crate::Reg<ecc_aggr_regsregs_reserved_svbus::EccAggrRegsregsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_aggr_regsregs_reserved_svbus;
#[doc = "ECC_AGGR_REGSREGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_sec_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_sec_eoi_reg")]
pub type EccAggrRegsregsSecEoiReg =
    crate::Reg<ecc_aggr_regsregs_sec_eoi_reg::EccAggrRegsregsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_regsregs_sec_eoi_reg;
#[doc = "ECC_AGGR_REGSREGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_sec_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_sec_status_reg0")]
pub type EccAggrRegsregsSecStatusReg0 =
    crate::Reg<ecc_aggr_regsregs_sec_status_reg0::EccAggrRegsregsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_regsregs_sec_status_reg0;
#[doc = "ECC_AGGR_REGSREGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_sec_enable_set_reg0")]
pub type EccAggrRegsregsSecEnableSetReg0 =
    crate::Reg<ecc_aggr_regsregs_sec_enable_set_reg0::EccAggrRegsregsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_regsregs_sec_enable_set_reg0;
#[doc = "ECC_AGGR_REGSREGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_sec_enable_clr_reg0")]
pub type EccAggrRegsregsSecEnableClrReg0 =
    crate::Reg<ecc_aggr_regsregs_sec_enable_clr_reg0::EccAggrRegsregsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_regsregs_sec_enable_clr_reg0;
#[doc = "ECC_AGGR_REGSREGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_ded_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_ded_eoi_reg")]
pub type EccAggrRegsregsDedEoiReg =
    crate::Reg<ecc_aggr_regsregs_ded_eoi_reg::EccAggrRegsregsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_regsregs_ded_eoi_reg;
#[doc = "ECC_AGGR_REGSREGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_ded_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_ded_status_reg0")]
pub type EccAggrRegsregsDedStatusReg0 =
    crate::Reg<ecc_aggr_regsregs_ded_status_reg0::EccAggrRegsregsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_regsregs_ded_status_reg0;
#[doc = "ECC_AGGR_REGSREGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_ded_enable_set_reg0")]
pub type EccAggrRegsregsDedEnableSetReg0 =
    crate::Reg<ecc_aggr_regsregs_ded_enable_set_reg0::EccAggrRegsregsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_regsregs_ded_enable_set_reg0;
#[doc = "ECC_AGGR_REGSREGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_ded_enable_clr_reg0")]
pub type EccAggrRegsregsDedEnableClrReg0 =
    crate::Reg<ecc_aggr_regsregs_ded_enable_clr_reg0::EccAggrRegsregsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_regsregs_ded_enable_clr_reg0;
#[doc = "ECC_AGGR_REGSREGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_aggr_enable_set`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_aggr_enable_set")]
pub type EccAggrRegsregsAggrEnableSet =
    crate::Reg<ecc_aggr_regsregs_aggr_enable_set::EccAggrRegsregsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod ecc_aggr_regsregs_aggr_enable_set;
#[doc = "ECC_AGGR_REGSREGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_aggr_enable_clr`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_aggr_enable_clr")]
pub type EccAggrRegsregsAggrEnableClr =
    crate::Reg<ecc_aggr_regsregs_aggr_enable_clr::EccAggrRegsregsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ecc_aggr_regsregs_aggr_enable_clr;
#[doc = "ECC_AGGR_REGSREGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_aggr_status_set`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_aggr_status_set")]
pub type EccAggrRegsregsAggrStatusSet =
    crate::Reg<ecc_aggr_regsregs_aggr_status_set::EccAggrRegsregsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod ecc_aggr_regsregs_aggr_status_set;
#[doc = "ECC_AGGR_REGSREGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_regsregs_aggr_status_clr`]
module"]
#[doc(alias = "ECC_AGGR_REGSREGS_aggr_status_clr")]
pub type EccAggrRegsregsAggrStatusClr =
    crate::Reg<ecc_aggr_regsregs_aggr_status_clr::EccAggrRegsregsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod ecc_aggr_regsregs_aggr_status_clr;
