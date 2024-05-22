#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_iep1__slv__regs_global_cfg_reg: Pr1Iep1_Slv_RegsGlobalCfgReg,
    pr1_iep1__slv__regs_global_status_reg: Pr1Iep1_Slv_RegsGlobalStatusReg,
    pr1_iep1__slv__regs_compen_reg: Pr1Iep1_Slv_RegsCompenReg,
    pr1_iep1__slv__regs_slow_compen_reg: Pr1Iep1_Slv_RegsSlowCompenReg,
    pr1_iep1__slv__regs_count_reg0: Pr1Iep1_Slv_RegsCountReg0,
    pr1_iep1__slv__regs_count_reg1: Pr1Iep1_Slv_RegsCountReg1,
    pr1_iep1__slv__regs_cap_cfg_reg: Pr1Iep1_Slv_RegsCapCfgReg,
    pr1_iep1__slv__regs_cap_status_reg: Pr1Iep1_Slv_RegsCapStatusReg,
    pr1_iep1__slv__regs_capr0_reg0: Pr1Iep1_Slv_RegsCapr0Reg0,
    pr1_iep1__slv__regs_capr0_reg1: Pr1Iep1_Slv_RegsCapr0Reg1,
    pr1_iep1__slv__regs_capr1_reg0: Pr1Iep1_Slv_RegsCapr1Reg0,
    pr1_iep1__slv__regs_capr1_reg1: Pr1Iep1_Slv_RegsCapr1Reg1,
    pr1_iep1__slv__regs_capr2_reg0: Pr1Iep1_Slv_RegsCapr2Reg0,
    pr1_iep1__slv__regs_capr2_reg1: Pr1Iep1_Slv_RegsCapr2Reg1,
    pr1_iep1__slv__regs_capr3_reg0: Pr1Iep1_Slv_RegsCapr3Reg0,
    pr1_iep1__slv__regs_capr3_reg1: Pr1Iep1_Slv_RegsCapr3Reg1,
    pr1_iep1__slv__regs_capr4_reg0: Pr1Iep1_Slv_RegsCapr4Reg0,
    pr1_iep1__slv__regs_capr4_reg1: Pr1Iep1_Slv_RegsCapr4Reg1,
    pr1_iep1__slv__regs_capr5_reg0: Pr1Iep1_Slv_RegsCapr5Reg0,
    pr1_iep1__slv__regs_capr5_reg1: Pr1Iep1_Slv_RegsCapr5Reg1,
    pr1_iep1__slv__regs_capr6_reg0: Pr1Iep1_Slv_RegsCapr6Reg0,
    pr1_iep1__slv__regs_capr6_reg1: Pr1Iep1_Slv_RegsCapr6Reg1,
    pr1_iep1__slv__regs_capf6_reg0: Pr1Iep1_Slv_RegsCapf6Reg0,
    pr1_iep1__slv__regs_capf6_reg1: Pr1Iep1_Slv_RegsCapf6Reg1,
    pr1_iep1__slv__regs_capr7_reg0: Pr1Iep1_Slv_RegsCapr7Reg0,
    pr1_iep1__slv__regs_capr7_reg1: Pr1Iep1_Slv_RegsCapr7Reg1,
    pr1_iep1__slv__regs_capf7_reg0: Pr1Iep1_Slv_RegsCapf7Reg0,
    pr1_iep1__slv__regs_capf7_reg1: Pr1Iep1_Slv_RegsCapf7Reg1,
    pr1_iep1__slv__regs_cmp_cfg_reg: Pr1Iep1_Slv_RegsCmpCfgReg,
    pr1_iep1__slv__regs_cmp_status_reg: Pr1Iep1_Slv_RegsCmpStatusReg,
    pr1_iep1__slv__regs_cmp0_reg0: Pr1Iep1_Slv_RegsCmp0Reg0,
    pr1_iep1__slv__regs_cmp0_reg1: Pr1Iep1_Slv_RegsCmp0Reg1,
    pr1_iep1__slv__regs_cmp1_reg0: Pr1Iep1_Slv_RegsCmp1Reg0,
    pr1_iep1__slv__regs_cmp1_reg1: Pr1Iep1_Slv_RegsCmp1Reg1,
    pr1_iep1__slv__regs_cmp2_reg0: Pr1Iep1_Slv_RegsCmp2Reg0,
    pr1_iep1__slv__regs_cmp2_reg1: Pr1Iep1_Slv_RegsCmp2Reg1,
    pr1_iep1__slv__regs_cmp3_reg0: Pr1Iep1_Slv_RegsCmp3Reg0,
    pr1_iep1__slv__regs_cmp3_reg1: Pr1Iep1_Slv_RegsCmp3Reg1,
    pr1_iep1__slv__regs_cmp4_reg0: Pr1Iep1_Slv_RegsCmp4Reg0,
    pr1_iep1__slv__regs_cmp4_reg1: Pr1Iep1_Slv_RegsCmp4Reg1,
    pr1_iep1__slv__regs_cmp5_reg0: Pr1Iep1_Slv_RegsCmp5Reg0,
    pr1_iep1__slv__regs_cmp5_reg1: Pr1Iep1_Slv_RegsCmp5Reg1,
    pr1_iep1__slv__regs_cmp6_reg0: Pr1Iep1_Slv_RegsCmp6Reg0,
    pr1_iep1__slv__regs_cmp6_reg1: Pr1Iep1_Slv_RegsCmp6Reg1,
    pr1_iep1__slv__regs_cmp7_reg0: Pr1Iep1_Slv_RegsCmp7Reg0,
    pr1_iep1__slv__regs_cmp7_reg1: Pr1Iep1_Slv_RegsCmp7Reg1,
    pr1_iep1__slv__regs_rxipg0_reg: Pr1Iep1_Slv_RegsRxipg0Reg,
    pr1_iep1__slv__regs_rxipg1_reg: Pr1Iep1_Slv_RegsRxipg1Reg,
    pr1_iep1__slv__regs_cmp8_reg0: Pr1Iep1_Slv_RegsCmp8Reg0,
    pr1_iep1__slv__regs_cmp8_reg1: Pr1Iep1_Slv_RegsCmp8Reg1,
    pr1_iep1__slv__regs_cmp9_reg0: Pr1Iep1_Slv_RegsCmp9Reg0,
    pr1_iep1__slv__regs_cmp9_reg1: Pr1Iep1_Slv_RegsCmp9Reg1,
    pr1_iep1__slv__regs_cmp10_reg0: Pr1Iep1_Slv_RegsCmp10Reg0,
    pr1_iep1__slv__regs_cmp10_reg1: Pr1Iep1_Slv_RegsCmp10Reg1,
    pr1_iep1__slv__regs_cmp11_reg0: Pr1Iep1_Slv_RegsCmp11Reg0,
    pr1_iep1__slv__regs_cmp11_reg1: Pr1Iep1_Slv_RegsCmp11Reg1,
    pr1_iep1__slv__regs_cmp12_reg0: Pr1Iep1_Slv_RegsCmp12Reg0,
    pr1_iep1__slv__regs_cmp12_reg1: Pr1Iep1_Slv_RegsCmp12Reg1,
    pr1_iep1__slv__regs_cmp13_reg0: Pr1Iep1_Slv_RegsCmp13Reg0,
    pr1_iep1__slv__regs_cmp13_reg1: Pr1Iep1_Slv_RegsCmp13Reg1,
    pr1_iep1__slv__regs_cmp14_reg0: Pr1Iep1_Slv_RegsCmp14Reg0,
    pr1_iep1__slv__regs_cmp14_reg1: Pr1Iep1_Slv_RegsCmp14Reg1,
    pr1_iep1__slv__regs_cmp15_reg0: Pr1Iep1_Slv_RegsCmp15Reg0,
    pr1_iep1__slv__regs_cmp15_reg1: Pr1Iep1_Slv_RegsCmp15Reg1,
    pr1_iep1__slv__regs_count_reset_val_reg0: Pr1Iep1_Slv_RegsCountResetValReg0,
    pr1_iep1__slv__regs_count_reset_val_reg1: Pr1Iep1_Slv_RegsCountResetValReg1,
    pr1_iep1__slv__regs_pwm_reg: Pr1Iep1_Slv_RegsPwmReg,
    pr1_iep1__slv__regs_capr0_bi_reg0: Pr1Iep1_Slv_RegsCapr0BiReg0,
    pr1_iep1__slv__regs_capr0_bi_reg1: Pr1Iep1_Slv_RegsCapr0BiReg1,
    pr1_iep1__slv__regs_capr1_bi_reg0: Pr1Iep1_Slv_RegsCapr1BiReg0,
    pr1_iep1__slv__regs_capr1_bi_reg1: Pr1Iep1_Slv_RegsCapr1BiReg1,
    pr1_iep1__slv__regs_capr2_bi_reg0: Pr1Iep1_Slv_RegsCapr2BiReg0,
    pr1_iep1__slv__regs_capr2_bi_reg1: Pr1Iep1_Slv_RegsCapr2BiReg1,
    pr1_iep1__slv__regs_capr3_bi_reg0: Pr1Iep1_Slv_RegsCapr3BiReg0,
    pr1_iep1__slv__regs_capr3_bi_reg1: Pr1Iep1_Slv_RegsCapr3BiReg1,
    pr1_iep1__slv__regs_capr4_bi_reg0: Pr1Iep1_Slv_RegsCapr4BiReg0,
    pr1_iep1__slv__regs_capr4_bi_reg1: Pr1Iep1_Slv_RegsCapr4BiReg1,
    pr1_iep1__slv__regs_capr5_bi_reg0: Pr1Iep1_Slv_RegsCapr5BiReg0,
    pr1_iep1__slv__regs_capr5_bi_reg1: Pr1Iep1_Slv_RegsCapr5BiReg1,
    pr1_iep1__slv__regs_capr6_bi_reg0: Pr1Iep1_Slv_RegsCapr6BiReg0,
    pr1_iep1__slv__regs_capr6_bi_reg1: Pr1Iep1_Slv_RegsCapr6BiReg1,
    pr1_iep1__slv__regs_capf6_bi_reg0: Pr1Iep1_Slv_RegsCapf6BiReg0,
    pr1_iep1__slv__regs_capf6_bi_reg1: Pr1Iep1_Slv_RegsCapf6BiReg1,
    pr1_iep1__slv__regs_capr7_bi_reg0: Pr1Iep1_Slv_RegsCapr7BiReg0,
    pr1_iep1__slv__regs_capr7_bi_reg1: Pr1Iep1_Slv_RegsCapr7BiReg1,
    pr1_iep1__slv__regs_capf7_bi_reg0: Pr1Iep1_Slv_RegsCapf7BiReg0,
    pr1_iep1__slv__regs_capf7_bi_reg1: Pr1Iep1_Slv_RegsCapf7BiReg1,
    _reserved87: [u8; 0x24],
    pr1_iep1__slv__regs_sync_ctrl_reg: Pr1Iep1_Slv_RegsSyncCtrlReg,
    pr1_iep1__slv__regs_sync_first_stat_reg: Pr1Iep1_Slv_RegsSyncFirstStatReg,
    pr1_iep1__slv__regs_sync0_stat_reg: Pr1Iep1_Slv_RegsSync0StatReg,
    pr1_iep1__slv__regs_sync1_stat_reg: Pr1Iep1_Slv_RegsSync1StatReg,
    pr1_iep1__slv__regs_sync_pwidth_reg: Pr1Iep1_Slv_RegsSyncPwidthReg,
    pr1_iep1__slv__regs_sync0_period_reg: Pr1Iep1_Slv_RegsSync0PeriodReg,
    pr1_iep1__slv__regs_sync1_delay_reg: Pr1Iep1_Slv_RegsSync1DelayReg,
    pr1_iep1__slv__regs_sync_start_reg: Pr1Iep1_Slv_RegsSyncStartReg,
    _reserved95: [u8; 0x60],
    pr1_iep1__slv__regs_wd_prediv_reg: Pr1Iep1_Slv_RegsWdPredivReg,
    pr1_iep1__slv__regs_pdi_wd_tim_reg: Pr1Iep1_Slv_RegsPdiWdTimReg,
    pr1_iep1__slv__regs_pd_wd_tim_reg: Pr1Iep1_Slv_RegsPdWdTimReg,
    pr1_iep1__slv__regs_wd_status_reg: Pr1Iep1_Slv_RegsWdStatusReg,
    pr1_iep1__slv__regs_wd_exp_cnt_reg: Pr1Iep1_Slv_RegsWdExpCntReg,
    pr1_iep1__slv__regs_wd_ctrl_reg: Pr1Iep1_Slv_RegsWdCtrlReg,
    _reserved101: [u8; 0xe8],
    pr1_iep1__slv__regs_digio_ctrl_reg: Pr1Iep1_Slv_RegsDigioCtrlReg,
    pr1_iep1__slv__regs_digio_status_reg: Pr1Iep1_Slv_RegsDigioStatusReg,
    pr1_iep1__slv__regs_digio_data_in_reg: Pr1Iep1_Slv_RegsDigioDataInReg,
    pr1_iep1__slv__regs_digio_data_in_raw_reg: Pr1Iep1_Slv_RegsDigioDataInRawReg,
    pr1_iep1__slv__regs_digio_data_out_reg: Pr1Iep1_Slv_RegsDigioDataOutReg,
    pr1_iep1__slv__regs_digio_data_out_en_reg: Pr1Iep1_Slv_RegsDigioDataOutEnReg,
    pr1_iep1__slv__regs_digio_exp_reg: Pr1Iep1_Slv_RegsDigioExpReg,
}
impl RegisterBlock {
    #[doc = "0x00 - PR1_IEP1__SLV__REGS_global_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_global_cfg_reg(&self) -> &Pr1Iep1_Slv_RegsGlobalCfgReg {
        &self.pr1_iep1__slv__regs_global_cfg_reg
    }
    #[doc = "0x04 - PR1_IEP1__SLV__REGS_global_status_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_global_status_reg(&self) -> &Pr1Iep1_Slv_RegsGlobalStatusReg {
        &self.pr1_iep1__slv__regs_global_status_reg
    }
    #[doc = "0x08 - PR1_IEP1__SLV__REGS_compen_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_compen_reg(&self) -> &Pr1Iep1_Slv_RegsCompenReg {
        &self.pr1_iep1__slv__regs_compen_reg
    }
    #[doc = "0x0c - PR1_IEP1__SLV__REGS_slow_compen_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_slow_compen_reg(&self) -> &Pr1Iep1_Slv_RegsSlowCompenReg {
        &self.pr1_iep1__slv__regs_slow_compen_reg
    }
    #[doc = "0x10 - PR1_IEP1__SLV__REGS_count_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_count_reg0(&self) -> &Pr1Iep1_Slv_RegsCountReg0 {
        &self.pr1_iep1__slv__regs_count_reg0
    }
    #[doc = "0x14 - PR1_IEP1__SLV__REGS_count_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_count_reg1(&self) -> &Pr1Iep1_Slv_RegsCountReg1 {
        &self.pr1_iep1__slv__regs_count_reg1
    }
    #[doc = "0x18 - PR1_IEP1__SLV__REGS_cap_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cap_cfg_reg(&self) -> &Pr1Iep1_Slv_RegsCapCfgReg {
        &self.pr1_iep1__slv__regs_cap_cfg_reg
    }
    #[doc = "0x1c - PR1_IEP1__SLV__REGS_cap_status_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cap_status_reg(&self) -> &Pr1Iep1_Slv_RegsCapStatusReg {
        &self.pr1_iep1__slv__regs_cap_status_reg
    }
    #[doc = "0x20 - PR1_IEP1__SLV__REGS_capr0_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr0_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr0Reg0 {
        &self.pr1_iep1__slv__regs_capr0_reg0
    }
    #[doc = "0x24 - PR1_IEP1__SLV__REGS_capr0_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr0_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr0Reg1 {
        &self.pr1_iep1__slv__regs_capr0_reg1
    }
    #[doc = "0x28 - PR1_IEP1__SLV__REGS_capr1_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr1_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr1Reg0 {
        &self.pr1_iep1__slv__regs_capr1_reg0
    }
    #[doc = "0x2c - PR1_IEP1__SLV__REGS_capr1_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr1_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr1Reg1 {
        &self.pr1_iep1__slv__regs_capr1_reg1
    }
    #[doc = "0x30 - PR1_IEP1__SLV__REGS_capr2_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr2_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr2Reg0 {
        &self.pr1_iep1__slv__regs_capr2_reg0
    }
    #[doc = "0x34 - PR1_IEP1__SLV__REGS_capr2_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr2_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr2Reg1 {
        &self.pr1_iep1__slv__regs_capr2_reg1
    }
    #[doc = "0x38 - PR1_IEP1__SLV__REGS_capr3_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr3_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr3Reg0 {
        &self.pr1_iep1__slv__regs_capr3_reg0
    }
    #[doc = "0x3c - PR1_IEP1__SLV__REGS_capr3_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr3_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr3Reg1 {
        &self.pr1_iep1__slv__regs_capr3_reg1
    }
    #[doc = "0x40 - PR1_IEP1__SLV__REGS_capr4_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr4_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr4Reg0 {
        &self.pr1_iep1__slv__regs_capr4_reg0
    }
    #[doc = "0x44 - PR1_IEP1__SLV__REGS_capr4_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr4_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr4Reg1 {
        &self.pr1_iep1__slv__regs_capr4_reg1
    }
    #[doc = "0x48 - PR1_IEP1__SLV__REGS_capr5_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr5_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr5Reg0 {
        &self.pr1_iep1__slv__regs_capr5_reg0
    }
    #[doc = "0x4c - PR1_IEP1__SLV__REGS_capr5_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr5_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr5Reg1 {
        &self.pr1_iep1__slv__regs_capr5_reg1
    }
    #[doc = "0x50 - PR1_IEP1__SLV__REGS_capr6_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr6_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr6Reg0 {
        &self.pr1_iep1__slv__regs_capr6_reg0
    }
    #[doc = "0x54 - PR1_IEP1__SLV__REGS_capr6_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr6_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr6Reg1 {
        &self.pr1_iep1__slv__regs_capr6_reg1
    }
    #[doc = "0x58 - PR1_IEP1__SLV__REGS_capf6_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf6_reg0(&self) -> &Pr1Iep1_Slv_RegsCapf6Reg0 {
        &self.pr1_iep1__slv__regs_capf6_reg0
    }
    #[doc = "0x5c - PR1_IEP1__SLV__REGS_capf6_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf6_reg1(&self) -> &Pr1Iep1_Slv_RegsCapf6Reg1 {
        &self.pr1_iep1__slv__regs_capf6_reg1
    }
    #[doc = "0x60 - PR1_IEP1__SLV__REGS_capr7_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr7_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr7Reg0 {
        &self.pr1_iep1__slv__regs_capr7_reg0
    }
    #[doc = "0x64 - PR1_IEP1__SLV__REGS_capr7_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr7_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr7Reg1 {
        &self.pr1_iep1__slv__regs_capr7_reg1
    }
    #[doc = "0x68 - PR1_IEP1__SLV__REGS_capf7_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf7_reg0(&self) -> &Pr1Iep1_Slv_RegsCapf7Reg0 {
        &self.pr1_iep1__slv__regs_capf7_reg0
    }
    #[doc = "0x6c - PR1_IEP1__SLV__REGS_capf7_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf7_reg1(&self) -> &Pr1Iep1_Slv_RegsCapf7Reg1 {
        &self.pr1_iep1__slv__regs_capf7_reg1
    }
    #[doc = "0x70 - PR1_IEP1__SLV__REGS_cmp_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp_cfg_reg(&self) -> &Pr1Iep1_Slv_RegsCmpCfgReg {
        &self.pr1_iep1__slv__regs_cmp_cfg_reg
    }
    #[doc = "0x74 - PR1_IEP1__SLV__REGS_cmp_status_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp_status_reg(&self) -> &Pr1Iep1_Slv_RegsCmpStatusReg {
        &self.pr1_iep1__slv__regs_cmp_status_reg
    }
    #[doc = "0x78 - PR1_IEP1__SLV__REGS_cmp0_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp0_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp0Reg0 {
        &self.pr1_iep1__slv__regs_cmp0_reg0
    }
    #[doc = "0x7c - PR1_IEP1__SLV__REGS_cmp0_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp0_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp0Reg1 {
        &self.pr1_iep1__slv__regs_cmp0_reg1
    }
    #[doc = "0x80 - PR1_IEP1__SLV__REGS_cmp1_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp1_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp1Reg0 {
        &self.pr1_iep1__slv__regs_cmp1_reg0
    }
    #[doc = "0x84 - PR1_IEP1__SLV__REGS_cmp1_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp1_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp1Reg1 {
        &self.pr1_iep1__slv__regs_cmp1_reg1
    }
    #[doc = "0x88 - PR1_IEP1__SLV__REGS_cmp2_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp2_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp2Reg0 {
        &self.pr1_iep1__slv__regs_cmp2_reg0
    }
    #[doc = "0x8c - PR1_IEP1__SLV__REGS_cmp2_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp2_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp2Reg1 {
        &self.pr1_iep1__slv__regs_cmp2_reg1
    }
    #[doc = "0x90 - PR1_IEP1__SLV__REGS_cmp3_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp3_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp3Reg0 {
        &self.pr1_iep1__slv__regs_cmp3_reg0
    }
    #[doc = "0x94 - PR1_IEP1__SLV__REGS_cmp3_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp3_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp3Reg1 {
        &self.pr1_iep1__slv__regs_cmp3_reg1
    }
    #[doc = "0x98 - PR1_IEP1__SLV__REGS_cmp4_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp4_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp4Reg0 {
        &self.pr1_iep1__slv__regs_cmp4_reg0
    }
    #[doc = "0x9c - PR1_IEP1__SLV__REGS_cmp4_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp4_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp4Reg1 {
        &self.pr1_iep1__slv__regs_cmp4_reg1
    }
    #[doc = "0xa0 - PR1_IEP1__SLV__REGS_cmp5_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp5_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp5Reg0 {
        &self.pr1_iep1__slv__regs_cmp5_reg0
    }
    #[doc = "0xa4 - PR1_IEP1__SLV__REGS_cmp5_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp5_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp5Reg1 {
        &self.pr1_iep1__slv__regs_cmp5_reg1
    }
    #[doc = "0xa8 - PR1_IEP1__SLV__REGS_cmp6_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp6_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp6Reg0 {
        &self.pr1_iep1__slv__regs_cmp6_reg0
    }
    #[doc = "0xac - PR1_IEP1__SLV__REGS_cmp6_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp6_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp6Reg1 {
        &self.pr1_iep1__slv__regs_cmp6_reg1
    }
    #[doc = "0xb0 - PR1_IEP1__SLV__REGS_cmp7_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp7_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp7Reg0 {
        &self.pr1_iep1__slv__regs_cmp7_reg0
    }
    #[doc = "0xb4 - PR1_IEP1__SLV__REGS_cmp7_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp7_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp7Reg1 {
        &self.pr1_iep1__slv__regs_cmp7_reg1
    }
    #[doc = "0xb8 - PR1_IEP1__SLV__REGS_rxipg0_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_rxipg0_reg(&self) -> &Pr1Iep1_Slv_RegsRxipg0Reg {
        &self.pr1_iep1__slv__regs_rxipg0_reg
    }
    #[doc = "0xbc - PR1_IEP1__SLV__REGS_rxipg1_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_rxipg1_reg(&self) -> &Pr1Iep1_Slv_RegsRxipg1Reg {
        &self.pr1_iep1__slv__regs_rxipg1_reg
    }
    #[doc = "0xc0 - PR1_IEP1__SLV__REGS_cmp8_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp8_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp8Reg0 {
        &self.pr1_iep1__slv__regs_cmp8_reg0
    }
    #[doc = "0xc4 - PR1_IEP1__SLV__REGS_cmp8_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp8_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp8Reg1 {
        &self.pr1_iep1__slv__regs_cmp8_reg1
    }
    #[doc = "0xc8 - PR1_IEP1__SLV__REGS_cmp9_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp9_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp9Reg0 {
        &self.pr1_iep1__slv__regs_cmp9_reg0
    }
    #[doc = "0xcc - PR1_IEP1__SLV__REGS_cmp9_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp9_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp9Reg1 {
        &self.pr1_iep1__slv__regs_cmp9_reg1
    }
    #[doc = "0xd0 - PR1_IEP1__SLV__REGS_cmp10_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp10_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp10Reg0 {
        &self.pr1_iep1__slv__regs_cmp10_reg0
    }
    #[doc = "0xd4 - PR1_IEP1__SLV__REGS_cmp10_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp10_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp10Reg1 {
        &self.pr1_iep1__slv__regs_cmp10_reg1
    }
    #[doc = "0xd8 - PR1_IEP1__SLV__REGS_cmp11_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp11_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp11Reg0 {
        &self.pr1_iep1__slv__regs_cmp11_reg0
    }
    #[doc = "0xdc - PR1_IEP1__SLV__REGS_cmp11_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp11_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp11Reg1 {
        &self.pr1_iep1__slv__regs_cmp11_reg1
    }
    #[doc = "0xe0 - PR1_IEP1__SLV__REGS_cmp12_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp12_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp12Reg0 {
        &self.pr1_iep1__slv__regs_cmp12_reg0
    }
    #[doc = "0xe4 - PR1_IEP1__SLV__REGS_cmp12_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp12_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp12Reg1 {
        &self.pr1_iep1__slv__regs_cmp12_reg1
    }
    #[doc = "0xe8 - PR1_IEP1__SLV__REGS_cmp13_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp13_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp13Reg0 {
        &self.pr1_iep1__slv__regs_cmp13_reg0
    }
    #[doc = "0xec - PR1_IEP1__SLV__REGS_cmp13_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp13_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp13Reg1 {
        &self.pr1_iep1__slv__regs_cmp13_reg1
    }
    #[doc = "0xf0 - PR1_IEP1__SLV__REGS_cmp14_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp14_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp14Reg0 {
        &self.pr1_iep1__slv__regs_cmp14_reg0
    }
    #[doc = "0xf4 - PR1_IEP1__SLV__REGS_cmp14_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp14_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp14Reg1 {
        &self.pr1_iep1__slv__regs_cmp14_reg1
    }
    #[doc = "0xf8 - PR1_IEP1__SLV__REGS_cmp15_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp15_reg0(&self) -> &Pr1Iep1_Slv_RegsCmp15Reg0 {
        &self.pr1_iep1__slv__regs_cmp15_reg0
    }
    #[doc = "0xfc - PR1_IEP1__SLV__REGS_cmp15_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_cmp15_reg1(&self) -> &Pr1Iep1_Slv_RegsCmp15Reg1 {
        &self.pr1_iep1__slv__regs_cmp15_reg1
    }
    #[doc = "0x100 - PR1_IEP1__SLV__REGS_count_reset_val_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_count_reset_val_reg0(
        &self,
    ) -> &Pr1Iep1_Slv_RegsCountResetValReg0 {
        &self.pr1_iep1__slv__regs_count_reset_val_reg0
    }
    #[doc = "0x104 - PR1_IEP1__SLV__REGS_count_reset_val_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_count_reset_val_reg1(
        &self,
    ) -> &Pr1Iep1_Slv_RegsCountResetValReg1 {
        &self.pr1_iep1__slv__regs_count_reset_val_reg1
    }
    #[doc = "0x108 - PR1_IEP1__SLV__REGS_pwm_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_pwm_reg(&self) -> &Pr1Iep1_Slv_RegsPwmReg {
        &self.pr1_iep1__slv__regs_pwm_reg
    }
    #[doc = "0x10c - PR1_IEP1__SLV__REGS_capr0_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr0_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr0BiReg0 {
        &self.pr1_iep1__slv__regs_capr0_bi_reg0
    }
    #[doc = "0x110 - PR1_IEP1__SLV__REGS_capr0_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr0_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr0BiReg1 {
        &self.pr1_iep1__slv__regs_capr0_bi_reg1
    }
    #[doc = "0x114 - PR1_IEP1__SLV__REGS_capr1_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr1_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr1BiReg0 {
        &self.pr1_iep1__slv__regs_capr1_bi_reg0
    }
    #[doc = "0x118 - PR1_IEP1__SLV__REGS_capr1_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr1_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr1BiReg1 {
        &self.pr1_iep1__slv__regs_capr1_bi_reg1
    }
    #[doc = "0x11c - PR1_IEP1__SLV__REGS_capr2_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr2_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr2BiReg0 {
        &self.pr1_iep1__slv__regs_capr2_bi_reg0
    }
    #[doc = "0x120 - PR1_IEP1__SLV__REGS_capr2_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr2_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr2BiReg1 {
        &self.pr1_iep1__slv__regs_capr2_bi_reg1
    }
    #[doc = "0x124 - PR1_IEP1__SLV__REGS_capr3_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr3_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr3BiReg0 {
        &self.pr1_iep1__slv__regs_capr3_bi_reg0
    }
    #[doc = "0x128 - PR1_IEP1__SLV__REGS_capr3_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr3_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr3BiReg1 {
        &self.pr1_iep1__slv__regs_capr3_bi_reg1
    }
    #[doc = "0x12c - PR1_IEP1__SLV__REGS_capr4_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr4_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr4BiReg0 {
        &self.pr1_iep1__slv__regs_capr4_bi_reg0
    }
    #[doc = "0x130 - PR1_IEP1__SLV__REGS_capr4_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr4_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr4BiReg1 {
        &self.pr1_iep1__slv__regs_capr4_bi_reg1
    }
    #[doc = "0x134 - PR1_IEP1__SLV__REGS_capr5_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr5_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr5BiReg0 {
        &self.pr1_iep1__slv__regs_capr5_bi_reg0
    }
    #[doc = "0x138 - PR1_IEP1__SLV__REGS_capr5_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr5_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr5BiReg1 {
        &self.pr1_iep1__slv__regs_capr5_bi_reg1
    }
    #[doc = "0x13c - PR1_IEP1__SLV__REGS_capr6_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr6_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr6BiReg0 {
        &self.pr1_iep1__slv__regs_capr6_bi_reg0
    }
    #[doc = "0x140 - PR1_IEP1__SLV__REGS_capr6_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr6_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr6BiReg1 {
        &self.pr1_iep1__slv__regs_capr6_bi_reg1
    }
    #[doc = "0x144 - PR1_IEP1__SLV__REGS_capf6_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf6_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapf6BiReg0 {
        &self.pr1_iep1__slv__regs_capf6_bi_reg0
    }
    #[doc = "0x148 - PR1_IEP1__SLV__REGS_capf6_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf6_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapf6BiReg1 {
        &self.pr1_iep1__slv__regs_capf6_bi_reg1
    }
    #[doc = "0x14c - PR1_IEP1__SLV__REGS_capr7_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr7_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapr7BiReg0 {
        &self.pr1_iep1__slv__regs_capr7_bi_reg0
    }
    #[doc = "0x150 - PR1_IEP1__SLV__REGS_capr7_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capr7_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapr7BiReg1 {
        &self.pr1_iep1__slv__regs_capr7_bi_reg1
    }
    #[doc = "0x154 - PR1_IEP1__SLV__REGS_capf7_bi_reg0"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf7_bi_reg0(&self) -> &Pr1Iep1_Slv_RegsCapf7BiReg0 {
        &self.pr1_iep1__slv__regs_capf7_bi_reg0
    }
    #[doc = "0x158 - PR1_IEP1__SLV__REGS_capf7_bi_reg1"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_capf7_bi_reg1(&self) -> &Pr1Iep1_Slv_RegsCapf7BiReg1 {
        &self.pr1_iep1__slv__regs_capf7_bi_reg1
    }
    #[doc = "0x180 - PR1_IEP1__SLV__REGS_sync_ctrl_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync_ctrl_reg(&self) -> &Pr1Iep1_Slv_RegsSyncCtrlReg {
        &self.pr1_iep1__slv__regs_sync_ctrl_reg
    }
    #[doc = "0x184 - PR1_IEP1__SLV__REGS_sync_first_stat_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync_first_stat_reg(
        &self,
    ) -> &Pr1Iep1_Slv_RegsSyncFirstStatReg {
        &self.pr1_iep1__slv__regs_sync_first_stat_reg
    }
    #[doc = "0x188 - PR1_IEP1__SLV__REGS_sync0_stat_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync0_stat_reg(&self) -> &Pr1Iep1_Slv_RegsSync0StatReg {
        &self.pr1_iep1__slv__regs_sync0_stat_reg
    }
    #[doc = "0x18c - PR1_IEP1__SLV__REGS_sync1_stat_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync1_stat_reg(&self) -> &Pr1Iep1_Slv_RegsSync1StatReg {
        &self.pr1_iep1__slv__regs_sync1_stat_reg
    }
    #[doc = "0x190 - PR1_IEP1__SLV__REGS_sync_pwidth_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync_pwidth_reg(&self) -> &Pr1Iep1_Slv_RegsSyncPwidthReg {
        &self.pr1_iep1__slv__regs_sync_pwidth_reg
    }
    #[doc = "0x194 - PR1_IEP1__SLV__REGS_sync0_period_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync0_period_reg(&self) -> &Pr1Iep1_Slv_RegsSync0PeriodReg {
        &self.pr1_iep1__slv__regs_sync0_period_reg
    }
    #[doc = "0x198 - PR1_IEP1__SLV__REGS_sync1_delay_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync1_delay_reg(&self) -> &Pr1Iep1_Slv_RegsSync1DelayReg {
        &self.pr1_iep1__slv__regs_sync1_delay_reg
    }
    #[doc = "0x19c - PR1_IEP1__SLV__REGS_sync_start_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_sync_start_reg(&self) -> &Pr1Iep1_Slv_RegsSyncStartReg {
        &self.pr1_iep1__slv__regs_sync_start_reg
    }
    #[doc = "0x200 - PR1_IEP1__SLV__REGS_wd_prediv_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_wd_prediv_reg(&self) -> &Pr1Iep1_Slv_RegsWdPredivReg {
        &self.pr1_iep1__slv__regs_wd_prediv_reg
    }
    #[doc = "0x204 - PR1_IEP1__SLV__REGS_pdi_wd_tim_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_pdi_wd_tim_reg(&self) -> &Pr1Iep1_Slv_RegsPdiWdTimReg {
        &self.pr1_iep1__slv__regs_pdi_wd_tim_reg
    }
    #[doc = "0x208 - PR1_IEP1__SLV__REGS_pd_wd_tim_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_pd_wd_tim_reg(&self) -> &Pr1Iep1_Slv_RegsPdWdTimReg {
        &self.pr1_iep1__slv__regs_pd_wd_tim_reg
    }
    #[doc = "0x20c - PR1_IEP1__SLV__REGS_wd_status_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_wd_status_reg(&self) -> &Pr1Iep1_Slv_RegsWdStatusReg {
        &self.pr1_iep1__slv__regs_wd_status_reg
    }
    #[doc = "0x210 - PR1_IEP1__SLV__REGS_wd_exp_cnt_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_wd_exp_cnt_reg(&self) -> &Pr1Iep1_Slv_RegsWdExpCntReg {
        &self.pr1_iep1__slv__regs_wd_exp_cnt_reg
    }
    #[doc = "0x214 - PR1_IEP1__SLV__REGS_wd_ctrl_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_wd_ctrl_reg(&self) -> &Pr1Iep1_Slv_RegsWdCtrlReg {
        &self.pr1_iep1__slv__regs_wd_ctrl_reg
    }
    #[doc = "0x300 - PR1_IEP1__SLV__REGS_digio_ctrl_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_ctrl_reg(&self) -> &Pr1Iep1_Slv_RegsDigioCtrlReg {
        &self.pr1_iep1__slv__regs_digio_ctrl_reg
    }
    #[doc = "0x304 - PR1_IEP1__SLV__REGS_digio_status_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_status_reg(&self) -> &Pr1Iep1_Slv_RegsDigioStatusReg {
        &self.pr1_iep1__slv__regs_digio_status_reg
    }
    #[doc = "0x308 - PR1_IEP1__SLV__REGS_digio_data_in_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_data_in_reg(&self) -> &Pr1Iep1_Slv_RegsDigioDataInReg {
        &self.pr1_iep1__slv__regs_digio_data_in_reg
    }
    #[doc = "0x30c - PR1_IEP1__SLV__REGS_digio_data_in_raw_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_data_in_raw_reg(
        &self,
    ) -> &Pr1Iep1_Slv_RegsDigioDataInRawReg {
        &self.pr1_iep1__slv__regs_digio_data_in_raw_reg
    }
    #[doc = "0x310 - PR1_IEP1__SLV__REGS_digio_data_out_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_data_out_reg(&self) -> &Pr1Iep1_Slv_RegsDigioDataOutReg {
        &self.pr1_iep1__slv__regs_digio_data_out_reg
    }
    #[doc = "0x314 - PR1_IEP1__SLV__REGS_digio_data_out_en_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_data_out_en_reg(
        &self,
    ) -> &Pr1Iep1_Slv_RegsDigioDataOutEnReg {
        &self.pr1_iep1__slv__regs_digio_data_out_en_reg
    }
    #[doc = "0x318 - PR1_IEP1__SLV__REGS_digio_exp_reg"]
    #[inline(always)]
    pub const fn pr1_iep1__slv__regs_digio_exp_reg(&self) -> &Pr1Iep1_Slv_RegsDigioExpReg {
        &self.pr1_iep1__slv__regs_digio_exp_reg
    }
}
#[doc = "PR1_IEP1__SLV__REGS_global_cfg_reg (rw) register accessor: PR1_IEP1__SLV__REGS_global_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_global_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_global_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_global_cfg_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_global_cfg_reg")]
pub type Pr1Iep1_Slv_RegsGlobalCfgReg =
    crate::Reg<pr1_iep1__slv__regs_global_cfg_reg::Pr1Iep1_Slv_RegsGlobalCfgRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_global_cfg_reg"]
