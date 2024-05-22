#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision: MsgmemWrap_EccAggrVbp_RegsAggrRevision,
    _reserved1: [u8; 0x04],
    msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector: MsgmemWrap_EccAggrVbp_RegsEccVector,
    msgmem_wrap__ecc_aggr_vbp__regs_misc_status: MsgmemWrap_EccAggrVbp_RegsMiscStatus,
    _reserved3: [u8; 0x2c],
    msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg: MsgmemWrap_EccAggrVbp_RegsSecEoiReg,
    msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0: MsgmemWrap_EccAggrVbp_RegsSecStatusReg0,
    _reserved5: [u8; 0x3c],
    msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0: MsgmemWrap_EccAggrVbp_RegsSecEnableSetReg0,
    _reserved6: [u8; 0x3c],
    msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0: MsgmemWrap_EccAggrVbp_RegsSecEnableClrReg0,
    _reserved7: [u8; 0x78],
    msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg: MsgmemWrap_EccAggrVbp_RegsDedEoiReg,
    msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0: MsgmemWrap_EccAggrVbp_RegsDedStatusReg0,
    _reserved9: [u8; 0x3c],
    msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0: MsgmemWrap_EccAggrVbp_RegsDedEnableSetReg0,
    _reserved10: [u8; 0x3c],
    msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0: MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0,
    _reserved11: [u8; 0x3c],
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set: MsgmemWrap_EccAggrVbp_RegsAggrEnableSet,
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr: MsgmemWrap_EccAggrVbp_RegsAggrEnableClr,
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set: MsgmemWrap_EccAggrVbp_RegsAggrStatusSet,
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr: MsgmemWrap_EccAggrVbp_RegsAggrStatusClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Revision parameters"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsAggrRevision {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision
    }
    #[doc = "0x08 - ECC Vector Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsEccVector {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector
    }
    #[doc = "0x0c - Misc Status"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_misc_status(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsMiscStatus {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_misc_status
    }
    #[doc = "0x3c - EOI Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsSecEoiReg {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsSecStatusReg0 {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0
    }
    #[doc = "0x80 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsSecEnableSetReg0 {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0
    }
    #[doc = "0xc0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsSecEnableClrReg0 {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0
    }
    #[doc = "0x13c - EOI Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsDedEoiReg {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg
    }
    #[doc = "0x140 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsDedStatusReg0 {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0
    }
    #[doc = "0x180 - Interrupt Enable Set Register 0"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsDedEnableSetReg0 {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0
    }
    #[doc = "0x1c0 - Interrupt Enable Clear Register 0"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0 {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0
    }
    #[doc = "0x200 - AGGR interrupt enable set Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsAggrEnableSet {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set
    }
    #[doc = "0x204 - AGGR interrupt enable clear Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsAggrEnableClr {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr
    }
    #[doc = "0x208 - AGGR interrupt status set Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsAggrStatusSet {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set
    }
    #[doc = "0x20c - AGGR interrupt status clear Register"]
    #[inline(always)]
    pub const fn msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr(
        &self,
    ) -> &MsgmemWrap_EccAggrVbp_RegsAggrStatusClr {
        &self.msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr
    }
}
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_revision (rw) register accessor: Revision parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_revision")]
pub type MsgmemWrap_EccAggrVbp_RegsAggrRevision = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision::MsgmemWrap_EccAggrVbp_RegsAggrRevisionSpec,
>;
#[doc = "Revision parameters"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_aggr_revision;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ecc_vector (rw) register accessor: ECC Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ecc_vector")]
pub type MsgmemWrap_EccAggrVbp_RegsEccVector =
    crate::Reg<msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector::MsgmemWrap_EccAggrVbp_RegsEccVectorSpec>;
#[doc = "ECC Vector Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_ecc_vector;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_misc_status (rw) register accessor: Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_misc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_misc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_misc_status`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_misc_status")]
pub type MsgmemWrap_EccAggrVbp_RegsMiscStatus = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_misc_status::MsgmemWrap_EccAggrVbp_RegsMiscStatusSpec,
>;
#[doc = "Misc Status"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_misc_status;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_eoi_reg")]
pub type MsgmemWrap_EccAggrVbp_RegsSecEoiReg = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg::MsgmemWrap_EccAggrVbp_RegsSecEoiRegSpec,
>;
#[doc = "EOI Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_sec_eoi_reg;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0")]
pub type MsgmemWrap_EccAggrVbp_RegsSecStatusReg0 = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec,
>;
#[doc = "Interrupt Status Register 0"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_enable_set_reg0")]
pub type MsgmemWrap_EccAggrVbp_RegsSecEnableSetReg0 = crate :: Reg < msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0 :: MsgmemWrap_EccAggrVbp_RegsSecEnableSetReg0Spec > ;
#[doc = "Interrupt Enable Set Register 0"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_set_reg0;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0")]
pub type MsgmemWrap_EccAggrVbp_RegsSecEnableClrReg0 = crate :: Reg < msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0 :: MsgmemWrap_EccAggrVbp_RegsSecEnableClrReg0Spec > ;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg")]
pub type MsgmemWrap_EccAggrVbp_RegsDedEoiReg = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::MsgmemWrap_EccAggrVbp_RegsDedEoiRegSpec,
>;
#[doc = "EOI Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_ded_eoi_reg;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_status_reg0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_status_reg0")]
pub type MsgmemWrap_EccAggrVbp_RegsDedStatusReg0 = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0::MsgmemWrap_EccAggrVbp_RegsDedStatusReg0Spec,
>;
#[doc = "Interrupt Status Register 0"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_ded_status_reg0;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0 (rw) register accessor: Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0")]
pub type MsgmemWrap_EccAggrVbp_RegsDedEnableSetReg0 = crate :: Reg < msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0 :: MsgmemWrap_EccAggrVbp_RegsDedEnableSetReg0Spec > ;
#[doc = "Interrupt Enable Set Register 0"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0 (rw) register accessor: Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0")]
pub type MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0 = crate :: Reg < msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0 :: MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec > ;
#[doc = "Interrupt Enable Clear Register 0"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set (rw) register accessor: AGGR interrupt enable set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_set")]
pub type MsgmemWrap_EccAggrVbp_RegsAggrEnableSet = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set::MsgmemWrap_EccAggrVbp_RegsAggrEnableSetSpec,
>;
#[doc = "AGGR interrupt enable set Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_set;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_clr (rw) register accessor: AGGR interrupt enable clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_enable_clr")]
pub type MsgmemWrap_EccAggrVbp_RegsAggrEnableClr = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr::MsgmemWrap_EccAggrVbp_RegsAggrEnableClrSpec,
>;
#[doc = "AGGR interrupt enable clear Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_aggr_enable_clr;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_status_set (rw) register accessor: AGGR interrupt status set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_status_set")]
pub type MsgmemWrap_EccAggrVbp_RegsAggrStatusSet = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set::MsgmemWrap_EccAggrVbp_RegsAggrStatusSetSpec,
>;
#[doc = "AGGR interrupt status set Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_set;
#[doc = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_status_clr (rw) register accessor: AGGR interrupt status clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr`]
module"]
#[doc(alias = "MSGMEM_WRAP__ECC_AGGR_VBP__REGS_aggr_status_clr")]
pub type MsgmemWrap_EccAggrVbp_RegsAggrStatusClr = crate::Reg<
    msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr::MsgmemWrap_EccAggrVbp_RegsAggrStatusClrSpec,
>;
#[doc = "AGGR interrupt status clear Register"]
pub mod msgmem_wrap__ecc_aggr_vbp__regs_aggr_status_clr;
