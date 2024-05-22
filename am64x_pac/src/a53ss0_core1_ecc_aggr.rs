#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecc_aggr_core1_regs_aggr_revision: EccAggrCore1RegsAggrRevision,
    _reserved1: [u8; 0x04],
    ecc_aggr_core1_regs_ecc_vector: EccAggrCore1RegsEccVector,
    ecc_aggr_core1_regs_misc_status: EccAggrCore1RegsMiscStatus,
    ecc_aggr_core1_regs_reserved_svbus: EccAggrCore1RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    ecc_aggr_core1_regs_sec_eoi_reg: EccAggrCore1RegsSecEoiReg,
    ecc_aggr_core1_regs_sec_status_reg0: EccAggrCore1RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ecc_aggr_core1_regs_sec_enable_set_reg0: EccAggrCore1RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ecc_aggr_core1_regs_sec_enable_clr_reg0: EccAggrCore1RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ecc_aggr_core1_regs_ded_eoi_reg: EccAggrCore1RegsDedEoiReg,
    ecc_aggr_core1_regs_ded_status_reg0: EccAggrCore1RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ecc_aggr_core1_regs_ded_enable_set_reg0: EccAggrCore1RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ecc_aggr_core1_regs_ded_enable_clr_reg0: EccAggrCore1RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ecc_aggr_core1_regs_aggr_enable_set: EccAggrCore1RegsAggrEnableSet,
    ecc_aggr_core1_regs_aggr_enable_clr: EccAggrCore1RegsAggrEnableClr,
    ecc_aggr_core1_regs_aggr_status_set: EccAggrCore1RegsAggrStatusSet,
    ecc_aggr_core1_regs_aggr_status_clr: EccAggrCore1RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_aggr_revision(&self) -> &EccAggrCore1RegsAggrRevision {
        &self.ecc_aggr_core1_regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_ecc_vector(&self) -> &EccAggrCore1RegsEccVector {
        &self.ecc_aggr_core1_regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_misc_status(&self) -> &EccAggrCore1RegsMiscStatus {
        &self.ecc_aggr_core1_regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_reserved_svbus(&self) -> &EccAggrCore1RegsReservedSvbus {
        &self.ecc_aggr_core1_regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_sec_eoi_reg(&self) -> &EccAggrCore1RegsSecEoiReg {
        &self.ecc_aggr_core1_regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_sec_status_reg0(&self) -> &EccAggrCore1RegsSecStatusReg0 {
        &self.ecc_aggr_core1_regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_sec_enable_set_reg0(
        &self,
    ) -> &EccAggrCore1RegsSecEnableSetReg0 {
        &self.ecc_aggr_core1_regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_sec_enable_clr_reg0(
        &self,
    ) -> &EccAggrCore1RegsSecEnableClrReg0 {
        &self.ecc_aggr_core1_regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_ded_eoi_reg(&self) -> &EccAggrCore1RegsDedEoiReg {
        &self.ecc_aggr_core1_regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_ded_status_reg0(&self) -> &EccAggrCore1RegsDedStatusReg0 {
        &self.ecc_aggr_core1_regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_ded_enable_set_reg0(
        &self,
    ) -> &EccAggrCore1RegsDedEnableSetReg0 {
        &self.ecc_aggr_core1_regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_ded_enable_clr_reg0(
        &self,
    ) -> &EccAggrCore1RegsDedEnableClrReg0 {
        &self.ecc_aggr_core1_regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_aggr_enable_set(&self) -> &EccAggrCore1RegsAggrEnableSet {
        &self.ecc_aggr_core1_regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_aggr_enable_clr(&self) -> &EccAggrCore1RegsAggrEnableClr {
        &self.ecc_aggr_core1_regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_aggr_status_set(&self) -> &EccAggrCore1RegsAggrStatusSet {
        &self.ecc_aggr_core1_regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core1_regs_aggr_status_clr(&self) -> &EccAggrCore1RegsAggrStatusClr {
        &self.ecc_aggr_core1_regs_aggr_status_clr
    }
}
#[doc = "ECC_AGGR_CORE1_REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_aggr_revision`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_aggr_revision")]
pub type EccAggrCore1RegsAggrRevision =
    crate::Reg<ecc_aggr_core1_regs_aggr_revision::EccAggrCore1RegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc_aggr_core1_regs_aggr_revision;
#[doc = "ECC_AGGR_CORE1_REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_ecc_vector`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_ecc_vector")]
pub type EccAggrCore1RegsEccVector =
    crate::Reg<ecc_aggr_core1_regs_ecc_vector::EccAggrCore1RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc_aggr_core1_regs_ecc_vector;
#[doc = "ECC_AGGR_CORE1_REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_misc_status`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_misc_status")]
pub type EccAggrCore1RegsMiscStatus =
    crate::Reg<ecc_aggr_core1_regs_misc_status::EccAggrCore1RegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod ecc_aggr_core1_regs_misc_status;
#[doc = "ECC_AGGR_CORE1_REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_reserved_svbus`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_reserved_svbus")]
pub type EccAggrCore1RegsReservedSvbus =
    crate::Reg<ecc_aggr_core1_regs_reserved_svbus::EccAggrCore1RegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_aggr_core1_regs_reserved_svbus;
#[doc = "ECC_AGGR_CORE1_REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_sec_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_sec_eoi_reg")]
pub type EccAggrCore1RegsSecEoiReg =
    crate::Reg<ecc_aggr_core1_regs_sec_eoi_reg::EccAggrCore1RegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_core1_regs_sec_eoi_reg;
#[doc = "ECC_AGGR_CORE1_REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_sec_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_sec_status_reg0")]
pub type EccAggrCore1RegsSecStatusReg0 =
    crate::Reg<ecc_aggr_core1_regs_sec_status_reg0::EccAggrCore1RegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_core1_regs_sec_status_reg0;
#[doc = "ECC_AGGR_CORE1_REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_sec_enable_set_reg0")]
pub type EccAggrCore1RegsSecEnableSetReg0 =
    crate::Reg<ecc_aggr_core1_regs_sec_enable_set_reg0::EccAggrCore1RegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_core1_regs_sec_enable_set_reg0;
#[doc = "ECC_AGGR_CORE1_REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_sec_enable_clr_reg0")]
pub type EccAggrCore1RegsSecEnableClrReg0 =
    crate::Reg<ecc_aggr_core1_regs_sec_enable_clr_reg0::EccAggrCore1RegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_core1_regs_sec_enable_clr_reg0;
#[doc = "ECC_AGGR_CORE1_REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_ded_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_ded_eoi_reg")]
pub type EccAggrCore1RegsDedEoiReg =
    crate::Reg<ecc_aggr_core1_regs_ded_eoi_reg::EccAggrCore1RegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_core1_regs_ded_eoi_reg;
#[doc = "ECC_AGGR_CORE1_REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_ded_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_ded_status_reg0")]
pub type EccAggrCore1RegsDedStatusReg0 =
    crate::Reg<ecc_aggr_core1_regs_ded_status_reg0::EccAggrCore1RegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_core1_regs_ded_status_reg0;
#[doc = "ECC_AGGR_CORE1_REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_ded_enable_set_reg0")]
pub type EccAggrCore1RegsDedEnableSetReg0 =
    crate::Reg<ecc_aggr_core1_regs_ded_enable_set_reg0::EccAggrCore1RegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_core1_regs_ded_enable_set_reg0;
#[doc = "ECC_AGGR_CORE1_REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_ded_enable_clr_reg0")]
pub type EccAggrCore1RegsDedEnableClrReg0 =
    crate::Reg<ecc_aggr_core1_regs_ded_enable_clr_reg0::EccAggrCore1RegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_core1_regs_ded_enable_clr_reg0;
#[doc = "ECC_AGGR_CORE1_REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_aggr_enable_set`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_aggr_enable_set")]
pub type EccAggrCore1RegsAggrEnableSet =
    crate::Reg<ecc_aggr_core1_regs_aggr_enable_set::EccAggrCore1RegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod ecc_aggr_core1_regs_aggr_enable_set;
#[doc = "ECC_AGGR_CORE1_REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_aggr_enable_clr`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_aggr_enable_clr")]
pub type EccAggrCore1RegsAggrEnableClr =
    crate::Reg<ecc_aggr_core1_regs_aggr_enable_clr::EccAggrCore1RegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ecc_aggr_core1_regs_aggr_enable_clr;
#[doc = "ECC_AGGR_CORE1_REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_aggr_status_set`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_aggr_status_set")]
pub type EccAggrCore1RegsAggrStatusSet =
    crate::Reg<ecc_aggr_core1_regs_aggr_status_set::EccAggrCore1RegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod ecc_aggr_core1_regs_aggr_status_set;
#[doc = "ECC_AGGR_CORE1_REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core1_regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core1_regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core1_regs_aggr_status_clr`]
module"]
#[doc(alias = "ECC_AGGR_CORE1_REGS_aggr_status_clr")]
pub type EccAggrCore1RegsAggrStatusClr =
    crate::Reg<ecc_aggr_core1_regs_aggr_status_clr::EccAggrCore1RegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod ecc_aggr_core1_regs_aggr_status_clr;
