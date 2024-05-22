#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eccregs_aggr_revision: EccregsAggrRevision,
    _reserved1: [u8; 0x04],
    eccregs_ecc_vector: EccregsEccVector,
    eccregs_misc_status: EccregsMiscStatus,
    _reserved3: [u8; 0x2c],
    eccregs_sec_eoi_reg: EccregsSecEoiReg,
    eccregs_sec_status_reg0: EccregsSecStatusReg0,
    _reserved5: [u8; 0x3c],
    eccregs_sec_enable_set_reg0: EccregsSecEnableSetReg0,
    _reserved6: [u8; 0x3c],
    eccregs_sec_enable_clr_reg0: EccregsSecEnableClrReg0,
    _reserved7: [u8; 0x78],
    eccregs_ded_eoi_reg: EccregsDedEoiReg,
    eccregs_ded_status_reg0: EccregsDedStatusReg0,
    _reserved9: [u8; 0x3c],
    eccregs_ded_enable_set_reg0: EccregsDedEnableSetReg0,
    _reserved10: [u8; 0x3c],
    eccregs_ded_enable_clr_reg0: EccregsDedEnableClrReg0,
    _reserved11: [u8; 0x3c],
    eccregs_aggr_enable_set: EccregsAggrEnableSet,
    eccregs_aggr_enable_clr: EccregsAggrEnableClr,
    eccregs_aggr_status_set: EccregsAggrStatusSet,
    eccregs_aggr_status_clr: EccregsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn eccregs_aggr_revision(&self) -> &EccregsAggrRevision {
        &self.eccregs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn eccregs_ecc_vector(&self) -> &EccregsEccVector {
        &self.eccregs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn eccregs_misc_status(&self) -> &EccregsMiscStatus {
        &self.eccregs_misc_status
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn eccregs_sec_eoi_reg(&self) -> &EccregsSecEoiReg {
        &self.eccregs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn eccregs_sec_status_reg0(&self) -> &EccregsSecStatusReg0 {
        &self.eccregs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn eccregs_sec_enable_set_reg0(&self) -> &EccregsSecEnableSetReg0 {
        &self.eccregs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn eccregs_sec_enable_clr_reg0(&self) -> &EccregsSecEnableClrReg0 {
        &self.eccregs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn eccregs_ded_eoi_reg(&self) -> &EccregsDedEoiReg {
        &self.eccregs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn eccregs_ded_status_reg0(&self) -> &EccregsDedStatusReg0 {
        &self.eccregs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn eccregs_ded_enable_set_reg0(&self) -> &EccregsDedEnableSetReg0 {
        &self.eccregs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn eccregs_ded_enable_clr_reg0(&self) -> &EccregsDedEnableClrReg0 {
        &self.eccregs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn eccregs_aggr_enable_set(&self) -> &EccregsAggrEnableSet {
        &self.eccregs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn eccregs_aggr_enable_clr(&self) -> &EccregsAggrEnableClr {
        &self.eccregs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn eccregs_aggr_status_set(&self) -> &EccregsAggrStatusSet {
        &self.eccregs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn eccregs_aggr_status_clr(&self) -> &EccregsAggrStatusClr {
        &self.eccregs_aggr_status_clr
    }
}
#[doc = "ECCREGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_aggr_revision`]
module"]
#[doc(alias = "ECCREGS_aggr_revision")]
pub type EccregsAggrRevision = crate::Reg<eccregs_aggr_revision::EccregsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod eccregs_aggr_revision;
#[doc = "ECCREGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_ecc_vector`]
module"]
#[doc(alias = "ECCREGS_ecc_vector")]
pub type EccregsEccVector = crate::Reg<eccregs_ecc_vector::EccregsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod eccregs_ecc_vector;
#[doc = "ECCREGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_misc_status`]
module"]
#[doc(alias = "ECCREGS_misc_status")]
pub type EccregsMiscStatus = crate::Reg<eccregs_misc_status::EccregsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod eccregs_misc_status;
#[doc = "ECCREGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_sec_eoi_reg`]
module"]
#[doc(alias = "ECCREGS_sec_eoi_reg")]
pub type EccregsSecEoiReg = crate::Reg<eccregs_sec_eoi_reg::EccregsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod eccregs_sec_eoi_reg;
#[doc = "ECCREGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_sec_status_reg0`]
module"]
#[doc(alias = "ECCREGS_sec_status_reg0")]
pub type EccregsSecStatusReg0 = crate::Reg<eccregs_sec_status_reg0::EccregsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod eccregs_sec_status_reg0;
#[doc = "ECCREGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECCREGS_sec_enable_set_reg0")]
pub type EccregsSecEnableSetReg0 =
    crate::Reg<eccregs_sec_enable_set_reg0::EccregsSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod eccregs_sec_enable_set_reg0;
#[doc = "ECCREGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECCREGS_sec_enable_clr_reg0")]
pub type EccregsSecEnableClrReg0 =
    crate::Reg<eccregs_sec_enable_clr_reg0::EccregsSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod eccregs_sec_enable_clr_reg0;
#[doc = "ECCREGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_ded_eoi_reg`]
module"]
#[doc(alias = "ECCREGS_ded_eoi_reg")]
pub type EccregsDedEoiReg = crate::Reg<eccregs_ded_eoi_reg::EccregsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod eccregs_ded_eoi_reg;
#[doc = "ECCREGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_ded_status_reg0`]
module"]
#[doc(alias = "ECCREGS_ded_status_reg0")]
pub type EccregsDedStatusReg0 = crate::Reg<eccregs_ded_status_reg0::EccregsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod eccregs_ded_status_reg0;
#[doc = "ECCREGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECCREGS_ded_enable_set_reg0")]
pub type EccregsDedEnableSetReg0 =
    crate::Reg<eccregs_ded_enable_set_reg0::EccregsDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod eccregs_ded_enable_set_reg0;
#[doc = "ECCREGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECCREGS_ded_enable_clr_reg0")]
pub type EccregsDedEnableClrReg0 =
    crate::Reg<eccregs_ded_enable_clr_reg0::EccregsDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod eccregs_ded_enable_clr_reg0;
#[doc = "ECCREGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_aggr_enable_set`]
module"]
#[doc(alias = "ECCREGS_aggr_enable_set")]
pub type EccregsAggrEnableSet = crate::Reg<eccregs_aggr_enable_set::EccregsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod eccregs_aggr_enable_set;
#[doc = "ECCREGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_aggr_enable_clr`]
module"]
#[doc(alias = "ECCREGS_aggr_enable_clr")]
pub type EccregsAggrEnableClr = crate::Reg<eccregs_aggr_enable_clr::EccregsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod eccregs_aggr_enable_clr;
#[doc = "ECCREGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_aggr_status_set`]
module"]
#[doc(alias = "ECCREGS_aggr_status_set")]
pub type EccregsAggrStatusSet = crate::Reg<eccregs_aggr_status_set::EccregsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod eccregs_aggr_status_set;
#[doc = "ECCREGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccregs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccregs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccregs_aggr_status_clr`]
module"]
#[doc(alias = "ECCREGS_aggr_status_clr")]
pub type EccregsAggrStatusClr = crate::Reg<eccregs_aggr_status_clr::EccregsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod eccregs_aggr_status_clr;