pub mod pr1_iep1__slv__regs_global_cfg_reg;
#[doc = "PR1_IEP1__SLV__REGS_global_status_reg (rw) register accessor: PR1_IEP1__SLV__REGS_global_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_global_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_global_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_global_status_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_global_status_reg")]
pub type Pr1Iep1_Slv_RegsGlobalStatusReg =
    crate::Reg<pr1_iep1__slv__regs_global_status_reg::Pr1Iep1_Slv_RegsGlobalStatusRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_global_status_reg"]
pub mod pr1_iep1__slv__regs_global_status_reg;
#[doc = "PR1_IEP1__SLV__REGS_compen_reg (rw) register accessor: PR1_IEP1__SLV__REGS_compen_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_compen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_compen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_compen_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_compen_reg")]
pub type Pr1Iep1_Slv_RegsCompenReg =
    crate::Reg<pr1_iep1__slv__regs_compen_reg::Pr1Iep1_Slv_RegsCompenRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_compen_reg"]
pub mod pr1_iep1__slv__regs_compen_reg;
#[doc = "PR1_IEP1__SLV__REGS_slow_compen_reg (rw) register accessor: PR1_IEP1__SLV__REGS_slow_compen_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_slow_compen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_slow_compen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_slow_compen_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_slow_compen_reg")]
pub type Pr1Iep1_Slv_RegsSlowCompenReg =
    crate::Reg<pr1_iep1__slv__regs_slow_compen_reg::Pr1Iep1_Slv_RegsSlowCompenRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_slow_compen_reg"]
