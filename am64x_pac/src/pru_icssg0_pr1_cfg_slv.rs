#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_cfg__slv__regs_pid_reg: Pr1Cfg_Slv_RegsPidReg,
    pr1_cfg__slv__regs_hwdis_reg: Pr1Cfg_Slv_RegsHwdisReg,
    pr1_cfg__slv__regs_gpcfg0_reg: Pr1Cfg_Slv_RegsGpcfg0Reg,
    pr1_cfg__slv__regs_gpcfg1_reg: Pr1Cfg_Slv_RegsGpcfg1Reg,
    pr1_cfg__slv__regs_cgr_reg: Pr1Cfg_Slv_RegsCgrReg,
    pr1_cfg__slv__regs_gpecfg0_reg: Pr1Cfg_Slv_RegsGpecfg0Reg,
    pr1_cfg__slv__regs_gpecfg1_reg: Pr1Cfg_Slv_RegsGpecfg1Reg,
    pr1_cfg__slv__regs_reset_iso_reg: Pr1Cfg_Slv_RegsResetIsoReg,
    _reserved8: [u8; 0x0c],
    pr1_cfg__slv__regs_mii_rt_reg: Pr1Cfg_Slv_RegsMiiRtReg,
    pr1_cfg__slv__regs_iepclk_reg: Pr1Cfg_Slv_RegsIepclkReg,
    pr1_cfg__slv__regs_spp_reg: Pr1Cfg_Slv_RegsSppReg,
    _reserved11: [u8; 0x04],
    pr1_cfg__slv__regs_core_sync_reg: Pr1Cfg_Slv_RegsCoreSyncReg,
    pr1_cfg__slv__regs_sa_mx_reg: Pr1Cfg_Slv_RegsSaMxReg,
    pr1_cfg__slv__regs_pru0_sd_clk_div_reg: Pr1Cfg_Slv_RegsPru0SdClkDivReg,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0: Pr1Cfg_Slv_RegsPru0SdClkSelReg0,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg0: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1: Pr1Cfg_Slv_RegsPru0SdClkSelReg1,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg1: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2: Pr1Cfg_Slv_RegsPru0SdClkSelReg2,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg2: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3: Pr1Cfg_Slv_RegsPru0SdClkSelReg3,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg3: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4: Pr1Cfg_Slv_RegsPru0SdClkSelReg4,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg4: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg4,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5: Pr1Cfg_Slv_RegsPru0SdClkSelReg5,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg5: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg5,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6: Pr1Cfg_Slv_RegsPru0SdClkSelReg6,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg6: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7: Pr1Cfg_Slv_RegsPru0SdClkSelReg7,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg7: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg7,
    pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8: Pr1Cfg_Slv_RegsPru0SdClkSelReg8,
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg8: Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8,
    pr1_cfg__slv__regs_pru1_sd_clk_div_reg: Pr1Cfg_Slv_RegsPru1SdClkDivReg,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0: Pr1Cfg_Slv_RegsPru1SdClkSelReg0,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg0: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1: Pr1Cfg_Slv_RegsPru1SdClkSelReg1,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg1: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2: Pr1Cfg_Slv_RegsPru1SdClkSelReg2,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg2: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3: Pr1Cfg_Slv_RegsPru1SdClkSelReg3,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg3: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4: Pr1Cfg_Slv_RegsPru1SdClkSelReg4,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg4: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5: Pr1Cfg_Slv_RegsPru1SdClkSelReg5,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg5: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6: Pr1Cfg_Slv_RegsPru1SdClkSelReg6,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg6: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7: Pr1Cfg_Slv_RegsPru1SdClkSelReg7,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg7: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7,
    pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8: Pr1Cfg_Slv_RegsPru1SdClkSelReg8,
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg8: Pr1Cfg_Slv_RegsPru1SdSampleSizeReg8,
    _reserved51: [u8; 0x04],
    pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg: Pr1Cfg_Slv_RegsPru0EdRxCfgReg,
    pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg: Pr1Cfg_Slv_RegsPru0EdTxCfgReg,
    pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg: Pr1Cfg_Slv_RegsPru0EdCh0Cfg0Reg,
    pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg: Pr1Cfg_Slv_RegsPru0EdCh0Cfg1Reg,
    pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg: Pr1Cfg_Slv_RegsPru0EdCh1Cfg0Reg,
    pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg: Pr1Cfg_Slv_RegsPru0EdCh1Cfg1Reg,
    pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg: Pr1Cfg_Slv_RegsPru0EdCh2Cfg0Reg,
    pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg: Pr1Cfg_Slv_RegsPru0EdCh2Cfg1Reg,
    pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg: Pr1Cfg_Slv_RegsPru1EdRxCfgReg,
    pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg: Pr1Cfg_Slv_RegsPru1EdTxCfgReg,
    pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg: Pr1Cfg_Slv_RegsPru1EdCh0Cfg0Reg,
    pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg: Pr1Cfg_Slv_RegsPru1EdCh0Cfg1Reg,
    pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg: Pr1Cfg_Slv_RegsPru1EdCh1Cfg0Reg,
    pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg: Pr1Cfg_Slv_RegsPru1EdCh1Cfg1Reg,
    pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg: Pr1Cfg_Slv_RegsPru1EdCh2Cfg0Reg,
    pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg: Pr1Cfg_Slv_RegsPru1EdCh2Cfg1Reg,
    _reserved67: [u8; 0x04],
    pr1_cfg__slv__regs_rtu0_poke_en0_reg: Pr1Cfg_Slv_RegsRtu0PokeEn0Reg,
    _reserved68: [u8; 0x04],
    pr1_cfg__slv__regs_rtu1_poke_en0_reg: Pr1Cfg_Slv_RegsRtu1PokeEn0Reg,
    pr1_cfg__slv__regs_pwm0: Pr1Cfg_Slv_RegsPwm0,
    pr1_cfg__slv__regs_pwm1: Pr1Cfg_Slv_RegsPwm1,
    pr1_cfg__slv__regs_pwm2: Pr1Cfg_Slv_RegsPwm2,
    pr1_cfg__slv__regs_pwm3: Pr1Cfg_Slv_RegsPwm3,
    pr1_cfg__slv__regs_pwm0_0: Pr1Cfg_Slv_RegsPwm0_0,
    pr1_cfg__slv__regs_pwm0_1: Pr1Cfg_Slv_RegsPwm0_1,
    pr1_cfg__slv__regs_pwm0_2: Pr1Cfg_Slv_RegsPwm0_2,
    pr1_cfg__slv__regs_pwm1_0: Pr1Cfg_Slv_RegsPwm1_0,
    pr1_cfg__slv__regs_pwm1_1: Pr1Cfg_Slv_RegsPwm1_1,
    pr1_cfg__slv__regs_pwm1_2: Pr1Cfg_Slv_RegsPwm1_2,
    pr1_cfg__slv__regs_pwm2_0: Pr1Cfg_Slv_RegsPwm2_0,
    pr1_cfg__slv__regs_pwm2_1: Pr1Cfg_Slv_RegsPwm2_1,
    pr1_cfg__slv__regs_pwm2_2: Pr1Cfg_Slv_RegsPwm2_2,
    pr1_cfg__slv__regs_pwm3_0: Pr1Cfg_Slv_RegsPwm3_0,
    pr1_cfg__slv__regs_pwm3_1: Pr1Cfg_Slv_RegsPwm3_1,
    pr1_cfg__slv__regs_pwm3_2: Pr1Cfg_Slv_RegsPwm3_2,
    pr1_cfg__slv__regs_spin_lock0: Pr1Cfg_Slv_RegsSpinLock0,
    pr1_cfg__slv__regs_spin_lock1: Pr1Cfg_Slv_RegsSpinLock1,
    pr1_cfg__slv__regs_pa_stat_pdsp_cfg0: Pr1Cfg_Slv_RegsPaStatPdspCfg0,
    pr1_cfg__slv__regs_pa_stat_pdsp_stat0: Pr1Cfg_Slv_RegsPaStatPdspStat0,
    pr1_cfg__slv__regs_pa_stat_pdsp_cfg1: Pr1Cfg_Slv_RegsPaStatPdspCfg1,
    pr1_cfg__slv__regs_pa_stat_pdsp_stat1: Pr1Cfg_Slv_RegsPaStatPdspStat1,
    pr1_cfg__slv__regs_pa_stat_pdsp_cfg2: Pr1Cfg_Slv_RegsPaStatPdspCfg2,
    pr1_cfg__slv__regs_pa_stat_pdsp_stat2: Pr1Cfg_Slv_RegsPaStatPdspStat2,
    pr1_cfg__slv__regs_pa_stat_pdsp_cfg3: Pr1Cfg_Slv_RegsPaStatPdspCfg3,
    pr1_cfg__slv__regs_pa_stat_pdsp_stat3: Pr1Cfg_Slv_RegsPaStatPdspStat3,
}
impl RegisterBlock {
    #[doc = "0x00 - PR1_CFG__SLV__REGS_pid_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pid_reg(&self) -> &Pr1Cfg_Slv_RegsPidReg {
        &self.pr1_cfg__slv__regs_pid_reg
    }
    #[doc = "0x04 - PR1_CFG__SLV__REGS_hwdis_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_hwdis_reg(&self) -> &Pr1Cfg_Slv_RegsHwdisReg {
        &self.pr1_cfg__slv__regs_hwdis_reg
    }
    #[doc = "0x08 - PR1_CFG__SLV__REGS_gpcfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_gpcfg0_reg(&self) -> &Pr1Cfg_Slv_RegsGpcfg0Reg {
        &self.pr1_cfg__slv__regs_gpcfg0_reg
    }
    #[doc = "0x0c - PR1_CFG__SLV__REGS_gpcfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_gpcfg1_reg(&self) -> &Pr1Cfg_Slv_RegsGpcfg1Reg {
        &self.pr1_cfg__slv__regs_gpcfg1_reg
    }
    #[doc = "0x10 - PR1_CFG__SLV__REGS_cgr_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_cgr_reg(&self) -> &Pr1Cfg_Slv_RegsCgrReg {
        &self.pr1_cfg__slv__regs_cgr_reg
    }
    #[doc = "0x14 - PR1_CFG__SLV__REGS_gpecfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_gpecfg0_reg(&self) -> &Pr1Cfg_Slv_RegsGpecfg0Reg {
        &self.pr1_cfg__slv__regs_gpecfg0_reg
    }
    #[doc = "0x18 - PR1_CFG__SLV__REGS_gpecfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_gpecfg1_reg(&self) -> &Pr1Cfg_Slv_RegsGpecfg1Reg {
        &self.pr1_cfg__slv__regs_gpecfg1_reg
    }
    #[doc = "0x1c - PR1_CFG__SLV__REGS_reset_iso_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_reset_iso_reg(&self) -> &Pr1Cfg_Slv_RegsResetIsoReg {
        &self.pr1_cfg__slv__regs_reset_iso_reg
    }
    #[doc = "0x2c - PR1_CFG__SLV__REGS_mii_rt_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_mii_rt_reg(&self) -> &Pr1Cfg_Slv_RegsMiiRtReg {
        &self.pr1_cfg__slv__regs_mii_rt_reg
    }
    #[doc = "0x30 - PR1_CFG__SLV__REGS_iepclk_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_iepclk_reg(&self) -> &Pr1Cfg_Slv_RegsIepclkReg {
        &self.pr1_cfg__slv__regs_iepclk_reg
    }
    #[doc = "0x34 - PR1_CFG__SLV__REGS_spp_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_spp_reg(&self) -> &Pr1Cfg_Slv_RegsSppReg {
        &self.pr1_cfg__slv__regs_spp_reg
    }
    #[doc = "0x3c - PR1_CFG__SLV__REGS_core_sync_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_core_sync_reg(&self) -> &Pr1Cfg_Slv_RegsCoreSyncReg {
        &self.pr1_cfg__slv__regs_core_sync_reg
    }
    #[doc = "0x40 - PR1_CFG__SLV__REGS_sa_mx_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_sa_mx_reg(&self) -> &Pr1Cfg_Slv_RegsSaMxReg {
        &self.pr1_cfg__slv__regs_sa_mx_reg
    }
    #[doc = "0x44 - PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_div_reg(&self) -> &Pr1Cfg_Slv_RegsPru0SdClkDivReg {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_div_reg
    }
    #[doc = "0x48 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg0 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0
    }
    #[doc = "0x4c - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg0(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg0
    }
    #[doc = "0x50 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg1 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1
    }
    #[doc = "0x54 - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg1(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg1
    }
    #[doc = "0x58 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg2 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2
    }
    #[doc = "0x5c - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg2(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg2
    }
    #[doc = "0x60 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg3 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3
    }
    #[doc = "0x64 - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg3(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg3
    }
    #[doc = "0x68 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg4"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg4 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4
    }
    #[doc = "0x6c - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg4"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg4(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg4 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg4
    }
    #[doc = "0x70 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg5 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5
    }
    #[doc = "0x74 - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg5"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg5(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg5 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg5
    }
    #[doc = "0x78 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg6 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6
    }
    #[doc = "0x7c - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg6(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg6
    }
    #[doc = "0x80 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg7 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7
    }
    #[doc = "0x84 - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg7"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg7(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg7 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg7
    }
    #[doc = "0x88 - PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdClkSelReg8 {
        &self.pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8
    }
    #[doc = "0x8c - PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_sd_sample_size_reg8(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8 {
        &self.pr1_cfg__slv__regs_pru0_sd_sample_size_reg8
    }
    #[doc = "0x90 - PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_div_reg(&self) -> &Pr1Cfg_Slv_RegsPru1SdClkDivReg {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_div_reg
    }
    #[doc = "0x94 - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg0 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0
    }
    #[doc = "0x98 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg0(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg0
    }
    #[doc = "0x9c - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg1 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1
    }
    #[doc = "0xa0 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg1(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg1
    }
    #[doc = "0xa4 - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg2 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2
    }
    #[doc = "0xa8 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg2(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg2
    }
    #[doc = "0xac - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg3 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3
    }
    #[doc = "0xb0 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg3(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg3
    }
    #[doc = "0xb4 - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg4 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4
    }
    #[doc = "0xb8 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg4(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg4
    }
    #[doc = "0xbc - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg5"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg5 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5
    }
    #[doc = "0xc0 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg5(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg5
    }
    #[doc = "0xc4 - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg6"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg6 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6
    }
    #[doc = "0xc8 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg6(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg6
    }
    #[doc = "0xcc - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg7 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7
    }
    #[doc = "0xd0 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg7(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg7
    }
    #[doc = "0xd4 - PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg8"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdClkSelReg8 {
        &self.pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8
    }
    #[doc = "0xd8 - PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg8"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_sd_sample_size_reg8(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1SdSampleSizeReg8 {
        &self.pr1_cfg__slv__regs_pru1_sd_sample_size_reg8
    }
    #[doc = "0xe0 - PR1_CFG__SLV__REGS_pru0_ed_rx_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg(&self) -> &Pr1Cfg_Slv_RegsPru0EdRxCfgReg {
        &self.pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg
    }
    #[doc = "0xe4 - PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg(&self) -> &Pr1Cfg_Slv_RegsPru0EdTxCfgReg {
        &self.pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg
    }
    #[doc = "0xe8 - PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0EdCh0Cfg0Reg {
        &self.pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg
    }
    #[doc = "0xec - PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0EdCh0Cfg1Reg {
        &self.pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg
    }
    #[doc = "0xf0 - PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0EdCh1Cfg0Reg {
        &self.pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg
    }
    #[doc = "0xf4 - PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0EdCh1Cfg1Reg {
        &self.pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg
    }
    #[doc = "0xf8 - PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0EdCh2Cfg0Reg {
        &self.pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg
    }
    #[doc = "0xfc - PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru0EdCh2Cfg1Reg {
        &self.pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg
    }
    #[doc = "0x100 - PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg(&self) -> &Pr1Cfg_Slv_RegsPru1EdRxCfgReg {
        &self.pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg
    }
    #[doc = "0x104 - PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg(&self) -> &Pr1Cfg_Slv_RegsPru1EdTxCfgReg {
        &self.pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg
    }
    #[doc = "0x108 - PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1EdCh0Cfg0Reg {
        &self.pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg
    }
    #[doc = "0x10c - PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1EdCh0Cfg1Reg {
        &self.pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg
    }
    #[doc = "0x110 - PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1EdCh1Cfg0Reg {
        &self.pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg
    }
    #[doc = "0x114 - PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1EdCh1Cfg1Reg {
        &self.pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg
    }
    #[doc = "0x118 - PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1EdCh2Cfg0Reg {
        &self.pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg
    }
    #[doc = "0x11c - PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg(
        &self,
    ) -> &Pr1Cfg_Slv_RegsPru1EdCh2Cfg1Reg {
        &self.pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg
    }
    #[doc = "0x124 - PR1_CFG__SLV__REGS_rtu0_poke_en0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_rtu0_poke_en0_reg(&self) -> &Pr1Cfg_Slv_RegsRtu0PokeEn0Reg {
        &self.pr1_cfg__slv__regs_rtu0_poke_en0_reg
    }
    #[doc = "0x12c - PR1_CFG__SLV__REGS_rtu1_poke_en0_reg"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_rtu1_poke_en0_reg(&self) -> &Pr1Cfg_Slv_RegsRtu1PokeEn0Reg {
        &self.pr1_cfg__slv__regs_rtu1_poke_en0_reg
    }
    #[doc = "0x130 - PR1_CFG__SLV__REGS_pwm0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm0(&self) -> &Pr1Cfg_Slv_RegsPwm0 {
        &self.pr1_cfg__slv__regs_pwm0
    }
    #[doc = "0x134 - PR1_CFG__SLV__REGS_pwm1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm1(&self) -> &Pr1Cfg_Slv_RegsPwm1 {
        &self.pr1_cfg__slv__regs_pwm1
    }
    #[doc = "0x138 - PR1_CFG__SLV__REGS_pwm2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm2(&self) -> &Pr1Cfg_Slv_RegsPwm2 {
        &self.pr1_cfg__slv__regs_pwm2
    }
    #[doc = "0x13c - PR1_CFG__SLV__REGS_pwm3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm3(&self) -> &Pr1Cfg_Slv_RegsPwm3 {
        &self.pr1_cfg__slv__regs_pwm3
    }
    #[doc = "0x140 - PR1_CFG__SLV__REGS_pwm0_0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm0_0(&self) -> &Pr1Cfg_Slv_RegsPwm0_0 {
        &self.pr1_cfg__slv__regs_pwm0_0
    }
    #[doc = "0x144 - PR1_CFG__SLV__REGS_pwm0_1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm0_1(&self) -> &Pr1Cfg_Slv_RegsPwm0_1 {
        &self.pr1_cfg__slv__regs_pwm0_1
    }
    #[doc = "0x148 - PR1_CFG__SLV__REGS_pwm0_2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm0_2(&self) -> &Pr1Cfg_Slv_RegsPwm0_2 {
        &self.pr1_cfg__slv__regs_pwm0_2
    }
    #[doc = "0x14c - PR1_CFG__SLV__REGS_pwm1_0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm1_0(&self) -> &Pr1Cfg_Slv_RegsPwm1_0 {
        &self.pr1_cfg__slv__regs_pwm1_0
    }
    #[doc = "0x150 - PR1_CFG__SLV__REGS_pwm1_1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm1_1(&self) -> &Pr1Cfg_Slv_RegsPwm1_1 {
        &self.pr1_cfg__slv__regs_pwm1_1
    }
    #[doc = "0x154 - PR1_CFG__SLV__REGS_pwm1_2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm1_2(&self) -> &Pr1Cfg_Slv_RegsPwm1_2 {
        &self.pr1_cfg__slv__regs_pwm1_2
    }
    #[doc = "0x158 - PR1_CFG__SLV__REGS_pwm2_0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm2_0(&self) -> &Pr1Cfg_Slv_RegsPwm2_0 {
        &self.pr1_cfg__slv__regs_pwm2_0
    }
    #[doc = "0x15c - PR1_CFG__SLV__REGS_pwm2_1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm2_1(&self) -> &Pr1Cfg_Slv_RegsPwm2_1 {
        &self.pr1_cfg__slv__regs_pwm2_1
    }
    #[doc = "0x160 - PR1_CFG__SLV__REGS_pwm2_2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm2_2(&self) -> &Pr1Cfg_Slv_RegsPwm2_2 {
        &self.pr1_cfg__slv__regs_pwm2_2
    }
    #[doc = "0x164 - PR1_CFG__SLV__REGS_pwm3_0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm3_0(&self) -> &Pr1Cfg_Slv_RegsPwm3_0 {
        &self.pr1_cfg__slv__regs_pwm3_0
    }
    #[doc = "0x168 - PR1_CFG__SLV__REGS_pwm3_1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm3_1(&self) -> &Pr1Cfg_Slv_RegsPwm3_1 {
        &self.pr1_cfg__slv__regs_pwm3_1
    }
    #[doc = "0x16c - PR1_CFG__SLV__REGS_pwm3_2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pwm3_2(&self) -> &Pr1Cfg_Slv_RegsPwm3_2 {
        &self.pr1_cfg__slv__regs_pwm3_2
    }
    #[doc = "0x170 - PR1_CFG__SLV__REGS_spin_lock0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_spin_lock0(&self) -> &Pr1Cfg_Slv_RegsSpinLock0 {
        &self.pr1_cfg__slv__regs_spin_lock0
    }
    #[doc = "0x174 - PR1_CFG__SLV__REGS_spin_lock1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_spin_lock1(&self) -> &Pr1Cfg_Slv_RegsSpinLock1 {
        &self.pr1_cfg__slv__regs_spin_lock1
    }
    #[doc = "0x178 - PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_cfg0(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspCfg0 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_cfg0
    }
    #[doc = "0x17c - PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_stat0(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspStat0 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_stat0
    }
    #[doc = "0x180 - PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_cfg1(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspCfg1 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_cfg1
    }
    #[doc = "0x184 - PR1_CFG__SLV__REGS_pa_stat_pdsp_stat1"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_stat1(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspStat1 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_stat1
    }
    #[doc = "0x188 - PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_cfg2(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspCfg2 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_cfg2
    }
    #[doc = "0x18c - PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_stat2(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspStat2 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_stat2
    }
    #[doc = "0x190 - PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_cfg3(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspCfg3 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_cfg3
    }
    #[doc = "0x194 - PR1_CFG__SLV__REGS_pa_stat_pdsp_stat3"]
    #[inline(always)]
    pub const fn pr1_cfg__slv__regs_pa_stat_pdsp_stat3(&self) -> &Pr1Cfg_Slv_RegsPaStatPdspStat3 {
        &self.pr1_cfg__slv__regs_pa_stat_pdsp_stat3
    }
}
#[doc = "PR1_CFG__SLV__REGS_pid_reg (rw) register accessor: PR1_CFG__SLV__REGS_pid_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pid_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pid_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pid_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pid_reg")]
pub type Pr1Cfg_Slv_RegsPidReg = crate::Reg<pr1_cfg__slv__regs_pid_reg::Pr1Cfg_Slv_RegsPidRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pid_reg"]
pub mod pr1_cfg__slv__regs_pid_reg;
#[doc = "PR1_CFG__SLV__REGS_hwdis_reg (rw) register accessor: PR1_CFG__SLV__REGS_hwdis_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_hwdis_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_hwdis_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_hwdis_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_hwdis_reg")]
pub type Pr1Cfg_Slv_RegsHwdisReg =
    crate::Reg<pr1_cfg__slv__regs_hwdis_reg::Pr1Cfg_Slv_RegsHwdisRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_hwdis_reg"]
