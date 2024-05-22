#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_intr_enable_set: IntaggrIntrEnableSet,
    intaggr_intr_enable_clr: IntaggrIntrEnableClr,
    intaggr_intr_status_set: IntaggrIntrStatusSet,
    intaggr_intr_status: IntaggrIntrStatus,
    intaggr_intr_status_mskd: IntaggrIntrStatusMskd,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - The Interrupt Enable Set register is written by software to enable (i.e. unmask) specified bits to allow their current status to be considered in the generation of the corresponding level sensitive virtual interrupt output."]
    #[inline(always)]
    pub const fn intaggr_intr_enable_set(&self) -> &IntaggrIntrEnableSet {
        &self.intaggr_intr_enable_set
    }
    #[doc = "0x08..0x10 - The Interrupt Enable Clear register is written by software to disable (i.e. mask) specified bits to disallow their current status from be considered in the generation of the corresponding level sensitive virtual interrupt output."]
    #[inline(always)]
    pub const fn intaggr_intr_enable_clr(&self) -> &IntaggrIntrEnableClr {
        &self.intaggr_intr_enable_clr
    }
    #[doc = "0x10..0x18 - The Interrupt Status register is read by software to determine the cause of an interrupt."]
    #[inline(always)]
    pub const fn intaggr_intr_status_set(&self) -> &IntaggrIntrStatusSet {
        &self.intaggr_intr_status_set
    }
    #[doc = "0x18..0x20 - The Interrupt Status register is read by software to determine the cause of an interrupt."]
    #[inline(always)]
    pub const fn intaggr_intr_status(&self) -> &IntaggrIntrStatus {
        &self.intaggr_intr_status
    }
    #[doc = "0x20..0x28 - The Interrupt Masked Status register can be read by software to determine the cause of an interrupt."]
    #[inline(always)]
    pub const fn intaggr_intr_status_mskd(&self) -> &IntaggrIntrStatusMskd {
        &self.intaggr_intr_status_mskd
    }
}
#[doc = "INTAGGR_INTR_ENABLE_SET (rw) register accessor: The Interrupt Enable Set register is written by software to enable (i.e. unmask) specified bits to allow their current status to be considered in the generation of the corresponding level sensitive virtual interrupt output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_intr_enable_set`]
module"]
#[doc(alias = "INTAGGR_INTR_ENABLE_SET")]
pub type IntaggrIntrEnableSet = crate::Reg<intaggr_intr_enable_set::IntaggrIntrEnableSetSpec>;
#[doc = "The Interrupt Enable Set register is written by software to enable (i.e. unmask) specified bits to allow their current status to be considered in the generation of the corresponding level sensitive virtual interrupt output."]
pub mod intaggr_intr_enable_set;
#[doc = "INTAGGR_INTR_ENABLE_CLR (rw) register accessor: The Interrupt Enable Clear register is written by software to disable (i.e. mask) specified bits to disallow their current status from be considered in the generation of the corresponding level sensitive virtual interrupt output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_intr_enable_clr`]
module"]
#[doc(alias = "INTAGGR_INTR_ENABLE_CLR")]
pub type IntaggrIntrEnableClr = crate::Reg<intaggr_intr_enable_clr::IntaggrIntrEnableClrSpec>;
#[doc = "The Interrupt Enable Clear register is written by software to disable (i.e. mask) specified bits to disallow their current status from be considered in the generation of the corresponding level sensitive virtual interrupt output."]
pub mod intaggr_intr_enable_clr;
#[doc = "INTAGGR_INTR_STATUS_SET (rw) register accessor: The Interrupt Status register is read by software to determine the cause of an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_status_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_status_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_intr_status_set`]
module"]
#[doc(alias = "INTAGGR_INTR_STATUS_SET")]
pub type IntaggrIntrStatusSet = crate::Reg<intaggr_intr_status_set::IntaggrIntrStatusSetSpec>;
#[doc = "The Interrupt Status register is read by software to determine the cause of an interrupt."]
pub mod intaggr_intr_status_set;
#[doc = "INTAGGR_INTR_STATUS (rw) register accessor: The Interrupt Status register is read by software to determine the cause of an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_intr_status`]
module"]
#[doc(alias = "INTAGGR_INTR_STATUS")]
pub type IntaggrIntrStatus = crate::Reg<intaggr_intr_status::IntaggrIntrStatusSpec>;
#[doc = "The Interrupt Status register is read by software to determine the cause of an interrupt."]
pub mod intaggr_intr_status;
#[doc = "INTAGGR_INTR_STATUS_MSKD (rw) register accessor: The Interrupt Masked Status register can be read by software to determine the cause of an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_intr_status_mskd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_intr_status_mskd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_intr_status_mskd`]
module"]
#[doc(alias = "INTAGGR_INTR_STATUS_MSKD")]
pub type IntaggrIntrStatusMskd = crate::Reg<intaggr_intr_status_mskd::IntaggrIntrStatusMskdSpec>;
#[doc = "The Interrupt Masked Status register can be read by software to determine the cause of an interrupt."]
pub mod intaggr_intr_status_mskd;