pub mod pr1_iep1__slv__regs_slow_compen_reg;
#[doc = "PR1_IEP1__SLV__REGS_count_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_count_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_count_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_count_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_count_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_count_reg0")]
pub type Pr1Iep1_Slv_RegsCountReg0 =
    crate::Reg<pr1_iep1__slv__regs_count_reg0::Pr1Iep1_Slv_RegsCountReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_count_reg0"]
pub mod pr1_iep1__slv__regs_count_reg0;
#[doc = "PR1_IEP1__SLV__REGS_count_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_count_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_count_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_count_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_count_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_count_reg1")]
pub type Pr1Iep1_Slv_RegsCountReg1 =
    crate::Reg<pr1_iep1__slv__regs_count_reg1::Pr1Iep1_Slv_RegsCountReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_count_reg1"]
pub mod pr1_iep1__slv__regs_count_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cap_cfg_reg (rw) register accessor: PR1_IEP1__SLV__REGS_cap_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cap_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cap_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cap_cfg_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cap_cfg_reg")]
pub type Pr1Iep1_Slv_RegsCapCfgReg =
    crate::Reg<pr1_iep1__slv__regs_cap_cfg_reg::Pr1Iep1_Slv_RegsCapCfgRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_cap_cfg_reg"]
pub mod pr1_iep1__slv__regs_cap_cfg_reg;
#[doc = "PR1_IEP1__SLV__REGS_cap_status_reg (rw) register accessor: PR1_IEP1__SLV__REGS_cap_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cap_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cap_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cap_status_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cap_status_reg")]
pub type Pr1Iep1_Slv_RegsCapStatusReg =
    crate::Reg<pr1_iep1__slv__regs_cap_status_reg::Pr1Iep1_Slv_RegsCapStatusRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_cap_status_reg"]
