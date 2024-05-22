#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_pll0_pid: CfgPll0Pid,
    _reserved1: [u8; 0x04],
    cfg_pll0_cfg: CfgPll0Cfg,
    _reserved2: [u8; 0x04],
    cfg_pll0_lockkey0: CfgPll0Lockkey0,
    cfg_pll0_lockkey1: CfgPll0Lockkey1,
    _reserved4: [u8; 0x08],
    cfg_pll0_ctrl: CfgPll0Ctrl,
    cfg_pll0_stat: CfgPll0Stat,
    _reserved6: [u8; 0x08],
    cfg_pll0_freq_ctrl0: CfgPll0FreqCtrl0,
    cfg_pll0_freq_ctrl1: CfgPll0FreqCtrl1,
    cfg_pll0_div_ctrl: CfgPll0DivCtrl,
    _reserved9: [u8; 0x04],
    cfg_pll0_ss_ctrl: CfgPll0SsCtrl,
    cfg_pll0_ss_spread: CfgPll0SsSpread,
    _reserved11: [u8; 0x18],
    cfg_pll0_cal_ctrl: CfgPll0CalCtrl,
    cfg_pll0_cal_stat: CfgPll0CalStat,
    _reserved13: [u8; 0x18],
    cfg_pll0_hsdiv_ctrl0: CfgPll0HsdivCtrl0,
    cfg_pll0_hsdiv_ctrl1: CfgPll0HsdivCtrl1,
    cfg_pll0_hsdiv_ctrl2: CfgPll0HsdivCtrl2,
    cfg_pll0_hsdiv_ctrl3: CfgPll0HsdivCtrl3,
    cfg_pll0_hsdiv_ctrl4: CfgPll0HsdivCtrl4,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG_pll0_PID"]
    #[inline(always)]
    pub const fn cfg_pll0_pid(&self) -> &CfgPll0Pid {
        &self.cfg_pll0_pid
    }
    #[doc = "0x08 - CFG_pll0_CFG"]
    #[inline(always)]
    pub const fn cfg_pll0_cfg(&self) -> &CfgPll0Cfg {
        &self.cfg_pll0_cfg
    }
    #[doc = "0x10 - CFG_pll0_LOCKKEY0"]
    #[inline(always)]
    pub const fn cfg_pll0_lockkey0(&self) -> &CfgPll0Lockkey0 {
        &self.cfg_pll0_lockkey0
    }
    #[doc = "0x14 - CFG_pll0_LOCKKEY1"]
    #[inline(always)]
    pub const fn cfg_pll0_lockkey1(&self) -> &CfgPll0Lockkey1 {
        &self.cfg_pll0_lockkey1
    }
    #[doc = "0x20 - CFG_pll0_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll0_ctrl(&self) -> &CfgPll0Ctrl {
        &self.cfg_pll0_ctrl
    }
    #[doc = "0x24 - CFG_pll0_STAT"]
    #[inline(always)]
    pub const fn cfg_pll0_stat(&self) -> &CfgPll0Stat {
        &self.cfg_pll0_stat
    }
    #[doc = "0x30 - CFG_pll0_FREQ_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll0_freq_ctrl0(&self) -> &CfgPll0FreqCtrl0 {
        &self.cfg_pll0_freq_ctrl0
    }
    #[doc = "0x34 - CFG_pll0_FREQ_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll0_freq_ctrl1(&self) -> &CfgPll0FreqCtrl1 {
        &self.cfg_pll0_freq_ctrl1
    }
    #[doc = "0x38 - CFG_pll0_DIV_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll0_div_ctrl(&self) -> &CfgPll0DivCtrl {
        &self.cfg_pll0_div_ctrl
    }
    #[doc = "0x40 - CFG_pll0_SS_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll0_ss_ctrl(&self) -> &CfgPll0SsCtrl {
        &self.cfg_pll0_ss_ctrl
    }
    #[doc = "0x44 - CFG_pll0_SS_SPREAD"]
    #[inline(always)]
    pub const fn cfg_pll0_ss_spread(&self) -> &CfgPll0SsSpread {
        &self.cfg_pll0_ss_spread
    }
    #[doc = "0x60 - CFG_pll0_CAL_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll0_cal_ctrl(&self) -> &CfgPll0CalCtrl {
        &self.cfg_pll0_cal_ctrl
    }
    #[doc = "0x64 - CFG_pll0_CAL_STAT"]
    #[inline(always)]
    pub const fn cfg_pll0_cal_stat(&self) -> &CfgPll0CalStat {
        &self.cfg_pll0_cal_stat
    }
    #[doc = "0x80 - CFG_pll0_HSDIV_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl0(&self) -> &CfgPll0HsdivCtrl0 {
        &self.cfg_pll0_hsdiv_ctrl0
    }
    #[doc = "0x84 - CFG_pll0_HSDIV_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl1(&self) -> &CfgPll0HsdivCtrl1 {
        &self.cfg_pll0_hsdiv_ctrl1
    }
    #[doc = "0x88 - CFG_pll0_HSDIV_CTRL2"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl2(&self) -> &CfgPll0HsdivCtrl2 {
        &self.cfg_pll0_hsdiv_ctrl2
    }
    #[doc = "0x8c - CFG_pll0_HSDIV_CTRL3"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl3(&self) -> &CfgPll0HsdivCtrl3 {
        &self.cfg_pll0_hsdiv_ctrl3
    }
    #[doc = "0x90 - CFG_pll0_HSDIV_CTRL4"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl4(&self) -> &CfgPll0HsdivCtrl4 {
        &self.cfg_pll0_hsdiv_ctrl4
    }
}
#[doc = "CFG_pll0_PID (rw) register accessor: CFG_pll0_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_pid`]
module"]
#[doc(alias = "CFG_pll0_PID")]
pub type CfgPll0Pid = crate::Reg<cfg_pll0_pid::CfgPll0PidSpec>;
#[doc = "CFG_pll0_PID"]
pub mod cfg_pll0_pid;
#[doc = "CFG_pll0_CFG (rw) register accessor: CFG_pll0_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_cfg`]
module"]
#[doc(alias = "CFG_pll0_CFG")]
pub type CfgPll0Cfg = crate::Reg<cfg_pll0_cfg::CfgPll0CfgSpec>;
#[doc = "CFG_pll0_CFG"]
pub mod cfg_pll0_cfg;
#[doc = "CFG_pll0_LOCKKEY0 (rw) register accessor: CFG_pll0_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_lockkey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_lockkey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_lockkey0`]
module"]
#[doc(alias = "CFG_pll0_LOCKKEY0")]
pub type CfgPll0Lockkey0 = crate::Reg<cfg_pll0_lockkey0::CfgPll0Lockkey0Spec>;
#[doc = "CFG_pll0_LOCKKEY0"]
pub mod cfg_pll0_lockkey0;
#[doc = "CFG_pll0_LOCKKEY1 (rw) register accessor: CFG_pll0_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_lockkey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_lockkey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_lockkey1`]
module"]
#[doc(alias = "CFG_pll0_LOCKKEY1")]
pub type CfgPll0Lockkey1 = crate::Reg<cfg_pll0_lockkey1::CfgPll0Lockkey1Spec>;
#[doc = "CFG_pll0_LOCKKEY1"]
pub mod cfg_pll0_lockkey1;
#[doc = "CFG_pll0_CTRL (rw) register accessor: CFG_pll0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_ctrl`]
module"]
#[doc(alias = "CFG_pll0_CTRL")]
pub type CfgPll0Ctrl = crate::Reg<cfg_pll0_ctrl::CfgPll0CtrlSpec>;
#[doc = "CFG_pll0_CTRL"]
pub mod cfg_pll0_ctrl;
#[doc = "CFG_pll0_STAT (rw) register accessor: CFG_pll0_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_stat`]
module"]
#[doc(alias = "CFG_pll0_STAT")]
pub type CfgPll0Stat = crate::Reg<cfg_pll0_stat::CfgPll0StatSpec>;
#[doc = "CFG_pll0_STAT"]
pub mod cfg_pll0_stat;
#[doc = "CFG_pll0_FREQ_CTRL0 (rw) register accessor: CFG_pll0_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_freq_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_freq_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_freq_ctrl0`]
module"]
#[doc(alias = "CFG_pll0_FREQ_CTRL0")]
pub type CfgPll0FreqCtrl0 = crate::Reg<cfg_pll0_freq_ctrl0::CfgPll0FreqCtrl0Spec>;
#[doc = "CFG_pll0_FREQ_CTRL0"]
pub mod cfg_pll0_freq_ctrl0;
#[doc = "CFG_pll0_FREQ_CTRL1 (rw) register accessor: CFG_pll0_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_freq_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_freq_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_freq_ctrl1`]
module"]
#[doc(alias = "CFG_pll0_FREQ_CTRL1")]
pub type CfgPll0FreqCtrl1 = crate::Reg<cfg_pll0_freq_ctrl1::CfgPll0FreqCtrl1Spec>;
#[doc = "CFG_pll0_FREQ_CTRL1"]
pub mod cfg_pll0_freq_ctrl1;
#[doc = "CFG_pll0_DIV_CTRL (rw) register accessor: CFG_pll0_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_div_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_div_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_div_ctrl`]
module"]
#[doc(alias = "CFG_pll0_DIV_CTRL")]
pub type CfgPll0DivCtrl = crate::Reg<cfg_pll0_div_ctrl::CfgPll0DivCtrlSpec>;
#[doc = "CFG_pll0_DIV_CTRL"]
pub mod cfg_pll0_div_ctrl;
#[doc = "CFG_pll0_SS_CTRL (rw) register accessor: CFG_pll0_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_ss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_ss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_ss_ctrl`]
module"]
#[doc(alias = "CFG_pll0_SS_CTRL")]
pub type CfgPll0SsCtrl = crate::Reg<cfg_pll0_ss_ctrl::CfgPll0SsCtrlSpec>;
#[doc = "CFG_pll0_SS_CTRL"]
pub mod cfg_pll0_ss_ctrl;
#[doc = "CFG_pll0_SS_SPREAD (rw) register accessor: CFG_pll0_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_ss_spread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_ss_spread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_ss_spread`]
module"]
#[doc(alias = "CFG_pll0_SS_SPREAD")]
pub type CfgPll0SsSpread = crate::Reg<cfg_pll0_ss_spread::CfgPll0SsSpreadSpec>;
#[doc = "CFG_pll0_SS_SPREAD"]
pub mod cfg_pll0_ss_spread;
#[doc = "CFG_pll0_CAL_CTRL (rw) register accessor: CFG_pll0_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_cal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_cal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_cal_ctrl`]
module"]
#[doc(alias = "CFG_pll0_CAL_CTRL")]
pub type CfgPll0CalCtrl = crate::Reg<cfg_pll0_cal_ctrl::CfgPll0CalCtrlSpec>;
#[doc = "CFG_pll0_CAL_CTRL"]
pub mod cfg_pll0_cal_ctrl;
#[doc = "CFG_pll0_CAL_STAT (rw) register accessor: CFG_pll0_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_cal_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_cal_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_cal_stat`]
module"]
#[doc(alias = "CFG_pll0_CAL_STAT")]
pub type CfgPll0CalStat = crate::Reg<cfg_pll0_cal_stat::CfgPll0CalStatSpec>;
#[doc = "CFG_pll0_CAL_STAT"]
pub mod cfg_pll0_cal_stat;
#[doc = "CFG_pll0_HSDIV_CTRL0 (rw) register accessor: CFG_pll0_HSDIV_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl0`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL0")]
pub type CfgPll0HsdivCtrl0 = crate::Reg<cfg_pll0_hsdiv_ctrl0::CfgPll0HsdivCtrl0Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL0"]
pub mod cfg_pll0_hsdiv_ctrl0;
#[doc = "CFG_pll0_HSDIV_CTRL1 (rw) register accessor: CFG_pll0_HSDIV_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl1`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL1")]
pub type CfgPll0HsdivCtrl1 = crate::Reg<cfg_pll0_hsdiv_ctrl1::CfgPll0HsdivCtrl1Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL1"]
pub mod cfg_pll0_hsdiv_ctrl1;
#[doc = "CFG_pll0_HSDIV_CTRL2 (rw) register accessor: CFG_pll0_HSDIV_CTRL2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl2`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL2")]
pub type CfgPll0HsdivCtrl2 = crate::Reg<cfg_pll0_hsdiv_ctrl2::CfgPll0HsdivCtrl2Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL2"]
pub mod cfg_pll0_hsdiv_ctrl2;
#[doc = "CFG_pll0_HSDIV_CTRL3 (rw) register accessor: CFG_pll0_HSDIV_CTRL3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl3`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL3")]
pub type CfgPll0HsdivCtrl3 = crate::Reg<cfg_pll0_hsdiv_ctrl3::CfgPll0HsdivCtrl3Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL3"]
pub mod cfg_pll0_hsdiv_ctrl3;
#[doc = "CFG_pll0_HSDIV_CTRL4 (rw) register accessor: CFG_pll0_HSDIV_CTRL4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl4`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL4")]
pub type CfgPll0HsdivCtrl4 = crate::Reg<cfg_pll0_hsdiv_ctrl4::CfgPll0HsdivCtrl4Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL4"]
pub mod cfg_pll0_hsdiv_ctrl4;