pub mod pr1_cfg__slv__regs_hwdis_reg;
#[doc = "PR1_CFG__SLV__REGS_gpcfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_gpcfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpcfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpcfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_gpcfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_gpcfg0_reg")]
pub type Pr1Cfg_Slv_RegsGpcfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_gpcfg0_reg::Pr1Cfg_Slv_RegsGpcfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_gpcfg0_reg"]
pub mod pr1_cfg__slv__regs_gpcfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_gpcfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_gpcfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpcfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpcfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_gpcfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_gpcfg1_reg")]
pub type Pr1Cfg_Slv_RegsGpcfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_gpcfg1_reg::Pr1Cfg_Slv_RegsGpcfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_gpcfg1_reg"]
pub mod pr1_cfg__slv__regs_gpcfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_cgr_reg (rw) register accessor: PR1_CFG__SLV__REGS_cgr_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_cgr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_cgr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_cgr_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_cgr_reg")]
pub type Pr1Cfg_Slv_RegsCgrReg = crate::Reg<pr1_cfg__slv__regs_cgr_reg::Pr1Cfg_Slv_RegsCgrRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_cgr_reg"]
pub mod pr1_cfg__slv__regs_cgr_reg;
#[doc = "PR1_CFG__SLV__REGS_gpecfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_gpecfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpecfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpecfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_gpecfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_gpecfg0_reg")]
pub type Pr1Cfg_Slv_RegsGpecfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_gpecfg0_reg::Pr1Cfg_Slv_RegsGpecfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_gpecfg0_reg"]
pub mod pr1_cfg__slv__regs_gpecfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_gpecfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_gpecfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpecfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpecfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_gpecfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_gpecfg1_reg")]
pub type Pr1Cfg_Slv_RegsGpecfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_gpecfg1_reg::Pr1Cfg_Slv_RegsGpecfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_gpecfg1_reg"]
pub mod pr1_cfg__slv__regs_gpecfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_reset_iso_reg (rw) register accessor: PR1_CFG__SLV__REGS_reset_iso_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_reset_iso_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_reset_iso_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_reset_iso_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_reset_iso_reg")]
pub type Pr1Cfg_Slv_RegsResetIsoReg =
    crate::Reg<pr1_cfg__slv__regs_reset_iso_reg::Pr1Cfg_Slv_RegsResetIsoRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_reset_iso_reg"]
