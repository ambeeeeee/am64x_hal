#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_mdio_v1p7__mdio__regs_mdio_version_reg: Pr1MdioV1p7_Mdio_RegsMdioVersionReg,
    pr1_mdio_v1p7__mdio__regs_control_reg: Pr1MdioV1p7_Mdio_RegsControlReg,
    pr1_mdio_v1p7__mdio__regs_alive_reg: Pr1MdioV1p7_Mdio_RegsAliveReg,
    pr1_mdio_v1p7__mdio__regs_link_reg: Pr1MdioV1p7_Mdio_RegsLinkReg,
    pr1_mdio_v1p7__mdio__regs_link_int_raw_reg: Pr1MdioV1p7_Mdio_RegsLinkIntRawReg,
    pr1_mdio_v1p7__mdio__regs_link_int_masked_reg: Pr1MdioV1p7_Mdio_RegsLinkIntMaskedReg,
    pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg: Pr1MdioV1p7_Mdio_RegsLinkIntMaskSetReg,
    pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg: Pr1MdioV1p7_Mdio_RegsLinkIntMaskClearReg,
    pr1_mdio_v1p7__mdio__regs_user_int_raw_reg: Pr1MdioV1p7_Mdio_RegsUserIntRawReg,
    pr1_mdio_v1p7__mdio__regs_user_int_masked_reg: Pr1MdioV1p7_Mdio_RegsUserIntMaskedReg,
    pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg: Pr1MdioV1p7_Mdio_RegsUserIntMaskSetReg,
    pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg: Pr1MdioV1p7_Mdio_RegsUserIntMaskClearReg,
    pr1_mdio_v1p7__mdio__regs_manual_if_reg: Pr1MdioV1p7_Mdio_RegsManualIfReg,
    pr1_mdio_v1p7__mdio__regs_poll_reg: Pr1MdioV1p7_Mdio_RegsPollReg,
    pr1_mdio_v1p7__mdio__regs_poll_en_reg: Pr1MdioV1p7_Mdio_RegsPollEnReg,
    pr1_mdio_v1p7__mdio__regs_claus45_reg: Pr1MdioV1p7_Mdio_RegsClaus45Reg,
    pr1_mdio_v1p7__mdio__regs_user_addr0_reg: Pr1MdioV1p7_Mdio_RegsUserAddr0Reg,
    pr1_mdio_v1p7__mdio__regs_user_addr1_reg: Pr1MdioV1p7_Mdio_RegsUserAddr1Reg,
    _reserved18: [u8; 0x38],
    pr1_mdio_v1p7__mdio__regs_user_access_reg: Pr1MdioV1p7_Mdio_RegsUserAccessReg,
    pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg: Pr1MdioV1p7_Mdio_RegsUserPhySelReg,
}
impl RegisterBlock {
    #[doc = "0x00 - version_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_mdio_version_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsMdioVersionReg {
        &self.pr1_mdio_v1p7__mdio__regs_mdio_version_reg
    }
    #[doc = "0x04 - control_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_control_reg(&self) -> &Pr1MdioV1p7_Mdio_RegsControlReg {
        &self.pr1_mdio_v1p7__mdio__regs_control_reg
    }
    #[doc = "0x08 - alive_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_alive_reg(&self) -> &Pr1MdioV1p7_Mdio_RegsAliveReg {
        &self.pr1_mdio_v1p7__mdio__regs_alive_reg
    }
    #[doc = "0x0c - link_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_link_reg(&self) -> &Pr1MdioV1p7_Mdio_RegsLinkReg {
        &self.pr1_mdio_v1p7__mdio__regs_link_reg
    }
    #[doc = "0x10 - link_int_raw_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_link_int_raw_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsLinkIntRawReg {
        &self.pr1_mdio_v1p7__mdio__regs_link_int_raw_reg
    }
    #[doc = "0x14 - link_int_masked_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_link_int_masked_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsLinkIntMaskedReg {
        &self.pr1_mdio_v1p7__mdio__regs_link_int_masked_reg
    }
    #[doc = "0x18 - link_int_mask_set_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsLinkIntMaskSetReg {
        &self.pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg
    }
    #[doc = "0x1c - link_int_mask_clear_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsLinkIntMaskClearReg {
        &self.pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg
    }
    #[doc = "0x20 - user_int_raw_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_int_raw_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserIntRawReg {
        &self.pr1_mdio_v1p7__mdio__regs_user_int_raw_reg
    }
    #[doc = "0x24 - user_int_masked_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_int_masked_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserIntMaskedReg {
        &self.pr1_mdio_v1p7__mdio__regs_user_int_masked_reg
    }
    #[doc = "0x28 - user_int_mask_set_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserIntMaskSetReg {
        &self.pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg
    }
    #[doc = "0x2c - user_int_mask_clear_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserIntMaskClearReg {
        &self.pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg
    }
    #[doc = "0x30 - manual_if_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_manual_if_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsManualIfReg {
        &self.pr1_mdio_v1p7__mdio__regs_manual_if_reg
    }
    #[doc = "0x34 - poll_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_poll_reg(&self) -> &Pr1MdioV1p7_Mdio_RegsPollReg {
        &self.pr1_mdio_v1p7__mdio__regs_poll_reg
    }
    #[doc = "0x38 - poll_en_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_poll_en_reg(&self) -> &Pr1MdioV1p7_Mdio_RegsPollEnReg {
        &self.pr1_mdio_v1p7__mdio__regs_poll_en_reg
    }
    #[doc = "0x3c - PR1_MDIO_V1P7__MDIO__REGS_CLAUS45_REG"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_claus45_reg(&self) -> &Pr1MdioV1p7_Mdio_RegsClaus45Reg {
        &self.pr1_mdio_v1p7__mdio__regs_claus45_reg
    }
    #[doc = "0x40 - MDIO USER Address 0"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_addr0_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserAddr0Reg {
        &self.pr1_mdio_v1p7__mdio__regs_user_addr0_reg
    }
    #[doc = "0x44 - MDIO USER Address 1"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_addr1_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserAddr1Reg {
        &self.pr1_mdio_v1p7__mdio__regs_user_addr1_reg
    }
    #[doc = "0x80 - user_access_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_access_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserAccessReg {
        &self.pr1_mdio_v1p7__mdio__regs_user_access_reg
    }
    #[doc = "0x84 - user_phy_sel_reg"]
    #[inline(always)]
    pub const fn pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg(
        &self,
    ) -> &Pr1MdioV1p7_Mdio_RegsUserPhySelReg {
        &self.pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg
    }
}
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_MDIO_VERSION_REG (rw) register accessor: version_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_mdio_version_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_mdio_version_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_mdio_version_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_MDIO_VERSION_REG")]
pub type Pr1MdioV1p7_Mdio_RegsMdioVersionReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_mdio_version_reg::Pr1MdioV1p7_Mdio_RegsMdioVersionRegSpec>;
#[doc = "version_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_mdio_version_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_CONTROL_REG (rw) register accessor: control_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_control_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_CONTROL_REG")]
pub type Pr1MdioV1p7_Mdio_RegsControlReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_control_reg::Pr1MdioV1p7_Mdio_RegsControlRegSpec>;
#[doc = "control_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_control_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_ALIVE_REG (rw) register accessor: alive_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_alive_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_alive_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_alive_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_ALIVE_REG")]
pub type Pr1MdioV1p7_Mdio_RegsAliveReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_alive_reg::Pr1MdioV1p7_Mdio_RegsAliveRegSpec>;
#[doc = "alive_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_alive_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_LINK_REG (rw) register accessor: link_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_link_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_link_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_link_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_LINK_REG")]
pub type Pr1MdioV1p7_Mdio_RegsLinkReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_link_reg::Pr1MdioV1p7_Mdio_RegsLinkRegSpec>;
#[doc = "link_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_link_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_RAW_REG (rw) register accessor: link_int_raw_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_link_int_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_link_int_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_link_int_raw_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_RAW_REG")]
pub type Pr1MdioV1p7_Mdio_RegsLinkIntRawReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_link_int_raw_reg::Pr1MdioV1p7_Mdio_RegsLinkIntRawRegSpec>;
#[doc = "link_int_raw_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_link_int_raw_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_MASKED_REG (rw) register accessor: link_int_masked_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_link_int_masked_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_link_int_masked_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_link_int_masked_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_MASKED_REG")]
pub type Pr1MdioV1p7_Mdio_RegsLinkIntMaskedReg = crate::Reg<
    pr1_mdio_v1p7__mdio__regs_link_int_masked_reg::Pr1MdioV1p7_Mdio_RegsLinkIntMaskedRegSpec,
