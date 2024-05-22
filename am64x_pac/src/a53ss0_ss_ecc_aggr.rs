#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecc_aggr_corepac_regs_aggr_revision: EccAggrCorepacRegsAggrRevision,
    _reserved1: [u8; 0x04],
    ecc_aggr_corepac_regs_ecc_vector: EccAggrCorepacRegsEccVector,
    ecc_aggr_corepac_regs_misc_status: EccAggrCorepacRegsMiscStatus,
    ecc_aggr_corepac_regs_reserved_svbus: EccAggrCorepacRegsReservedSvbus,
    _reserved4: [u8; 0x28],
    ecc_aggr_corepac_regs_sec_eoi_reg: EccAggrCorepacRegsSecEoiReg,
    ecc_aggr_corepac_regs_sec_status_reg0: EccAggrCorepacRegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ecc_aggr_corepac_regs_sec_enable_set_reg0: EccAggrCorepacRegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ecc_aggr_corepac_regs_sec_enable_clr_reg0: EccAggrCorepacRegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ecc_aggr_corepac_regs_ded_eoi_reg: EccAggrCorepacRegsDedEoiReg,
    ecc_aggr_corepac_regs_ded_status_reg0: EccAggrCorepacRegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ecc_aggr_corepac_regs_ded_enable_set_reg0: EccAggrCorepacRegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ecc_aggr_corepac_regs_ded_enable_clr_reg0: EccAggrCorepacRegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ecc_aggr_corepac_regs_aggr_enable_set: EccAggrCorepacRegsAggrEnableSet,
    ecc_aggr_corepac_regs_aggr_enable_clr: EccAggrCorepacRegsAggrEnableClr,
    ecc_aggr_corepac_regs_aggr_status_set: EccAggrCorepacRegsAggrStatusSet,
    ecc_aggr_corepac_regs_aggr_status_clr: EccAggrCorepacRegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_aggr_revision(&self) -> &EccAggrCorepacRegsAggrRevision {
        &self.ecc_aggr_corepac_regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_ecc_vector(&self) -> &EccAggrCorepacRegsEccVector {
        &self.ecc_aggr_corepac_regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_misc_status(&self) -> &EccAggrCorepacRegsMiscStatus {
        &self.ecc_aggr_corepac_regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_reserved_svbus(&self) -> &EccAggrCorepacRegsReservedSvbus {
        &self.ecc_aggr_corepac_regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_sec_eoi_reg(&self) -> &EccAggrCorepacRegsSecEoiReg {
        &self.ecc_aggr_corepac_regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_sec_status_reg0(&self) -> &EccAggrCorepacRegsSecStatusReg0 {
        &self.ecc_aggr_corepac_regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_sec_enable_set_reg0(
        &self,
    ) -> &EccAggrCorepacRegsSecEnableSetReg0 {
        &self.ecc_aggr_corepac_regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_sec_enable_clr_reg0(
        &self,
    ) -> &EccAggrCorepacRegsSecEnableClrReg0 {
        &self.ecc_aggr_corepac_regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_ded_eoi_reg(&self) -> &EccAggrCorepacRegsDedEoiReg {
        &self.ecc_aggr_corepac_regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_ded_status_reg0(&self) -> &EccAggrCorepacRegsDedStatusReg0 {
        &self.ecc_aggr_corepac_regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_ded_enable_set_reg0(
        &self,
    ) -> &EccAggrCorepacRegsDedEnableSetReg0 {
        &self.ecc_aggr_corepac_regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_ded_enable_clr_reg0(
        &self,
    ) -> &EccAggrCorepacRegsDedEnableClrReg0 {
        &self.ecc_aggr_corepac_regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_aggr_enable_set(&self) -> &EccAggrCorepacRegsAggrEnableSet {
        &self.ecc_aggr_corepac_regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_aggr_enable_clr(&self) -> &EccAggrCorepacRegsAggrEnableClr {
        &self.ecc_aggr_corepac_regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_aggr_status_set(&self) -> &EccAggrCorepacRegsAggrStatusSet {
        &self.ecc_aggr_corepac_regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_corepac_regs_aggr_status_clr(&self) -> &EccAggrCorepacRegsAggrStatusClr {
        &self.ecc_aggr_corepac_regs_aggr_status_clr
    }
}
#[doc = "ECC_AGGR_COREPAC_REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_aggr_revision`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_aggr_revision")]
pub type EccAggrCorepacRegsAggrRevision =
    crate::Reg<ecc_aggr_corepac_regs_aggr_revision::EccAggrCorepacRegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc_aggr_corepac_regs_aggr_revision;
#[doc = "ECC_AGGR_COREPAC_REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_ecc_vector`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_ecc_vector")]
pub type EccAggrCorepacRegsEccVector =
    crate::Reg<ecc_aggr_corepac_regs_ecc_vector::EccAggrCorepacRegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc_aggr_corepac_regs_ecc_vector;
#[doc = "ECC_AGGR_COREPAC_REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_misc_status`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_misc_status")]
pub type EccAggrCorepacRegsMiscStatus =
    crate::Reg<ecc_aggr_corepac_regs_misc_status::EccAggrCorepacRegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod ecc_aggr_corepac_regs_misc_status;
#[doc = "ECC_AGGR_COREPAC_REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_reserved_svbus`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_reserved_svbus")]
pub type EccAggrCorepacRegsReservedSvbus =
    crate::Reg<ecc_aggr_corepac_regs_reserved_svbus::EccAggrCorepacRegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_aggr_corepac_regs_reserved_svbus;
#[doc = "ECC_AGGR_COREPAC_REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_sec_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_sec_eoi_reg")]
pub type EccAggrCorepacRegsSecEoiReg =
    crate::Reg<ecc_aggr_corepac_regs_sec_eoi_reg::EccAggrCorepacRegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_corepac_regs_sec_eoi_reg;
#[doc = "ECC_AGGR_COREPAC_REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_sec_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_sec_status_reg0")]
pub type EccAggrCorepacRegsSecStatusReg0 =
    crate::Reg<ecc_aggr_corepac_regs_sec_status_reg0::EccAggrCorepacRegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_corepac_regs_sec_status_reg0;
#[doc = "ECC_AGGR_COREPAC_REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_sec_enable_set_reg0")]
pub type EccAggrCorepacRegsSecEnableSetReg0 =
    crate::Reg<ecc_aggr_corepac_regs_sec_enable_set_reg0::EccAggrCorepacRegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_corepac_regs_sec_enable_set_reg0;
#[doc = "ECC_AGGR_COREPAC_REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_sec_enable_clr_reg0")]
pub type EccAggrCorepacRegsSecEnableClrReg0 =
    crate::Reg<ecc_aggr_corepac_regs_sec_enable_clr_reg0::EccAggrCorepacRegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_corepac_regs_sec_enable_clr_reg0;
#[doc = "ECC_AGGR_COREPAC_REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_ded_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_ded_eoi_reg")]
pub type EccAggrCorepacRegsDedEoiReg =
    crate::Reg<ecc_aggr_corepac_regs_ded_eoi_reg::EccAggrCorepacRegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_corepac_regs_ded_eoi_reg;
#[doc = "ECC_AGGR_COREPAC_REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_ded_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_ded_status_reg0")]
pub type EccAggrCorepacRegsDedStatusReg0 =
    crate::Reg<ecc_aggr_corepac_regs_ded_status_reg0::EccAggrCorepacRegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_corepac_regs_ded_status_reg0;
#[doc = "ECC_AGGR_COREPAC_REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_ded_enable_set_reg0")]
pub type EccAggrCorepacRegsDedEnableSetReg0 =
    crate::Reg<ecc_aggr_corepac_regs_ded_enable_set_reg0::EccAggrCorepacRegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_corepac_regs_ded_enable_set_reg0;
#[doc = "ECC_AGGR_COREPAC_REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_ded_enable_clr_reg0")]
pub type EccAggrCorepacRegsDedEnableClrReg0 =
    crate::Reg<ecc_aggr_corepac_regs_ded_enable_clr_reg0::EccAggrCorepacRegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_corepac_regs_ded_enable_clr_reg0;
#[doc = "ECC_AGGR_COREPAC_REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_aggr_enable_set`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_aggr_enable_set")]
pub type EccAggrCorepacRegsAggrEnableSet =
    crate::Reg<ecc_aggr_corepac_regs_aggr_enable_set::EccAggrCorepacRegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod ecc_aggr_corepac_regs_aggr_enable_set;
#[doc = "ECC_AGGR_COREPAC_REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_aggr_enable_clr`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_aggr_enable_clr")]
pub type EccAggrCorepacRegsAggrEnableClr =
    crate::Reg<ecc_aggr_corepac_regs_aggr_enable_clr::EccAggrCorepacRegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ecc_aggr_corepac_regs_aggr_enable_clr;
#[doc = "ECC_AGGR_COREPAC_REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_aggr_status_set`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_aggr_status_set")]
pub type EccAggrCorepacRegsAggrStatusSet =
    crate::Reg<ecc_aggr_corepac_regs_aggr_status_set::EccAggrCorepacRegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod ecc_aggr_corepac_regs_aggr_status_set;
#[doc = "ECC_AGGR_COREPAC_REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_corepac_regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_corepac_regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_corepac_regs_aggr_status_clr`]
module"]
#[doc(alias = "ECC_AGGR_COREPAC_REGS_aggr_status_clr")]
pub type EccAggrCorepacRegsAggrStatusClr =
    crate::Reg<ecc_aggr_corepac_regs_aggr_status_clr::EccAggrCorepacRegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod ecc_aggr_corepac_regs_aggr_status_clr;
