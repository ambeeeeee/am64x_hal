#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecc_aggr_core0_regs_aggr_revision: EccAggrCore0RegsAggrRevision,
    _reserved1: [u8; 0x04],
    ecc_aggr_core0_regs_ecc_vector: EccAggrCore0RegsEccVector,
    ecc_aggr_core0_regs_misc_status: EccAggrCore0RegsMiscStatus,
    ecc_aggr_core0_regs_reserved_svbus: EccAggrCore0RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    ecc_aggr_core0_regs_sec_eoi_reg: EccAggrCore0RegsSecEoiReg,
    ecc_aggr_core0_regs_sec_status_reg0: EccAggrCore0RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ecc_aggr_core0_regs_sec_enable_set_reg0: EccAggrCore0RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ecc_aggr_core0_regs_sec_enable_clr_reg0: EccAggrCore0RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ecc_aggr_core0_regs_ded_eoi_reg: EccAggrCore0RegsDedEoiReg,
    ecc_aggr_core0_regs_ded_status_reg0: EccAggrCore0RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ecc_aggr_core0_regs_ded_enable_set_reg0: EccAggrCore0RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ecc_aggr_core0_regs_ded_enable_clr_reg0: EccAggrCore0RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ecc_aggr_core0_regs_aggr_enable_set: EccAggrCore0RegsAggrEnableSet,
    ecc_aggr_core0_regs_aggr_enable_clr: EccAggrCore0RegsAggrEnableClr,
    ecc_aggr_core0_regs_aggr_status_set: EccAggrCore0RegsAggrStatusSet,
    ecc_aggr_core0_regs_aggr_status_clr: EccAggrCore0RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_aggr_revision(&self) -> &EccAggrCore0RegsAggrRevision {
        &self.ecc_aggr_core0_regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_ecc_vector(&self) -> &EccAggrCore0RegsEccVector {
        &self.ecc_aggr_core0_regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_misc_status(&self) -> &EccAggrCore0RegsMiscStatus {
        &self.ecc_aggr_core0_regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_reserved_svbus(&self) -> &EccAggrCore0RegsReservedSvbus {
        &self.ecc_aggr_core0_regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_sec_eoi_reg(&self) -> &EccAggrCore0RegsSecEoiReg {
        &self.ecc_aggr_core0_regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_sec_status_reg0(&self) -> &EccAggrCore0RegsSecStatusReg0 {
        &self.ecc_aggr_core0_regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_sec_enable_set_reg0(
        &self,
    ) -> &EccAggrCore0RegsSecEnableSetReg0 {
        &self.ecc_aggr_core0_regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_sec_enable_clr_reg0(
        &self,
    ) -> &EccAggrCore0RegsSecEnableClrReg0 {
        &self.ecc_aggr_core0_regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_ded_eoi_reg(&self) -> &EccAggrCore0RegsDedEoiReg {
        &self.ecc_aggr_core0_regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_ded_status_reg0(&self) -> &EccAggrCore0RegsDedStatusReg0 {
        &self.ecc_aggr_core0_regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_ded_enable_set_reg0(
        &self,
    ) -> &EccAggrCore0RegsDedEnableSetReg0 {
        &self.ecc_aggr_core0_regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_ded_enable_clr_reg0(
        &self,
    ) -> &EccAggrCore0RegsDedEnableClrReg0 {
        &self.ecc_aggr_core0_regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_aggr_enable_set(&self) -> &EccAggrCore0RegsAggrEnableSet {
        &self.ecc_aggr_core0_regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_aggr_enable_clr(&self) -> &EccAggrCore0RegsAggrEnableClr {
        &self.ecc_aggr_core0_regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_aggr_status_set(&self) -> &EccAggrCore0RegsAggrStatusSet {
        &self.ecc_aggr_core0_regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_core0_regs_aggr_status_clr(&self) -> &EccAggrCore0RegsAggrStatusClr {
        &self.ecc_aggr_core0_regs_aggr_status_clr
    }
}
#[doc = "ECC_AGGR_CORE0_REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_aggr_revision`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_aggr_revision")]
pub type EccAggrCore0RegsAggrRevision =
    crate::Reg<ecc_aggr_core0_regs_aggr_revision::EccAggrCore0RegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc_aggr_core0_regs_aggr_revision;
#[doc = "ECC_AGGR_CORE0_REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_ecc_vector`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_ecc_vector")]
pub type EccAggrCore0RegsEccVector =
    crate::Reg<ecc_aggr_core0_regs_ecc_vector::EccAggrCore0RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc_aggr_core0_regs_ecc_vector;
#[doc = "ECC_AGGR_CORE0_REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_misc_status`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_misc_status")]
pub type EccAggrCore0RegsMiscStatus =
    crate::Reg<ecc_aggr_core0_regs_misc_status::EccAggrCore0RegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod ecc_aggr_core0_regs_misc_status;
#[doc = "ECC_AGGR_CORE0_REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_reserved_svbus`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_reserved_svbus")]
pub type EccAggrCore0RegsReservedSvbus =
    crate::Reg<ecc_aggr_core0_regs_reserved_svbus::EccAggrCore0RegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_aggr_core0_regs_reserved_svbus;
#[doc = "ECC_AGGR_CORE0_REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_sec_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_sec_eoi_reg")]
pub type EccAggrCore0RegsSecEoiReg =
    crate::Reg<ecc_aggr_core0_regs_sec_eoi_reg::EccAggrCore0RegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_core0_regs_sec_eoi_reg;
#[doc = "ECC_AGGR_CORE0_REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_sec_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_sec_status_reg0")]
pub type EccAggrCore0RegsSecStatusReg0 =
    crate::Reg<ecc_aggr_core0_regs_sec_status_reg0::EccAggrCore0RegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_core0_regs_sec_status_reg0;
#[doc = "ECC_AGGR_CORE0_REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_sec_enable_set_reg0")]
pub type EccAggrCore0RegsSecEnableSetReg0 =
    crate::Reg<ecc_aggr_core0_regs_sec_enable_set_reg0::EccAggrCore0RegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_core0_regs_sec_enable_set_reg0;
#[doc = "ECC_AGGR_CORE0_REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_sec_enable_clr_reg0")]
pub type EccAggrCore0RegsSecEnableClrReg0 =
    crate::Reg<ecc_aggr_core0_regs_sec_enable_clr_reg0::EccAggrCore0RegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_core0_regs_sec_enable_clr_reg0;
#[doc = "ECC_AGGR_CORE0_REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_ded_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_ded_eoi_reg")]
pub type EccAggrCore0RegsDedEoiReg =
    crate::Reg<ecc_aggr_core0_regs_ded_eoi_reg::EccAggrCore0RegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_core0_regs_ded_eoi_reg;
#[doc = "ECC_AGGR_CORE0_REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_ded_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_ded_status_reg0")]
pub type EccAggrCore0RegsDedStatusReg0 =
    crate::Reg<ecc_aggr_core0_regs_ded_status_reg0::EccAggrCore0RegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_core0_regs_ded_status_reg0;
#[doc = "ECC_AGGR_CORE0_REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_ded_enable_set_reg0")]
pub type EccAggrCore0RegsDedEnableSetReg0 =
    crate::Reg<ecc_aggr_core0_regs_ded_enable_set_reg0::EccAggrCore0RegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_core0_regs_ded_enable_set_reg0;
#[doc = "ECC_AGGR_CORE0_REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_ded_enable_clr_reg0")]
pub type EccAggrCore0RegsDedEnableClrReg0 =
    crate::Reg<ecc_aggr_core0_regs_ded_enable_clr_reg0::EccAggrCore0RegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_core0_regs_ded_enable_clr_reg0;
#[doc = "ECC_AGGR_CORE0_REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_aggr_enable_set`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_aggr_enable_set")]
pub type EccAggrCore0RegsAggrEnableSet =
    crate::Reg<ecc_aggr_core0_regs_aggr_enable_set::EccAggrCore0RegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod ecc_aggr_core0_regs_aggr_enable_set;
#[doc = "ECC_AGGR_CORE0_REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_aggr_enable_clr`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_aggr_enable_clr")]
pub type EccAggrCore0RegsAggrEnableClr =
    crate::Reg<ecc_aggr_core0_regs_aggr_enable_clr::EccAggrCore0RegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ecc_aggr_core0_regs_aggr_enable_clr;
#[doc = "ECC_AGGR_CORE0_REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_aggr_status_set`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_aggr_status_set")]
pub type EccAggrCore0RegsAggrStatusSet =
    crate::Reg<ecc_aggr_core0_regs_aggr_status_set::EccAggrCore0RegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod ecc_aggr_core0_regs_aggr_status_set;
#[doc = "ECC_AGGR_CORE0_REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_core0_regs_aggr_status_clr`]
module"]
#[doc(alias = "ECC_AGGR_CORE0_REGS_aggr_status_clr")]
pub type EccAggrCore0RegsAggrStatusClr =
    crate::Reg<ecc_aggr_core0_regs_aggr_status_clr::EccAggrCore0RegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod ecc_aggr_core0_regs_aggr_status_clr;
