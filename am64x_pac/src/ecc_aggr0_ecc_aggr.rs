#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs_aggr_revision: RegsAggrRevision,
    _reserved1: [u8; 0x04],
    regs_ecc_vector: RegsEccVector,
    regs_misc_status: RegsMiscStatus,
    regs_reserved_svbus: RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    regs_sec_eoi_reg: RegsSecEoiReg,
    regs_sec_status_reg0: RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    regs_sec_enable_set_reg0: RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    regs_sec_enable_clr_reg0: RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    regs_ded_eoi_reg: RegsDedEoiReg,
    regs_ded_status_reg0: RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    regs_ded_enable_set_reg0: RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    regs_ded_enable_clr_reg0: RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    regs_aggr_enable_set: RegsAggrEnableSet,
    regs_aggr_enable_clr: RegsAggrEnableClr,
    regs_aggr_status_set: RegsAggrStatusSet,
    regs_aggr_status_clr: RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn regs_aggr_revision(&self) -> &RegsAggrRevision {
        &self.regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn regs_ecc_vector(&self) -> &RegsEccVector {
        &self.regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn regs_misc_status(&self) -> &RegsMiscStatus {
        &self.regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn regs_reserved_svbus(&self) -> &RegsReservedSvbus {
        &self.regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn regs_sec_eoi_reg(&self) -> &RegsSecEoiReg {
        &self.regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn regs_sec_status_reg0(&self) -> &RegsSecStatusReg0 {
        &self.regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn regs_sec_enable_set_reg0(&self) -> &RegsSecEnableSetReg0 {
        &self.regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn regs_sec_enable_clr_reg0(&self) -> &RegsSecEnableClrReg0 {
        &self.regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn regs_ded_eoi_reg(&self) -> &RegsDedEoiReg {
        &self.regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn regs_ded_status_reg0(&self) -> &RegsDedStatusReg0 {
        &self.regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn regs_ded_enable_set_reg0(&self) -> &RegsDedEnableSetReg0 {
        &self.regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn regs_ded_enable_clr_reg0(&self) -> &RegsDedEnableClrReg0 {
        &self.regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn regs_aggr_enable_set(&self) -> &RegsAggrEnableSet {
        &self.regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn regs_aggr_enable_clr(&self) -> &RegsAggrEnableClr {
        &self.regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn regs_aggr_status_set(&self) -> &RegsAggrStatusSet {
        &self.regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn regs_aggr_status_clr(&self) -> &RegsAggrStatusClr {
        &self.regs_aggr_status_clr
    }
}
#[doc = "REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_aggr_revision`]
module"]
#[doc(alias = "REGS_aggr_revision")]
pub type RegsAggrRevision = crate::Reg<regs_aggr_revision::RegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod regs_aggr_revision;
#[doc = "REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_ecc_vector`]
module"]
#[doc(alias = "REGS_ecc_vector")]
pub type RegsEccVector = crate::Reg<regs_ecc_vector::RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod regs_ecc_vector;
#[doc = "REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_misc_status`]
module"]
#[doc(alias = "REGS_misc_status")]
pub type RegsMiscStatus = crate::Reg<regs_misc_status::RegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod regs_misc_status;
#[doc = "REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_reserved_svbus`]
module"]
#[doc(alias = "REGS_reserved_svbus")]
pub type RegsReservedSvbus = crate::Reg<regs_reserved_svbus::RegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod regs_reserved_svbus;
#[doc = "REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_sec_eoi_reg`]
module"]
#[doc(alias = "REGS_sec_eoi_reg")]
pub type RegsSecEoiReg = crate::Reg<regs_sec_eoi_reg::RegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod regs_sec_eoi_reg;
#[doc = "REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_sec_status_reg0`]
module"]
#[doc(alias = "REGS_sec_status_reg0")]
pub type RegsSecStatusReg0 = crate::Reg<regs_sec_status_reg0::RegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod regs_sec_status_reg0;
#[doc = "REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "REGS_sec_enable_set_reg0")]
pub type RegsSecEnableSetReg0 = crate::Reg<regs_sec_enable_set_reg0::RegsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod regs_sec_enable_set_reg0;
#[doc = "REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "REGS_sec_enable_clr_reg0")]
pub type RegsSecEnableClrReg0 = crate::Reg<regs_sec_enable_clr_reg0::RegsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod regs_sec_enable_clr_reg0;
#[doc = "REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_ded_eoi_reg`]
module"]
#[doc(alias = "REGS_ded_eoi_reg")]
pub type RegsDedEoiReg = crate::Reg<regs_ded_eoi_reg::RegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod regs_ded_eoi_reg;
#[doc = "REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_ded_status_reg0`]
module"]
#[doc(alias = "REGS_ded_status_reg0")]
pub type RegsDedStatusReg0 = crate::Reg<regs_ded_status_reg0::RegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod regs_ded_status_reg0;
#[doc = "REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "REGS_ded_enable_set_reg0")]
pub type RegsDedEnableSetReg0 = crate::Reg<regs_ded_enable_set_reg0::RegsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod regs_ded_enable_set_reg0;
#[doc = "REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "REGS_ded_enable_clr_reg0")]
pub type RegsDedEnableClrReg0 = crate::Reg<regs_ded_enable_clr_reg0::RegsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod regs_ded_enable_clr_reg0;
#[doc = "REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_aggr_enable_set`]
module"]
#[doc(alias = "REGS_aggr_enable_set")]
pub type RegsAggrEnableSet = crate::Reg<regs_aggr_enable_set::RegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod regs_aggr_enable_set;
#[doc = "REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_aggr_enable_clr`]
module"]
#[doc(alias = "REGS_aggr_enable_clr")]
pub type RegsAggrEnableClr = crate::Reg<regs_aggr_enable_clr::RegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod regs_aggr_enable_clr;
#[doc = "REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_aggr_status_set`]
module"]
#[doc(alias = "REGS_aggr_status_set")]
pub type RegsAggrStatusSet = crate::Reg<regs_aggr_status_set::RegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod regs_aggr_status_set;
#[doc = "REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_aggr_status_clr`]
module"]
#[doc(alias = "REGS_aggr_status_clr")]
pub type RegsAggrStatusClr = crate::Reg<regs_aggr_status_clr::RegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod regs_aggr_status_clr;
