#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_cfg_revision: IntaggrCfgRevision,
    intaggr_cfg_intcap: IntaggrCfgIntcap,
    intaggr_cfg_auxcap: IntaggrCfgAuxcap,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn intaggr_cfg_revision(&self) -> &IntaggrCfgRevision {
        &self.intaggr_cfg_revision
    }
    #[doc = "0x08..0x10 - The IntCap Register contains information on virtual interrupts."]
    #[inline(always)]
    pub const fn intaggr_cfg_intcap(&self) -> &IntaggrCfgIntcap {
        &self.intaggr_cfg_intcap
    }
    #[doc = "0x10..0x18 - The AuxCap Register contains information on additional capabilities."]
    #[inline(always)]
    pub const fn intaggr_cfg_auxcap(&self) -> &IntaggrCfgAuxcap {
        &self.intaggr_cfg_auxcap
    }
}
#[doc = "INTAGGR_CFG_REVISION (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_cfg_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_cfg_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_cfg_revision`]
module"]
#[doc(alias = "INTAGGR_CFG_REVISION")]
pub type IntaggrCfgRevision = crate::Reg<intaggr_cfg_revision::IntaggrCfgRevisionSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod intaggr_cfg_revision;
#[doc = "INTAGGR_CFG_INTCAP (rw) register accessor: The IntCap Register contains information on virtual interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_cfg_intcap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_cfg_intcap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_cfg_intcap`]
module"]
#[doc(alias = "INTAGGR_CFG_INTCAP")]
pub type IntaggrCfgIntcap = crate::Reg<intaggr_cfg_intcap::IntaggrCfgIntcapSpec>;
#[doc = "The IntCap Register contains information on virtual interrupts."]
pub mod intaggr_cfg_intcap;
#[doc = "INTAGGR_CFG_AUXCAP (rw) register accessor: The AuxCap Register contains information on additional capabilities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_cfg_auxcap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_cfg_auxcap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_cfg_auxcap`]
module"]
#[doc(alias = "INTAGGR_CFG_AUXCAP")]
pub type IntaggrCfgAuxcap = crate::Reg<intaggr_cfg_auxcap::IntaggrCfgAuxcapSpec>;
#[doc = "The AuxCap Register contains information on additional capabilities."]
pub mod intaggr_cfg_auxcap;