pub mod pr1_cfg__slv__regs_reset_iso_reg;
#[doc = "PR1_CFG__SLV__REGS_mii_rt_reg (rw) register accessor: PR1_CFG__SLV__REGS_mii_rt_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_mii_rt_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_mii_rt_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_mii_rt_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_mii_rt_reg")]
pub type Pr1Cfg_Slv_RegsMiiRtReg =
    crate::Reg<pr1_cfg__slv__regs_mii_rt_reg::Pr1Cfg_Slv_RegsMiiRtRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_mii_rt_reg"]
pub mod pr1_cfg__slv__regs_mii_rt_reg;
#[doc = "PR1_CFG__SLV__REGS_iepclk_reg (rw) register accessor: PR1_CFG__SLV__REGS_iepclk_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_iepclk_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_iepclk_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_iepclk_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_iepclk_reg")]
pub type Pr1Cfg_Slv_RegsIepclkReg =
    crate::Reg<pr1_cfg__slv__regs_iepclk_reg::Pr1Cfg_Slv_RegsIepclkRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_iepclk_reg"]
pub mod pr1_cfg__slv__regs_iepclk_reg;
#[doc = "PR1_CFG__SLV__REGS_spp_reg (rw) register accessor: PR1_CFG__SLV__REGS_spp_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_spp_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_spp_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_spp_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_spp_reg")]
pub type Pr1Cfg_Slv_RegsSppReg = crate::Reg<pr1_cfg__slv__regs_spp_reg::Pr1Cfg_Slv_RegsSppRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_spp_reg"]
pub mod pr1_cfg__slv__regs_spp_reg;
#[doc = "PR1_CFG__SLV__REGS_core_sync_reg (rw) register accessor: PR1_CFG__SLV__REGS_core_sync_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_core_sync_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_core_sync_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_core_sync_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_core_sync_reg")]
pub type Pr1Cfg_Slv_RegsCoreSyncReg =
    crate::Reg<pr1_cfg__slv__regs_core_sync_reg::Pr1Cfg_Slv_RegsCoreSyncRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_core_sync_reg"]
