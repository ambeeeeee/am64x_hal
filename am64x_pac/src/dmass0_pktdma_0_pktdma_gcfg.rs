#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pktdma_gcfg_revision: PktdmaGcfgRevision,
    pktdma_gcfg_perf_ctrl: PktdmaGcfgPerfCtrl,
    pktdma_gcfg_emu_ctrl: PktdmaGcfgEmuCtrl,
    _reserved3: [u8; 0x04],
    pktdma_gcfg_psil_to: PktdmaGcfgPsilTo,
    _reserved4: [u8; 0x0c],
    pktdma_gcfg_cap0: PktdmaGcfgCap0,
    pktdma_gcfg_cap1: PktdmaGcfgCap1,
    pktdma_gcfg_cap2: PktdmaGcfgCap2,
    pktdma_gcfg_cap3: PktdmaGcfgCap3,
    pktdma_gcfg_cap4: PktdmaGcfgCap4,
    _reserved9: [u8; 0x2c],
    pktdma_gcfg_pm0: PktdmaGcfgPm0,
    pktdma_gcfg_pm1: PktdmaGcfgPm1,
    _reserved11: [u8; 0x10],
    pktdma_gcfg_dbga: PktdmaGcfgDbga,
    pktdma_gcfg_dbgd: PktdmaGcfgDbgd,
    _reserved13: [u8; 0x08],
    pktdma_gcfg_rflowfwstat: PktdmaGcfgRflowfwstat,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn pktdma_gcfg_revision(&self) -> &PktdmaGcfgRevision {
        &self.pktdma_gcfg_revision
    }
    #[doc = "0x04 - The performance control register contains fields which can be used to adjust the performance of the PKTDMA in the system."]
    #[inline(always)]
    pub const fn pktdma_gcfg_perf_ctrl(&self) -> &PktdmaGcfgPerfCtrl {
        &self.pktdma_gcfg_perf_ctrl
    }
    #[doc = "0x08 - The emulation control register is used to control the behavior of the DMA when the emususp input is asserted."]
    #[inline(always)]
    pub const fn pktdma_gcfg_emu_ctrl(&self) -> &PktdmaGcfgEmuCtrl {
        &self.pktdma_gcfg_emu_ctrl
    }
    #[doc = "0x10 - The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy."]
    #[inline(always)]
    pub const fn pktdma_gcfg_psil_to(&self) -> &PktdmaGcfgPsilTo {
        &self.pktdma_gcfg_psil_to
    }
    #[doc = "0x20 - The Capabilities Register 0 specifies which standard features this PKTDMA instance supports."]
    #[inline(always)]
    pub const fn pktdma_gcfg_cap0(&self) -> &PktdmaGcfgCap0 {
        &self.pktdma_gcfg_cap0
    }
    #[doc = "0x24 - The Capabilities Register 1 specifies which standard features this PKTDMA instance supports."]
    #[inline(always)]
    pub const fn pktdma_gcfg_cap1(&self) -> &PktdmaGcfgCap1 {
        &self.pktdma_gcfg_cap1
    }
    #[doc = "0x28 - The Capabilities Register 2 specifies how many resources this PKTDMA instance supports."]
    #[inline(always)]
    pub const fn pktdma_gcfg_cap2(&self) -> &PktdmaGcfgCap2 {
        &self.pktdma_gcfg_cap2
    }
    #[doc = "0x2c - The Capabilities Register 3 specifies how many resources this PKTDMA instance supports."]
    #[inline(always)]
    pub const fn pktdma_gcfg_cap3(&self) -> &PktdmaGcfgCap3 {
        &self.pktdma_gcfg_cap3
    }
    #[doc = "0x30 - The Capabilities Register 4 specifies how many resources this PKTDMA instance supports."]
    #[inline(always)]
    pub const fn pktdma_gcfg_cap4(&self) -> &PktdmaGcfgCap4 {
        &self.pktdma_gcfg_cap4
    }
    #[doc = "0x60 - This register enables or inhibits automatic clock gating to individual sub-blocks"]
    #[inline(always)]
    pub const fn pktdma_gcfg_pm0(&self) -> &PktdmaGcfgPm0 {
        &self.pktdma_gcfg_pm0
    }
    #[doc = "0x64 - This register enables or inhibits automatic clock gating to individual sub-blocks"]
    #[inline(always)]
    pub const fn pktdma_gcfg_pm1(&self) -> &PktdmaGcfgPm1 {
        &self.pktdma_gcfg_pm1
    }
    #[doc = "0x78 - This register provides a writable address which allows debug information to be read from the Debug Data Register"]
    #[inline(always)]
    pub const fn pktdma_gcfg_dbga(&self) -> &PktdmaGcfgDbga {
        &self.pktdma_gcfg_dbga
    }
    #[doc = "0x7c - This register provides read only debug data"]
    #[inline(always)]
    pub const fn pktdma_gcfg_dbgd(&self) -> &PktdmaGcfgDbgd {
        &self.pktdma_gcfg_dbgd
    }
    #[doc = "0x88 - The Rx Flow FW Status Register 0 captures information about the thread/channel and received flow ID which failed a range check. Values in this register will remain persistent once an exception has been detected until the pend bit is written back to 0"]
    #[inline(always)]
    pub const fn pktdma_gcfg_rflowfwstat(&self) -> &PktdmaGcfgRflowfwstat {
        &self.pktdma_gcfg_rflowfwstat
    }
}
#[doc = "PKTDMA_GCFG_REVISION (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_revision`]
module"]
#[doc(alias = "PKTDMA_GCFG_REVISION")]
pub type PktdmaGcfgRevision = crate::Reg<pktdma_gcfg_revision::PktdmaGcfgRevisionSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod pktdma_gcfg_revision;
#[doc = "PKTDMA_GCFG_PERF_CTRL (rw) register accessor: The performance control register contains fields which can be used to adjust the performance of the PKTDMA in the system.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_perf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_perf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_perf_ctrl`]
module"]
#[doc(alias = "PKTDMA_GCFG_PERF_CTRL")]
pub type PktdmaGcfgPerfCtrl = crate::Reg<pktdma_gcfg_perf_ctrl::PktdmaGcfgPerfCtrlSpec>;
#[doc = "The performance control register contains fields which can be used to adjust the performance of the PKTDMA in the system."]
pub mod pktdma_gcfg_perf_ctrl;
#[doc = "PKTDMA_GCFG_EMU_CTRL (rw) register accessor: The emulation control register is used to control the behavior of the DMA when the emususp input is asserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_emu_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_emu_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_emu_ctrl`]
module"]
#[doc(alias = "PKTDMA_GCFG_EMU_CTRL")]
pub type PktdmaGcfgEmuCtrl = crate::Reg<pktdma_gcfg_emu_ctrl::PktdmaGcfgEmuCtrlSpec>;
#[doc = "The emulation control register is used to control the behavior of the DMA when the emususp input is asserted."]
pub mod pktdma_gcfg_emu_ctrl;
#[doc = "PKTDMA_GCFG_PSIL_TO (rw) register accessor: The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_psil_to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_psil_to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_psil_to`]
module"]
#[doc(alias = "PKTDMA_GCFG_PSIL_TO")]
pub type PktdmaGcfgPsilTo = crate::Reg<pktdma_gcfg_psil_to::PktdmaGcfgPsilToSpec>;
#[doc = "The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy."]
pub mod pktdma_gcfg_psil_to;
#[doc = "PKTDMA_GCFG_CAP0 (rw) register accessor: The Capabilities Register 0 specifies which standard features this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_cap0`]
module"]
#[doc(alias = "PKTDMA_GCFG_CAP0")]
pub type PktdmaGcfgCap0 = crate::Reg<pktdma_gcfg_cap0::PktdmaGcfgCap0Spec>;
#[doc = "The Capabilities Register 0 specifies which standard features this PKTDMA instance supports."]
pub mod pktdma_gcfg_cap0;
#[doc = "PKTDMA_GCFG_CAP1 (rw) register accessor: The Capabilities Register 1 specifies which standard features this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_cap1`]
module"]
#[doc(alias = "PKTDMA_GCFG_CAP1")]
pub type PktdmaGcfgCap1 = crate::Reg<pktdma_gcfg_cap1::PktdmaGcfgCap1Spec>;
#[doc = "The Capabilities Register 1 specifies which standard features this PKTDMA instance supports."]
pub mod pktdma_gcfg_cap1;
#[doc = "PKTDMA_GCFG_CAP2 (rw) register accessor: The Capabilities Register 2 specifies how many resources this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_cap2`]
module"]
#[doc(alias = "PKTDMA_GCFG_CAP2")]
pub type PktdmaGcfgCap2 = crate::Reg<pktdma_gcfg_cap2::PktdmaGcfgCap2Spec>;
#[doc = "The Capabilities Register 2 specifies how many resources this PKTDMA instance supports."]
pub mod pktdma_gcfg_cap2;
#[doc = "PKTDMA_GCFG_CAP3 (rw) register accessor: The Capabilities Register 3 specifies how many resources this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_cap3`]
module"]
#[doc(alias = "PKTDMA_GCFG_CAP3")]
pub type PktdmaGcfgCap3 = crate::Reg<pktdma_gcfg_cap3::PktdmaGcfgCap3Spec>;
#[doc = "The Capabilities Register 3 specifies how many resources this PKTDMA instance supports."]
pub mod pktdma_gcfg_cap3;
#[doc = "PKTDMA_GCFG_CAP4 (rw) register accessor: The Capabilities Register 4 specifies how many resources this PKTDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_cap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_cap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_cap4`]
module"]
#[doc(alias = "PKTDMA_GCFG_CAP4")]
pub type PktdmaGcfgCap4 = crate::Reg<pktdma_gcfg_cap4::PktdmaGcfgCap4Spec>;
#[doc = "The Capabilities Register 4 specifies how many resources this PKTDMA instance supports."]
pub mod pktdma_gcfg_cap4;
#[doc = "PKTDMA_GCFG_PM0 (rw) register accessor: This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_pm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_pm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_pm0`]
module"]
#[doc(alias = "PKTDMA_GCFG_PM0")]
pub type PktdmaGcfgPm0 = crate::Reg<pktdma_gcfg_pm0::PktdmaGcfgPm0Spec>;
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks"]
pub mod pktdma_gcfg_pm0;
#[doc = "PKTDMA_GCFG_PM1 (rw) register accessor: This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_pm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_pm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_pm1`]
module"]
#[doc(alias = "PKTDMA_GCFG_PM1")]
pub type PktdmaGcfgPm1 = crate::Reg<pktdma_gcfg_pm1::PktdmaGcfgPm1Spec>;
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks"]
pub mod pktdma_gcfg_pm1;
#[doc = "PKTDMA_GCFG_DBGA (rw) register accessor: This register provides a writable address which allows debug information to be read from the Debug Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_dbga::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_dbga::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_dbga`]
module"]
#[doc(alias = "PKTDMA_GCFG_DBGA")]
pub type PktdmaGcfgDbga = crate::Reg<pktdma_gcfg_dbga::PktdmaGcfgDbgaSpec>;
#[doc = "This register provides a writable address which allows debug information to be read from the Debug Data Register"]
pub mod pktdma_gcfg_dbga;
#[doc = "PKTDMA_GCFG_DBGD (rw) register accessor: This register provides read only debug data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_dbgd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_dbgd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_dbgd`]
module"]
#[doc(alias = "PKTDMA_GCFG_DBGD")]
pub type PktdmaGcfgDbgd = crate::Reg<pktdma_gcfg_dbgd::PktdmaGcfgDbgdSpec>;
#[doc = "This register provides read only debug data"]
pub mod pktdma_gcfg_dbgd;
#[doc = "PKTDMA_GCFG_RFLOWFWSTAT (rw) register accessor: The Rx Flow FW Status Register 0 captures information about the thread/channel and received flow ID which failed a range check. Values in this register will remain persistent once an exception has been detected until the pend bit is written back to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_rflowfwstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_rflowfwstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_gcfg_rflowfwstat`]
module"]
#[doc(alias = "PKTDMA_GCFG_RFLOWFWSTAT")]
pub type PktdmaGcfgRflowfwstat = crate::Reg<pktdma_gcfg_rflowfwstat::PktdmaGcfgRflowfwstatSpec>;
#[doc = "The Rx Flow FW Status Register 0 captures information about the thread/channel and received flow ID which failed a range check. Values in this register will remain persistent once an exception has been detected until the pend bit is written back to 0"]
pub mod pktdma_gcfg_rflowfwstat;
