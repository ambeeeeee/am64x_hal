#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ringacc_gcfg_revision: RingaccGcfgRevision,
    _reserved1: [u8; 0x0c],
    ringacc_gcfg_trace_ctl: RingaccGcfgTraceCtl,
    _reserved2: [u8; 0x0c],
    ringacc_gcfg_overflow: RingaccGcfgOverflow,
    _reserved3: [u8; 0x1c],
    ringacc_gcfg_error_evt: RingaccGcfgErrorEvt,
    ringacc_gcfg_error_log: RingaccGcfgErrorLog,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn ringacc_gcfg_revision(&self) -> &RingaccGcfgRevision {
        &self.ringacc_gcfg_revision
    }
    #[doc = "0x10 - Trace Control Register"]
    #[inline(always)]
    pub const fn ringacc_gcfg_trace_ctl(&self) -> &RingaccGcfgTraceCtl {
        &self.ringacc_gcfg_trace_ctl
    }
    #[doc = "0x20 - Overflow Queue Register"]
    #[inline(always)]
    pub const fn ringacc_gcfg_overflow(&self) -> &RingaccGcfgOverflow {
        &self.ringacc_gcfg_overflow
    }
    #[doc = "0x40 - The Error Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote detection of a ring memory transaction bus error."]
    #[inline(always)]
    pub const fn ringacc_gcfg_error_evt(&self) -> &RingaccGcfgErrorEvt {
        &self.ringacc_gcfg_error_evt
    }
    #[doc = "0x44 - Error Log Register. A read of this register will clear the pending error log event and allow a new error to be captured. It does not clear the contents of this register which are only valid while the error event is pending."]
    #[inline(always)]
    pub const fn ringacc_gcfg_error_log(&self) -> &RingaccGcfgErrorLog {
        &self.ringacc_gcfg_error_log
    }
}
#[doc = "RINGACC_GCFG_revision (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_gcfg_revision`]
module"]
#[doc(alias = "RINGACC_GCFG_revision")]
pub type RingaccGcfgRevision = crate::Reg<ringacc_gcfg_revision::RingaccGcfgRevisionSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod ringacc_gcfg_revision;
#[doc = "RINGACC_GCFG_trace_ctl (rw) register accessor: Trace Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_trace_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_trace_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_gcfg_trace_ctl`]
module"]
#[doc(alias = "RINGACC_GCFG_trace_ctl")]
pub type RingaccGcfgTraceCtl = crate::Reg<ringacc_gcfg_trace_ctl::RingaccGcfgTraceCtlSpec>;
#[doc = "Trace Control Register"]
pub mod ringacc_gcfg_trace_ctl;
#[doc = "RINGACC_GCFG_overflow (rw) register accessor: Overflow Queue Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_overflow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_overflow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_gcfg_overflow`]
module"]
#[doc(alias = "RINGACC_GCFG_overflow")]
pub type RingaccGcfgOverflow = crate::Reg<ringacc_gcfg_overflow::RingaccGcfgOverflowSpec>;
#[doc = "Overflow Queue Register"]
pub mod ringacc_gcfg_overflow;
#[doc = "RINGACC_GCFG_error_evt (rw) register accessor: The Error Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote detection of a ring memory transaction bus error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_error_evt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_error_evt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_gcfg_error_evt`]
module"]
#[doc(alias = "RINGACC_GCFG_error_evt")]
pub type RingaccGcfgErrorEvt = crate::Reg<ringacc_gcfg_error_evt::RingaccGcfgErrorEvtSpec>;
#[doc = "The Error Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote detection of a ring memory transaction bus error."]
pub mod ringacc_gcfg_error_evt;
#[doc = "RINGACC_GCFG_error_log (rw) register accessor: Error Log Register. A read of this register will clear the pending error log event and allow a new error to be captured. It does not clear the contents of this register which are only valid while the error event is pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_error_log::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_error_log::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_gcfg_error_log`]
module"]
#[doc(alias = "RINGACC_GCFG_error_log")]
pub type RingaccGcfgErrorLog = crate::Reg<ringacc_gcfg_error_log::RingaccGcfgErrorLogSpec>;
#[doc = "Error Log Register. A read of this register will clear the pending error log event and allow a new error to be captured. It does not clear the contents of this register which are only valid while the error event is pending."]
pub mod ringacc_gcfg_error_log;