pub mod pr1_iep1__slv__regs_cap_status_reg;
#[doc = "PR1_IEP1__SLV__REGS_capr0_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr0_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr0_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr0_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr0_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr0_reg0")]
pub type Pr1Iep1_Slv_RegsCapr0Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr0_reg0::Pr1Iep1_Slv_RegsCapr0Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr0_reg0"]
pub mod pr1_iep1__slv__regs_capr0_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr0_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr0_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr0_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr0_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr0_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr0_reg1")]
pub type Pr1Iep1_Slv_RegsCapr0Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr0_reg1::Pr1Iep1_Slv_RegsCapr0Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr0_reg1"]
pub mod pr1_iep1__slv__regs_capr0_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr1_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr1_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr1_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr1_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr1_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr1_reg0")]
pub type Pr1Iep1_Slv_RegsCapr1Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr1_reg0::Pr1Iep1_Slv_RegsCapr1Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr1_reg0"]
pub mod pr1_iep1__slv__regs_capr1_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr1_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr1_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr1_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr1_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr1_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr1_reg1")]
pub type Pr1Iep1_Slv_RegsCapr1Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr1_reg1::Pr1Iep1_Slv_RegsCapr1Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr1_reg1"]
pub mod pr1_iep1__slv__regs_capr1_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr2_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr2_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr2_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr2_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr2_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr2_reg0")]
pub type Pr1Iep1_Slv_RegsCapr2Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr2_reg0::Pr1Iep1_Slv_RegsCapr2Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr2_reg0"]
pub mod pr1_iep1__slv__regs_capr2_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr2_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr2_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr2_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr2_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr2_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr2_reg1")]
pub type Pr1Iep1_Slv_RegsCapr2Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr2_reg1::Pr1Iep1_Slv_RegsCapr2Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr2_reg1"]
pub mod pr1_iep1__slv__regs_capr2_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr3_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr3_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr3_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr3_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr3_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr3_reg0")]
pub type Pr1Iep1_Slv_RegsCapr3Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr3_reg0::Pr1Iep1_Slv_RegsCapr3Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr3_reg0"]
pub mod pr1_iep1__slv__regs_capr3_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr3_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr3_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr3_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr3_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr3_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr3_reg1")]
pub type Pr1Iep1_Slv_RegsCapr3Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr3_reg1::Pr1Iep1_Slv_RegsCapr3Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr3_reg1"]
pub mod pr1_iep1__slv__regs_capr3_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr4_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr4_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr4_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr4_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr4_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr4_reg0")]
pub type Pr1Iep1_Slv_RegsCapr4Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr4_reg0::Pr1Iep1_Slv_RegsCapr4Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr4_reg0"]
pub mod pr1_iep1__slv__regs_capr4_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr4_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr4_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr4_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr4_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr4_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr4_reg1")]
pub type Pr1Iep1_Slv_RegsCapr4Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr4_reg1::Pr1Iep1_Slv_RegsCapr4Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr4_reg1"]
pub mod pr1_iep1__slv__regs_capr4_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr5_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr5_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr5_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr5_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr5_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr5_reg0")]
pub type Pr1Iep1_Slv_RegsCapr5Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr5_reg0::Pr1Iep1_Slv_RegsCapr5Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr5_reg0"]
pub mod pr1_iep1__slv__regs_capr5_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr5_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr5_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr5_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr5_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr5_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr5_reg1")]
pub type Pr1Iep1_Slv_RegsCapr5Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr5_reg1::Pr1Iep1_Slv_RegsCapr5Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr5_reg1"]
pub mod pr1_iep1__slv__regs_capr5_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr6_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr6_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr6_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr6_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr6_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr6_reg0")]
pub type Pr1Iep1_Slv_RegsCapr6Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr6_reg0::Pr1Iep1_Slv_RegsCapr6Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr6_reg0"]
pub mod pr1_iep1__slv__regs_capr6_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr6_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr6_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr6_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr6_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr6_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr6_reg1")]
pub type Pr1Iep1_Slv_RegsCapr6Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr6_reg1::Pr1Iep1_Slv_RegsCapr6Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr6_reg1"]
pub mod pr1_iep1__slv__regs_capr6_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capf6_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capf6_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf6_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf6_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf6_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf6_reg0")]
pub type Pr1Iep1_Slv_RegsCapf6Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capf6_reg0::Pr1Iep1_Slv_RegsCapf6Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf6_reg0"]
pub mod pr1_iep1__slv__regs_capf6_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capf6_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capf6_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf6_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf6_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf6_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf6_reg1")]
pub type Pr1Iep1_Slv_RegsCapf6Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capf6_reg1::Pr1Iep1_Slv_RegsCapf6Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf6_reg1"]
pub mod pr1_iep1__slv__regs_capf6_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr7_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr7_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr7_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr7_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr7_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr7_reg0")]
pub type Pr1Iep1_Slv_RegsCapr7Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capr7_reg0::Pr1Iep1_Slv_RegsCapr7Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr7_reg0"]
pub mod pr1_iep1__slv__regs_capr7_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr7_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr7_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr7_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr7_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr7_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr7_reg1")]
pub type Pr1Iep1_Slv_RegsCapr7Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capr7_reg1::Pr1Iep1_Slv_RegsCapr7Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr7_reg1"]
pub mod pr1_iep1__slv__regs_capr7_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capf7_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capf7_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf7_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf7_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf7_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf7_reg0")]
pub type Pr1Iep1_Slv_RegsCapf7Reg0 =
    crate::Reg<pr1_iep1__slv__regs_capf7_reg0::Pr1Iep1_Slv_RegsCapf7Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf7_reg0"]