pub mod pr1_cfg__slv__regs_core_sync_reg;
#[doc = "PR1_CFG__SLV__REGS_sa_mx_reg (rw) register accessor: PR1_CFG__SLV__REGS_sa_mx_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_sa_mx_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_sa_mx_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_sa_mx_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_sa_mx_reg")]
pub type Pr1Cfg_Slv_RegsSaMxReg =
    crate::Reg<pr1_cfg__slv__regs_sa_mx_reg::Pr1Cfg_Slv_RegsSaMxRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_sa_mx_reg"]
pub mod pr1_cfg__slv__regs_sa_mx_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_div_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_div_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_div_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg")]
pub type Pr1Cfg_Slv_RegsPru0SdClkDivReg =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_div_reg::Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_div_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg0 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg0;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg1 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg1")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg1 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1::Pr1Cfg_Slv_RegsPru0SdClkSelReg1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg1"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg1;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg1;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg2 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg2;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg3 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg3")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg3 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3::Pr1Cfg_Slv_RegsPru0SdClkSelReg3Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg3"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg3;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg3;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg4 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg4")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg4 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4::Pr1Cfg_Slv_RegsPru0SdClkSelReg4Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg4"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg4;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg4 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg4`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg4")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg4 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg4::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg4Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg4"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg4;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg5 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg5 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg5`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg5")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg5 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg5::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg5Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg5"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg5;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg6 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg6`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg6;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg7 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg7 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg7`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg7")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg7 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg7::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg7Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg7"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg7;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8")]
pub type Pr1Cfg_Slv_RegsPru0SdClkSelReg8 =
    crate::Reg<pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8"]
