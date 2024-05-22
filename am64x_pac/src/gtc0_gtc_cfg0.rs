#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtc_cfg0_pid: GtcCfg0Pid,
    gtc_cfg0_gtc_pid: GtcCfg0GtcPid,
    gtc_cfg0_pushevt: GtcCfg0Pushevt,
}
impl RegisterBlock {
    #[doc = "0x00 - GTC_CFG0_PID"]
    #[inline(always)]
    pub const fn gtc_cfg0_pid(&self) -> &GtcCfg0Pid {
        &self.gtc_cfg0_pid
    }
    #[doc = "0x04 - GTC_CFG0_GTC_PID"]
    #[inline(always)]
    pub const fn gtc_cfg0_gtc_pid(&self) -> &GtcCfg0GtcPid {
        &self.gtc_cfg0_gtc_pid
    }
    #[doc = "0x08 - GTC_CFG0_PUSHEVT"]
    #[inline(always)]
    pub const fn gtc_cfg0_pushevt(&self) -> &GtcCfg0Pushevt {
        &self.gtc_cfg0_pushevt
    }
}
#[doc = "GTC_CFG0_PID (rw) register accessor: GTC_CFG0_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg0_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg0_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg0_pid`]
module"]
#[doc(alias = "GTC_CFG0_PID")]
pub type GtcCfg0Pid = crate::Reg<gtc_cfg0_pid::GtcCfg0PidSpec>;
#[doc = "GTC_CFG0_PID"]
pub mod gtc_cfg0_pid;
#[doc = "GTC_CFG0_GTC_PID (rw) register accessor: GTC_CFG0_GTC_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg0_gtc_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg0_gtc_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg0_gtc_pid`]
module"]
#[doc(alias = "GTC_CFG0_GTC_PID")]
pub type GtcCfg0GtcPid = crate::Reg<gtc_cfg0_gtc_pid::GtcCfg0GtcPidSpec>;
#[doc = "GTC_CFG0_GTC_PID"]
pub mod gtc_cfg0_gtc_pid;
#[doc = "GTC_CFG0_PUSHEVT (rw) register accessor: GTC_CFG0_PUSHEVT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg0_pushevt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg0_pushevt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg0_pushevt`]
module"]
#[doc(alias = "GTC_CFG0_PUSHEVT")]
pub type GtcCfg0Pushevt = crate::Reg<gtc_cfg0_pushevt::GtcCfg0PushevtSpec>;
#[doc = "GTC_CFG0_PUSHEVT"]
pub mod gtc_cfg0_pushevt;