>;
#[doc = "link_int_masked_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_link_int_masked_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_MASK_SET_REG (rw) register accessor: link_int_mask_set_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_MASK_SET_REG")]
pub type Pr1MdioV1p7_Mdio_RegsLinkIntMaskSetReg = crate::Reg<
    pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg::Pr1MdioV1p7_Mdio_RegsLinkIntMaskSetRegSpec,
>;
#[doc = "link_int_mask_set_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_link_int_mask_set_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_MASK_CLEAR_REG (rw) register accessor: link_int_mask_clear_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_LINK_INT_MASK_CLEAR_REG")]
pub type Pr1MdioV1p7_Mdio_RegsLinkIntMaskClearReg = crate::Reg<
    pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg::Pr1MdioV1p7_Mdio_RegsLinkIntMaskClearRegSpec,
>;
#[doc = "link_int_mask_clear_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_link_int_mask_clear_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_RAW_REG (rw) register accessor: user_int_raw_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_int_raw_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_RAW_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserIntRawReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec>;
#[doc = "user_int_raw_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_user_int_raw_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASKED_REG (rw) register accessor: user_int_masked_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_int_masked_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASKED_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserIntMaskedReg = crate::Reg<
    pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec,
>;
#[doc = "user_int_masked_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_user_int_masked_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASK_SET_REG (rw) register accessor: user_int_mask_set_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASK_SET_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserIntMaskSetReg = crate::Reg<
    pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg::Pr1MdioV1p7_Mdio_RegsUserIntMaskSetRegSpec,