pub mod pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8 (rw) register accessor: PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_sd_sample_size_reg8`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8")]
pub type Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8 = crate::Reg<
    pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8"]
pub mod pr1_cfg__slv__regs_pru0_sd_sample_size_reg8;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_div_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_div_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_div_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg")]
pub type Pr1Cfg_Slv_RegsPru1SdClkDivReg =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_div_reg::Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_div_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg0 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg0")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg0 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0::Pr1Cfg_Slv_RegsPru1SdClkSelReg0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg0"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg0;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg0;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg1 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg1;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg2 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg2;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg3 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg3;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg4 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg4`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg4;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg5 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg5")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg5 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5::Pr1Cfg_Slv_RegsPru1SdClkSelReg5Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg5"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg5;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg5`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg5;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg6 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg6")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg6 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6::Pr1Cfg_Slv_RegsPru1SdClkSelReg6Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg6"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg6;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg6`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg6;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg7 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg7`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg7;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg8 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg8")]
pub type Pr1Cfg_Slv_RegsPru1SdClkSelReg8 =
    crate::Reg<pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8::Pr1Cfg_Slv_RegsPru1SdClkSelReg8Spec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg8"]
pub mod pr1_cfg__slv__regs_pru1_sd_clk_sel_reg8;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg8 (rw) register accessor: PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_sd_sample_size_reg8`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg8")]
pub type Pr1Cfg_Slv_RegsPru1SdSampleSizeReg8 = crate::Reg<
    pr1_cfg__slv__regs_pru1_sd_sample_size_reg8::Pr1Cfg_Slv_RegsPru1SdSampleSizeReg8Spec,
