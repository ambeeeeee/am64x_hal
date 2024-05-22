#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ecc_aggr_rxmem__cfg__regs_aggr_revision: EccAggrRxmem_Cfg_RegsAggrRevision,
    _reserved1: [u8; 0x04],
    ecc_aggr_rxmem__cfg__regs_ecc_vector: EccAggrRxmem_Cfg_RegsEccVector,
    ecc_aggr_rxmem__cfg__regs_misc_status: EccAggrRxmem_Cfg_RegsMiscStatus,
    ecc_aggr_rxmem__cfg__regs_reserved_svbus: EccAggrRxmem_Cfg_RegsReservedSvbus,
    _reserved4: [u8; 0x28],
    ecc_aggr_rxmem__cfg__regs_sec_eoi_reg: EccAggrRxmem_Cfg_RegsSecEoiReg,
    ecc_aggr_rxmem__cfg__regs_sec_status_reg0: EccAggrRxmem_Cfg_RegsSecStatusReg0,
    _reserved6: [u8; 0x3c],
    ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0: EccAggrRxmem_Cfg_RegsSecEnableSetReg0,
    _reserved7: [u8; 0x3c],
    ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0: EccAggrRxmem_Cfg_RegsSecEnableClrReg0,
    _reserved8: [u8; 0x78],
    ecc_aggr_rxmem__cfg__regs_ded_eoi_reg: EccAggrRxmem_Cfg_RegsDedEoiReg,
    ecc_aggr_rxmem__cfg__regs_ded_status_reg0: EccAggrRxmem_Cfg_RegsDedStatusReg0,
    _reserved10: [u8; 0x3c],
    ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0: EccAggrRxmem_Cfg_RegsDedEnableSetReg0,
    _reserved11: [u8; 0x3c],
    ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0: EccAggrRxmem_Cfg_RegsDedEnableClrReg0,
    _reserved12: [u8; 0x3c],
    ecc_aggr_rxmem__cfg__regs_aggr_enable_set: EccAggrRxmem_Cfg_RegsAggrEnableSet,
    ecc_aggr_rxmem__cfg__regs_aggr_enable_clr: EccAggrRxmem_Cfg_RegsAggrEnableClr,
    ecc_aggr_rxmem__cfg__regs_aggr_status_set: EccAggrRxmem_Cfg_RegsAggrStatusSet,
    ecc_aggr_rxmem__cfg__regs_aggr_status_clr: EccAggrRxmem_Cfg_RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_aggr_revision(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsAggrRevision {
        &self.ecc_aggr_rxmem__cfg__regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_ecc_vector(&self) -> &EccAggrRxmem_Cfg_RegsEccVector {
        &self.ecc_aggr_rxmem__cfg__regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_misc_status(&self) -> &EccAggrRxmem_Cfg_RegsMiscStatus {
        &self.ecc_aggr_rxmem__cfg__regs_misc_status
    }
    #[doc = "0x10 - Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_reserved_svbus(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsReservedSvbus {
        &self.ecc_aggr_rxmem__cfg__regs_reserved_svbus
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_sec_eoi_reg(&self) -> &EccAggrRxmem_Cfg_RegsSecEoiReg {
        &self.ecc_aggr_rxmem__cfg__regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_sec_status_reg0(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsSecStatusReg0 {
        &self.ecc_aggr_rxmem__cfg__regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsSecEnableSetReg0 {
        &self.ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsSecEnableClrReg0 {
        &self.ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_ded_eoi_reg(&self) -> &EccAggrRxmem_Cfg_RegsDedEoiReg {
        &self.ecc_aggr_rxmem__cfg__regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_ded_status_reg0(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsDedStatusReg0 {
        &self.ecc_aggr_rxmem__cfg__regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsDedEnableSetReg0 {
        &self.ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsDedEnableClrReg0 {
        &self.ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_aggr_enable_set(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsAggrEnableSet {
        &self.ecc_aggr_rxmem__cfg__regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_aggr_enable_clr(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsAggrEnableClr {
        &self.ecc_aggr_rxmem__cfg__regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_aggr_status_set(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsAggrStatusSet {
        &self.ecc_aggr_rxmem__cfg__regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn ecc_aggr_rxmem__cfg__regs_aggr_status_clr(
        &self,
    ) -> &EccAggrRxmem_Cfg_RegsAggrStatusClr {
        &self.ecc_aggr_rxmem__cfg__regs_aggr_status_clr
    }
}
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_aggr_revision`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_aggr_revision")]
pub type EccAggrRxmem_Cfg_RegsAggrRevision =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_aggr_revision::EccAggrRxmem_Cfg_RegsAggrRevisionSpec>;
#[doc = "Revision parameters"]
pub mod ecc_aggr_rxmem__cfg__regs_aggr_revision;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_ecc_vector`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_ecc_vector")]
pub type EccAggrRxmem_Cfg_RegsEccVector =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_ecc_vector::EccAggrRxmem_Cfg_RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod ecc_aggr_rxmem__cfg__regs_ecc_vector;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_misc_status`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_misc_status")]
pub type EccAggrRxmem_Cfg_RegsMiscStatus =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_misc_status::EccAggrRxmem_Cfg_RegsMiscStatusSpec>;
#[doc = "Misc Status"]
pub mod ecc_aggr_rxmem__cfg__regs_misc_status;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_reserved_svbus (rw) register accessor: Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_reserved_svbus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_reserved_svbus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_reserved_svbus`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_reserved_svbus")]
pub type EccAggrRxmem_Cfg_RegsReservedSvbus =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_reserved_svbus::EccAggrRxmem_Cfg_RegsReservedSvbusSpec>;
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets."]
pub mod ecc_aggr_rxmem__cfg__regs_reserved_svbus;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_sec_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_sec_eoi_reg")]
pub type EccAggrRxmem_Cfg_RegsSecEoiReg =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_sec_eoi_reg::EccAggrRxmem_Cfg_RegsSecEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_rxmem__cfg__regs_sec_eoi_reg;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_sec_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_sec_status_reg0")]
pub type EccAggrRxmem_Cfg_RegsSecStatusReg0 =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_sec_status_reg0::EccAggrRxmem_Cfg_RegsSecStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_rxmem__cfg__regs_sec_status_reg0;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_sec_enable_set_reg0")]
pub type EccAggrRxmem_Cfg_RegsSecEnableSetReg0 = crate::Reg<
    ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0::EccAggrRxmem_Cfg_RegsSecEnableSetReg0Spec,
>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_rxmem__cfg__regs_sec_enable_set_reg0;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_sec_enable_clr_reg0")]
pub type EccAggrRxmem_Cfg_RegsSecEnableClrReg0 = crate::Reg<
    ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0::EccAggrRxmem_Cfg_RegsSecEnableClrReg0Spec,
>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_rxmem__cfg__regs_sec_enable_clr_reg0;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_ded_eoi_reg`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_ded_eoi_reg")]
pub type EccAggrRxmem_Cfg_RegsDedEoiReg =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::EccAggrRxmem_Cfg_RegsDedEoiRegSpec>;
#[doc = "EOI Register"]
pub mod ecc_aggr_rxmem__cfg__regs_ded_eoi_reg;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_ded_status_reg0`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_ded_status_reg0")]
pub type EccAggrRxmem_Cfg_RegsDedStatusReg0 =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_ded_status_reg0::EccAggrRxmem_Cfg_RegsDedStatusReg0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod ecc_aggr_rxmem__cfg__regs_ded_status_reg0;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_ded_enable_set_reg0")]
pub type EccAggrRxmem_Cfg_RegsDedEnableSetReg0 = crate::Reg<
    ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec,
>;
#[doc = "Interrupt Enable Set Register 0"]
pub mod ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_ded_enable_clr_reg0")]
pub type EccAggrRxmem_Cfg_RegsDedEnableClrReg0 = crate::Reg<
    ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0::EccAggrRxmem_Cfg_RegsDedEnableClrReg0Spec,
>;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod ecc_aggr_rxmem__cfg__regs_ded_enable_clr_reg0;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_aggr_enable_set`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_aggr_enable_set")]
pub type EccAggrRxmem_Cfg_RegsAggrEnableSet =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_aggr_enable_set::EccAggrRxmem_Cfg_RegsAggrEnableSetSpec>;
#[doc = "AGGR interrupt enable set Register"]
pub mod ecc_aggr_rxmem__cfg__regs_aggr_enable_set;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_aggr_enable_clr`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_aggr_enable_clr")]
pub type EccAggrRxmem_Cfg_RegsAggrEnableClr =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_aggr_enable_clr::EccAggrRxmem_Cfg_RegsAggrEnableClrSpec>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod ecc_aggr_rxmem__cfg__regs_aggr_enable_clr;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_aggr_status_set`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_aggr_status_set")]
pub type EccAggrRxmem_Cfg_RegsAggrStatusSet =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_aggr_status_set::EccAggrRxmem_Cfg_RegsAggrStatusSetSpec>;
#[doc = "AGGR interrupt status set Register"]
pub mod ecc_aggr_rxmem__cfg__regs_aggr_status_set;
#[doc = "ECC_AGGR_RXMEM__CFG__REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_aggr_rxmem__cfg__regs_aggr_status_clr`]
module"]
#[doc(alias = "ECC_AGGR_RXMEM__CFG__REGS_aggr_status_clr")]
pub type EccAggrRxmem_Cfg_RegsAggrStatusClr =
    crate::Reg<ecc_aggr_rxmem__cfg__regs_aggr_status_clr::EccAggrRxmem_Cfg_RegsAggrStatusClrSpec>;
#[doc = "AGGR interrupt status clear Register"]
pub mod ecc_aggr_rxmem__cfg__regs_aggr_status_clr;