pub mod pr1_iep1__slv__regs_capf7_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capf7_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capf7_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf7_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf7_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf7_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf7_reg1")]
pub type Pr1Iep1_Slv_RegsCapf7Reg1 =
    crate::Reg<pr1_iep1__slv__regs_capf7_reg1::Pr1Iep1_Slv_RegsCapf7Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf7_reg1"]
pub mod pr1_iep1__slv__regs_capf7_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp_cfg_reg (rw) register accessor: PR1_IEP1__SLV__REGS_cmp_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp_cfg_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp_cfg_reg")]
pub type Pr1Iep1_Slv_RegsCmpCfgReg =
    crate::Reg<pr1_iep1__slv__regs_cmp_cfg_reg::Pr1Iep1_Slv_RegsCmpCfgRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp_cfg_reg"]
pub mod pr1_iep1__slv__regs_cmp_cfg_reg;
#[doc = "PR1_IEP1__SLV__REGS_cmp_status_reg (rw) register accessor: PR1_IEP1__SLV__REGS_cmp_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp_status_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp_status_reg")]
pub type Pr1Iep1_Slv_RegsCmpStatusReg =
    crate::Reg<pr1_iep1__slv__regs_cmp_status_reg::Pr1Iep1_Slv_RegsCmpStatusRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp_status_reg"]