>;
#[doc = "user_int_mask_set_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_user_int_mask_set_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASK_CLEAR_REG (rw) register accessor: user_int_mask_clear_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASK_CLEAR_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserIntMaskClearReg = crate::Reg<
    pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg::Pr1MdioV1p7_Mdio_RegsUserIntMaskClearRegSpec,
>;
#[doc = "user_int_mask_clear_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_user_int_mask_clear_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_MANUAL_IF_REG (rw) register accessor: manual_if_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_manual_if_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_manual_if_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_manual_if_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_MANUAL_IF_REG")]
pub type Pr1MdioV1p7_Mdio_RegsManualIfReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_manual_if_reg::Pr1MdioV1p7_Mdio_RegsManualIfRegSpec>;
#[doc = "manual_if_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_manual_if_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_POLL_REG (rw) register accessor: poll_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_poll_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_poll_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_poll_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_POLL_REG")]
pub type Pr1MdioV1p7_Mdio_RegsPollReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_poll_reg::Pr1MdioV1p7_Mdio_RegsPollRegSpec>;
#[doc = "poll_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_poll_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_POLL_EN_REG (rw) register accessor: poll_en_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_poll_en_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_poll_en_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_poll_en_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_POLL_EN_REG")]
pub type Pr1MdioV1p7_Mdio_RegsPollEnReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_poll_en_reg::Pr1MdioV1p7_Mdio_RegsPollEnRegSpec>;
#[doc = "poll_en_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_poll_en_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_CLAUS45_REG (rw) register accessor: PR1_MDIO_V1P7__MDIO__REGS_CLAUS45_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_claus45_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_claus45_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_claus45_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_CLAUS45_REG")]
pub type Pr1MdioV1p7_Mdio_RegsClaus45Reg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_claus45_reg::Pr1MdioV1p7_Mdio_RegsClaus45RegSpec>;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_CLAUS45_REG"]
pub mod pr1_mdio_v1p7__mdio__regs_claus45_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR0_REG (rw) register accessor: MDIO USER Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_addr0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_addr0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_addr0_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR0_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserAddr0Reg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_user_addr0_reg::Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec>;
#[doc = "MDIO USER Address 0"]
pub mod pr1_mdio_v1p7__mdio__regs_user_addr0_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR1_REG (rw) register accessor: MDIO USER Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_addr1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_addr1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_addr1_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR1_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserAddr1Reg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_user_addr1_reg::Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec>;
#[doc = "MDIO USER Address 1"]
pub mod pr1_mdio_v1p7__mdio__regs_user_addr1_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_ACCESS_REG (rw) register accessor: user_access_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_access_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_access_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_access_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_ACCESS_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserAccessReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_user_access_reg::Pr1MdioV1p7_Mdio_RegsUserAccessRegSpec>;
#[doc = "user_access_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_user_access_reg;
#[doc = "PR1_MDIO_V1P7__MDIO__REGS_USER_PHY_SEL_REG (rw) register accessor: user_phy_sel_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg`]
module"]
#[doc(alias = "PR1_MDIO_V1P7__MDIO__REGS_USER_PHY_SEL_REG")]
pub type Pr1MdioV1p7_Mdio_RegsUserPhySelReg =
    crate::Reg<pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec>;
#[doc = "user_phy_sel_reg"]
pub mod pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg;