>;
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg8"]
pub mod pr1_cfg__slv__regs_pru1_sd_sample_size_reg8;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_rx_cfg_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_rx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_rx_cfg_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdRxCfgReg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg::Pr1Cfg_Slv_RegsPru0EdRxCfgRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_rx_cfg_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_rx_cfg_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdTxCfgReg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg0_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdCh0Cfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg::Pr1Cfg_Slv_RegsPru0EdCh0Cfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg0_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_ch0_cfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg1_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdCh0Cfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg::Pr1Cfg_Slv_RegsPru0EdCh0Cfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch0_cfg1_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_ch0_cfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg0_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdCh1Cfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg::Pr1Cfg_Slv_RegsPru0EdCh1Cfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg0_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_ch1_cfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg1_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdCh1Cfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg::Pr1Cfg_Slv_RegsPru0EdCh1Cfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch1_cfg1_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_ch1_cfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdCh2Cfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg::Pr1Cfg_Slv_RegsPru0EdCh2Cfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg0_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_ch2_cfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg1_reg")]
pub type Pr1Cfg_Slv_RegsPru0EdCh2Cfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg::Pr1Cfg_Slv_RegsPru0EdCh2Cfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_ch2_cfg1_reg"]
pub mod pr1_cfg__slv__regs_pru0_ed_ch2_cfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdRxCfgReg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdTxCfgReg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdCh0Cfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg::Pr1Cfg_Slv_RegsPru1EdCh0Cfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg0_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_ch0_cfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg1_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdCh0Cfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg::Pr1Cfg_Slv_RegsPru1EdCh0Cfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch0_cfg1_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_ch0_cfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdCh1Cfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg::Pr1Cfg_Slv_RegsPru1EdCh1Cfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg0_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_ch1_cfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg1_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdCh1Cfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg::Pr1Cfg_Slv_RegsPru1EdCh1Cfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch1_cfg1_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_ch1_cfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg0_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg0_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdCh2Cfg0Reg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg::Pr1Cfg_Slv_RegsPru1EdCh2Cfg0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg0_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_ch2_cfg0_reg;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg (rw) register accessor: PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg")]
pub type Pr1Cfg_Slv_RegsPru1EdCh2Cfg1Reg =
    crate::Reg<pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg::Pr1Cfg_Slv_RegsPru1EdCh2Cfg1RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_ch2_cfg1_reg"]
pub mod pr1_cfg__slv__regs_pru1_ed_ch2_cfg1_reg;
#[doc = "PR1_CFG__SLV__REGS_rtu0_poke_en0_reg (rw) register accessor: PR1_CFG__SLV__REGS_rtu0_poke_en0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_rtu0_poke_en0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_rtu0_poke_en0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_rtu0_poke_en0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_rtu0_poke_en0_reg")]
pub type Pr1Cfg_Slv_RegsRtu0PokeEn0Reg =
    crate::Reg<pr1_cfg__slv__regs_rtu0_poke_en0_reg::Pr1Cfg_Slv_RegsRtu0PokeEn0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_rtu0_poke_en0_reg"]