pub mod pr1_iep1__slv__regs_cmp_status_reg;
#[doc = "PR1_IEP1__SLV__REGS_cmp0_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp0_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp0_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp0_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp0_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp0_reg0")]
pub type Pr1Iep1_Slv_RegsCmp0Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp0_reg0::Pr1Iep1_Slv_RegsCmp0Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp0_reg0"]
pub mod pr1_iep1__slv__regs_cmp0_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp0_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp0_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp0_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp0_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp0_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp0_reg1")]
pub type Pr1Iep1_Slv_RegsCmp0Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp0_reg1::Pr1Iep1_Slv_RegsCmp0Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp0_reg1"]
pub mod pr1_iep1__slv__regs_cmp0_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp1_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp1_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp1_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp1_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp1_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp1_reg0")]
pub type Pr1Iep1_Slv_RegsCmp1Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp1_reg0::Pr1Iep1_Slv_RegsCmp1Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp1_reg0"]
pub mod pr1_iep1__slv__regs_cmp1_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp1_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp1_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp1_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp1_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp1_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp1_reg1")]
pub type Pr1Iep1_Slv_RegsCmp1Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp1_reg1::Pr1Iep1_Slv_RegsCmp1Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp1_reg1"]
pub mod pr1_iep1__slv__regs_cmp1_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp2_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp2_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp2_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp2_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp2_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp2_reg0")]
pub type Pr1Iep1_Slv_RegsCmp2Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp2_reg0::Pr1Iep1_Slv_RegsCmp2Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp2_reg0"]
pub mod pr1_iep1__slv__regs_cmp2_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp2_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp2_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp2_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp2_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp2_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp2_reg1")]
pub type Pr1Iep1_Slv_RegsCmp2Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp2_reg1::Pr1Iep1_Slv_RegsCmp2Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp2_reg1"]
pub mod pr1_iep1__slv__regs_cmp2_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp3_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp3_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp3_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp3_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp3_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp3_reg0")]
pub type Pr1Iep1_Slv_RegsCmp3Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp3_reg0::Pr1Iep1_Slv_RegsCmp3Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp3_reg0"]
pub mod pr1_iep1__slv__regs_cmp3_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp3_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp3_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp3_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp3_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp3_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp3_reg1")]
pub type Pr1Iep1_Slv_RegsCmp3Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp3_reg1::Pr1Iep1_Slv_RegsCmp3Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp3_reg1"]
pub mod pr1_iep1__slv__regs_cmp3_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp4_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp4_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp4_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp4_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp4_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp4_reg0")]
pub type Pr1Iep1_Slv_RegsCmp4Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp4_reg0::Pr1Iep1_Slv_RegsCmp4Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp4_reg0"]
pub mod pr1_iep1__slv__regs_cmp4_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp4_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp4_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp4_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp4_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp4_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp4_reg1")]
pub type Pr1Iep1_Slv_RegsCmp4Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp4_reg1::Pr1Iep1_Slv_RegsCmp4Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp4_reg1"]
pub mod pr1_iep1__slv__regs_cmp4_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp5_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp5_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp5_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp5_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp5_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp5_reg0")]
pub type Pr1Iep1_Slv_RegsCmp5Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp5_reg0::Pr1Iep1_Slv_RegsCmp5Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp5_reg0"]
pub mod pr1_iep1__slv__regs_cmp5_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp5_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp5_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp5_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp5_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp5_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp5_reg1")]
pub type Pr1Iep1_Slv_RegsCmp5Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp5_reg1::Pr1Iep1_Slv_RegsCmp5Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp5_reg1"]
pub mod pr1_iep1__slv__regs_cmp5_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp6_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp6_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp6_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp6_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp6_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp6_reg0")]
pub type Pr1Iep1_Slv_RegsCmp6Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp6_reg0::Pr1Iep1_Slv_RegsCmp6Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp6_reg0"]
pub mod pr1_iep1__slv__regs_cmp6_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp6_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp6_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp6_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp6_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp6_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp6_reg1")]
pub type Pr1Iep1_Slv_RegsCmp6Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp6_reg1::Pr1Iep1_Slv_RegsCmp6Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp6_reg1"]
pub mod pr1_iep1__slv__regs_cmp6_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp7_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp7_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp7_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp7_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp7_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp7_reg0")]
pub type Pr1Iep1_Slv_RegsCmp7Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp7_reg0::Pr1Iep1_Slv_RegsCmp7Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp7_reg0"]
pub mod pr1_iep1__slv__regs_cmp7_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp7_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp7_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp7_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp7_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp7_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp7_reg1")]
pub type Pr1Iep1_Slv_RegsCmp7Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp7_reg1::Pr1Iep1_Slv_RegsCmp7Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp7_reg1"]
pub mod pr1_iep1__slv__regs_cmp7_reg1;
#[doc = "PR1_IEP1__SLV__REGS_rxipg0_reg (rw) register accessor: PR1_IEP1__SLV__REGS_rxipg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_rxipg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_rxipg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_rxipg0_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_rxipg0_reg")]
pub type Pr1Iep1_Slv_RegsRxipg0Reg =
    crate::Reg<pr1_iep1__slv__regs_rxipg0_reg::Pr1Iep1_Slv_RegsRxipg0RegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_rxipg0_reg"]
pub mod pr1_iep1__slv__regs_rxipg0_reg;
#[doc = "PR1_IEP1__SLV__REGS_rxipg1_reg (rw) register accessor: PR1_IEP1__SLV__REGS_rxipg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_rxipg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_rxipg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_rxipg1_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_rxipg1_reg")]
pub type Pr1Iep1_Slv_RegsRxipg1Reg =
    crate::Reg<pr1_iep1__slv__regs_rxipg1_reg::Pr1Iep1_Slv_RegsRxipg1RegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_rxipg1_reg"]
pub mod pr1_iep1__slv__regs_rxipg1_reg;
#[doc = "PR1_IEP1__SLV__REGS_cmp8_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp8_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp8_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp8_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp8_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp8_reg0")]
pub type Pr1Iep1_Slv_RegsCmp8Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp8_reg0::Pr1Iep1_Slv_RegsCmp8Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp8_reg0"]
pub mod pr1_iep1__slv__regs_cmp8_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp8_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp8_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp8_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp8_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp8_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp8_reg1")]
pub type Pr1Iep1_Slv_RegsCmp8Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp8_reg1::Pr1Iep1_Slv_RegsCmp8Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp8_reg1"]
pub mod pr1_iep1__slv__regs_cmp8_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp9_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp9_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp9_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp9_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp9_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp9_reg0")]
pub type Pr1Iep1_Slv_RegsCmp9Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp9_reg0::Pr1Iep1_Slv_RegsCmp9Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp9_reg0"]
pub mod pr1_iep1__slv__regs_cmp9_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp9_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp9_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp9_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp9_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp9_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp9_reg1")]
pub type Pr1Iep1_Slv_RegsCmp9Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp9_reg1::Pr1Iep1_Slv_RegsCmp9Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp9_reg1"]
pub mod pr1_iep1__slv__regs_cmp9_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp10_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp10_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp10_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp10_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp10_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp10_reg0")]
pub type Pr1Iep1_Slv_RegsCmp10Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp10_reg0::Pr1Iep1_Slv_RegsCmp10Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp10_reg0"]
pub mod pr1_iep1__slv__regs_cmp10_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp10_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp10_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp10_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp10_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp10_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp10_reg1")]
pub type Pr1Iep1_Slv_RegsCmp10Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp10_reg1::Pr1Iep1_Slv_RegsCmp10Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp10_reg1"]
pub mod pr1_iep1__slv__regs_cmp10_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp11_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp11_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp11_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp11_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp11_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp11_reg0")]
pub type Pr1Iep1_Slv_RegsCmp11Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp11_reg0::Pr1Iep1_Slv_RegsCmp11Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp11_reg0"]
pub mod pr1_iep1__slv__regs_cmp11_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp11_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp11_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp11_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp11_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp11_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp11_reg1")]
pub type Pr1Iep1_Slv_RegsCmp11Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp11_reg1::Pr1Iep1_Slv_RegsCmp11Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp11_reg1"]
pub mod pr1_iep1__slv__regs_cmp11_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp12_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp12_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp12_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp12_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp12_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp12_reg0")]
pub type Pr1Iep1_Slv_RegsCmp12Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp12_reg0::Pr1Iep1_Slv_RegsCmp12Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp12_reg0"]
pub mod pr1_iep1__slv__regs_cmp12_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp12_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp12_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp12_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp12_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp12_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp12_reg1")]
pub type Pr1Iep1_Slv_RegsCmp12Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp12_reg1::Pr1Iep1_Slv_RegsCmp12Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp12_reg1"]
pub mod pr1_iep1__slv__regs_cmp12_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp13_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp13_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp13_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp13_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp13_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp13_reg0")]
pub type Pr1Iep1_Slv_RegsCmp13Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp13_reg0::Pr1Iep1_Slv_RegsCmp13Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp13_reg0"]
pub mod pr1_iep1__slv__regs_cmp13_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp13_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp13_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp13_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp13_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp13_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp13_reg1")]
pub type Pr1Iep1_Slv_RegsCmp13Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp13_reg1::Pr1Iep1_Slv_RegsCmp13Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp13_reg1"]
pub mod pr1_iep1__slv__regs_cmp13_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp14_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp14_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp14_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp14_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp14_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp14_reg0")]
pub type Pr1Iep1_Slv_RegsCmp14Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp14_reg0::Pr1Iep1_Slv_RegsCmp14Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp14_reg0"]
pub mod pr1_iep1__slv__regs_cmp14_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp14_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp14_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp14_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp14_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp14_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp14_reg1")]
pub type Pr1Iep1_Slv_RegsCmp14Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp14_reg1::Pr1Iep1_Slv_RegsCmp14Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp14_reg1"]
pub mod pr1_iep1__slv__regs_cmp14_reg1;
#[doc = "PR1_IEP1__SLV__REGS_cmp15_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp15_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp15_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp15_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp15_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp15_reg0")]
pub type Pr1Iep1_Slv_RegsCmp15Reg0 =
    crate::Reg<pr1_iep1__slv__regs_cmp15_reg0::Pr1Iep1_Slv_RegsCmp15Reg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp15_reg0"]
