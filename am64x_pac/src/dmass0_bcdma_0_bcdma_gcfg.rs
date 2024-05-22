#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcdma_gcfg_revision: BcdmaGcfgRevision,
    bcdma_gcfg_perf_ctrl: BcdmaGcfgPerfCtrl,
    bcdma_gcfg_emu_ctrl: BcdmaGcfgEmuCtrl,
    _reserved3: [u8; 0x04],
    bcdma_gcfg_psil_to: BcdmaGcfgPsilTo,
    _reserved4: [u8; 0x0c],
    bcdma_gcfg_cap0: BcdmaGcfgCap0,
    bcdma_gcfg_cap1: BcdmaGcfgCap1,
    bcdma_gcfg_cap2: BcdmaGcfgCap2,
    bcdma_gcfg_cap3: BcdmaGcfgCap3,
    bcdma_gcfg_cap4: BcdmaGcfgCap4,
    _reserved9: [u8; 0x2c],
    bcdma_gcfg_pm0: BcdmaGcfgPm0,
    bcdma_gcfg_pm1: BcdmaGcfgPm1,
    _reserved11: [u8; 0x10],
    bcdma_gcfg_dbga: BcdmaGcfgDbga,
    bcdma_gcfg_dbgd: BcdmaGcfgDbgd,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn bcdma_gcfg_revision(&self) -> &BcdmaGcfgRevision {
        &self.bcdma_gcfg_revision
    }
    #[doc = "0x04 - The performance control register contains fields which can be used to adjust the performance of the BCDMA in the system."]
    #[inline(always)]
    pub const fn bcdma_gcfg_perf_ctrl(&self) -> &BcdmaGcfgPerfCtrl {
        &self.bcdma_gcfg_perf_ctrl
    }
    #[doc = "0x08 - The emulation control register is used to control the behavior of the DMA when the emususp input is asserted."]
    #[inline(always)]
    pub const fn bcdma_gcfg_emu_ctrl(&self) -> &BcdmaGcfgEmuCtrl {
        &self.bcdma_gcfg_emu_ctrl
    }
    #[doc = "0x10 - The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy."]
    #[inline(always)]
    pub const fn bcdma_gcfg_psil_to(&self) -> &BcdmaGcfgPsilTo {
        &self.bcdma_gcfg_psil_to
    }
    #[doc = "0x20 - The Capabilities Register 0 specifies which standard features this BCDMA instance supports."]
    #[inline(always)]
    pub const fn bcdma_gcfg_cap0(&self) -> &BcdmaGcfgCap0 {
        &self.bcdma_gcfg_cap0
    }
    #[doc = "0x24 - The Capabilities Register 1 specifies which standard features this BCDMA instance supports."]
    #[inline(always)]
    pub const fn bcdma_gcfg_cap1(&self) -> &BcdmaGcfgCap1 {
        &self.bcdma_gcfg_cap1
    }
    #[doc = "0x28 - The Capabilities Register 2 specifies how many resources this BCDMA instance supports."]
    #[inline(always)]
    pub const fn bcdma_gcfg_cap2(&self) -> &BcdmaGcfgCap2 {
        &self.bcdma_gcfg_cap2
    }
    #[doc = "0x2c - The Capabilities Register 3 specifies how many resources this BCDMA instance supports."]
    #[inline(always)]
    pub const fn bcdma_gcfg_cap3(&self) -> &BcdmaGcfgCap3 {
        &self.bcdma_gcfg_cap3
    }
    #[doc = "0x30 - The Capabilities Register 4 specifies how many resources this BCDMA instance supports."]
    #[inline(always)]
    pub const fn bcdma_gcfg_cap4(&self) -> &BcdmaGcfgCap4 {
        &self.bcdma_gcfg_cap4
    }
    #[doc = "0x60 - This register enables or inhibits automatic clock gating to individual sub-blocks"]
    #[inline(always)]
    pub const fn bcdma_gcfg_pm0(&self) -> &BcdmaGcfgPm0 {
        &self.bcdma_gcfg_pm0
    }
    #[doc = "0x64 - This register enables or inhibits automatic clock gating to individual sub-blocks"]
    #[inline(always)]
    pub const fn bcdma_gcfg_pm1(&self) -> &BcdmaGcfgPm1 {
        &self.bcdma_gcfg_pm1
    }
    #[doc = "0x78 - This register provides a writable address which allows debug information to be read from the Debug Data Register"]
    #[inline(always)]
    pub const fn bcdma_gcfg_dbga(&self) -> &BcdmaGcfgDbga {
        &self.bcdma_gcfg_dbga
    }
    #[doc = "0x7c - This register provides read only debug data"]
    #[inline(always)]
    pub const fn bcdma_gcfg_dbgd(&self) -> &BcdmaGcfgDbgd {
        &self.bcdma_gcfg_dbgd
    }
}
#[doc = "BCDMA_GCFG_REVISION (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_revision`]
module"]
#[doc(alias = "BCDMA_GCFG_REVISION")]
pub type BcdmaGcfgRevision = crate::Reg<bcdma_gcfg_revision::BcdmaGcfgRevisionSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod bcdma_gcfg_revision;
#[doc = "BCDMA_GCFG_PERF_CTRL (rw) register accessor: The performance control register contains fields which can be used to adjust the performance of the BCDMA in the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_perf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_perf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_perf_ctrl`]
module"]
#[doc(alias = "BCDMA_GCFG_PERF_CTRL")]
pub type BcdmaGcfgPerfCtrl = crate::Reg<bcdma_gcfg_perf_ctrl::BcdmaGcfgPerfCtrlSpec>;
#[doc = "The performance control register contains fields which can be used to adjust the performance of the BCDMA in the system."]
pub mod bcdma_gcfg_perf_ctrl;
#[doc = "BCDMA_GCFG_EMU_CTRL (rw) register accessor: The emulation control register is used to control the behavior of the DMA when the emususp input is asserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_emu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_emu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_emu_ctrl`]
module"]
#[doc(alias = "BCDMA_GCFG_EMU_CTRL")]
pub type BcdmaGcfgEmuCtrl = crate::Reg<bcdma_gcfg_emu_ctrl::BcdmaGcfgEmuCtrlSpec>;
#[doc = "The emulation control register is used to control the behavior of the DMA when the emususp input is asserted."]
pub mod bcdma_gcfg_emu_ctrl;
#[doc = "BCDMA_GCFG_PSIL_TO (rw) register accessor: The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_psil_to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_psil_to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_psil_to`]
module"]
#[doc(alias = "BCDMA_GCFG_PSIL_TO")]
pub type BcdmaGcfgPsilTo = crate::Reg<bcdma_gcfg_psil_to::BcdmaGcfgPsilToSpec>;
#[doc = "The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy."]
pub mod bcdma_gcfg_psil_to;
#[doc = "BCDMA_GCFG_CAP0 (rw) register accessor: The Capabilities Register 0 specifies which standard features this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_cap0`]
module"]
#[doc(alias = "BCDMA_GCFG_CAP0")]
pub type BcdmaGcfgCap0 = crate::Reg<bcdma_gcfg_cap0::BcdmaGcfgCap0Spec>;
#[doc = "The Capabilities Register 0 specifies which standard features this BCDMA instance supports."]
pub mod bcdma_gcfg_cap0;
#[doc = "BCDMA_GCFG_CAP1 (rw) register accessor: The Capabilities Register 1 specifies which standard features this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_cap1`]
module"]
#[doc(alias = "BCDMA_GCFG_CAP1")]
pub type BcdmaGcfgCap1 = crate::Reg<bcdma_gcfg_cap1::BcdmaGcfgCap1Spec>;
#[doc = "The Capabilities Register 1 specifies which standard features this BCDMA instance supports."]
pub mod bcdma_gcfg_cap1;
#[doc = "BCDMA_GCFG_CAP2 (rw) register accessor: The Capabilities Register 2 specifies how many resources this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_cap2`]
module"]
#[doc(alias = "BCDMA_GCFG_CAP2")]
pub type BcdmaGcfgCap2 = crate::Reg<bcdma_gcfg_cap2::BcdmaGcfgCap2Spec>;
#[doc = "The Capabilities Register 2 specifies how many resources this BCDMA instance supports."]
pub mod bcdma_gcfg_cap2;
#[doc = "BCDMA_GCFG_CAP3 (rw) register accessor: The Capabilities Register 3 specifies how many resources this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_cap3`]
module"]
#[doc(alias = "BCDMA_GCFG_CAP3")]
pub type BcdmaGcfgCap3 = crate::Reg<bcdma_gcfg_cap3::BcdmaGcfgCap3Spec>;
#[doc = "The Capabilities Register 3 specifies how many resources this BCDMA instance supports."]
pub mod bcdma_gcfg_cap3;
#[doc = "BCDMA_GCFG_CAP4 (rw) register accessor: The Capabilities Register 4 specifies how many resources this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_cap4`]
module"]
#[doc(alias = "BCDMA_GCFG_CAP4")]
pub type BcdmaGcfgCap4 = crate::Reg<bcdma_gcfg_cap4::BcdmaGcfgCap4Spec>;
#[doc = "The Capabilities Register 4 specifies how many resources this BCDMA instance supports."]
pub mod bcdma_gcfg_cap4;
#[doc = "BCDMA_GCFG_PM0 (rw) register accessor: This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_pm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_pm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_pm0`]
module"]
#[doc(alias = "BCDMA_GCFG_PM0")]
pub type BcdmaGcfgPm0 = crate::Reg<bcdma_gcfg_pm0::BcdmaGcfgPm0Spec>;
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks"]
pub mod bcdma_gcfg_pm0;
#[doc = "BCDMA_GCFG_PM1 (rw) register accessor: This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_pm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_pm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_pm1`]
module"]
#[doc(alias = "BCDMA_GCFG_PM1")]
pub type BcdmaGcfgPm1 = crate::Reg<bcdma_gcfg_pm1::BcdmaGcfgPm1Spec>;
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks"]
pub mod bcdma_gcfg_pm1;
#[doc = "BCDMA_GCFG_DBGA (rw) register accessor: This register provides a writable address which allows debug information to be read from the Debug Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_dbga::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_dbga::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_dbga`]
module"]
#[doc(alias = "BCDMA_GCFG_DBGA")]
pub type BcdmaGcfgDbga = crate::Reg<bcdma_gcfg_dbga::BcdmaGcfgDbgaSpec>;
#[doc = "This register provides a writable address which allows debug information to be read from the Debug Data Register"]
pub mod bcdma_gcfg_dbga;
#[doc = "BCDMA_GCFG_DBGD (rw) register accessor: This register provides read only debug data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_dbgd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_dbgd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcdma_gcfg_dbgd`]
module"]
#[doc(alias = "BCDMA_GCFG_DBGD")]
pub type BcdmaGcfgDbgd = crate::Reg<bcdma_gcfg_dbgd::BcdmaGcfgDbgdSpec>;
#[doc = "This register provides read only debug data"]
pub mod bcdma_gcfg_dbgd;