pub mod pr1_cfg__slv__regs_rtu0_poke_en0_reg;
#[doc = "PR1_CFG__SLV__REGS_rtu1_poke_en0_reg (rw) register accessor: PR1_CFG__SLV__REGS_rtu1_poke_en0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_rtu1_poke_en0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_rtu1_poke_en0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_rtu1_poke_en0_reg`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_rtu1_poke_en0_reg")]
pub type Pr1Cfg_Slv_RegsRtu1PokeEn0Reg =
    crate::Reg<pr1_cfg__slv__regs_rtu1_poke_en0_reg::Pr1Cfg_Slv_RegsRtu1PokeEn0RegSpec>;
#[doc = "PR1_CFG__SLV__REGS_rtu1_poke_en0_reg"]
pub mod pr1_cfg__slv__regs_rtu1_poke_en0_reg;
#[doc = "PR1_CFG__SLV__REGS_pwm0 (rw) register accessor: PR1_CFG__SLV__REGS_pwm0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm0")]
pub type Pr1Cfg_Slv_RegsPwm0 = crate::Reg<pr1_cfg__slv__regs_pwm0::Pr1Cfg_Slv_RegsPwm0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm0"]
pub mod pr1_cfg__slv__regs_pwm0;
#[doc = "PR1_CFG__SLV__REGS_pwm1 (rw) register accessor: PR1_CFG__SLV__REGS_pwm1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm1")]
pub type Pr1Cfg_Slv_RegsPwm1 = crate::Reg<pr1_cfg__slv__regs_pwm1::Pr1Cfg_Slv_RegsPwm1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm1"]
pub mod pr1_cfg__slv__regs_pwm1;
#[doc = "PR1_CFG__SLV__REGS_pwm2 (rw) register accessor: PR1_CFG__SLV__REGS_pwm2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm2")]
pub type Pr1Cfg_Slv_RegsPwm2 = crate::Reg<pr1_cfg__slv__regs_pwm2::Pr1Cfg_Slv_RegsPwm2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm2"]
pub mod pr1_cfg__slv__regs_pwm2;
#[doc = "PR1_CFG__SLV__REGS_pwm3 (rw) register accessor: PR1_CFG__SLV__REGS_pwm3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm3")]
pub type Pr1Cfg_Slv_RegsPwm3 = crate::Reg<pr1_cfg__slv__regs_pwm3::Pr1Cfg_Slv_RegsPwm3Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm3"]
pub mod pr1_cfg__slv__regs_pwm3;
#[doc = "PR1_CFG__SLV__REGS_pwm0_0 (rw) register accessor: PR1_CFG__SLV__REGS_pwm0_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm0_0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm0_0")]
pub type Pr1Cfg_Slv_RegsPwm0_0 = crate::Reg<pr1_cfg__slv__regs_pwm0_0::Pr1Cfg_Slv_RegsPwm0_0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm0_0"]
pub mod pr1_cfg__slv__regs_pwm0_0;
#[doc = "PR1_CFG__SLV__REGS_pwm0_1 (rw) register accessor: PR1_CFG__SLV__REGS_pwm0_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm0_1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm0_1")]
pub type Pr1Cfg_Slv_RegsPwm0_1 = crate::Reg<pr1_cfg__slv__regs_pwm0_1::Pr1Cfg_Slv_RegsPwm0_1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm0_1"]
pub mod pr1_cfg__slv__regs_pwm0_1;
#[doc = "PR1_CFG__SLV__REGS_pwm0_2 (rw) register accessor: PR1_CFG__SLV__REGS_pwm0_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm0_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm0_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm0_2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm0_2")]
pub type Pr1Cfg_Slv_RegsPwm0_2 = crate::Reg<pr1_cfg__slv__regs_pwm0_2::Pr1Cfg_Slv_RegsPwm0_2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm0_2"]
pub mod pr1_cfg__slv__regs_pwm0_2;
#[doc = "PR1_CFG__SLV__REGS_pwm1_0 (rw) register accessor: PR1_CFG__SLV__REGS_pwm1_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm1_0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm1_0")]
pub type Pr1Cfg_Slv_RegsPwm1_0 = crate::Reg<pr1_cfg__slv__regs_pwm1_0::Pr1Cfg_Slv_RegsPwm1_0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm1_0"]
pub mod pr1_cfg__slv__regs_pwm1_0;
#[doc = "PR1_CFG__SLV__REGS_pwm1_1 (rw) register accessor: PR1_CFG__SLV__REGS_pwm1_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm1_1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm1_1")]
pub type Pr1Cfg_Slv_RegsPwm1_1 = crate::Reg<pr1_cfg__slv__regs_pwm1_1::Pr1Cfg_Slv_RegsPwm1_1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm1_1"]
pub mod pr1_cfg__slv__regs_pwm1_1;
#[doc = "PR1_CFG__SLV__REGS_pwm1_2 (rw) register accessor: PR1_CFG__SLV__REGS_pwm1_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm1_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm1_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm1_2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm1_2")]
pub type Pr1Cfg_Slv_RegsPwm1_2 = crate::Reg<pr1_cfg__slv__regs_pwm1_2::Pr1Cfg_Slv_RegsPwm1_2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm1_2"]
pub mod pr1_cfg__slv__regs_pwm1_2;
#[doc = "PR1_CFG__SLV__REGS_pwm2_0 (rw) register accessor: PR1_CFG__SLV__REGS_pwm2_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm2_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm2_0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm2_0")]
pub type Pr1Cfg_Slv_RegsPwm2_0 = crate::Reg<pr1_cfg__slv__regs_pwm2_0::Pr1Cfg_Slv_RegsPwm2_0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm2_0"]
pub mod pr1_cfg__slv__regs_pwm2_0;
#[doc = "PR1_CFG__SLV__REGS_pwm2_1 (rw) register accessor: PR1_CFG__SLV__REGS_pwm2_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm2_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm2_1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm2_1")]
pub type Pr1Cfg_Slv_RegsPwm2_1 = crate::Reg<pr1_cfg__slv__regs_pwm2_1::Pr1Cfg_Slv_RegsPwm2_1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm2_1"]
pub mod pr1_cfg__slv__regs_pwm2_1;
#[doc = "PR1_CFG__SLV__REGS_pwm2_2 (rw) register accessor: PR1_CFG__SLV__REGS_pwm2_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm2_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm2_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm2_2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm2_2")]
pub type Pr1Cfg_Slv_RegsPwm2_2 = crate::Reg<pr1_cfg__slv__regs_pwm2_2::Pr1Cfg_Slv_RegsPwm2_2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm2_2"]
pub mod pr1_cfg__slv__regs_pwm2_2;
#[doc = "PR1_CFG__SLV__REGS_pwm3_0 (rw) register accessor: PR1_CFG__SLV__REGS_pwm3_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm3_0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm3_0")]
pub type Pr1Cfg_Slv_RegsPwm3_0 = crate::Reg<pr1_cfg__slv__regs_pwm3_0::Pr1Cfg_Slv_RegsPwm3_0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm3_0"]
pub mod pr1_cfg__slv__regs_pwm3_0;
#[doc = "PR1_CFG__SLV__REGS_pwm3_1 (rw) register accessor: PR1_CFG__SLV__REGS_pwm3_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm3_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm3_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm3_1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm3_1")]
pub type Pr1Cfg_Slv_RegsPwm3_1 = crate::Reg<pr1_cfg__slv__regs_pwm3_1::Pr1Cfg_Slv_RegsPwm3_1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm3_1"]
pub mod pr1_cfg__slv__regs_pwm3_1;
#[doc = "PR1_CFG__SLV__REGS_pwm3_2 (rw) register accessor: PR1_CFG__SLV__REGS_pwm3_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pwm3_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pwm3_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pwm3_2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pwm3_2")]
pub type Pr1Cfg_Slv_RegsPwm3_2 = crate::Reg<pr1_cfg__slv__regs_pwm3_2::Pr1Cfg_Slv_RegsPwm3_2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pwm3_2"]
pub mod pr1_cfg__slv__regs_pwm3_2;
#[doc = "PR1_CFG__SLV__REGS_spin_lock0 (rw) register accessor: PR1_CFG__SLV__REGS_spin_lock0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_spin_lock0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_spin_lock0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_spin_lock0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_spin_lock0")]
pub type Pr1Cfg_Slv_RegsSpinLock0 =
    crate::Reg<pr1_cfg__slv__regs_spin_lock0::Pr1Cfg_Slv_RegsSpinLock0Spec>;