pub mod pr1_iep1__slv__regs_cmp15_reg0;
#[doc = "PR1_IEP1__SLV__REGS_cmp15_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_cmp15_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_cmp15_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_cmp15_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_cmp15_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_cmp15_reg1")]
pub type Pr1Iep1_Slv_RegsCmp15Reg1 =
    crate::Reg<pr1_iep1__slv__regs_cmp15_reg1::Pr1Iep1_Slv_RegsCmp15Reg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_cmp15_reg1"]
pub mod pr1_iep1__slv__regs_cmp15_reg1;
#[doc = "PR1_IEP1__SLV__REGS_count_reset_val_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_count_reset_val_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_count_reset_val_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_count_reset_val_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_count_reset_val_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_count_reset_val_reg0")]
pub type Pr1Iep1_Slv_RegsCountResetValReg0 =
    crate::Reg<pr1_iep1__slv__regs_count_reset_val_reg0::Pr1Iep1_Slv_RegsCountResetValReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_count_reset_val_reg0"]
pub mod pr1_iep1__slv__regs_count_reset_val_reg0;
#[doc = "PR1_IEP1__SLV__REGS_count_reset_val_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_count_reset_val_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_count_reset_val_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_count_reset_val_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_count_reset_val_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_count_reset_val_reg1")]
pub type Pr1Iep1_Slv_RegsCountResetValReg1 =
    crate::Reg<pr1_iep1__slv__regs_count_reset_val_reg1::Pr1Iep1_Slv_RegsCountResetValReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_count_reset_val_reg1"]
pub mod pr1_iep1__slv__regs_count_reset_val_reg1;
#[doc = "PR1_IEP1__SLV__REGS_pwm_reg (rw) register accessor: PR1_IEP1__SLV__REGS_pwm_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_pwm_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_pwm_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_pwm_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_pwm_reg")]
pub type Pr1Iep1_Slv_RegsPwmReg =
    crate::Reg<pr1_iep1__slv__regs_pwm_reg::Pr1Iep1_Slv_RegsPwmRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_pwm_reg"]
pub mod pr1_iep1__slv__regs_pwm_reg;
#[doc = "PR1_IEP1__SLV__REGS_capr0_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr0_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr0_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr0_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr0_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr0_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr0BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr0_bi_reg0::Pr1Iep1_Slv_RegsCapr0BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr0_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr0_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr0_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr0_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr0_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr0_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr0_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr0_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr0BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr0_bi_reg1::Pr1Iep1_Slv_RegsCapr0BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr0_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr0_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr1_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr1_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr1_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr1_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr1_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr1_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr1BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr1_bi_reg0::Pr1Iep1_Slv_RegsCapr1BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr1_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr1_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr1_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr1_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr1_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr1_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr1_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr1_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr1BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr1_bi_reg1::Pr1Iep1_Slv_RegsCapr1BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr1_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr1_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr2_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr2_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr2_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr2_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr2_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr2_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr2BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr2_bi_reg0::Pr1Iep1_Slv_RegsCapr2BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr2_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr2_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr2_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr2_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr2_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr2_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr2_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr2_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr2BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr2_bi_reg1::Pr1Iep1_Slv_RegsCapr2BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr2_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr2_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr3_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr3_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr3_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr3_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr3_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr3_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr3BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr3_bi_reg0::Pr1Iep1_Slv_RegsCapr3BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr3_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr3_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr3_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr3_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr3_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr3_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr3_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr3_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr3BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr3_bi_reg1::Pr1Iep1_Slv_RegsCapr3BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr3_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr3_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr4_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr4_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr4_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr4_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr4_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr4_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr4BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr4_bi_reg0::Pr1Iep1_Slv_RegsCapr4BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr4_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr4_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr4_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr4_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr4_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr4_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr4_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr4_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr4BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr4_bi_reg1::Pr1Iep1_Slv_RegsCapr4BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr4_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr4_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr5_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr5_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr5_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr5_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr5_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr5_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr5BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr5_bi_reg0::Pr1Iep1_Slv_RegsCapr5BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr5_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr5_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr5_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr5_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr5_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr5_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr5_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr5_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr5BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr5_bi_reg1::Pr1Iep1_Slv_RegsCapr5BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr5_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr5_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr6_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr6_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr6_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr6_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr6_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr6_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr6BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr6_bi_reg0::Pr1Iep1_Slv_RegsCapr6BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr6_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr6_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr6_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr6_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr6_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr6_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr6_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr6_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr6BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr6_bi_reg1::Pr1Iep1_Slv_RegsCapr6BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr6_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr6_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capf6_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capf6_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf6_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf6_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf6_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf6_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapf6BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capf6_bi_reg0::Pr1Iep1_Slv_RegsCapf6BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf6_bi_reg0"]
pub mod pr1_iep1__slv__regs_capf6_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capf6_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capf6_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf6_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf6_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf6_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf6_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapf6BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capf6_bi_reg1::Pr1Iep1_Slv_RegsCapf6BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf6_bi_reg1"]
pub mod pr1_iep1__slv__regs_capf6_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capr7_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capr7_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr7_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr7_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr7_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr7_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapr7BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capr7_bi_reg0::Pr1Iep1_Slv_RegsCapr7BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr7_bi_reg0"]
pub mod pr1_iep1__slv__regs_capr7_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capr7_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capr7_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capr7_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capr7_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capr7_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capr7_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapr7BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capr7_bi_reg1::Pr1Iep1_Slv_RegsCapr7BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capr7_bi_reg1"]
pub mod pr1_iep1__slv__regs_capr7_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_capf7_bi_reg0 (rw) register accessor: PR1_IEP1__SLV__REGS_capf7_bi_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf7_bi_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf7_bi_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf7_bi_reg0`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf7_bi_reg0")]
pub type Pr1Iep1_Slv_RegsCapf7BiReg0 =
    crate::Reg<pr1_iep1__slv__regs_capf7_bi_reg0::Pr1Iep1_Slv_RegsCapf7BiReg0Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf7_bi_reg0"]
pub mod pr1_iep1__slv__regs_capf7_bi_reg0;
#[doc = "PR1_IEP1__SLV__REGS_capf7_bi_reg1 (rw) register accessor: PR1_IEP1__SLV__REGS_capf7_bi_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_capf7_bi_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_capf7_bi_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_capf7_bi_reg1`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_capf7_bi_reg1")]
pub type Pr1Iep1_Slv_RegsCapf7BiReg1 =
    crate::Reg<pr1_iep1__slv__regs_capf7_bi_reg1::Pr1Iep1_Slv_RegsCapf7BiReg1Spec>;
#[doc = "PR1_IEP1__SLV__REGS_capf7_bi_reg1"]
pub mod pr1_iep1__slv__regs_capf7_bi_reg1;
#[doc = "PR1_IEP1__SLV__REGS_sync_ctrl_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync_ctrl_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync_ctrl_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync_ctrl_reg")]
pub type Pr1Iep1_Slv_RegsSyncCtrlReg =
    crate::Reg<pr1_iep1__slv__regs_sync_ctrl_reg::Pr1Iep1_Slv_RegsSyncCtrlRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync_ctrl_reg"]
pub mod pr1_iep1__slv__regs_sync_ctrl_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync_first_stat_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync_first_stat_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync_first_stat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync_first_stat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync_first_stat_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync_first_stat_reg")]
pub type Pr1Iep1_Slv_RegsSyncFirstStatReg =
    crate::Reg<pr1_iep1__slv__regs_sync_first_stat_reg::Pr1Iep1_Slv_RegsSyncFirstStatRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync_first_stat_reg"]
