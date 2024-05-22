#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_pid: CfgPid,
    cfg_cfg: CfgCfg,
    cfg_info: CfgInfo,
    cfg_enable: CfgEnable,
    cfg_flush: CfgFlush,
    cfg_timeout: CfgTimeout,
    cfg_timer: CfgTimer,
    _reserved7: [u8; 0x04],
    cfg_err_raw: CfgErrRaw,
    cfg_err: CfgErr,
    cfg_err_msk_set: CfgErrMskSet,
    cfg_err_msk_clr: CfgErrMskClr,
    cfg_err_tm_info: CfgErrTmInfo,
    cfg_err_un_info: CfgErrUnInfo,
    cfg_err_val: CfgErrVal,
    cfg_err_tag: CfgErrTag,
    cfg_err_byt: CfgErrByt,
    cfg_err_addr_u: CfgErrAddrU,
    cfg_err_addr_l: CfgErrAddrL,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn cfg_pid(&self) -> &CfgPid {
        &self.cfg_pid
    }
    #[doc = "0x04 - The Configuration Register contains information about the configuration of the gasket."]
    #[inline(always)]
    pub const fn cfg_cfg(&self) -> &CfgCfg {
        &self.cfg_cfg
    }
    #[doc = "0x08 - The Info Register contains information about the current state of the gasket."]
    #[inline(always)]
    pub const fn cfg_info(&self) -> &CfgInfo {
        &self.cfg_info
    }
    #[doc = "0x0c - The Enable Register contains the gasket enable."]
    #[inline(always)]
    pub const fn cfg_enable(&self) -> &CfgEnable {
        &self.cfg_enable
    }
    #[doc = "0x10 - The Flush Register contains software flush control."]
    #[inline(always)]
    pub const fn cfg_flush(&self) -> &CfgFlush {
        &self.cfg_flush
    }
    #[doc = "0x14 - The Timeout Value Register contains the timeout value for scoreboarded transactions."]
    #[inline(always)]
    pub const fn cfg_timeout(&self) -> &CfgTimeout {
        &self.cfg_timeout
    }
    #[doc = "0x18 - The Timer Register contains the current value for free-running timer."]
    #[inline(always)]
    pub const fn cfg_timer(&self) -> &CfgTimer {
        &self.cfg_timer
    }
    #[doc = "0x20 - This register contains the masked interrupt bits"]
    #[inline(always)]
    pub const fn cfg_err_raw(&self) -> &CfgErrRaw {
        &self.cfg_err_raw
    }
    #[doc = "0x24 - This register contains the masked interrupt bits"]
    #[inline(always)]
    pub const fn cfg_err(&self) -> &CfgErr {
        &self.cfg_err
    }
    #[doc = "0x28 - This register contains interrupt mask set bits"]
    #[inline(always)]
    pub const fn cfg_err_msk_set(&self) -> &CfgErrMskSet {
        &self.cfg_err_msk_set
    }
    #[doc = "0x2c - This register contains interrupt mask clear bits"]
    #[inline(always)]
    pub const fn cfg_err_msk_clr(&self) -> &CfgErrMskClr {
        &self.cfg_err_msk_clr
    }
    #[doc = "0x30 - This register contains information about timeout interrupts"]
    #[inline(always)]
    pub const fn cfg_err_tm_info(&self) -> &CfgErrTmInfo {
        &self.cfg_err_tm_info
    }
    #[doc = "0x34 - This register contains information about unexpected interrupts"]
    #[inline(always)]
    pub const fn cfg_err_un_info(&self) -> &CfgErrUnInfo {
        &self.cfg_err_un_info
    }
    #[doc = "0x38 - This register contains information about transaction that caused the interrupt"]
    #[inline(always)]
    pub const fn cfg_err_val(&self) -> &CfgErrVal {
        &self.cfg_err_val
    }
    #[doc = "0x3c - This register contains information about transaction that caused the interrupt"]
    #[inline(always)]
    pub const fn cfg_err_tag(&self) -> &CfgErrTag {
        &self.cfg_err_tag
    }
    #[doc = "0x40 - This register contains information about transaction that caused the interrupt"]
    #[inline(always)]
    pub const fn cfg_err_byt(&self) -> &CfgErrByt {
        &self.cfg_err_byt
    }
    #[doc = "0x44 - This register contains information about transaction that caused the interrupt"]
    #[inline(always)]
    pub const fn cfg_err_addr_u(&self) -> &CfgErrAddrU {
        &self.cfg_err_addr_u
    }
    #[doc = "0x48 - This register contains information about transaction that caused the interrupt"]
    #[inline(always)]
    pub const fn cfg_err_addr_l(&self) -> &CfgErrAddrL {
        &self.cfg_err_addr_l
    }
}
#[doc = "CFG_PID (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pid`]
module"]
#[doc(alias = "CFG_PID")]
pub type CfgPid = crate::Reg<cfg_pid::CfgPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod cfg_pid;
#[doc = "CFG_CFG (rw) register accessor: The Configuration Register contains information about the configuration of the gasket.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cfg`]
module"]
#[doc(alias = "CFG_CFG")]
pub type CfgCfg = crate::Reg<cfg_cfg::CfgCfgSpec>;
#[doc = "The Configuration Register contains information about the configuration of the gasket."]
pub mod cfg_cfg;
#[doc = "CFG_INFO (rw) register accessor: The Info Register contains information about the current state of the gasket.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_info`]
module"]
#[doc(alias = "CFG_INFO")]
pub type CfgInfo = crate::Reg<cfg_info::CfgInfoSpec>;
#[doc = "The Info Register contains information about the current state of the gasket."]
pub mod cfg_info;
#[doc = "CFG_ENABLE (rw) register accessor: The Enable Register contains the gasket enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_enable`]
module"]
#[doc(alias = "CFG_ENABLE")]
pub type CfgEnable = crate::Reg<cfg_enable::CfgEnableSpec>;
#[doc = "The Enable Register contains the gasket enable."]
pub mod cfg_enable;
#[doc = "CFG_FLUSH (rw) register accessor: The Flush Register contains software flush control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_flush::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_flush::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_flush`]
module"]
#[doc(alias = "CFG_FLUSH")]
pub type CfgFlush = crate::Reg<cfg_flush::CfgFlushSpec>;
#[doc = "The Flush Register contains software flush control."]
pub mod cfg_flush;
#[doc = "CFG_TIMEOUT (rw) register accessor: The Timeout Value Register contains the timeout value for scoreboarded transactions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_timeout`]
module"]
#[doc(alias = "CFG_TIMEOUT")]
pub type CfgTimeout = crate::Reg<cfg_timeout::CfgTimeoutSpec>;
#[doc = "The Timeout Value Register contains the timeout value for scoreboarded transactions."]
pub mod cfg_timeout;
#[doc = "CFG_TIMER (rw) register accessor: The Timer Register contains the current value for free-running timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_timer`]
module"]
#[doc(alias = "CFG_TIMER")]
pub type CfgTimer = crate::Reg<cfg_timer::CfgTimerSpec>;
#[doc = "The Timer Register contains the current value for free-running timer."]
pub mod cfg_timer;
#[doc = "CFG_ERR_RAW (rw) register accessor: This register contains the masked interrupt bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_raw`]
module"]
#[doc(alias = "CFG_ERR_RAW")]
pub type CfgErrRaw = crate::Reg<cfg_err_raw::CfgErrRawSpec>;
#[doc = "This register contains the masked interrupt bits"]
pub mod cfg_err_raw;
#[doc = "CFG_ERR (rw) register accessor: This register contains the masked interrupt bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err`]
module"]
#[doc(alias = "CFG_ERR")]
pub type CfgErr = crate::Reg<cfg_err::CfgErrSpec>;
#[doc = "This register contains the masked interrupt bits"]
pub mod cfg_err;
#[doc = "CFG_ERR_MSK_SET (rw) register accessor: This register contains interrupt mask set bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_msk_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_msk_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_msk_set`]
module"]
#[doc(alias = "CFG_ERR_MSK_SET")]
pub type CfgErrMskSet = crate::Reg<cfg_err_msk_set::CfgErrMskSetSpec>;
#[doc = "This register contains interrupt mask set bits"]
pub mod cfg_err_msk_set;
#[doc = "CFG_ERR_MSK_CLR (rw) register accessor: This register contains interrupt mask clear bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_msk_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_msk_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_msk_clr`]
module"]
#[doc(alias = "CFG_ERR_MSK_CLR")]
pub type CfgErrMskClr = crate::Reg<cfg_err_msk_clr::CfgErrMskClrSpec>;
#[doc = "This register contains interrupt mask clear bits"]
pub mod cfg_err_msk_clr;
#[doc = "CFG_ERR_TM_INFO (rw) register accessor: This register contains information about timeout interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_tm_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_tm_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_tm_info`]
module"]
#[doc(alias = "CFG_ERR_TM_INFO")]
pub type CfgErrTmInfo = crate::Reg<cfg_err_tm_info::CfgErrTmInfoSpec>;
#[doc = "This register contains information about timeout interrupts"]
pub mod cfg_err_tm_info;
#[doc = "CFG_ERR_UN_INFO (rw) register accessor: This register contains information about unexpected interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_un_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_un_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_un_info`]
module"]
#[doc(alias = "CFG_ERR_UN_INFO")]
pub type CfgErrUnInfo = crate::Reg<cfg_err_un_info::CfgErrUnInfoSpec>;
#[doc = "This register contains information about unexpected interrupts"]
pub mod cfg_err_un_info;
#[doc = "CFG_ERR_VAL (rw) register accessor: This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_val`]
module"]
#[doc(alias = "CFG_ERR_VAL")]
pub type CfgErrVal = crate::Reg<cfg_err_val::CfgErrValSpec>;
#[doc = "This register contains information about transaction that caused the interrupt"]
pub mod cfg_err_val;
#[doc = "CFG_ERR_TAG (rw) register accessor: This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_tag`]
module"]
#[doc(alias = "CFG_ERR_TAG")]
pub type CfgErrTag = crate::Reg<cfg_err_tag::CfgErrTagSpec>;
#[doc = "This register contains information about transaction that caused the interrupt"]
pub mod cfg_err_tag;
#[doc = "CFG_ERR_BYT (rw) register accessor: This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_byt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_byt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_byt`]
module"]
#[doc(alias = "CFG_ERR_BYT")]
pub type CfgErrByt = crate::Reg<cfg_err_byt::CfgErrBytSpec>;
#[doc = "This register contains information about transaction that caused the interrupt"]
pub mod cfg_err_byt;
#[doc = "CFG_ERR_ADDR_U (rw) register accessor: This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_addr_u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_addr_u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_addr_u`]
module"]
#[doc(alias = "CFG_ERR_ADDR_U")]
pub type CfgErrAddrU = crate::Reg<cfg_err_addr_u::CfgErrAddrUSpec>;
#[doc = "This register contains information about transaction that caused the interrupt"]
pub mod cfg_err_addr_u;
#[doc = "CFG_ERR_ADDR_L (rw) register accessor: This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_addr_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_addr_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_err_addr_l`]
module"]
#[doc(alias = "CFG_ERR_ADDR_L")]
pub type CfgErrAddrL = crate::Reg<cfg_err_addr_l::CfgErrAddrLSpec>;
#[doc = "This register contains information about transaction that caused the interrupt"]
pub mod cfg_err_addr_l;
