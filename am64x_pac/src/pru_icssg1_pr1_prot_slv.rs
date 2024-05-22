#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_protect__slv__regs_unlock_key: Pr1Protect_Slv_RegsUnlockKey,
    pr1_protect__slv__regs_cfg: Pr1Protect_Slv_RegsCfg,
}
impl RegisterBlock {
    #[doc = "0x00 - PR1_PROTECT__SLV__REGS_unlock_key"]
    #[inline(always)]
    pub const fn pr1_protect__slv__regs_unlock_key(&self) -> &Pr1Protect_Slv_RegsUnlockKey {
        &self.pr1_protect__slv__regs_unlock_key
    }
    #[doc = "0x04 - Config"]
    #[inline(always)]
    pub const fn pr1_protect__slv__regs_cfg(&self) -> &Pr1Protect_Slv_RegsCfg {
        &self.pr1_protect__slv__regs_cfg
    }
}
#[doc = "PR1_PROTECT__SLV__REGS_unlock_key (rw) register accessor: PR1_PROTECT__SLV__REGS_unlock_key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_protect__slv__regs_unlock_key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_protect__slv__regs_unlock_key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_protect__slv__regs_unlock_key`]
module"]
#[doc(alias = "PR1_PROTECT__SLV__REGS_unlock_key")]
pub type Pr1Protect_Slv_RegsUnlockKey =
    crate::Reg<pr1_protect__slv__regs_unlock_key::Pr1Protect_Slv_RegsUnlockKeySpec>;
#[doc = "PR1_PROTECT__SLV__REGS_unlock_key"]
pub mod pr1_protect__slv__regs_unlock_key;
#[doc = "PR1_PROTECT__SLV__REGS_cfg (rw) register accessor: Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_protect__slv__regs_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_protect__slv__regs_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_protect__slv__regs_cfg`]
module"]
#[doc(alias = "PR1_PROTECT__SLV__REGS_cfg")]
pub type Pr1Protect_Slv_RegsCfg =
    crate::Reg<pr1_protect__slv__regs_cfg::Pr1Protect_Slv_RegsCfgSpec>;
#[doc = "Config"]
pub mod pr1_protect__slv__regs_cfg;