pub mod pr1_iep1__slv__regs_sync_first_stat_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync0_stat_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync0_stat_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync0_stat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync0_stat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync0_stat_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync0_stat_reg")]
pub type Pr1Iep1_Slv_RegsSync0StatReg =
    crate::Reg<pr1_iep1__slv__regs_sync0_stat_reg::Pr1Iep1_Slv_RegsSync0StatRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync0_stat_reg"]
pub mod pr1_iep1__slv__regs_sync0_stat_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync1_stat_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync1_stat_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync1_stat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync1_stat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync1_stat_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync1_stat_reg")]
pub type Pr1Iep1_Slv_RegsSync1StatReg =
    crate::Reg<pr1_iep1__slv__regs_sync1_stat_reg::Pr1Iep1_Slv_RegsSync1StatRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync1_stat_reg"]
pub mod pr1_iep1__slv__regs_sync1_stat_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync_pwidth_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync_pwidth_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync_pwidth_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync_pwidth_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync_pwidth_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync_pwidth_reg")]
pub type Pr1Iep1_Slv_RegsSyncPwidthReg =
    crate::Reg<pr1_iep1__slv__regs_sync_pwidth_reg::Pr1Iep1_Slv_RegsSyncPwidthRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync_pwidth_reg"]
pub mod pr1_iep1__slv__regs_sync_pwidth_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync0_period_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync0_period_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync0_period_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync0_period_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync0_period_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync0_period_reg")]
pub type Pr1Iep1_Slv_RegsSync0PeriodReg =
    crate::Reg<pr1_iep1__slv__regs_sync0_period_reg::Pr1Iep1_Slv_RegsSync0PeriodRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync0_period_reg"]
pub mod pr1_iep1__slv__regs_sync0_period_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync1_delay_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync1_delay_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync1_delay_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync1_delay_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync1_delay_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync1_delay_reg")]
pub type Pr1Iep1_Slv_RegsSync1DelayReg =
    crate::Reg<pr1_iep1__slv__regs_sync1_delay_reg::Pr1Iep1_Slv_RegsSync1DelayRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync1_delay_reg"]
pub mod pr1_iep1__slv__regs_sync1_delay_reg;
#[doc = "PR1_IEP1__SLV__REGS_sync_start_reg (rw) register accessor: PR1_IEP1__SLV__REGS_sync_start_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync_start_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync_start_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_sync_start_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_sync_start_reg")]
pub type Pr1Iep1_Slv_RegsSyncStartReg =
    crate::Reg<pr1_iep1__slv__regs_sync_start_reg::Pr1Iep1_Slv_RegsSyncStartRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_sync_start_reg"]
pub mod pr1_iep1__slv__regs_sync_start_reg;
#[doc = "PR1_IEP1__SLV__REGS_wd_prediv_reg (rw) register accessor: PR1_IEP1__SLV__REGS_wd_prediv_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_prediv_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_prediv_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_wd_prediv_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_wd_prediv_reg")]
pub type Pr1Iep1_Slv_RegsWdPredivReg =
    crate::Reg<pr1_iep1__slv__regs_wd_prediv_reg::Pr1Iep1_Slv_RegsWdPredivRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_wd_prediv_reg"]
pub mod pr1_iep1__slv__regs_wd_prediv_reg;
#[doc = "PR1_IEP1__SLV__REGS_pdi_wd_tim_reg (rw) register accessor: PR1_IEP1__SLV__REGS_pdi_wd_tim_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_pdi_wd_tim_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_pdi_wd_tim_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_pdi_wd_tim_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_pdi_wd_tim_reg")]
pub type Pr1Iep1_Slv_RegsPdiWdTimReg =
    crate::Reg<pr1_iep1__slv__regs_pdi_wd_tim_reg::Pr1Iep1_Slv_RegsPdiWdTimRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_pdi_wd_tim_reg"]
pub mod pr1_iep1__slv__regs_pdi_wd_tim_reg;
#[doc = "PR1_IEP1__SLV__REGS_pd_wd_tim_reg (rw) register accessor: PR1_IEP1__SLV__REGS_pd_wd_tim_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_pd_wd_tim_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_pd_wd_tim_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_pd_wd_tim_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_pd_wd_tim_reg")]
pub type Pr1Iep1_Slv_RegsPdWdTimReg =
    crate::Reg<pr1_iep1__slv__regs_pd_wd_tim_reg::Pr1Iep1_Slv_RegsPdWdTimRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_pd_wd_tim_reg"]
pub mod pr1_iep1__slv__regs_pd_wd_tim_reg;
#[doc = "PR1_IEP1__SLV__REGS_wd_status_reg (rw) register accessor: PR1_IEP1__SLV__REGS_wd_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_wd_status_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_wd_status_reg")]
pub type Pr1Iep1_Slv_RegsWdStatusReg =
    crate::Reg<pr1_iep1__slv__regs_wd_status_reg::Pr1Iep1_Slv_RegsWdStatusRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_wd_status_reg"]
pub mod pr1_iep1__slv__regs_wd_status_reg;
#[doc = "PR1_IEP1__SLV__REGS_wd_exp_cnt_reg (rw) register accessor: PR1_IEP1__SLV__REGS_wd_exp_cnt_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_exp_cnt_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_exp_cnt_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_wd_exp_cnt_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_wd_exp_cnt_reg")]
pub type Pr1Iep1_Slv_RegsWdExpCntReg =
    crate::Reg<pr1_iep1__slv__regs_wd_exp_cnt_reg::Pr1Iep1_Slv_RegsWdExpCntRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_wd_exp_cnt_reg"]
pub mod pr1_iep1__slv__regs_wd_exp_cnt_reg;
#[doc = "PR1_IEP1__SLV__REGS_wd_ctrl_reg (rw) register accessor: PR1_IEP1__SLV__REGS_wd_ctrl_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_wd_ctrl_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_wd_ctrl_reg")]
pub type Pr1Iep1_Slv_RegsWdCtrlReg =
    crate::Reg<pr1_iep1__slv__regs_wd_ctrl_reg::Pr1Iep1_Slv_RegsWdCtrlRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_wd_ctrl_reg"]
pub mod pr1_iep1__slv__regs_wd_ctrl_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_ctrl_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_ctrl_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_ctrl_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_ctrl_reg")]
pub type Pr1Iep1_Slv_RegsDigioCtrlReg =
    crate::Reg<pr1_iep1__slv__regs_digio_ctrl_reg::Pr1Iep1_Slv_RegsDigioCtrlRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_ctrl_reg"]
pub mod pr1_iep1__slv__regs_digio_ctrl_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_status_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_status_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_status_reg")]
pub type Pr1Iep1_Slv_RegsDigioStatusReg =
    crate::Reg<pr1_iep1__slv__regs_digio_status_reg::Pr1Iep1_Slv_RegsDigioStatusRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_status_reg"]
pub mod pr1_iep1__slv__regs_digio_status_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_in_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_data_in_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_data_in_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_data_in_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_data_in_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_data_in_reg")]
pub type Pr1Iep1_Slv_RegsDigioDataInReg =
    crate::Reg<pr1_iep1__slv__regs_digio_data_in_reg::Pr1Iep1_Slv_RegsDigioDataInRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_in_reg"]
pub mod pr1_iep1__slv__regs_digio_data_in_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_in_raw_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_data_in_raw_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_data_in_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_data_in_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_data_in_raw_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_data_in_raw_reg")]
pub type Pr1Iep1_Slv_RegsDigioDataInRawReg =
    crate::Reg<pr1_iep1__slv__regs_digio_data_in_raw_reg::Pr1Iep1_Slv_RegsDigioDataInRawRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_in_raw_reg"]
pub mod pr1_iep1__slv__regs_digio_data_in_raw_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_out_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_data_out_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_data_out_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_data_out_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_data_out_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_data_out_reg")]
pub type Pr1Iep1_Slv_RegsDigioDataOutReg =
    crate::Reg<pr1_iep1__slv__regs_digio_data_out_reg::Pr1Iep1_Slv_RegsDigioDataOutRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_out_reg"]
pub mod pr1_iep1__slv__regs_digio_data_out_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_out_en_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_data_out_en_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_data_out_en_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_data_out_en_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_data_out_en_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_data_out_en_reg")]
pub type Pr1Iep1_Slv_RegsDigioDataOutEnReg =
    crate::Reg<pr1_iep1__slv__regs_digio_data_out_en_reg::Pr1Iep1_Slv_RegsDigioDataOutEnRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_data_out_en_reg"]
pub mod pr1_iep1__slv__regs_digio_data_out_en_reg;
#[doc = "PR1_IEP1__SLV__REGS_digio_exp_reg (rw) register accessor: PR1_IEP1__SLV__REGS_digio_exp_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_exp_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_exp_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_iep1__slv__regs_digio_exp_reg`]
module"]
#[doc(alias = "PR1_IEP1__SLV__REGS_digio_exp_reg")]
pub type Pr1Iep1_Slv_RegsDigioExpReg =
    crate::Reg<pr1_iep1__slv__regs_digio_exp_reg::Pr1Iep1_Slv_RegsDigioExpRegSpec>;
#[doc = "PR1_IEP1__SLV__REGS_digio_exp_reg"]
pub mod pr1_iep1__slv__regs_digio_exp_reg;