#[doc = "PR1_CFG__SLV__REGS_spin_lock0"]
pub mod pr1_cfg__slv__regs_spin_lock0;
#[doc = "PR1_CFG__SLV__REGS_spin_lock1 (rw) register accessor: PR1_CFG__SLV__REGS_spin_lock1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_spin_lock1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_spin_lock1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_spin_lock1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_spin_lock1")]
pub type Pr1Cfg_Slv_RegsSpinLock1 =
    crate::Reg<pr1_cfg__slv__regs_spin_lock1::Pr1Cfg_Slv_RegsSpinLock1Spec>;
#[doc = "PR1_CFG__SLV__REGS_spin_lock1"]
pub mod pr1_cfg__slv__regs_spin_lock1;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_cfg0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0")]
pub type Pr1Cfg_Slv_RegsPaStatPdspCfg0 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_cfg0::Pr1Cfg_Slv_RegsPaStatPdspCfg0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg0"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_cfg0;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_stat0`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0")]
pub type Pr1Cfg_Slv_RegsPaStatPdspStat0 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_stat0::Pr1Cfg_Slv_RegsPaStatPdspStat0Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat0"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_stat0;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg1 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_cfg1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg1")]
pub type Pr1Cfg_Slv_RegsPaStatPdspCfg1 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_cfg1::Pr1Cfg_Slv_RegsPaStatPdspCfg1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg1"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_cfg1;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat1 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_stat1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_stat1`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat1")]
pub type Pr1Cfg_Slv_RegsPaStatPdspStat1 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_stat1::Pr1Cfg_Slv_RegsPaStatPdspStat1Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat1"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_stat1;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg2 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_cfg2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg2")]
pub type Pr1Cfg_Slv_RegsPaStatPdspCfg2 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_cfg2::Pr1Cfg_Slv_RegsPaStatPdspCfg2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg2"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_cfg2;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_stat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_stat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_stat2`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2")]
pub type Pr1Cfg_Slv_RegsPaStatPdspStat2 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_stat2::Pr1Cfg_Slv_RegsPaStatPdspStat2Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat2"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_stat2;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg3 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_cfg3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg3")]
pub type Pr1Cfg_Slv_RegsPaStatPdspCfg3 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_cfg3::Pr1Cfg_Slv_RegsPaStatPdspCfg3Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_cfg3"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_cfg3;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat3 (rw) register accessor: PR1_CFG__SLV__REGS_pa_stat_pdsp_stat3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pa_stat_pdsp_stat3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pa_stat_pdsp_stat3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_cfg__slv__regs_pa_stat_pdsp_stat3`]
module"]
#[doc(alias = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat3")]
pub type Pr1Cfg_Slv_RegsPaStatPdspStat3 =
    crate::Reg<pr1_cfg__slv__regs_pa_stat_pdsp_stat3::Pr1Cfg_Slv_RegsPaStatPdspStat3Spec>;
#[doc = "PR1_CFG__SLV__REGS_pa_stat_pdsp_stat3"]
pub mod pr1_cfg__slv__regs_pa_stat_pdsp_stat3;
