#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpsw_nuss_vbusp_ecc_aggr_revision: CpswNussVbuspEccAggrRevision,
    _reserved1: [u8; 0x04],
    cpsw_nuss_vbusp_ecc_ecc_vector: CpswNussVbuspEccEccVector,
    cpsw_nuss_vbusp_ecc_misc_status: CpswNussVbuspEccMiscStatus,
    cpsw_nuss_vbusp_ecc_reserved_svbus: CpswNussVbuspEccReservedSvbus,
    _reserved4: [u8; 0x28],
    cpsw_nuss_vbusp_ecc_sec_eoi_reg: CpswNussVbuspEccSecEoiReg,
    cpsw_nuss_vbusp_ecc_sec_status_reg0: CpswNussVbuspEccSecStatusReg0,
    _reserved6: [u8; 0x3c],
    cpsw_nuss_vbusp_ecc_sec_enable_set_reg0: CpswNussVbuspEccSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0: CpswNussVbuspEccSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    cpsw_nuss_vbusp_ecc_ded_eoi_reg: CpswNussVbuspEccDedEoiReg,
    cpsw_nuss_vbusp_ecc_ded_status_reg0: CpswNussVbuspEccDedStatusReg0,
    _reserved10: [u8; 0x3c],
    cpsw_nuss_vbusp_ecc_ded_enable_set_reg0: CpswNussVbuspEccDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0: CpswNussVbuspEccDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    cpsw_nuss_vbusp_ecc_aggr_enable_set: CpswNussVbuspEccAggrEnableSet,
    cpsw_nuss_vbusp_ecc_aggr_enable_clr: CpswNussVbuspEccAggrEnableClr,
    cpsw_nuss_vbusp_ecc_aggr_status_set: CpswNussVbuspEccAggrStatusSet,
    cpsw_nuss_vbusp_ecc_aggr_status_clr: CpswNussVbuspEccAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_aggr_revision(&self) -> &CpswNussVbuspEccAggrRevision {
        &self.cpsw_nuss_vbusp_ecc_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_ecc_vector(&self) -> &CpswNussVbuspEccEccVector {
        &self.cpsw_nuss_vbusp_ecc_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_misc_status(&self) -> &CpswNussVbuspEccMiscStatus {
        &self.cpsw_nuss_vbusp_ecc_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_reserved_svbus(&self) -> &CpswNussVbuspEccReservedSvbus {
        &self.cpsw_nuss_vbusp_ecc_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_sec_eoi_reg(&self) -> &CpswNussVbuspEccSecEoiReg {
        &self.cpsw_nuss_vbusp_ecc_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_sec_status_reg0(&self) -> &CpswNussVbuspEccSecStatusReg0 {
        &self.cpsw_nuss_vbusp_ecc_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_sec_enable_set_reg0(
        &self,
    ) -> &CpswNussVbuspEccSecEnableSetReg0 {
        &self.cpsw_nuss_vbusp_ecc_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0(
        &self,
    ) -> &CpswNussVbuspEccSecEnableClrReg0 {
        &self.cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_ded_eoi_reg(&self) -> &CpswNussVbuspEccDedEoiReg {
        &self.cpsw_nuss_vbusp_ecc_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_ded_status_reg0(&self) -> &CpswNussVbuspEccDedStatusReg0 {
        &self.cpsw_nuss_vbusp_ecc_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_ded_enable_set_reg0(
        &self,
    ) -> &CpswNussVbuspEccDedEnableSetReg0 {
        &self.cpsw_nuss_vbusp_ecc_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0(
        &self,
    ) -> &CpswNussVbuspEccDedEnableClrReg0 {
        &self.cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_aggr_enable_set(&self) -> &CpswNussVbuspEccAggrEnableSet {
        &self.cpsw_nuss_vbusp_ecc_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_aggr_enable_clr(&self) -> &CpswNussVbuspEccAggrEnableClr {
        &self.cpsw_nuss_vbusp_ecc_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_aggr_status_set(&self) -> &CpswNussVbuspEccAggrStatusSet {
        &self.cpsw_nuss_vbusp_ecc_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ecc_aggr_status_clr(&self) -> &CpswNussVbuspEccAggrStatusClr {
        &self.cpsw_nuss_vbusp_ecc_aggr_status_clr
    }
}
#[doc = "CPSW_NUSS_VBUSP_ECC_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_aggr_revision`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_aggr_revision")]
pub type CpswNussVbuspEccAggrRevision =
    crate::Reg<cpsw_nuss_vbusp_ecc_aggr_revision::CpswNussVbuspEccAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod cpsw_nuss_vbusp_ecc_aggr_revision;
#[doc = "CPSW_NUSS_VBUSP_ECC_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_ecc_vector`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_ecc_vector")]
pub type CpswNussVbuspEccEccVector =
    crate::Reg<cpsw_nuss_vbusp_ecc_ecc_vector::CpswNussVbuspEccEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod cpsw_nuss_vbusp_ecc_ecc_vector;
#[doc = "CPSW_NUSS_VBUSP_ECC_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_misc_status`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_misc_status")]
pub type CpswNussVbuspEccMiscStatus =
    crate::Reg<cpsw_nuss_vbusp_ecc_misc_status::CpswNussVbuspEccMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod cpsw_nuss_vbusp_ecc_misc_status;
#[doc = "CPSW_NUSS_VBUSP_ECC_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_reserved_svbus`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_reserved_svbus")]
pub type CpswNussVbuspEccReservedSvbus =
    crate::Reg<cpsw_nuss_vbusp_ecc_reserved_svbus::CpswNussVbuspEccReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod cpsw_nuss_vbusp_ecc_reserved_svbus;
#[doc = "CPSW_NUSS_VBUSP_ECC_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_sec_eoi_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_sec_eoi_reg")]
pub type CpswNussVbuspEccSecEoiReg =
    crate::Reg<cpsw_nuss_vbusp_ecc_sec_eoi_reg::CpswNussVbuspEccSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod cpsw_nuss_vbusp_ecc_sec_eoi_reg;
#[doc = "CPSW_NUSS_VBUSP_ECC_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_sec_status_reg0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_sec_status_reg0")]
pub type CpswNussVbuspEccSecStatusReg0 =
    crate::Reg<cpsw_nuss_vbusp_ecc_sec_status_reg0::CpswNussVbuspEccSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod cpsw_nuss_vbusp_ecc_sec_status_reg0;
#[doc = "CPSW_NUSS_VBUSP_ECC_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_sec_enable_set_reg0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_sec_enable_set_reg0")]
pub type CpswNussVbuspEccSecEnableSetReg0 =
    crate::Reg<cpsw_nuss_vbusp_ecc_sec_enable_set_reg0::CpswNussVbuspEccSecEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod cpsw_nuss_vbusp_ecc_sec_enable_set_reg0;
#[doc = "CPSW_NUSS_VBUSP_ECC_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_sec_enable_clr_reg0")]
pub type CpswNussVbuspEccSecEnableClrReg0 =
    crate::Reg<cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0::CpswNussVbuspEccSecEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod cpsw_nuss_vbusp_ecc_sec_enable_clr_reg0;
#[doc = "CPSW_NUSS_VBUSP_ECC_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_ded_eoi_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_ded_eoi_reg")]
pub type CpswNussVbuspEccDedEoiReg =
    crate::Reg<cpsw_nuss_vbusp_ecc_ded_eoi_reg::CpswNussVbuspEccDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod cpsw_nuss_vbusp_ecc_ded_eoi_reg;
#[doc = "CPSW_NUSS_VBUSP_ECC_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_ded_status_reg0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_ded_status_reg0")]
pub type CpswNussVbuspEccDedStatusReg0 =
    crate::Reg<cpsw_nuss_vbusp_ecc_ded_status_reg0::CpswNussVbuspEccDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod cpsw_nuss_vbusp_ecc_ded_status_reg0;
#[doc = "CPSW_NUSS_VBUSP_ECC_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_ded_enable_set_reg0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_ded_enable_set_reg0")]
pub type CpswNussVbuspEccDedEnableSetReg0 =
    crate::Reg<cpsw_nuss_vbusp_ecc_ded_enable_set_reg0::CpswNussVbuspEccDedEnableSetReg0Spec>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod cpsw_nuss_vbusp_ecc_ded_enable_set_reg0;
#[doc = "CPSW_NUSS_VBUSP_ECC_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_ded_enable_clr_reg0")]
pub type CpswNussVbuspEccDedEnableClrReg0 =
    crate::Reg<cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0::CpswNussVbuspEccDedEnableClrReg0Spec>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod cpsw_nuss_vbusp_ecc_ded_enable_clr_reg0;
#[doc = "CPSW_NUSS_VBUSP_ECC_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_aggr_enable_set`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_aggr_enable_set")]
pub type CpswNussVbuspEccAggrEnableSet =
    crate::Reg<cpsw_nuss_vbusp_ecc_aggr_enable_set::CpswNussVbuspEccAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod cpsw_nuss_vbusp_ecc_aggr_enable_set;
#[doc = "CPSW_NUSS_VBUSP_ECC_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_aggr_enable_clr`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_aggr_enable_clr")]
pub type CpswNussVbuspEccAggrEnableClr =
    crate::Reg<cpsw_nuss_vbusp_ecc_aggr_enable_clr::CpswNussVbuspEccAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod cpsw_nuss_vbusp_ecc_aggr_enable_clr;
#[doc = "CPSW_NUSS_VBUSP_ECC_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_aggr_status_set`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_aggr_status_set")]
pub type CpswNussVbuspEccAggrStatusSet =
    crate::Reg<cpsw_nuss_vbusp_ecc_aggr_status_set::CpswNussVbuspEccAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod cpsw_nuss_vbusp_ecc_aggr_status_set;
#[doc = "CPSW_NUSS_VBUSP_ECC_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ecc_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ecc_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ecc_aggr_status_clr`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ECC_aggr_status_clr")]
pub type CpswNussVbuspEccAggrStatusClr =
    crate::Reg<cpsw_nuss_vbusp_ecc_aggr_status_clr::CpswNussVbuspEccAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod cpsw_nuss_vbusp_ecc_aggr_status_clr;
