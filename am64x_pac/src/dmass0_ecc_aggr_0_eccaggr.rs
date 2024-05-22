#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eccaggr_regs_aggr_revision: EccaggrRegsAggrRevision,
    _reserved1: [u8; 0x04],
    eccaggr_regs_ecc_vector: EccaggrRegsEccVector,
    eccaggr_regs_misc_status: EccaggrRegsMiscStatus,
    eccaggr_regs_reserved_svbus: EccaggrRegsReservedSvbus,
    _reserved4: [u8; 0x28],
    eccaggr_regs_sec_eoi_reg: EccaggrRegsSecEoiReg,
    eccaggr_regs_sec_status_reg0: EccaggrRegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    eccaggr_regs_sec_enable_set_reg0: EccaggrRegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    eccaggr_regs_sec_enable_clr_reg0: EccaggrRegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    eccaggr_regs_ded_eoi_reg: EccaggrRegsDedEoiReg,
    eccaggr_regs_ded_status_reg0: EccaggrRegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    eccaggr_regs_ded_enable_set_reg0: EccaggrRegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    eccaggr_regs_ded_enable_clr_reg0: EccaggrRegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    eccaggr_regs_aggr_enable_set: EccaggrRegsAggrEnableSet,
    eccaggr_regs_aggr_enable_clr: EccaggrRegsAggrEnableClr,
    eccaggr_regs_aggr_status_set: EccaggrRegsAggrStatusSet,
    eccaggr_regs_aggr_status_clr: EccaggrRegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn eccaggr_regs_aggr_revision(&self) -> &EccaggrRegsAggrRevision {
        &self.eccaggr_regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_ecc_vector(&self) -> &EccaggrRegsEccVector {
        &self.eccaggr_regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn eccaggr_regs_misc_status(&self) -> &EccaggrRegsMiscStatus {
        &self.eccaggr_regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn eccaggr_regs_reserved_svbus(&self) -> &EccaggrRegsReservedSvbus {
        &self.eccaggr_regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_sec_eoi_reg(&self) -> &EccaggrRegsSecEoiReg {
        &self.eccaggr_regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn eccaggr_regs_sec_status_reg0(&self) -> &EccaggrRegsSecStatusReg0 {
        &self.eccaggr_regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn eccaggr_regs_sec_enable_set_reg0(&self) -> &EccaggrRegsSecEnableSetReg0 {
        &self.eccaggr_regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn eccaggr_regs_sec_enable_clr_reg0(&self) -> &EccaggrRegsSecEnableClrReg0 {
        &self.eccaggr_regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_ded_eoi_reg(&self) -> &EccaggrRegsDedEoiReg {
        &self.eccaggr_regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn eccaggr_regs_ded_status_reg0(&self) -> &EccaggrRegsDedStatusReg0 {
        &self.eccaggr_regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn eccaggr_regs_ded_enable_set_reg0(&self) -> &EccaggrRegsDedEnableSetReg0 {
        &self.eccaggr_regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn eccaggr_regs_ded_enable_clr_reg0(&self) -> &EccaggrRegsDedEnableClrReg0 {
        &self.eccaggr_regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_aggr_enable_set(&self) -> &EccaggrRegsAggrEnableSet {
        &self.eccaggr_regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_aggr_enable_clr(&self) -> &EccaggrRegsAggrEnableClr {
        &self.eccaggr_regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_aggr_status_set(&self) -> &EccaggrRegsAggrStatusSet {
        &self.eccaggr_regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn eccaggr_regs_aggr_status_clr(&self) -> &EccaggrRegsAggrStatusClr {
        &self.eccaggr_regs_aggr_status_clr
    }
}
#[doc = "ECCAGGR_REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_aggr_revision`]
module"]
#[doc(alias = "ECCAGGR_REGS_aggr_revision")]
pub type EccaggrRegsAggrRevision =
    crate::Reg<eccaggr_regs_aggr_revision::EccaggrRegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod eccaggr_regs_aggr_revision;
#[doc = "ECCAGGR_REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_ecc_vector`]
module"]
#[doc(alias = "ECCAGGR_REGS_ecc_vector")]
pub type EccaggrRegsEccVector = crate::Reg<eccaggr_regs_ecc_vector::EccaggrRegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod eccaggr_regs_ecc_vector;
#[doc = "ECCAGGR_REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_misc_status`]
module"]
#[doc(alias = "ECCAGGR_REGS_misc_status")]
pub type EccaggrRegsMiscStatus = crate::Reg<eccaggr_regs_misc_status::EccaggrRegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod eccaggr_regs_misc_status;
#[doc = "ECCAGGR_REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_reserved_svbus`]
module"]
#[doc(alias = "ECCAGGR_REGS_reserved_svbus")]
pub type EccaggrRegsReservedSvbus =
    crate::Reg<eccaggr_regs_reserved_svbus::EccaggrRegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod eccaggr_regs_reserved_svbus;
#[doc = "ECCAGGR_REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_sec_eoi_reg`]
module"]
#[doc(alias = "ECCAGGR_REGS_sec_eoi_reg")]
pub type EccaggrRegsSecEoiReg = crate::Reg<eccaggr_regs_sec_eoi_reg::EccaggrRegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod eccaggr_regs_sec_eoi_reg;
#[doc = "ECCAGGR_REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_sec_status_reg0`]
module"]
#[doc(alias = "ECCAGGR_REGS_sec_status_reg0")]
pub type EccaggrRegsSecStatusReg0 =
    crate::Reg<eccaggr_regs_sec_status_reg0::EccaggrRegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod eccaggr_regs_sec_status_reg0;
#[doc = "ECCAGGR_REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECCAGGR_REGS_sec_enable_set_reg0")]
pub type EccaggrRegsSecEnableSetReg0 =
    crate::Reg<eccaggr_regs_sec_enable_set_reg0::EccaggrRegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod eccaggr_regs_sec_enable_set_reg0;
#[doc = "ECCAGGR_REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECCAGGR_REGS_sec_enable_clr_reg0")]
pub type EccaggrRegsSecEnableClrReg0 =
    crate::Reg<eccaggr_regs_sec_enable_clr_reg0::EccaggrRegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod eccaggr_regs_sec_enable_clr_reg0;
#[doc = "ECCAGGR_REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_ded_eoi_reg`]
module"]
#[doc(alias = "ECCAGGR_REGS_ded_eoi_reg")]
pub type EccaggrRegsDedEoiReg = crate::Reg<eccaggr_regs_ded_eoi_reg::EccaggrRegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod eccaggr_regs_ded_eoi_reg;
#[doc = "ECCAGGR_REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_ded_status_reg0`]
module"]
#[doc(alias = "ECCAGGR_REGS_ded_status_reg0")]
pub type EccaggrRegsDedStatusReg0 =
    crate::Reg<eccaggr_regs_ded_status_reg0::EccaggrRegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod eccaggr_regs_ded_status_reg0;
#[doc = "ECCAGGR_REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECCAGGR_REGS_ded_enable_set_reg0")]
pub type EccaggrRegsDedEnableSetReg0 =
    crate::Reg<eccaggr_regs_ded_enable_set_reg0::EccaggrRegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod eccaggr_regs_ded_enable_set_reg0;
#[doc = "ECCAGGR_REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECCAGGR_REGS_ded_enable_clr_reg0")]
pub type EccaggrRegsDedEnableClrReg0 =
    crate::Reg<eccaggr_regs_ded_enable_clr_reg0::EccaggrRegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod eccaggr_regs_ded_enable_clr_reg0;
#[doc = "ECCAGGR_REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_aggr_enable_set`]
module"]
#[doc(alias = "ECCAGGR_REGS_aggr_enable_set")]
pub type EccaggrRegsAggrEnableSet =
    crate::Reg<eccaggr_regs_aggr_enable_set::EccaggrRegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod eccaggr_regs_aggr_enable_set;
#[doc = "ECCAGGR_REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_aggr_enable_clr`]
module"]
#[doc(alias = "ECCAGGR_REGS_aggr_enable_clr")]
pub type EccaggrRegsAggrEnableClr =
    crate::Reg<eccaggr_regs_aggr_enable_clr::EccaggrRegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod eccaggr_regs_aggr_enable_clr;
#[doc = "ECCAGGR_REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_aggr_status_set`]
module"]
#[doc(alias = "ECCAGGR_REGS_aggr_status_set")]
pub type EccaggrRegsAggrStatusSet =
    crate::Reg<eccaggr_regs_aggr_status_set::EccaggrRegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod eccaggr_regs_aggr_status_set;
#[doc = "ECCAGGR_REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccaggr_regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccaggr_regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaggr_regs_aggr_status_clr`]
module"]
#[doc(alias = "ECCAGGR_REGS_aggr_status_clr")]
pub type EccaggrRegsAggrStatusClr =
    crate::Reg<eccaggr_regs_aggr_status_clr::EccaggrRegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod eccaggr_regs_aggr_status_clr;
