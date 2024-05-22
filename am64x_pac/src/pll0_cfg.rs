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
    cfg_pll0_hsdiv_ctrl5: CfgPll0HsdivCtrl5,
    cfg_pll0_hsdiv_ctrl6: CfgPll0HsdivCtrl6,
    cfg_pll0_hsdiv_ctrl7: CfgPll0HsdivCtrl7,
    cfg_pll0_hsdiv_ctrl8: CfgPll0HsdivCtrl8,
    cfg_pll0_hsdiv_ctrl9: CfgPll0HsdivCtrl9,
    _reserved23: [u8; 0x0f58],
    cfg_pll1_pid: CfgPll1Pid,
    _reserved24: [u8; 0x04],
    cfg_pll1_cfg: CfgPll1Cfg,
    _reserved25: [u8; 0x04],
    cfg_pll1_lockkey0: CfgPll1Lockkey0,
    cfg_pll1_lockkey1: CfgPll1Lockkey1,
    _reserved27: [u8; 0x08],
    cfg_pll1_ctrl: CfgPll1Ctrl,
    cfg_pll1_stat: CfgPll1Stat,
    _reserved29: [u8; 0x08],
    cfg_pll1_freq_ctrl0: CfgPll1FreqCtrl0,
    cfg_pll1_freq_ctrl1: CfgPll1FreqCtrl1,
    cfg_pll1_div_ctrl: CfgPll1DivCtrl,
    _reserved32: [u8; 0x04],
    cfg_pll1_ss_ctrl: CfgPll1SsCtrl,
    cfg_pll1_ss_spread: CfgPll1SsSpread,
    _reserved34: [u8; 0x18],
    cfg_pll1_cal_ctrl: CfgPll1CalCtrl,
    cfg_pll1_cal_stat: CfgPll1CalStat,
    _reserved36: [u8; 0x18],
    cfg_pll1_hsdiv_ctrl0: CfgPll1HsdivCtrl0,
    cfg_pll1_hsdiv_ctrl1: CfgPll1HsdivCtrl1,
    cfg_pll1_hsdiv_ctrl2: CfgPll1HsdivCtrl2,
    cfg_pll1_hsdiv_ctrl3: CfgPll1HsdivCtrl3,
    cfg_pll1_hsdiv_ctrl4: CfgPll1HsdivCtrl4,
    cfg_pll1_hsdiv_ctrl5: CfgPll1HsdivCtrl5,
    cfg_pll1_hsdiv_ctrl6: CfgPll1HsdivCtrl6,
    _reserved43: [u8; 0x0f64],
    cfg_pll2_pid: CfgPll2Pid,
    _reserved44: [u8; 0x04],
    cfg_pll2_cfg: CfgPll2Cfg,
    _reserved45: [u8; 0x04],
    cfg_pll2_lockkey0: CfgPll2Lockkey0,
    cfg_pll2_lockkey1: CfgPll2Lockkey1,
    _reserved47: [u8; 0x08],
    cfg_pll2_ctrl: CfgPll2Ctrl,
    cfg_pll2_stat: CfgPll2Stat,
    _reserved49: [u8; 0x08],
    cfg_pll2_freq_ctrl0: CfgPll2FreqCtrl0,
    cfg_pll2_freq_ctrl1: CfgPll2FreqCtrl1,
    cfg_pll2_div_ctrl: CfgPll2DivCtrl,
    _reserved52: [u8; 0x04],
    cfg_pll2_ss_ctrl: CfgPll2SsCtrl,
    cfg_pll2_ss_spread: CfgPll2SsSpread,
    _reserved54: [u8; 0x18],
    cfg_pll2_cal_ctrl: CfgPll2CalCtrl,
    cfg_pll2_cal_stat: CfgPll2CalStat,
    _reserved56: [u8; 0x18],
    cfg_pll2_hsdiv_ctrl0: CfgPll2HsdivCtrl0,
    cfg_pll2_hsdiv_ctrl1: CfgPll2HsdivCtrl1,
    cfg_pll2_hsdiv_ctrl2: CfgPll2HsdivCtrl2,
    cfg_pll2_hsdiv_ctrl3: CfgPll2HsdivCtrl3,
    cfg_pll2_hsdiv_ctrl4: CfgPll2HsdivCtrl4,
    cfg_pll2_hsdiv_ctrl5: CfgPll2HsdivCtrl5,
    cfg_pll2_hsdiv_ctrl6: CfgPll2HsdivCtrl6,
    cfg_pll2_hsdiv_ctrl7: CfgPll2HsdivCtrl7,
    cfg_pll2_hsdiv_ctrl8: CfgPll2HsdivCtrl8,
    cfg_pll2_hsdiv_ctrl9: CfgPll2HsdivCtrl9,
    _reserved66: [u8; 0x5f58],
    cfg_pll8_pid: CfgPll8Pid,
    _reserved67: [u8; 0x04],
    cfg_pll8_cfg: CfgPll8Cfg,
    _reserved68: [u8; 0x04],
    cfg_pll8_lockkey0: CfgPll8Lockkey0,
    cfg_pll8_lockkey1: CfgPll8Lockkey1,
    _reserved70: [u8; 0x08],
    cfg_pll8_ctrl: CfgPll8Ctrl,
    cfg_pll8_stat: CfgPll8Stat,
    _reserved72: [u8; 0x08],
    cfg_pll8_freq_ctrl0: CfgPll8FreqCtrl0,
    cfg_pll8_freq_ctrl1: CfgPll8FreqCtrl1,
    cfg_pll8_div_ctrl: CfgPll8DivCtrl,
    _reserved75: [u8; 0x04],
    cfg_pll8_ss_ctrl: CfgPll8SsCtrl,
    cfg_pll8_ss_spread: CfgPll8SsSpread,
    _reserved77: [u8; 0x18],
    cfg_pll8_cal_ctrl: CfgPll8CalCtrl,
    cfg_pll8_cal_stat: CfgPll8CalStat,
    _reserved79: [u8; 0x18],
    cfg_pll8_hsdiv_ctrl0: CfgPll8HsdivCtrl0,
    _reserved80: [u8; 0x3f7c],
    cfg_pll12_pid: CfgPll12Pid,
    _reserved81: [u8; 0x04],
    cfg_pll12_cfg: CfgPll12Cfg,
    _reserved82: [u8; 0x04],
    cfg_pll12_lockkey0: CfgPll12Lockkey0,
    cfg_pll12_lockkey1: CfgPll12Lockkey1,
    _reserved84: [u8; 0x08],
    cfg_pll12_ctrl: CfgPll12Ctrl,
    cfg_pll12_stat: CfgPll12Stat,
    _reserved86: [u8; 0x08],
    cfg_pll12_freq_ctrl0: CfgPll12FreqCtrl0,
    cfg_pll12_freq_ctrl1: CfgPll12FreqCtrl1,
    cfg_pll12_div_ctrl: CfgPll12DivCtrl,
    _reserved89: [u8; 0x04],
    cfg_pll12_ss_ctrl: CfgPll12SsCtrl,
    cfg_pll12_ss_spread: CfgPll12SsSpread,
    _reserved91: [u8; 0x18],
    cfg_pll12_cal_ctrl: CfgPll12CalCtrl,
    cfg_pll12_cal_stat: CfgPll12CalStat,
    _reserved93: [u8; 0x18],
    cfg_pll12_hsdiv_ctrl0: CfgPll12HsdivCtrl0,
    _reserved94: [u8; 0x1f7c],
    cfg_pll14_pid: CfgPll14Pid,
    _reserved95: [u8; 0x04],
    cfg_pll14_cfg: CfgPll14Cfg,
    _reserved96: [u8; 0x04],
    cfg_pll14_lockkey0: CfgPll14Lockkey0,
    cfg_pll14_lockkey1: CfgPll14Lockkey1,
    _reserved98: [u8; 0x08],
    cfg_pll14_ctrl: CfgPll14Ctrl,
    cfg_pll14_stat: CfgPll14Stat,
    _reserved100: [u8; 0x08],
    cfg_pll14_freq_ctrl0: CfgPll14FreqCtrl0,
    cfg_pll14_freq_ctrl1: CfgPll14FreqCtrl1,
    cfg_pll14_div_ctrl: CfgPll14DivCtrl,
    _reserved103: [u8; 0x04],
    cfg_pll14_ss_ctrl: CfgPll14SsCtrl,
    cfg_pll14_ss_spread: CfgPll14SsSpread,
    _reserved105: [u8; 0x18],
    cfg_pll14_cal_ctrl: CfgPll14CalCtrl,
    cfg_pll14_cal_stat: CfgPll14CalStat,
    _reserved107: [u8; 0x18],
    cfg_pll14_hsdiv_ctrl0: CfgPll14HsdivCtrl0,
    cfg_pll14_hsdiv_ctrl1: CfgPll14HsdivCtrl1,
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
    #[doc = "0x94 - CFG_pll0_HSDIV_CTRL5"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl5(&self) -> &CfgPll0HsdivCtrl5 {
        &self.cfg_pll0_hsdiv_ctrl5
    }
    #[doc = "0x98 - CFG_pll0_HSDIV_CTRL6"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl6(&self) -> &CfgPll0HsdivCtrl6 {
        &self.cfg_pll0_hsdiv_ctrl6
    }
    #[doc = "0x9c - CFG_pll0_HSDIV_CTRL7"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl7(&self) -> &CfgPll0HsdivCtrl7 {
        &self.cfg_pll0_hsdiv_ctrl7
    }
    #[doc = "0xa0 - CFG_pll0_HSDIV_CTRL8"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl8(&self) -> &CfgPll0HsdivCtrl8 {
        &self.cfg_pll0_hsdiv_ctrl8
    }
    #[doc = "0xa4 - CFG_pll0_HSDIV_CTRL9"]
    #[inline(always)]
    pub const fn cfg_pll0_hsdiv_ctrl9(&self) -> &CfgPll0HsdivCtrl9 {
        &self.cfg_pll0_hsdiv_ctrl9
    }
    #[doc = "0x1000 - CFG_pll1_PID"]
    #[inline(always)]
    pub const fn cfg_pll1_pid(&self) -> &CfgPll1Pid {
        &self.cfg_pll1_pid
    }
    #[doc = "0x1008 - CFG_pll1_CFG"]
    #[inline(always)]
    pub const fn cfg_pll1_cfg(&self) -> &CfgPll1Cfg {
        &self.cfg_pll1_cfg
    }
    #[doc = "0x1010 - CFG_pll1_LOCKKEY0"]
    #[inline(always)]
    pub const fn cfg_pll1_lockkey0(&self) -> &CfgPll1Lockkey0 {
        &self.cfg_pll1_lockkey0
    }
    #[doc = "0x1014 - CFG_pll1_LOCKKEY1"]
    #[inline(always)]
    pub const fn cfg_pll1_lockkey1(&self) -> &CfgPll1Lockkey1 {
        &self.cfg_pll1_lockkey1
    }
    #[doc = "0x1020 - CFG_pll1_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll1_ctrl(&self) -> &CfgPll1Ctrl {
        &self.cfg_pll1_ctrl
    }
    #[doc = "0x1024 - CFG_pll1_STAT"]
    #[inline(always)]
    pub const fn cfg_pll1_stat(&self) -> &CfgPll1Stat {
        &self.cfg_pll1_stat
    }
    #[doc = "0x1030 - CFG_pll1_FREQ_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll1_freq_ctrl0(&self) -> &CfgPll1FreqCtrl0 {
        &self.cfg_pll1_freq_ctrl0
    }
    #[doc = "0x1034 - CFG_pll1_FREQ_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll1_freq_ctrl1(&self) -> &CfgPll1FreqCtrl1 {
        &self.cfg_pll1_freq_ctrl1
    }
    #[doc = "0x1038 - CFG_pll1_DIV_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll1_div_ctrl(&self) -> &CfgPll1DivCtrl {
        &self.cfg_pll1_div_ctrl
    }
    #[doc = "0x1040 - CFG_pll1_SS_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll1_ss_ctrl(&self) -> &CfgPll1SsCtrl {
        &self.cfg_pll1_ss_ctrl
    }
    #[doc = "0x1044 - CFG_pll1_SS_SPREAD"]
    #[inline(always)]
    pub const fn cfg_pll1_ss_spread(&self) -> &CfgPll1SsSpread {
        &self.cfg_pll1_ss_spread
    }
    #[doc = "0x1060 - CFG_pll1_CAL_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll1_cal_ctrl(&self) -> &CfgPll1CalCtrl {
        &self.cfg_pll1_cal_ctrl
    }
    #[doc = "0x1064 - CFG_pll1_CAL_STAT"]
    #[inline(always)]
    pub const fn cfg_pll1_cal_stat(&self) -> &CfgPll1CalStat {
        &self.cfg_pll1_cal_stat
    }
    #[doc = "0x1080 - CFG_pll1_HSDIV_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl0(&self) -> &CfgPll1HsdivCtrl0 {
        &self.cfg_pll1_hsdiv_ctrl0
    }
    #[doc = "0x1084 - CFG_pll1_HSDIV_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl1(&self) -> &CfgPll1HsdivCtrl1 {
        &self.cfg_pll1_hsdiv_ctrl1
    }
    #[doc = "0x1088 - CFG_pll1_HSDIV_CTRL2"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl2(&self) -> &CfgPll1HsdivCtrl2 {
        &self.cfg_pll1_hsdiv_ctrl2
    }
    #[doc = "0x108c - CFG_pll1_HSDIV_CTRL3"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl3(&self) -> &CfgPll1HsdivCtrl3 {
        &self.cfg_pll1_hsdiv_ctrl3
    }
    #[doc = "0x1090 - CFG_pll1_HSDIV_CTRL4"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl4(&self) -> &CfgPll1HsdivCtrl4 {
        &self.cfg_pll1_hsdiv_ctrl4
    }
    #[doc = "0x1094 - CFG_pll1_HSDIV_CTRL5"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl5(&self) -> &CfgPll1HsdivCtrl5 {
        &self.cfg_pll1_hsdiv_ctrl5
    }
    #[doc = "0x1098 - CFG_pll1_HSDIV_CTRL6"]
    #[inline(always)]
    pub const fn cfg_pll1_hsdiv_ctrl6(&self) -> &CfgPll1HsdivCtrl6 {
        &self.cfg_pll1_hsdiv_ctrl6
    }
    #[doc = "0x2000 - CFG_pll2_PID"]
    #[inline(always)]
    pub const fn cfg_pll2_pid(&self) -> &CfgPll2Pid {
        &self.cfg_pll2_pid
    }
    #[doc = "0x2008 - CFG_pll2_CFG"]
    #[inline(always)]
    pub const fn cfg_pll2_cfg(&self) -> &CfgPll2Cfg {
        &self.cfg_pll2_cfg
    }
    #[doc = "0x2010 - CFG_pll2_LOCKKEY0"]
    #[inline(always)]
    pub const fn cfg_pll2_lockkey0(&self) -> &CfgPll2Lockkey0 {
        &self.cfg_pll2_lockkey0
    }
    #[doc = "0x2014 - CFG_pll2_LOCKKEY1"]
    #[inline(always)]
    pub const fn cfg_pll2_lockkey1(&self) -> &CfgPll2Lockkey1 {
        &self.cfg_pll2_lockkey1
    }
    #[doc = "0x2020 - CFG_pll2_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll2_ctrl(&self) -> &CfgPll2Ctrl {
        &self.cfg_pll2_ctrl
    }
    #[doc = "0x2024 - CFG_pll2_STAT"]
    #[inline(always)]
    pub const fn cfg_pll2_stat(&self) -> &CfgPll2Stat {
        &self.cfg_pll2_stat
    }
    #[doc = "0x2030 - CFG_pll2_FREQ_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll2_freq_ctrl0(&self) -> &CfgPll2FreqCtrl0 {
        &self.cfg_pll2_freq_ctrl0
    }
    #[doc = "0x2034 - CFG_pll2_FREQ_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll2_freq_ctrl1(&self) -> &CfgPll2FreqCtrl1 {
        &self.cfg_pll2_freq_ctrl1
    }
    #[doc = "0x2038 - CFG_pll2_DIV_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll2_div_ctrl(&self) -> &CfgPll2DivCtrl {
        &self.cfg_pll2_div_ctrl
    }
    #[doc = "0x2040 - CFG_pll2_SS_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll2_ss_ctrl(&self) -> &CfgPll2SsCtrl {
        &self.cfg_pll2_ss_ctrl
    }
    #[doc = "0x2044 - CFG_pll2_SS_SPREAD"]
    #[inline(always)]
    pub const fn cfg_pll2_ss_spread(&self) -> &CfgPll2SsSpread {
        &self.cfg_pll2_ss_spread
    }
    #[doc = "0x2060 - CFG_pll2_CAL_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll2_cal_ctrl(&self) -> &CfgPll2CalCtrl {
        &self.cfg_pll2_cal_ctrl
    }
    #[doc = "0x2064 - CFG_pll2_CAL_STAT"]
    #[inline(always)]
    pub const fn cfg_pll2_cal_stat(&self) -> &CfgPll2CalStat {
        &self.cfg_pll2_cal_stat
    }
    #[doc = "0x2080 - CFG_pll2_HSDIV_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl0(&self) -> &CfgPll2HsdivCtrl0 {
        &self.cfg_pll2_hsdiv_ctrl0
    }
    #[doc = "0x2084 - CFG_pll2_HSDIV_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl1(&self) -> &CfgPll2HsdivCtrl1 {
        &self.cfg_pll2_hsdiv_ctrl1
    }
    #[doc = "0x2088 - CFG_pll2_HSDIV_CTRL2"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl2(&self) -> &CfgPll2HsdivCtrl2 {
        &self.cfg_pll2_hsdiv_ctrl2
    }
    #[doc = "0x208c - CFG_pll2_HSDIV_CTRL3"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl3(&self) -> &CfgPll2HsdivCtrl3 {
        &self.cfg_pll2_hsdiv_ctrl3
    }
    #[doc = "0x2090 - CFG_pll2_HSDIV_CTRL4"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl4(&self) -> &CfgPll2HsdivCtrl4 {
        &self.cfg_pll2_hsdiv_ctrl4
    }
    #[doc = "0x2094 - CFG_pll2_HSDIV_CTRL5"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl5(&self) -> &CfgPll2HsdivCtrl5 {
        &self.cfg_pll2_hsdiv_ctrl5
    }
    #[doc = "0x2098 - CFG_pll2_HSDIV_CTRL6"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl6(&self) -> &CfgPll2HsdivCtrl6 {
        &self.cfg_pll2_hsdiv_ctrl6
    }
    #[doc = "0x209c - CFG_pll2_HSDIV_CTRL7"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl7(&self) -> &CfgPll2HsdivCtrl7 {
        &self.cfg_pll2_hsdiv_ctrl7
    }
    #[doc = "0x20a0 - CFG_pll2_HSDIV_CTRL8"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl8(&self) -> &CfgPll2HsdivCtrl8 {
        &self.cfg_pll2_hsdiv_ctrl8
    }
    #[doc = "0x20a4 - CFG_pll2_HSDIV_CTRL9"]
    #[inline(always)]
    pub const fn cfg_pll2_hsdiv_ctrl9(&self) -> &CfgPll2HsdivCtrl9 {
        &self.cfg_pll2_hsdiv_ctrl9
    }
    #[doc = "0x8000 - CFG_pll8_PID"]
    #[inline(always)]
    pub const fn cfg_pll8_pid(&self) -> &CfgPll8Pid {
        &self.cfg_pll8_pid
    }
    #[doc = "0x8008 - CFG_pll8_CFG"]
    #[inline(always)]
    pub const fn cfg_pll8_cfg(&self) -> &CfgPll8Cfg {
        &self.cfg_pll8_cfg
    }
    #[doc = "0x8010 - CFG_pll8_LOCKKEY0"]
    #[inline(always)]
    pub const fn cfg_pll8_lockkey0(&self) -> &CfgPll8Lockkey0 {
        &self.cfg_pll8_lockkey0
    }
    #[doc = "0x8014 - CFG_pll8_LOCKKEY1"]
    #[inline(always)]
    pub const fn cfg_pll8_lockkey1(&self) -> &CfgPll8Lockkey1 {
        &self.cfg_pll8_lockkey1
    }
    #[doc = "0x8020 - CFG_pll8_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll8_ctrl(&self) -> &CfgPll8Ctrl {
        &self.cfg_pll8_ctrl
    }
    #[doc = "0x8024 - CFG_pll8_STAT"]
    #[inline(always)]
    pub const fn cfg_pll8_stat(&self) -> &CfgPll8Stat {
        &self.cfg_pll8_stat
    }
    #[doc = "0x8030 - CFG_pll8_FREQ_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll8_freq_ctrl0(&self) -> &CfgPll8FreqCtrl0 {
        &self.cfg_pll8_freq_ctrl0
    }
    #[doc = "0x8034 - CFG_pll8_FREQ_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll8_freq_ctrl1(&self) -> &CfgPll8FreqCtrl1 {
        &self.cfg_pll8_freq_ctrl1
    }
    #[doc = "0x8038 - CFG_pll8_DIV_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll8_div_ctrl(&self) -> &CfgPll8DivCtrl {
        &self.cfg_pll8_div_ctrl
    }
    #[doc = "0x8040 - CFG_pll8_SS_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll8_ss_ctrl(&self) -> &CfgPll8SsCtrl {
        &self.cfg_pll8_ss_ctrl
    }
    #[doc = "0x8044 - CFG_pll8_SS_SPREAD"]
    #[inline(always)]
    pub const fn cfg_pll8_ss_spread(&self) -> &CfgPll8SsSpread {
        &self.cfg_pll8_ss_spread
    }
    #[doc = "0x8060 - CFG_pll8_CAL_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll8_cal_ctrl(&self) -> &CfgPll8CalCtrl {
        &self.cfg_pll8_cal_ctrl
    }
    #[doc = "0x8064 - CFG_pll8_CAL_STAT"]
    #[inline(always)]
    pub const fn cfg_pll8_cal_stat(&self) -> &CfgPll8CalStat {
        &self.cfg_pll8_cal_stat
    }
    #[doc = "0x8080 - CFG_pll8_HSDIV_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll8_hsdiv_ctrl0(&self) -> &CfgPll8HsdivCtrl0 {
        &self.cfg_pll8_hsdiv_ctrl0
    }
    #[doc = "0xc000 - CFG_pll12_PID"]
    #[inline(always)]
    pub const fn cfg_pll12_pid(&self) -> &CfgPll12Pid {
        &self.cfg_pll12_pid
    }
    #[doc = "0xc008 - CFG_pll12_CFG"]
    #[inline(always)]
    pub const fn cfg_pll12_cfg(&self) -> &CfgPll12Cfg {
        &self.cfg_pll12_cfg
    }
    #[doc = "0xc010 - CFG_pll12_LOCKKEY0"]
    #[inline(always)]
    pub const fn cfg_pll12_lockkey0(&self) -> &CfgPll12Lockkey0 {
        &self.cfg_pll12_lockkey0
    }
    #[doc = "0xc014 - CFG_pll12_LOCKKEY1"]
    #[inline(always)]
    pub const fn cfg_pll12_lockkey1(&self) -> &CfgPll12Lockkey1 {
        &self.cfg_pll12_lockkey1
    }
    #[doc = "0xc020 - CFG_pll12_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll12_ctrl(&self) -> &CfgPll12Ctrl {
        &self.cfg_pll12_ctrl
    }
    #[doc = "0xc024 - CFG_pll12_STAT"]
    #[inline(always)]
    pub const fn cfg_pll12_stat(&self) -> &CfgPll12Stat {
        &self.cfg_pll12_stat
    }
    #[doc = "0xc030 - CFG_pll12_FREQ_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll12_freq_ctrl0(&self) -> &CfgPll12FreqCtrl0 {
        &self.cfg_pll12_freq_ctrl0
    }
    #[doc = "0xc034 - CFG_pll12_FREQ_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll12_freq_ctrl1(&self) -> &CfgPll12FreqCtrl1 {
        &self.cfg_pll12_freq_ctrl1
    }
    #[doc = "0xc038 - CFG_pll12_DIV_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll12_div_ctrl(&self) -> &CfgPll12DivCtrl {
        &self.cfg_pll12_div_ctrl
    }
    #[doc = "0xc040 - CFG_pll12_SS_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll12_ss_ctrl(&self) -> &CfgPll12SsCtrl {
        &self.cfg_pll12_ss_ctrl
    }
    #[doc = "0xc044 - CFG_pll12_SS_SPREAD"]
    #[inline(always)]
    pub const fn cfg_pll12_ss_spread(&self) -> &CfgPll12SsSpread {
        &self.cfg_pll12_ss_spread
    }
    #[doc = "0xc060 - CFG_pll12_CAL_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll12_cal_ctrl(&self) -> &CfgPll12CalCtrl {
        &self.cfg_pll12_cal_ctrl
    }
    #[doc = "0xc064 - CFG_pll12_CAL_STAT"]
    #[inline(always)]
    pub const fn cfg_pll12_cal_stat(&self) -> &CfgPll12CalStat {
        &self.cfg_pll12_cal_stat
    }
    #[doc = "0xc080 - CFG_pll12_HSDIV_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll12_hsdiv_ctrl0(&self) -> &CfgPll12HsdivCtrl0 {
        &self.cfg_pll12_hsdiv_ctrl0
    }
    #[doc = "0xe000 - CFG_pll14_PID"]
    #[inline(always)]
    pub const fn cfg_pll14_pid(&self) -> &CfgPll14Pid {
        &self.cfg_pll14_pid
    }
    #[doc = "0xe008 - CFG_pll14_CFG"]
    #[inline(always)]
    pub const fn cfg_pll14_cfg(&self) -> &CfgPll14Cfg {
        &self.cfg_pll14_cfg
    }
    #[doc = "0xe010 - CFG_pll14_LOCKKEY0"]
    #[inline(always)]
    pub const fn cfg_pll14_lockkey0(&self) -> &CfgPll14Lockkey0 {
        &self.cfg_pll14_lockkey0
    }
    #[doc = "0xe014 - CFG_pll14_LOCKKEY1"]
    #[inline(always)]
    pub const fn cfg_pll14_lockkey1(&self) -> &CfgPll14Lockkey1 {
        &self.cfg_pll14_lockkey1
    }
    #[doc = "0xe020 - CFG_pll14_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll14_ctrl(&self) -> &CfgPll14Ctrl {
        &self.cfg_pll14_ctrl
    }
    #[doc = "0xe024 - CFG_pll14_STAT"]
    #[inline(always)]
    pub const fn cfg_pll14_stat(&self) -> &CfgPll14Stat {
        &self.cfg_pll14_stat
    }
    #[doc = "0xe030 - CFG_pll14_FREQ_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll14_freq_ctrl0(&self) -> &CfgPll14FreqCtrl0 {
        &self.cfg_pll14_freq_ctrl0
    }
    #[doc = "0xe034 - CFG_pll14_FREQ_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll14_freq_ctrl1(&self) -> &CfgPll14FreqCtrl1 {
        &self.cfg_pll14_freq_ctrl1
    }
    #[doc = "0xe038 - CFG_pll14_DIV_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll14_div_ctrl(&self) -> &CfgPll14DivCtrl {
        &self.cfg_pll14_div_ctrl
    }
    #[doc = "0xe040 - CFG_pll14_SS_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll14_ss_ctrl(&self) -> &CfgPll14SsCtrl {
        &self.cfg_pll14_ss_ctrl
    }
    #[doc = "0xe044 - CFG_pll14_SS_SPREAD"]
    #[inline(always)]
    pub const fn cfg_pll14_ss_spread(&self) -> &CfgPll14SsSpread {
        &self.cfg_pll14_ss_spread
    }
    #[doc = "0xe060 - CFG_pll14_CAL_CTRL"]
    #[inline(always)]
    pub const fn cfg_pll14_cal_ctrl(&self) -> &CfgPll14CalCtrl {
        &self.cfg_pll14_cal_ctrl
    }
    #[doc = "0xe064 - CFG_pll14_CAL_STAT"]
    #[inline(always)]
    pub const fn cfg_pll14_cal_stat(&self) -> &CfgPll14CalStat {
        &self.cfg_pll14_cal_stat
    }
    #[doc = "0xe080 - CFG_pll14_HSDIV_CTRL0"]
    #[inline(always)]
    pub const fn cfg_pll14_hsdiv_ctrl0(&self) -> &CfgPll14HsdivCtrl0 {
        &self.cfg_pll14_hsdiv_ctrl0
    }
    #[doc = "0xe084 - CFG_pll14_HSDIV_CTRL1"]
    #[inline(always)]
    pub const fn cfg_pll14_hsdiv_ctrl1(&self) -> &CfgPll14HsdivCtrl1 {
        &self.cfg_pll14_hsdiv_ctrl1
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
#[doc = "CFG_pll0_HSDIV_CTRL5 (rw) register accessor: CFG_pll0_HSDIV_CTRL5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl5`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL5")]
pub type CfgPll0HsdivCtrl5 = crate::Reg<cfg_pll0_hsdiv_ctrl5::CfgPll0HsdivCtrl5Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL5"]
pub mod cfg_pll0_hsdiv_ctrl5;
#[doc = "CFG_pll0_HSDIV_CTRL6 (rw) register accessor: CFG_pll0_HSDIV_CTRL6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl6`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL6")]
pub type CfgPll0HsdivCtrl6 = crate::Reg<cfg_pll0_hsdiv_ctrl6::CfgPll0HsdivCtrl6Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL6"]
pub mod cfg_pll0_hsdiv_ctrl6;
#[doc = "CFG_pll0_HSDIV_CTRL7 (rw) register accessor: CFG_pll0_HSDIV_CTRL7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl7`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL7")]
pub type CfgPll0HsdivCtrl7 = crate::Reg<cfg_pll0_hsdiv_ctrl7::CfgPll0HsdivCtrl7Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL7"]
pub mod cfg_pll0_hsdiv_ctrl7;
#[doc = "CFG_pll0_HSDIV_CTRL8 (rw) register accessor: CFG_pll0_HSDIV_CTRL8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl8`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL8")]
pub type CfgPll0HsdivCtrl8 = crate::Reg<cfg_pll0_hsdiv_ctrl8::CfgPll0HsdivCtrl8Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL8"]
pub mod cfg_pll0_hsdiv_ctrl8;
#[doc = "CFG_pll0_HSDIV_CTRL9 (rw) register accessor: CFG_pll0_HSDIV_CTRL9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_hsdiv_ctrl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_hsdiv_ctrl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll0_hsdiv_ctrl9`]
module"]
#[doc(alias = "CFG_pll0_HSDIV_CTRL9")]
pub type CfgPll0HsdivCtrl9 = crate::Reg<cfg_pll0_hsdiv_ctrl9::CfgPll0HsdivCtrl9Spec>;
#[doc = "CFG_pll0_HSDIV_CTRL9"]
pub mod cfg_pll0_hsdiv_ctrl9;
#[doc = "CFG_pll1_PID (rw) register accessor: CFG_pll1_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_pid`]
module"]
#[doc(alias = "CFG_pll1_PID")]
pub type CfgPll1Pid = crate::Reg<cfg_pll1_pid::CfgPll1PidSpec>;
#[doc = "CFG_pll1_PID"]
pub mod cfg_pll1_pid;
#[doc = "CFG_pll1_CFG (rw) register accessor: CFG_pll1_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_cfg`]
module"]
#[doc(alias = "CFG_pll1_CFG")]
pub type CfgPll1Cfg = crate::Reg<cfg_pll1_cfg::CfgPll1CfgSpec>;
#[doc = "CFG_pll1_CFG"]
pub mod cfg_pll1_cfg;
#[doc = "CFG_pll1_LOCKKEY0 (rw) register accessor: CFG_pll1_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_lockkey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_lockkey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_lockkey0`]
module"]
#[doc(alias = "CFG_pll1_LOCKKEY0")]
pub type CfgPll1Lockkey0 = crate::Reg<cfg_pll1_lockkey0::CfgPll1Lockkey0Spec>;
#[doc = "CFG_pll1_LOCKKEY0"]
pub mod cfg_pll1_lockkey0;
#[doc = "CFG_pll1_LOCKKEY1 (rw) register accessor: CFG_pll1_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_lockkey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_lockkey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_lockkey1`]
module"]
#[doc(alias = "CFG_pll1_LOCKKEY1")]
pub type CfgPll1Lockkey1 = crate::Reg<cfg_pll1_lockkey1::CfgPll1Lockkey1Spec>;
#[doc = "CFG_pll1_LOCKKEY1"]
pub mod cfg_pll1_lockkey1;
#[doc = "CFG_pll1_CTRL (rw) register accessor: CFG_pll1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_ctrl`]
module"]
#[doc(alias = "CFG_pll1_CTRL")]
pub type CfgPll1Ctrl = crate::Reg<cfg_pll1_ctrl::CfgPll1CtrlSpec>;
#[doc = "CFG_pll1_CTRL"]
pub mod cfg_pll1_ctrl;
#[doc = "CFG_pll1_STAT (rw) register accessor: CFG_pll1_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_stat`]
module"]
#[doc(alias = "CFG_pll1_STAT")]
pub type CfgPll1Stat = crate::Reg<cfg_pll1_stat::CfgPll1StatSpec>;
#[doc = "CFG_pll1_STAT"]
pub mod cfg_pll1_stat;
#[doc = "CFG_pll1_FREQ_CTRL0 (rw) register accessor: CFG_pll1_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_freq_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_freq_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_freq_ctrl0`]
module"]
#[doc(alias = "CFG_pll1_FREQ_CTRL0")]
pub type CfgPll1FreqCtrl0 = crate::Reg<cfg_pll1_freq_ctrl0::CfgPll1FreqCtrl0Spec>;
#[doc = "CFG_pll1_FREQ_CTRL0"]
pub mod cfg_pll1_freq_ctrl0;
#[doc = "CFG_pll1_FREQ_CTRL1 (rw) register accessor: CFG_pll1_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_freq_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_freq_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_freq_ctrl1`]
module"]
#[doc(alias = "CFG_pll1_FREQ_CTRL1")]
pub type CfgPll1FreqCtrl1 = crate::Reg<cfg_pll1_freq_ctrl1::CfgPll1FreqCtrl1Spec>;
#[doc = "CFG_pll1_FREQ_CTRL1"]
pub mod cfg_pll1_freq_ctrl1;
#[doc = "CFG_pll1_DIV_CTRL (rw) register accessor: CFG_pll1_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_div_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_div_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_div_ctrl`]
module"]
#[doc(alias = "CFG_pll1_DIV_CTRL")]
pub type CfgPll1DivCtrl = crate::Reg<cfg_pll1_div_ctrl::CfgPll1DivCtrlSpec>;
#[doc = "CFG_pll1_DIV_CTRL"]
pub mod cfg_pll1_div_ctrl;
#[doc = "CFG_pll1_SS_CTRL (rw) register accessor: CFG_pll1_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_ss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_ss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_ss_ctrl`]
module"]
#[doc(alias = "CFG_pll1_SS_CTRL")]
pub type CfgPll1SsCtrl = crate::Reg<cfg_pll1_ss_ctrl::CfgPll1SsCtrlSpec>;
#[doc = "CFG_pll1_SS_CTRL"]
pub mod cfg_pll1_ss_ctrl;
#[doc = "CFG_pll1_SS_SPREAD (rw) register accessor: CFG_pll1_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_ss_spread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_ss_spread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_ss_spread`]
module"]
#[doc(alias = "CFG_pll1_SS_SPREAD")]
pub type CfgPll1SsSpread = crate::Reg<cfg_pll1_ss_spread::CfgPll1SsSpreadSpec>;
#[doc = "CFG_pll1_SS_SPREAD"]
pub mod cfg_pll1_ss_spread;
#[doc = "CFG_pll1_CAL_CTRL (rw) register accessor: CFG_pll1_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_cal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_cal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_cal_ctrl`]
module"]
#[doc(alias = "CFG_pll1_CAL_CTRL")]
pub type CfgPll1CalCtrl = crate::Reg<cfg_pll1_cal_ctrl::CfgPll1CalCtrlSpec>;
#[doc = "CFG_pll1_CAL_CTRL"]
pub mod cfg_pll1_cal_ctrl;
#[doc = "CFG_pll1_CAL_STAT (rw) register accessor: CFG_pll1_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_cal_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_cal_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_cal_stat`]
module"]
#[doc(alias = "CFG_pll1_CAL_STAT")]
pub type CfgPll1CalStat = crate::Reg<cfg_pll1_cal_stat::CfgPll1CalStatSpec>;
#[doc = "CFG_pll1_CAL_STAT"]
pub mod cfg_pll1_cal_stat;
#[doc = "CFG_pll1_HSDIV_CTRL0 (rw) register accessor: CFG_pll1_HSDIV_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl0`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL0")]
pub type CfgPll1HsdivCtrl0 = crate::Reg<cfg_pll1_hsdiv_ctrl0::CfgPll1HsdivCtrl0Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL0"]
pub mod cfg_pll1_hsdiv_ctrl0;
#[doc = "CFG_pll1_HSDIV_CTRL1 (rw) register accessor: CFG_pll1_HSDIV_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl1`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL1")]
pub type CfgPll1HsdivCtrl1 = crate::Reg<cfg_pll1_hsdiv_ctrl1::CfgPll1HsdivCtrl1Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL1"]
pub mod cfg_pll1_hsdiv_ctrl1;
#[doc = "CFG_pll1_HSDIV_CTRL2 (rw) register accessor: CFG_pll1_HSDIV_CTRL2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl2`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL2")]
pub type CfgPll1HsdivCtrl2 = crate::Reg<cfg_pll1_hsdiv_ctrl2::CfgPll1HsdivCtrl2Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL2"]
pub mod cfg_pll1_hsdiv_ctrl2;
#[doc = "CFG_pll1_HSDIV_CTRL3 (rw) register accessor: CFG_pll1_HSDIV_CTRL3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl3`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL3")]
pub type CfgPll1HsdivCtrl3 = crate::Reg<cfg_pll1_hsdiv_ctrl3::CfgPll1HsdivCtrl3Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL3"]
pub mod cfg_pll1_hsdiv_ctrl3;
#[doc = "CFG_pll1_HSDIV_CTRL4 (rw) register accessor: CFG_pll1_HSDIV_CTRL4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl4`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL4")]
pub type CfgPll1HsdivCtrl4 = crate::Reg<cfg_pll1_hsdiv_ctrl4::CfgPll1HsdivCtrl4Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL4"]
pub mod cfg_pll1_hsdiv_ctrl4;
#[doc = "CFG_pll1_HSDIV_CTRL5 (rw) register accessor: CFG_pll1_HSDIV_CTRL5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl5`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL5")]
pub type CfgPll1HsdivCtrl5 = crate::Reg<cfg_pll1_hsdiv_ctrl5::CfgPll1HsdivCtrl5Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL5"]
pub mod cfg_pll1_hsdiv_ctrl5;
#[doc = "CFG_pll1_HSDIV_CTRL6 (rw) register accessor: CFG_pll1_HSDIV_CTRL6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_hsdiv_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_hsdiv_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll1_hsdiv_ctrl6`]
module"]
#[doc(alias = "CFG_pll1_HSDIV_CTRL6")]
pub type CfgPll1HsdivCtrl6 = crate::Reg<cfg_pll1_hsdiv_ctrl6::CfgPll1HsdivCtrl6Spec>;
#[doc = "CFG_pll1_HSDIV_CTRL6"]
pub mod cfg_pll1_hsdiv_ctrl6;
#[doc = "CFG_pll2_PID (rw) register accessor: CFG_pll2_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_pid`]
module"]
#[doc(alias = "CFG_pll2_PID")]
pub type CfgPll2Pid = crate::Reg<cfg_pll2_pid::CfgPll2PidSpec>;
#[doc = "CFG_pll2_PID"]
pub mod cfg_pll2_pid;
#[doc = "CFG_pll2_CFG (rw) register accessor: CFG_pll2_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_cfg`]
module"]
#[doc(alias = "CFG_pll2_CFG")]
pub type CfgPll2Cfg = crate::Reg<cfg_pll2_cfg::CfgPll2CfgSpec>;
#[doc = "CFG_pll2_CFG"]
pub mod cfg_pll2_cfg;
#[doc = "CFG_pll2_LOCKKEY0 (rw) register accessor: CFG_pll2_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_lockkey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_lockkey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_lockkey0`]
module"]
#[doc(alias = "CFG_pll2_LOCKKEY0")]
pub type CfgPll2Lockkey0 = crate::Reg<cfg_pll2_lockkey0::CfgPll2Lockkey0Spec>;
#[doc = "CFG_pll2_LOCKKEY0"]
pub mod cfg_pll2_lockkey0;
#[doc = "CFG_pll2_LOCKKEY1 (rw) register accessor: CFG_pll2_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_lockkey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_lockkey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_lockkey1`]
module"]
#[doc(alias = "CFG_pll2_LOCKKEY1")]
pub type CfgPll2Lockkey1 = crate::Reg<cfg_pll2_lockkey1::CfgPll2Lockkey1Spec>;
#[doc = "CFG_pll2_LOCKKEY1"]
pub mod cfg_pll2_lockkey1;
#[doc = "CFG_pll2_CTRL (rw) register accessor: CFG_pll2_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_ctrl`]
module"]
#[doc(alias = "CFG_pll2_CTRL")]
pub type CfgPll2Ctrl = crate::Reg<cfg_pll2_ctrl::CfgPll2CtrlSpec>;
#[doc = "CFG_pll2_CTRL"]
pub mod cfg_pll2_ctrl;
#[doc = "CFG_pll2_STAT (rw) register accessor: CFG_pll2_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_stat`]
module"]
#[doc(alias = "CFG_pll2_STAT")]
pub type CfgPll2Stat = crate::Reg<cfg_pll2_stat::CfgPll2StatSpec>;
#[doc = "CFG_pll2_STAT"]
pub mod cfg_pll2_stat;
#[doc = "CFG_pll2_FREQ_CTRL0 (rw) register accessor: CFG_pll2_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_freq_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_freq_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_freq_ctrl0`]
module"]
#[doc(alias = "CFG_pll2_FREQ_CTRL0")]
pub type CfgPll2FreqCtrl0 = crate::Reg<cfg_pll2_freq_ctrl0::CfgPll2FreqCtrl0Spec>;
#[doc = "CFG_pll2_FREQ_CTRL0"]
pub mod cfg_pll2_freq_ctrl0;
#[doc = "CFG_pll2_FREQ_CTRL1 (rw) register accessor: CFG_pll2_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_freq_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_freq_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_freq_ctrl1`]
module"]
#[doc(alias = "CFG_pll2_FREQ_CTRL1")]
pub type CfgPll2FreqCtrl1 = crate::Reg<cfg_pll2_freq_ctrl1::CfgPll2FreqCtrl1Spec>;
#[doc = "CFG_pll2_FREQ_CTRL1"]
pub mod cfg_pll2_freq_ctrl1;
#[doc = "CFG_pll2_DIV_CTRL (rw) register accessor: CFG_pll2_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_div_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_div_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_div_ctrl`]
module"]
#[doc(alias = "CFG_pll2_DIV_CTRL")]
pub type CfgPll2DivCtrl = crate::Reg<cfg_pll2_div_ctrl::CfgPll2DivCtrlSpec>;
#[doc = "CFG_pll2_DIV_CTRL"]
pub mod cfg_pll2_div_ctrl;
#[doc = "CFG_pll2_SS_CTRL (rw) register accessor: CFG_pll2_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_ss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_ss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_ss_ctrl`]
module"]
#[doc(alias = "CFG_pll2_SS_CTRL")]
pub type CfgPll2SsCtrl = crate::Reg<cfg_pll2_ss_ctrl::CfgPll2SsCtrlSpec>;
#[doc = "CFG_pll2_SS_CTRL"]
pub mod cfg_pll2_ss_ctrl;
#[doc = "CFG_pll2_SS_SPREAD (rw) register accessor: CFG_pll2_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_ss_spread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_ss_spread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_ss_spread`]
module"]
#[doc(alias = "CFG_pll2_SS_SPREAD")]
pub type CfgPll2SsSpread = crate::Reg<cfg_pll2_ss_spread::CfgPll2SsSpreadSpec>;
#[doc = "CFG_pll2_SS_SPREAD"]
pub mod cfg_pll2_ss_spread;
#[doc = "CFG_pll2_CAL_CTRL (rw) register accessor: CFG_pll2_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_cal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_cal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_cal_ctrl`]
module"]
#[doc(alias = "CFG_pll2_CAL_CTRL")]
pub type CfgPll2CalCtrl = crate::Reg<cfg_pll2_cal_ctrl::CfgPll2CalCtrlSpec>;
#[doc = "CFG_pll2_CAL_CTRL"]
pub mod cfg_pll2_cal_ctrl;
#[doc = "CFG_pll2_CAL_STAT (rw) register accessor: CFG_pll2_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_cal_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_cal_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_cal_stat`]
module"]
#[doc(alias = "CFG_pll2_CAL_STAT")]
pub type CfgPll2CalStat = crate::Reg<cfg_pll2_cal_stat::CfgPll2CalStatSpec>;
#[doc = "CFG_pll2_CAL_STAT"]
pub mod cfg_pll2_cal_stat;
#[doc = "CFG_pll2_HSDIV_CTRL0 (rw) register accessor: CFG_pll2_HSDIV_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl0`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL0")]
pub type CfgPll2HsdivCtrl0 = crate::Reg<cfg_pll2_hsdiv_ctrl0::CfgPll2HsdivCtrl0Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL0"]
pub mod cfg_pll2_hsdiv_ctrl0;
#[doc = "CFG_pll2_HSDIV_CTRL1 (rw) register accessor: CFG_pll2_HSDIV_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl1`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL1")]
pub type CfgPll2HsdivCtrl1 = crate::Reg<cfg_pll2_hsdiv_ctrl1::CfgPll2HsdivCtrl1Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL1"]
pub mod cfg_pll2_hsdiv_ctrl1;
#[doc = "CFG_pll2_HSDIV_CTRL2 (rw) register accessor: CFG_pll2_HSDIV_CTRL2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl2`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL2")]
pub type CfgPll2HsdivCtrl2 = crate::Reg<cfg_pll2_hsdiv_ctrl2::CfgPll2HsdivCtrl2Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL2"]
pub mod cfg_pll2_hsdiv_ctrl2;
#[doc = "CFG_pll2_HSDIV_CTRL3 (rw) register accessor: CFG_pll2_HSDIV_CTRL3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl3`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL3")]
pub type CfgPll2HsdivCtrl3 = crate::Reg<cfg_pll2_hsdiv_ctrl3::CfgPll2HsdivCtrl3Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL3"]
pub mod cfg_pll2_hsdiv_ctrl3;
#[doc = "CFG_pll2_HSDIV_CTRL4 (rw) register accessor: CFG_pll2_HSDIV_CTRL4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl4`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL4")]
pub type CfgPll2HsdivCtrl4 = crate::Reg<cfg_pll2_hsdiv_ctrl4::CfgPll2HsdivCtrl4Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL4"]
pub mod cfg_pll2_hsdiv_ctrl4;
#[doc = "CFG_pll2_HSDIV_CTRL5 (rw) register accessor: CFG_pll2_HSDIV_CTRL5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl5`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL5")]
pub type CfgPll2HsdivCtrl5 = crate::Reg<cfg_pll2_hsdiv_ctrl5::CfgPll2HsdivCtrl5Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL5"]
pub mod cfg_pll2_hsdiv_ctrl5;
#[doc = "CFG_pll2_HSDIV_CTRL6 (rw) register accessor: CFG_pll2_HSDIV_CTRL6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl6`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL6")]
pub type CfgPll2HsdivCtrl6 = crate::Reg<cfg_pll2_hsdiv_ctrl6::CfgPll2HsdivCtrl6Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL6"]
pub mod cfg_pll2_hsdiv_ctrl6;
#[doc = "CFG_pll2_HSDIV_CTRL7 (rw) register accessor: CFG_pll2_HSDIV_CTRL7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl7`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL7")]
pub type CfgPll2HsdivCtrl7 = crate::Reg<cfg_pll2_hsdiv_ctrl7::CfgPll2HsdivCtrl7Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL7"]
pub mod cfg_pll2_hsdiv_ctrl7;
#[doc = "CFG_pll2_HSDIV_CTRL8 (rw) register accessor: CFG_pll2_HSDIV_CTRL8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl8`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL8")]
pub type CfgPll2HsdivCtrl8 = crate::Reg<cfg_pll2_hsdiv_ctrl8::CfgPll2HsdivCtrl8Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL8"]
pub mod cfg_pll2_hsdiv_ctrl8;
#[doc = "CFG_pll2_HSDIV_CTRL9 (rw) register accessor: CFG_pll2_HSDIV_CTRL9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_hsdiv_ctrl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_hsdiv_ctrl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll2_hsdiv_ctrl9`]
module"]
#[doc(alias = "CFG_pll2_HSDIV_CTRL9")]
pub type CfgPll2HsdivCtrl9 = crate::Reg<cfg_pll2_hsdiv_ctrl9::CfgPll2HsdivCtrl9Spec>;
#[doc = "CFG_pll2_HSDIV_CTRL9"]
pub mod cfg_pll2_hsdiv_ctrl9;
#[doc = "CFG_pll8_PID (rw) register accessor: CFG_pll8_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_pid`]
module"]
#[doc(alias = "CFG_pll8_PID")]
pub type CfgPll8Pid = crate::Reg<cfg_pll8_pid::CfgPll8PidSpec>;
#[doc = "CFG_pll8_PID"]
pub mod cfg_pll8_pid;
#[doc = "CFG_pll8_CFG (rw) register accessor: CFG_pll8_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_cfg`]
module"]
#[doc(alias = "CFG_pll8_CFG")]
pub type CfgPll8Cfg = crate::Reg<cfg_pll8_cfg::CfgPll8CfgSpec>;
#[doc = "CFG_pll8_CFG"]
pub mod cfg_pll8_cfg;
#[doc = "CFG_pll8_LOCKKEY0 (rw) register accessor: CFG_pll8_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_lockkey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_lockkey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_lockkey0`]
module"]
#[doc(alias = "CFG_pll8_LOCKKEY0")]
pub type CfgPll8Lockkey0 = crate::Reg<cfg_pll8_lockkey0::CfgPll8Lockkey0Spec>;
#[doc = "CFG_pll8_LOCKKEY0"]
pub mod cfg_pll8_lockkey0;
#[doc = "CFG_pll8_LOCKKEY1 (rw) register accessor: CFG_pll8_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_lockkey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_lockkey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_lockkey1`]
module"]
#[doc(alias = "CFG_pll8_LOCKKEY1")]
pub type CfgPll8Lockkey1 = crate::Reg<cfg_pll8_lockkey1::CfgPll8Lockkey1Spec>;
#[doc = "CFG_pll8_LOCKKEY1"]
pub mod cfg_pll8_lockkey1;
#[doc = "CFG_pll8_CTRL (rw) register accessor: CFG_pll8_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_ctrl`]
module"]
#[doc(alias = "CFG_pll8_CTRL")]
pub type CfgPll8Ctrl = crate::Reg<cfg_pll8_ctrl::CfgPll8CtrlSpec>;
#[doc = "CFG_pll8_CTRL"]
pub mod cfg_pll8_ctrl;
#[doc = "CFG_pll8_STAT (rw) register accessor: CFG_pll8_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_stat`]
module"]
#[doc(alias = "CFG_pll8_STAT")]
pub type CfgPll8Stat = crate::Reg<cfg_pll8_stat::CfgPll8StatSpec>;
#[doc = "CFG_pll8_STAT"]
pub mod cfg_pll8_stat;
#[doc = "CFG_pll8_FREQ_CTRL0 (rw) register accessor: CFG_pll8_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_freq_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_freq_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_freq_ctrl0`]
module"]
#[doc(alias = "CFG_pll8_FREQ_CTRL0")]
pub type CfgPll8FreqCtrl0 = crate::Reg<cfg_pll8_freq_ctrl0::CfgPll8FreqCtrl0Spec>;
#[doc = "CFG_pll8_FREQ_CTRL0"]
pub mod cfg_pll8_freq_ctrl0;
#[doc = "CFG_pll8_FREQ_CTRL1 (rw) register accessor: CFG_pll8_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_freq_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_freq_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_freq_ctrl1`]
module"]
#[doc(alias = "CFG_pll8_FREQ_CTRL1")]
pub type CfgPll8FreqCtrl1 = crate::Reg<cfg_pll8_freq_ctrl1::CfgPll8FreqCtrl1Spec>;
#[doc = "CFG_pll8_FREQ_CTRL1"]
pub mod cfg_pll8_freq_ctrl1;
#[doc = "CFG_pll8_DIV_CTRL (rw) register accessor: CFG_pll8_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_div_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_div_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_div_ctrl`]
module"]
#[doc(alias = "CFG_pll8_DIV_CTRL")]
pub type CfgPll8DivCtrl = crate::Reg<cfg_pll8_div_ctrl::CfgPll8DivCtrlSpec>;
#[doc = "CFG_pll8_DIV_CTRL"]
pub mod cfg_pll8_div_ctrl;
#[doc = "CFG_pll8_SS_CTRL (rw) register accessor: CFG_pll8_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_ss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_ss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_ss_ctrl`]
module"]
#[doc(alias = "CFG_pll8_SS_CTRL")]
pub type CfgPll8SsCtrl = crate::Reg<cfg_pll8_ss_ctrl::CfgPll8SsCtrlSpec>;
#[doc = "CFG_pll8_SS_CTRL"]
pub mod cfg_pll8_ss_ctrl;
#[doc = "CFG_pll8_SS_SPREAD (rw) register accessor: CFG_pll8_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_ss_spread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_ss_spread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_ss_spread`]
module"]
#[doc(alias = "CFG_pll8_SS_SPREAD")]
pub type CfgPll8SsSpread = crate::Reg<cfg_pll8_ss_spread::CfgPll8SsSpreadSpec>;
#[doc = "CFG_pll8_SS_SPREAD"]
pub mod cfg_pll8_ss_spread;
#[doc = "CFG_pll8_CAL_CTRL (rw) register accessor: CFG_pll8_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_cal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_cal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_cal_ctrl`]
module"]
#[doc(alias = "CFG_pll8_CAL_CTRL")]
pub type CfgPll8CalCtrl = crate::Reg<cfg_pll8_cal_ctrl::CfgPll8CalCtrlSpec>;
#[doc = "CFG_pll8_CAL_CTRL"]
pub mod cfg_pll8_cal_ctrl;
#[doc = "CFG_pll8_CAL_STAT (rw) register accessor: CFG_pll8_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_cal_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_cal_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_cal_stat`]
module"]
#[doc(alias = "CFG_pll8_CAL_STAT")]
pub type CfgPll8CalStat = crate::Reg<cfg_pll8_cal_stat::CfgPll8CalStatSpec>;
#[doc = "CFG_pll8_CAL_STAT"]
pub mod cfg_pll8_cal_stat;
#[doc = "CFG_pll8_HSDIV_CTRL0 (rw) register accessor: CFG_pll8_HSDIV_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_hsdiv_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_hsdiv_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll8_hsdiv_ctrl0`]
module"]
#[doc(alias = "CFG_pll8_HSDIV_CTRL0")]
pub type CfgPll8HsdivCtrl0 = crate::Reg<cfg_pll8_hsdiv_ctrl0::CfgPll8HsdivCtrl0Spec>;
#[doc = "CFG_pll8_HSDIV_CTRL0"]
pub mod cfg_pll8_hsdiv_ctrl0;
#[doc = "CFG_pll12_PID (rw) register accessor: CFG_pll12_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_pid`]
module"]
#[doc(alias = "CFG_pll12_PID")]
pub type CfgPll12Pid = crate::Reg<cfg_pll12_pid::CfgPll12PidSpec>;
#[doc = "CFG_pll12_PID"]
pub mod cfg_pll12_pid;
#[doc = "CFG_pll12_CFG (rw) register accessor: CFG_pll12_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_cfg`]
module"]
#[doc(alias = "CFG_pll12_CFG")]
pub type CfgPll12Cfg = crate::Reg<cfg_pll12_cfg::CfgPll12CfgSpec>;
#[doc = "CFG_pll12_CFG"]
pub mod cfg_pll12_cfg;
#[doc = "CFG_pll12_LOCKKEY0 (rw) register accessor: CFG_pll12_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_lockkey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_lockkey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_lockkey0`]
module"]
#[doc(alias = "CFG_pll12_LOCKKEY0")]
pub type CfgPll12Lockkey0 = crate::Reg<cfg_pll12_lockkey0::CfgPll12Lockkey0Spec>;
#[doc = "CFG_pll12_LOCKKEY0"]
pub mod cfg_pll12_lockkey0;
#[doc = "CFG_pll12_LOCKKEY1 (rw) register accessor: CFG_pll12_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_lockkey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_lockkey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_lockkey1`]
module"]
#[doc(alias = "CFG_pll12_LOCKKEY1")]
pub type CfgPll12Lockkey1 = crate::Reg<cfg_pll12_lockkey1::CfgPll12Lockkey1Spec>;
#[doc = "CFG_pll12_LOCKKEY1"]
pub mod cfg_pll12_lockkey1;
#[doc = "CFG_pll12_CTRL (rw) register accessor: CFG_pll12_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_ctrl`]
module"]
#[doc(alias = "CFG_pll12_CTRL")]
pub type CfgPll12Ctrl = crate::Reg<cfg_pll12_ctrl::CfgPll12CtrlSpec>;
#[doc = "CFG_pll12_CTRL"]
pub mod cfg_pll12_ctrl;
#[doc = "CFG_pll12_STAT (rw) register accessor: CFG_pll12_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_stat`]
module"]
#[doc(alias = "CFG_pll12_STAT")]
pub type CfgPll12Stat = crate::Reg<cfg_pll12_stat::CfgPll12StatSpec>;
#[doc = "CFG_pll12_STAT"]
pub mod cfg_pll12_stat;
#[doc = "CFG_pll12_FREQ_CTRL0 (rw) register accessor: CFG_pll12_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_freq_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_freq_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_freq_ctrl0`]
module"]
#[doc(alias = "CFG_pll12_FREQ_CTRL0")]
pub type CfgPll12FreqCtrl0 = crate::Reg<cfg_pll12_freq_ctrl0::CfgPll12FreqCtrl0Spec>;
#[doc = "CFG_pll12_FREQ_CTRL0"]
pub mod cfg_pll12_freq_ctrl0;
#[doc = "CFG_pll12_FREQ_CTRL1 (rw) register accessor: CFG_pll12_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_freq_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_freq_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_freq_ctrl1`]
module"]
#[doc(alias = "CFG_pll12_FREQ_CTRL1")]
pub type CfgPll12FreqCtrl1 = crate::Reg<cfg_pll12_freq_ctrl1::CfgPll12FreqCtrl1Spec>;
#[doc = "CFG_pll12_FREQ_CTRL1"]
pub mod cfg_pll12_freq_ctrl1;
#[doc = "CFG_pll12_DIV_CTRL (rw) register accessor: CFG_pll12_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_div_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_div_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_div_ctrl`]
module"]
#[doc(alias = "CFG_pll12_DIV_CTRL")]
pub type CfgPll12DivCtrl = crate::Reg<cfg_pll12_div_ctrl::CfgPll12DivCtrlSpec>;
#[doc = "CFG_pll12_DIV_CTRL"]
pub mod cfg_pll12_div_ctrl;
#[doc = "CFG_pll12_SS_CTRL (rw) register accessor: CFG_pll12_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_ss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_ss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_ss_ctrl`]
module"]
#[doc(alias = "CFG_pll12_SS_CTRL")]
pub type CfgPll12SsCtrl = crate::Reg<cfg_pll12_ss_ctrl::CfgPll12SsCtrlSpec>;
#[doc = "CFG_pll12_SS_CTRL"]
pub mod cfg_pll12_ss_ctrl;
#[doc = "CFG_pll12_SS_SPREAD (rw) register accessor: CFG_pll12_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_ss_spread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_ss_spread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_ss_spread`]
module"]
#[doc(alias = "CFG_pll12_SS_SPREAD")]
pub type CfgPll12SsSpread = crate::Reg<cfg_pll12_ss_spread::CfgPll12SsSpreadSpec>;
#[doc = "CFG_pll12_SS_SPREAD"]
pub mod cfg_pll12_ss_spread;
#[doc = "CFG_pll12_CAL_CTRL (rw) register accessor: CFG_pll12_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_cal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_cal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_cal_ctrl`]
module"]
#[doc(alias = "CFG_pll12_CAL_CTRL")]
pub type CfgPll12CalCtrl = crate::Reg<cfg_pll12_cal_ctrl::CfgPll12CalCtrlSpec>;
#[doc = "CFG_pll12_CAL_CTRL"]
pub mod cfg_pll12_cal_ctrl;
#[doc = "CFG_pll12_CAL_STAT (rw) register accessor: CFG_pll12_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_cal_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_cal_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_cal_stat`]
module"]
#[doc(alias = "CFG_pll12_CAL_STAT")]
pub type CfgPll12CalStat = crate::Reg<cfg_pll12_cal_stat::CfgPll12CalStatSpec>;
#[doc = "CFG_pll12_CAL_STAT"]
pub mod cfg_pll12_cal_stat;
#[doc = "CFG_pll12_HSDIV_CTRL0 (rw) register accessor: CFG_pll12_HSDIV_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_hsdiv_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_hsdiv_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll12_hsdiv_ctrl0`]
module"]
#[doc(alias = "CFG_pll12_HSDIV_CTRL0")]
pub type CfgPll12HsdivCtrl0 = crate::Reg<cfg_pll12_hsdiv_ctrl0::CfgPll12HsdivCtrl0Spec>;
#[doc = "CFG_pll12_HSDIV_CTRL0"]
pub mod cfg_pll12_hsdiv_ctrl0;
#[doc = "CFG_pll14_PID (rw) register accessor: CFG_pll14_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_pid`]
module"]
#[doc(alias = "CFG_pll14_PID")]
pub type CfgPll14Pid = crate::Reg<cfg_pll14_pid::CfgPll14PidSpec>;
#[doc = "CFG_pll14_PID"]
pub mod cfg_pll14_pid;
#[doc = "CFG_pll14_CFG (rw) register accessor: CFG_pll14_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_cfg`]
module"]
#[doc(alias = "CFG_pll14_CFG")]
pub type CfgPll14Cfg = crate::Reg<cfg_pll14_cfg::CfgPll14CfgSpec>;
#[doc = "CFG_pll14_CFG"]
pub mod cfg_pll14_cfg;
#[doc = "CFG_pll14_LOCKKEY0 (rw) register accessor: CFG_pll14_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_lockkey0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_lockkey0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_lockkey0`]
module"]
#[doc(alias = "CFG_pll14_LOCKKEY0")]
pub type CfgPll14Lockkey0 = crate::Reg<cfg_pll14_lockkey0::CfgPll14Lockkey0Spec>;
#[doc = "CFG_pll14_LOCKKEY0"]
pub mod cfg_pll14_lockkey0;
#[doc = "CFG_pll14_LOCKKEY1 (rw) register accessor: CFG_pll14_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_lockkey1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_lockkey1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_lockkey1`]
module"]
#[doc(alias = "CFG_pll14_LOCKKEY1")]
pub type CfgPll14Lockkey1 = crate::Reg<cfg_pll14_lockkey1::CfgPll14Lockkey1Spec>;
#[doc = "CFG_pll14_LOCKKEY1"]
pub mod cfg_pll14_lockkey1;
#[doc = "CFG_pll14_CTRL (rw) register accessor: CFG_pll14_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_ctrl`]
module"]
#[doc(alias = "CFG_pll14_CTRL")]
pub type CfgPll14Ctrl = crate::Reg<cfg_pll14_ctrl::CfgPll14CtrlSpec>;
#[doc = "CFG_pll14_CTRL"]
pub mod cfg_pll14_ctrl;
#[doc = "CFG_pll14_STAT (rw) register accessor: CFG_pll14_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_stat`]
module"]
#[doc(alias = "CFG_pll14_STAT")]
pub type CfgPll14Stat = crate::Reg<cfg_pll14_stat::CfgPll14StatSpec>;
#[doc = "CFG_pll14_STAT"]
pub mod cfg_pll14_stat;
#[doc = "CFG_pll14_FREQ_CTRL0 (rw) register accessor: CFG_pll14_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_freq_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_freq_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_freq_ctrl0`]
module"]
#[doc(alias = "CFG_pll14_FREQ_CTRL0")]
pub type CfgPll14FreqCtrl0 = crate::Reg<cfg_pll14_freq_ctrl0::CfgPll14FreqCtrl0Spec>;
#[doc = "CFG_pll14_FREQ_CTRL0"]
pub mod cfg_pll14_freq_ctrl0;
#[doc = "CFG_pll14_FREQ_CTRL1 (rw) register accessor: CFG_pll14_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_freq_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_freq_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_freq_ctrl1`]
module"]
#[doc(alias = "CFG_pll14_FREQ_CTRL1")]
pub type CfgPll14FreqCtrl1 = crate::Reg<cfg_pll14_freq_ctrl1::CfgPll14FreqCtrl1Spec>;
#[doc = "CFG_pll14_FREQ_CTRL1"]
pub mod cfg_pll14_freq_ctrl1;
#[doc = "CFG_pll14_DIV_CTRL (rw) register accessor: CFG_pll14_DIV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_div_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_div_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_div_ctrl`]
module"]
#[doc(alias = "CFG_pll14_DIV_CTRL")]
pub type CfgPll14DivCtrl = crate::Reg<cfg_pll14_div_ctrl::CfgPll14DivCtrlSpec>;
#[doc = "CFG_pll14_DIV_CTRL"]
pub mod cfg_pll14_div_ctrl;
#[doc = "CFG_pll14_SS_CTRL (rw) register accessor: CFG_pll14_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_ss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_ss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_ss_ctrl`]
module"]
#[doc(alias = "CFG_pll14_SS_CTRL")]
pub type CfgPll14SsCtrl = crate::Reg<cfg_pll14_ss_ctrl::CfgPll14SsCtrlSpec>;
#[doc = "CFG_pll14_SS_CTRL"]
pub mod cfg_pll14_ss_ctrl;
#[doc = "CFG_pll14_SS_SPREAD (rw) register accessor: CFG_pll14_SS_SPREAD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_ss_spread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_ss_spread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_ss_spread`]
module"]
#[doc(alias = "CFG_pll14_SS_SPREAD")]
pub type CfgPll14SsSpread = crate::Reg<cfg_pll14_ss_spread::CfgPll14SsSpreadSpec>;
#[doc = "CFG_pll14_SS_SPREAD"]
pub mod cfg_pll14_ss_spread;
#[doc = "CFG_pll14_CAL_CTRL (rw) register accessor: CFG_pll14_CAL_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_cal_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_cal_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_cal_ctrl`]
module"]
#[doc(alias = "CFG_pll14_CAL_CTRL")]
pub type CfgPll14CalCtrl = crate::Reg<cfg_pll14_cal_ctrl::CfgPll14CalCtrlSpec>;
#[doc = "CFG_pll14_CAL_CTRL"]
pub mod cfg_pll14_cal_ctrl;
#[doc = "CFG_pll14_CAL_STAT (rw) register accessor: CFG_pll14_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_cal_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_cal_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_cal_stat`]
module"]
#[doc(alias = "CFG_pll14_CAL_STAT")]
pub type CfgPll14CalStat = crate::Reg<cfg_pll14_cal_stat::CfgPll14CalStatSpec>;
#[doc = "CFG_pll14_CAL_STAT"]
pub mod cfg_pll14_cal_stat;
#[doc = "CFG_pll14_HSDIV_CTRL0 (rw) register accessor: CFG_pll14_HSDIV_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_hsdiv_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_hsdiv_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_hsdiv_ctrl0`]
module"]
#[doc(alias = "CFG_pll14_HSDIV_CTRL0")]
pub type CfgPll14HsdivCtrl0 = crate::Reg<cfg_pll14_hsdiv_ctrl0::CfgPll14HsdivCtrl0Spec>;
#[doc = "CFG_pll14_HSDIV_CTRL0"]
pub mod cfg_pll14_hsdiv_ctrl0;
#[doc = "CFG_pll14_HSDIV_CTRL1 (rw) register accessor: CFG_pll14_HSDIV_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_hsdiv_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_hsdiv_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_pll14_hsdiv_ctrl1`]
module"]
#[doc(alias = "CFG_pll14_HSDIV_CTRL1")]
pub type CfgPll14HsdivCtrl1 = crate::Reg<cfg_pll14_hsdiv_ctrl1::CfgPll14HsdivCtrl1Spec>;
#[doc = "CFG_pll14_HSDIV_CTRL1"]
pub mod cfg_pll14_hsdiv_ctrl1;
