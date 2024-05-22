#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg0_pid: Cfg0Pid,
    _reserved1: [u8; 0x04],
    cfg0_mmr_cfg1: Cfg0MmrCfg1,
    _reserved2: [u8; 0x0ffc],
    cfg0_lock0_kick0: Cfg0Lock0Kick0,
    cfg0_lock0_kick1: Cfg0Lock0Kick1,
    cfg0_intr_raw_status: Cfg0IntrRawStatus,
    cfg0_intr_enabled_status_clear: Cfg0IntrEnabledStatusClear,
    cfg0_intr_enable: Cfg0IntrEnable,
    cfg0_intr_enable_clear: Cfg0IntrEnableClear,
    cfg0_eoi: Cfg0Eoi,
    cfg0_fault_address: Cfg0FaultAddress,
    cfg0_fault_type_status: Cfg0FaultTypeStatus,
    cfg0_fault_attr_status: Cfg0FaultAttrStatus,
    cfg0_fault_clear: Cfg0FaultClear,
    _reserved13: [u8; 0xcc],
    cfg0_claimreg_p0_r0_readonly: Cfg0ClaimregP0R0Readonly,
    _reserved14: [u8; 0x0efc],
    cfg0_pid_proxy: Cfg0PidProxy,
    _reserved15: [u8; 0x04],
    cfg0_mmr_cfg1_proxy: Cfg0MmrCfg1Proxy,
    _reserved16: [u8; 0x0ffc],
    cfg0_lock0_kick0_proxy: Cfg0Lock0Kick0Proxy,
    cfg0_lock0_kick1_proxy: Cfg0Lock0Kick1Proxy,
    cfg0_intr_raw_status_proxy: Cfg0IntrRawStatusProxy,
    cfg0_intr_enabled_status_clear_proxy: Cfg0IntrEnabledStatusClearProxy,
    cfg0_intr_enable_proxy: Cfg0IntrEnableProxy,
    cfg0_intr_enable_clear_proxy: Cfg0IntrEnableClearProxy,
    cfg0_eoi_proxy: Cfg0EoiProxy,
    cfg0_fault_address_proxy: Cfg0FaultAddressProxy,
    cfg0_fault_type_status_proxy: Cfg0FaultTypeStatusProxy,
    cfg0_fault_attr_status_proxy: Cfg0FaultAttrStatusProxy,
    cfg0_fault_clear_proxy: Cfg0FaultClearProxy,
    _reserved27: [u8; 0xcc],
    cfg0_claimreg_p0_r0: Cfg0ClaimregP0R0,
    _reserved28: [u8; 0x0f80],
    cfg0_dbounce_cfg1: Cfg0DbounceCfg1,
    cfg0_dbounce_cfg2: Cfg0DbounceCfg2,
    cfg0_dbounce_cfg3: Cfg0DbounceCfg3,
    cfg0_dbounce_cfg4: Cfg0DbounceCfg4,
    cfg0_dbounce_cfg5: Cfg0DbounceCfg5,
    cfg0_dbounce_cfg6: Cfg0DbounceCfg6,
    _reserved34: [u8; 0x04],
    cfg0_temp_diode_trim: Cfg0TempDiodeTrim,
    _reserved35: [u8; 0x0c],
    cfg0_io_voltage_stat: Cfg0IoVoltageStat,
    _reserved36: [u8; 0x0150],
    cfg0_mcu_timer1_ctrl: Cfg0McuTimer1Ctrl,
    _reserved37: [u8; 0x04],
    cfg0_mcu_timer3_ctrl: Cfg0McuTimer3Ctrl,
    _reserved38: [u8; 0xd0],
    cfg0_mcu_i2c0_ctrl: Cfg0McuI2c0Ctrl,
    _reserved39: [u8; 0x031c],
    cfg0_mcu_mtog_ctrl: Cfg0McuMtogCtrl,
    _reserved40: [u8; 0x0a04],
    cfg0_lock1_kick0: Cfg0Lock1Kick0,
    cfg0_lock1_kick1: Cfg0Lock1Kick1,
    _reserved42: [u8; 0xf0],
    cfg0_claimreg_p1_r0_readonly: Cfg0ClaimregP1R0Readonly,
    cfg0_claimreg_p1_r1_readonly: Cfg0ClaimregP1R1Readonly,
    cfg0_claimreg_p1_r2_readonly: Cfg0ClaimregP1R2Readonly,
    cfg0_claimreg_p1_r3_readonly: Cfg0ClaimregP1R3Readonly,
    cfg0_claimreg_p1_r4_readonly: Cfg0ClaimregP1R4Readonly,
    cfg0_claimreg_p1_r5_readonly: Cfg0ClaimregP1R5Readonly,
    cfg0_claimreg_p1_r6_readonly: Cfg0ClaimregP1R6Readonly,
    cfg0_claimreg_p1_r7_readonly: Cfg0ClaimregP1R7Readonly,
    cfg0_claimreg_p1_r8_readonly: Cfg0ClaimregP1R8Readonly,
    cfg0_claimreg_p1_r9_readonly: Cfg0ClaimregP1R9Readonly,
    cfg0_claimreg_p1_r10_readonly: Cfg0ClaimregP1R10Readonly,
    cfg0_claimreg_p1_r11_readonly: Cfg0ClaimregP1R11Readonly,
    cfg0_claimreg_p1_r12_readonly: Cfg0ClaimregP1R12Readonly,
    _reserved55: [u8; 0x0f50],
    cfg0_dbounce_cfg1_proxy: Cfg0DbounceCfg1Proxy,
    cfg0_dbounce_cfg2_proxy: Cfg0DbounceCfg2Proxy,
    cfg0_dbounce_cfg3_proxy: Cfg0DbounceCfg3Proxy,
    cfg0_dbounce_cfg4_proxy: Cfg0DbounceCfg4Proxy,
    cfg0_dbounce_cfg5_proxy: Cfg0DbounceCfg5Proxy,
    cfg0_dbounce_cfg6_proxy: Cfg0DbounceCfg6Proxy,
    _reserved61: [u8; 0x04],
    cfg0_temp_diode_trim_proxy: Cfg0TempDiodeTrimProxy,
    _reserved62: [u8; 0x0c],
    cfg0_io_voltage_stat_proxy: Cfg0IoVoltageStatProxy,
    _reserved63: [u8; 0x0150],
    cfg0_mcu_timer1_ctrl_proxy: Cfg0McuTimer1CtrlProxy,
    _reserved64: [u8; 0x04],
    cfg0_mcu_timer3_ctrl_proxy: Cfg0McuTimer3CtrlProxy,
    _reserved65: [u8; 0xd0],
    cfg0_mcu_i2c0_ctrl_proxy: Cfg0McuI2c0CtrlProxy,
    _reserved66: [u8; 0x031c],
    cfg0_mcu_mtog_ctrl_proxy: Cfg0McuMtogCtrlProxy,
    _reserved67: [u8; 0x0a04],
    cfg0_lock1_kick0_proxy: Cfg0Lock1Kick0Proxy,
    cfg0_lock1_kick1_proxy: Cfg0Lock1Kick1Proxy,
    _reserved69: [u8; 0xf0],
    cfg0_claimreg_p1_r0: Cfg0ClaimregP1R0,
    cfg0_claimreg_p1_r1: Cfg0ClaimregP1R1,
    cfg0_claimreg_p1_r2: Cfg0ClaimregP1R2,
    cfg0_claimreg_p1_r3: Cfg0ClaimregP1R3,
    cfg0_claimreg_p1_r4: Cfg0ClaimregP1R4,
    cfg0_claimreg_p1_r5: Cfg0ClaimregP1R5,
    cfg0_claimreg_p1_r6: Cfg0ClaimregP1R6,
    cfg0_claimreg_p1_r7: Cfg0ClaimregP1R7,
    cfg0_claimreg_p1_r8: Cfg0ClaimregP1R8,
    cfg0_claimreg_p1_r9: Cfg0ClaimregP1R9,
    cfg0_claimreg_p1_r10: Cfg0ClaimregP1R10,
    cfg0_claimreg_p1_r11: Cfg0ClaimregP1R11,
    cfg0_claimreg_p1_r12: Cfg0ClaimregP1R12,
    _reserved82: [u8; 0x0ecc],
    cfg0_mcu_obsclk_ctrl: Cfg0McuObsclkCtrl,
    _reserved83: [u8; 0x0c],
    cfg0_hfosc0_ctrl: Cfg0Hfosc0Ctrl,
    _reserved84: [u8; 0x04],
    cfg0_hfosc0_trim: Cfg0Hfosc0Trim,
    _reserved85: [u8; 0x08],
    cfg0_rc12m_osc_trim: Cfg0Rc12mOscTrim,
    _reserved86: [u8; 0x08],
    cfg0_hfosc0_clkout_32k_ctrl: Cfg0Hfosc0Clkout32kCtrl,
    _reserved87: [u8; 0x0c],
    cfg0_mcu_m4fss_clksel: Cfg0McuM4fssClksel,
    cfg0_mcu_m4fss_systick: Cfg0McuM4fssSystick,
    _reserved89: [u8; 0x08],
    cfg0_mcu_pll_clksel: Cfg0McuPllClksel,
    _reserved90: [u8; 0x0c],
    cfg0_mcu_timer0_clksel: Cfg0McuTimer0Clksel,
    cfg0_mcu_timer1_clksel: Cfg0McuTimer1Clksel,
    cfg0_mcu_timer2_clksel: Cfg0McuTimer2Clksel,
    cfg0_mcu_timer3_clksel: Cfg0McuTimer3Clksel,
    _reserved94: [u8; 0x30],
    cfg0_mcu_spi0_clksel: Cfg0McuSpi0Clksel,
    cfg0_mcu_spi1_clksel: Cfg0McuSpi1Clksel,
    _reserved96: [u8; 0x08],
    cfg0_mcu_wwd0_clksel: Cfg0McuWwd0Clksel,
    _reserved97: [u8; 0x1c],
    cfg0_ddr16ss_pmctrl: Cfg0Ddr16ssPmctrl,
    _reserved98: [u8; 0x0f34],
    cfg0_lock2_kick0: Cfg0Lock2Kick0,
    cfg0_lock2_kick1: Cfg0Lock2Kick1,
    _reserved100: [u8; 0xf0],
    cfg0_claimreg_p2_r0_readonly: Cfg0ClaimregP2R0Readonly,
    cfg0_claimreg_p2_r1_readonly: Cfg0ClaimregP2R1Readonly,
    _reserved102: [u8; 0x0ef8],
    cfg0_mcu_obsclk_ctrl_proxy: Cfg0McuObsclkCtrlProxy,
    _reserved103: [u8; 0x0c],
    cfg0_hfosc0_ctrl_proxy: Cfg0Hfosc0CtrlProxy,
    _reserved104: [u8; 0x04],
    cfg0_hfosc0_trim_proxy: Cfg0Hfosc0TrimProxy,
    _reserved105: [u8; 0x08],
    cfg0_rc12m_osc_trim_proxy: Cfg0Rc12mOscTrimProxy,
    _reserved106: [u8; 0x08],
    cfg0_hfosc0_clkout_32k_ctrl_proxy: Cfg0Hfosc0Clkout32kCtrlProxy,
    _reserved107: [u8; 0x0c],
    cfg0_mcu_m4fss_clksel_proxy: Cfg0McuM4fssClkselProxy,
    cfg0_mcu_m4fss_systick_proxy: Cfg0McuM4fssSystickProxy,
    _reserved109: [u8; 0x08],
    cfg0_mcu_pll_clksel_proxy: Cfg0McuPllClkselProxy,
    _reserved110: [u8; 0x0c],
    cfg0_mcu_timer0_clksel_proxy: Cfg0McuTimer0ClkselProxy,
    cfg0_mcu_timer1_clksel_proxy: Cfg0McuTimer1ClkselProxy,
    cfg0_mcu_timer2_clksel_proxy: Cfg0McuTimer2ClkselProxy,
    cfg0_mcu_timer3_clksel_proxy: Cfg0McuTimer3ClkselProxy,
    _reserved114: [u8; 0x30],
    cfg0_mcu_spi0_clksel_proxy: Cfg0McuSpi0ClkselProxy,
    cfg0_mcu_spi1_clksel_proxy: Cfg0McuSpi1ClkselProxy,
    _reserved116: [u8; 0x08],
    cfg0_mcu_wwd0_clksel_proxy: Cfg0McuWwd0ClkselProxy,
    _reserved117: [u8; 0x1c],
    cfg0_ddr16ss_pmctrl_proxy: Cfg0Ddr16ssPmctrlProxy,
    _reserved118: [u8; 0x0f34],
    cfg0_lock2_kick0_proxy: Cfg0Lock2Kick0Proxy,
    cfg0_lock2_kick1_proxy: Cfg0Lock2Kick1Proxy,
    _reserved120: [u8; 0xf0],
    cfg0_claimreg_p2_r0: Cfg0ClaimregP2R0,
    cfg0_claimreg_p2_r1: Cfg0ClaimregP2R1,
    _reserved122: [u8; 0x0f18],
    cfg0_mcu_m4fss0_lbist_ctrl: Cfg0McuM4fss0LbistCtrl,
    cfg0_mcu_m4fss0_lbist_patcount: Cfg0McuM4fss0LbistPatcount,
    cfg0_mcu_m4fss0_lbist_seed0: Cfg0McuM4fss0LbistSeed0,
    cfg0_mcu_m4fss0_lbist_seed1: Cfg0McuM4fss0LbistSeed1,
    cfg0_mcu_m4fss0_lbist_spare0: Cfg0McuM4fss0LbistSpare0,
    cfg0_mcu_m4fss0_lbist_spare1: Cfg0McuM4fss0LbistSpare1,
    cfg0_mcu_m4fss0_lbist_stat: Cfg0McuM4fss0LbistStat,
    cfg0_mcu_m4fss0_lbist_misr: Cfg0McuM4fss0LbistMisr,
    _reserved130: [u8; 0x0fc8],
    cfg0_lock3_kick0: Cfg0Lock3Kick0,
    cfg0_lock3_kick1: Cfg0Lock3Kick1,
    _reserved132: [u8; 0xf0],
    cfg0_claimreg_p3_r0_readonly: Cfg0ClaimregP3R0Readonly,
    _reserved133: [u8; 0x0f1c],
    cfg0_mcu_m4fss0_lbist_ctrl_proxy: Cfg0McuM4fss0LbistCtrlProxy,
    cfg0_mcu_m4fss0_lbist_patcount_proxy: Cfg0McuM4fss0LbistPatcountProxy,
    cfg0_mcu_m4fss0_lbist_seed0_proxy: Cfg0McuM4fss0LbistSeed0Proxy,
    cfg0_mcu_m4fss0_lbist_seed1_proxy: Cfg0McuM4fss0LbistSeed1Proxy,
    cfg0_mcu_m4fss0_lbist_spare0_proxy: Cfg0McuM4fss0LbistSpare0Proxy,
    cfg0_mcu_m4fss0_lbist_spare1_proxy: Cfg0McuM4fss0LbistSpare1Proxy,
    cfg0_mcu_m4fss0_lbist_stat_proxy: Cfg0McuM4fss0LbistStatProxy,
    cfg0_mcu_m4fss0_lbist_misr_proxy: Cfg0McuM4fss0LbistMisrProxy,
    _reserved141: [u8; 0x0fc8],
    cfg0_lock3_kick0_proxy: Cfg0Lock3Kick0Proxy,
    cfg0_lock3_kick1_proxy: Cfg0Lock3Kick1Proxy,
    _reserved143: [u8; 0xf0],
    cfg0_claimreg_p3_r0: Cfg0ClaimregP3R0,
    _reserved144: [u8; 0x1f04],
    cfg0_lock4_kick0: Cfg0Lock4Kick0,
    cfg0_lock4_kick1: Cfg0Lock4Kick1,
    _reserved146: [u8; 0xf0],
    cfg0_claimreg_p4_r0_readonly: Cfg0ClaimregP4R0Readonly,
    cfg0_claimreg_p4_r1_readonly: Cfg0ClaimregP4R1Readonly,
    cfg0_claimreg_p4_r2_readonly: Cfg0ClaimregP4R2Readonly,
    cfg0_claimreg_p4_r3_readonly: Cfg0ClaimregP4R3Readonly,
    cfg0_claimreg_p4_r4_readonly: Cfg0ClaimregP4R4Readonly,
    cfg0_claimreg_p4_r5_readonly: Cfg0ClaimregP4R5Readonly,
    cfg0_claimreg_p4_r6_readonly: Cfg0ClaimregP4R6Readonly,
    cfg0_claimreg_p4_r7_readonly: Cfg0ClaimregP4R7Readonly,
    cfg0_claimreg_p4_r8_readonly: Cfg0ClaimregP4R8Readonly,
    cfg0_claimreg_p4_r9_readonly: Cfg0ClaimregP4R9Readonly,
    cfg0_claimreg_p4_r10_readonly: Cfg0ClaimregP4R10Readonly,
    cfg0_claimreg_p4_r11_readonly: Cfg0ClaimregP4R11Readonly,
    cfg0_claimreg_p4_r12_readonly: Cfg0ClaimregP4R12Readonly,
    cfg0_claimreg_p4_r13_readonly: Cfg0ClaimregP4R13Readonly,
    cfg0_claimreg_p4_r14_readonly: Cfg0ClaimregP4R14Readonly,
    _reserved161: [u8; 0x1ecc],
    cfg0_lock4_kick0_proxy: Cfg0Lock4Kick0Proxy,
    cfg0_lock4_kick1_proxy: Cfg0Lock4Kick1Proxy,
    _reserved163: [u8; 0xf0],
    cfg0_claimreg_p4_r0: Cfg0ClaimregP4R0,
    cfg0_claimreg_p4_r1: Cfg0ClaimregP4R1,
    cfg0_claimreg_p4_r2: Cfg0ClaimregP4R2,
    cfg0_claimreg_p4_r3: Cfg0ClaimregP4R3,
    cfg0_claimreg_p4_r4: Cfg0ClaimregP4R4,
    cfg0_claimreg_p4_r5: Cfg0ClaimregP4R5,
    cfg0_claimreg_p4_r6: Cfg0ClaimregP4R6,
    cfg0_claimreg_p4_r7: Cfg0ClaimregP4R7,
    cfg0_claimreg_p4_r8: Cfg0ClaimregP4R8,
    cfg0_claimreg_p4_r9: Cfg0ClaimregP4R9,
    cfg0_claimreg_p4_r10: Cfg0ClaimregP4R10,
    cfg0_claimreg_p4_r11: Cfg0ClaimregP4R11,
    cfg0_claimreg_p4_r12: Cfg0ClaimregP4R12,
    cfg0_claimreg_p4_r13: Cfg0ClaimregP4R13,
    cfg0_claimreg_p4_r14: Cfg0ClaimregP4R14,
    _reserved178: [u8; 0x4ec4],
    cfg0_por_ctrl: Cfg0PorCtrl,
    cfg0_por_stat: Cfg0PorStat,
    _reserved180: [u8; 0xf8],
    cfg0_por_bandgap_ctrl: Cfg0PorBandgapCtrl,
    _reserved181: [u8; 0x0c],
    cfg0_pok_vdda_mcu_uv_ctrl: Cfg0PokVddaMcuUvCtrl,
    cfg0_pok_vdda_mcu_ov_ctrl: Cfg0PokVddaMcuOvCtrl,
    cfg0_pok_vdd_core_uv_ctrl: Cfg0PokVddCoreUvCtrl,
    cfg0_pok_vdd_core_ov_ctrl: Cfg0PokVddCoreOvCtrl,
    cfg0_pok_vddr_core_uv_ctrl: Cfg0PokVddrCoreUvCtrl,
    cfg0_pok_vddr_core_ov_ctrl: Cfg0PokVddrCoreOvCtrl,
    cfg0_pok_vddshv_mcu_1p8_uv_ctrl: Cfg0PokVddshvMcu1p8UvCtrl,
    cfg0_pok_vddshv_mcu_1p8_ov_ctrl: Cfg0PokVddshvMcu1p8OvCtrl,
    cfg0_pok_vddshv_mcu_3p3_uv_ctrl: Cfg0PokVddshvMcu3p3UvCtrl,
    cfg0_pok_vddshv_mcu_3p3_ov_ctrl: Cfg0PokVddshvMcu3p3OvCtrl,
    cfg0_pok_vmon_cap_mcu_general_uv_ctrl: Cfg0PokVmonCapMcuGeneralUvCtrl,
    cfg0_pok_vmon_cap_mcu_general_ov_ctrl: Cfg0PokVmonCapMcuGeneralOvCtrl,
    cfg0_pok_vddshv_main_1p8_uv_ctrl: Cfg0PokVddshvMain1p8UvCtrl,
    cfg0_pok_vddshv_main_1p8_ov_ctrl: Cfg0PokVddshvMain1p8OvCtrl,
    cfg0_pok_vddshv_main_3p3_uv_ctrl: Cfg0PokVddshvMain3p3UvCtrl,
    cfg0_pok_vddshv_main_3p3_ov_ctrl: Cfg0PokVddshvMain3p3OvCtrl,
    cfg0_pok_vdds_ddrio_uv_ctrl: Cfg0PokVddsDdrioUvCtrl,
    cfg0_pok_vdds_ddrio_ov_ctrl: Cfg0PokVddsDdrioOvCtrl,
    _reserved199: [u8; 0x08],
    cfg0_pok_vdda_pmic_in_ctrl: Cfg0PokVddaPmicInCtrl,
    _reserved200: [u8; 0x0c],
    cfg0_rst_ctrl: Cfg0RstCtrl,
    cfg0_rst_stat: Cfg0RstStat,
    cfg0_rst_src: Cfg0RstSrc,
    cfg0_rst_magic_word: Cfg0RstMagicWord,
    cfg0_iso_ctrl: Cfg0IsoCtrl,
    _reserved205: [u8; 0x0c],
    cfg0_vdd_core_gldtc_ctrl: Cfg0VddCoreGldtcCtrl,
    _reserved206: [u8; 0x1c],
    cfg0_vdd_core_gldtc_stat: Cfg0VddCoreGldtcStat,
    _reserved207: [u8; 0x4c],
    cfg0_prg_pp_0_ctrl: Cfg0PrgPp0Ctrl,
    _reserved208: [u8; 0x04],
    cfg0_prg_pp_1_ctrl: Cfg0PrgPp1Ctrl,
    _reserved209: [u8; 0x78],
    cfg0_mcu_clkgate_ctrl: Cfg0McuClkgateCtrl,
    cfg0_main_clkgate_ctrl0: Cfg0MainClkgateCtrl0,
    _reserved211: [u8; 0x0d7c],
    cfg0_lock6_kick0: Cfg0Lock6Kick0,
    cfg0_lock6_kick1: Cfg0Lock6Kick1,
    _reserved213: [u8; 0xf0],
    cfg0_claimreg_p6_r0_readonly: Cfg0ClaimregP6R0Readonly,
    cfg0_claimreg_p6_r1_readonly: Cfg0ClaimregP6R1Readonly,
    cfg0_claimreg_p6_r2_readonly: Cfg0ClaimregP6R2Readonly,
    cfg0_claimreg_p6_r3_readonly: Cfg0ClaimregP6R3Readonly,
    cfg0_claimreg_p6_r4_readonly: Cfg0ClaimregP6R4Readonly,
    cfg0_claimreg_p6_r5_readonly: Cfg0ClaimregP6R5Readonly,
    _reserved219: [u8; 0x0ee8],
    cfg0_por_ctrl_proxy: Cfg0PorCtrlProxy,
    cfg0_por_stat_proxy: Cfg0PorStatProxy,
    _reserved221: [u8; 0xf8],
    cfg0_por_bandgap_ctrl_proxy: Cfg0PorBandgapCtrlProxy,
    _reserved222: [u8; 0x0c],
    cfg0_pok_vdda_mcu_uv_ctrl_proxy: Cfg0PokVddaMcuUvCtrlProxy,
    cfg0_pok_vdda_mcu_ov_ctrl_proxy: Cfg0PokVddaMcuOvCtrlProxy,
    cfg0_pok_vdd_core_uv_ctrl_proxy: Cfg0PokVddCoreUvCtrlProxy,
    cfg0_pok_vdd_core_ov_ctrl_proxy: Cfg0PokVddCoreOvCtrlProxy,
    cfg0_pok_vddr_core_uv_ctrl_proxy: Cfg0PokVddrCoreUvCtrlProxy,
    cfg0_pok_vddr_core_ov_ctrl_proxy: Cfg0PokVddrCoreOvCtrlProxy,
    cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy: Cfg0PokVddshvMcu1p8UvCtrlProxy,
    cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy: Cfg0PokVddshvMcu1p8OvCtrlProxy,
    cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy: Cfg0PokVddshvMcu3p3UvCtrlProxy,
    cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy: Cfg0PokVddshvMcu3p3OvCtrlProxy,
    cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy: Cfg0PokVmonCapMcuGeneralUvCtrlProxy,
    cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy: Cfg0PokVmonCapMcuGeneralOvCtrlProxy,
    cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy: Cfg0PokVddshvMain1p8UvCtrlProxy,
    cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy: Cfg0PokVddshvMain1p8OvCtrlProxy,
    cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy: Cfg0PokVddshvMain3p3UvCtrlProxy,
    cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy: Cfg0PokVddshvMain3p3OvCtrlProxy,
    cfg0_pok_vdds_ddrio_uv_ctrl_proxy: Cfg0PokVddsDdrioUvCtrlProxy,
    cfg0_pok_vdds_ddrio_ov_ctrl_proxy: Cfg0PokVddsDdrioOvCtrlProxy,
    _reserved240: [u8; 0x08],
    cfg0_pok_vdda_pmic_in_ctrl_proxy: Cfg0PokVddaPmicInCtrlProxy,
    _reserved241: [u8; 0x0c],
    cfg0_rst_ctrl_proxy: Cfg0RstCtrlProxy,
    cfg0_rst_stat_proxy: Cfg0RstStatProxy,
    cfg0_rst_src_proxy: Cfg0RstSrcProxy,
    cfg0_rst_magic_word_proxy: Cfg0RstMagicWordProxy,
    cfg0_iso_ctrl_proxy: Cfg0IsoCtrlProxy,
    _reserved246: [u8; 0x0c],
    cfg0_vdd_core_gldtc_ctrl_proxy: Cfg0VddCoreGldtcCtrlProxy,
    _reserved247: [u8; 0x1c],
    cfg0_vdd_core_gldtc_stat_proxy: Cfg0VddCoreGldtcStatProxy,
    _reserved248: [u8; 0x4c],
    cfg0_prg_pp_0_ctrl_proxy: Cfg0PrgPp0CtrlProxy,
    _reserved249: [u8; 0x04],
    cfg0_prg_pp_1_ctrl_proxy: Cfg0PrgPp1CtrlProxy,
    _reserved250: [u8; 0x78],
    cfg0_mcu_clkgate_ctrl_proxy: Cfg0McuClkgateCtrlProxy,
    cfg0_main_clkgate_ctrl0_proxy: Cfg0MainClkgateCtrl0Proxy,
    _reserved252: [u8; 0x0d7c],
    cfg0_lock6_kick0_proxy: Cfg0Lock6Kick0Proxy,
    cfg0_lock6_kick1_proxy: Cfg0Lock6Kick1Proxy,
    _reserved254: [u8; 0xf0],
    cfg0_claimreg_p6_r0: Cfg0ClaimregP6R0,
    cfg0_claimreg_p6_r1: Cfg0ClaimregP6R1,
    cfg0_claimreg_p6_r2: Cfg0ClaimregP6R2,
    cfg0_claimreg_p6_r3: Cfg0ClaimregP6R3,
    cfg0_claimreg_p6_r4: Cfg0ClaimregP6R4,
    cfg0_claimreg_p6_r5: Cfg0ClaimregP6R5,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG0_PID"]
    #[inline(always)]
    pub const fn cfg0_pid(&self) -> &Cfg0Pid {
        &self.cfg0_pid
    }
    #[doc = "0x08 - CFG0_MMR_CFG1"]
    #[inline(always)]
    pub const fn cfg0_mmr_cfg1(&self) -> &Cfg0MmrCfg1 {
        &self.cfg0_mmr_cfg1
    }
    #[doc = "0x1008 - CFG0_LOCK0_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock0_kick0(&self) -> &Cfg0Lock0Kick0 {
        &self.cfg0_lock0_kick0
    }
    #[doc = "0x100c - CFG0_LOCK0_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock0_kick1(&self) -> &Cfg0Lock0Kick1 {
        &self.cfg0_lock0_kick1
    }
    #[doc = "0x1010 - CFG0_intr_raw_status"]
    #[inline(always)]
    pub const fn cfg0_intr_raw_status(&self) -> &Cfg0IntrRawStatus {
        &self.cfg0_intr_raw_status
    }
    #[doc = "0x1014 - CFG0_intr_enabled_status_clear"]
    #[inline(always)]
    pub const fn cfg0_intr_enabled_status_clear(&self) -> &Cfg0IntrEnabledStatusClear {
        &self.cfg0_intr_enabled_status_clear
    }
    #[doc = "0x1018 - CFG0_intr_enable"]
    #[inline(always)]
    pub const fn cfg0_intr_enable(&self) -> &Cfg0IntrEnable {
        &self.cfg0_intr_enable
    }
    #[doc = "0x101c - CFG0_intr_enable_clear"]
    #[inline(always)]
    pub const fn cfg0_intr_enable_clear(&self) -> &Cfg0IntrEnableClear {
        &self.cfg0_intr_enable_clear
    }
    #[doc = "0x1020 - CFG0_eoi"]
    #[inline(always)]
    pub const fn cfg0_eoi(&self) -> &Cfg0Eoi {
        &self.cfg0_eoi
    }
    #[doc = "0x1024 - CFG0_fault_address"]
    #[inline(always)]
    pub const fn cfg0_fault_address(&self) -> &Cfg0FaultAddress {
        &self.cfg0_fault_address
    }
    #[doc = "0x1028 - CFG0_fault_type_status"]
    #[inline(always)]
    pub const fn cfg0_fault_type_status(&self) -> &Cfg0FaultTypeStatus {
        &self.cfg0_fault_type_status
    }
    #[doc = "0x102c - CFG0_fault_attr_status"]
    #[inline(always)]
    pub const fn cfg0_fault_attr_status(&self) -> &Cfg0FaultAttrStatus {
        &self.cfg0_fault_attr_status
    }
    #[doc = "0x1030 - CFG0_fault_clear"]
    #[inline(always)]
    pub const fn cfg0_fault_clear(&self) -> &Cfg0FaultClear {
        &self.cfg0_fault_clear
    }
    #[doc = "0x1100 - CFG0_CLAIMREG_P0_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r0_readonly(&self) -> &Cfg0ClaimregP0R0Readonly {
        &self.cfg0_claimreg_p0_r0_readonly
    }
    #[doc = "0x2000 - CFG0_PID_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pid_proxy(&self) -> &Cfg0PidProxy {
        &self.cfg0_pid_proxy
    }
    #[doc = "0x2008 - CFG0_MMR_CFG1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mmr_cfg1_proxy(&self) -> &Cfg0MmrCfg1Proxy {
        &self.cfg0_mmr_cfg1_proxy
    }
    #[doc = "0x3008 - CFG0_LOCK0_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock0_kick0_proxy(&self) -> &Cfg0Lock0Kick0Proxy {
        &self.cfg0_lock0_kick0_proxy
    }
    #[doc = "0x300c - CFG0_LOCK0_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock0_kick1_proxy(&self) -> &Cfg0Lock0Kick1Proxy {
        &self.cfg0_lock0_kick1_proxy
    }
    #[doc = "0x3010 - CFG0_intr_raw_status_PROXY"]
    #[inline(always)]
    pub const fn cfg0_intr_raw_status_proxy(&self) -> &Cfg0IntrRawStatusProxy {
        &self.cfg0_intr_raw_status_proxy
    }
    #[doc = "0x3014 - CFG0_intr_enabled_status_clear_PROXY"]
    #[inline(always)]
    pub const fn cfg0_intr_enabled_status_clear_proxy(&self) -> &Cfg0IntrEnabledStatusClearProxy {
        &self.cfg0_intr_enabled_status_clear_proxy
    }
    #[doc = "0x3018 - CFG0_intr_enable_PROXY"]
    #[inline(always)]
    pub const fn cfg0_intr_enable_proxy(&self) -> &Cfg0IntrEnableProxy {
        &self.cfg0_intr_enable_proxy
    }
    #[doc = "0x301c - CFG0_intr_enable_clear_PROXY"]
    #[inline(always)]
    pub const fn cfg0_intr_enable_clear_proxy(&self) -> &Cfg0IntrEnableClearProxy {
        &self.cfg0_intr_enable_clear_proxy
    }
    #[doc = "0x3020 - CFG0_eoi_PROXY"]
    #[inline(always)]
    pub const fn cfg0_eoi_proxy(&self) -> &Cfg0EoiProxy {
        &self.cfg0_eoi_proxy
    }
    #[doc = "0x3024 - CFG0_fault_address_PROXY"]
    #[inline(always)]
    pub const fn cfg0_fault_address_proxy(&self) -> &Cfg0FaultAddressProxy {
        &self.cfg0_fault_address_proxy
    }
    #[doc = "0x3028 - CFG0_fault_type_status_PROXY"]
    #[inline(always)]
    pub const fn cfg0_fault_type_status_proxy(&self) -> &Cfg0FaultTypeStatusProxy {
        &self.cfg0_fault_type_status_proxy
    }
    #[doc = "0x302c - CFG0_fault_attr_status_PROXY"]
    #[inline(always)]
    pub const fn cfg0_fault_attr_status_proxy(&self) -> &Cfg0FaultAttrStatusProxy {
        &self.cfg0_fault_attr_status_proxy
    }
    #[doc = "0x3030 - CFG0_fault_clear_PROXY"]
    #[inline(always)]
    pub const fn cfg0_fault_clear_proxy(&self) -> &Cfg0FaultClearProxy {
        &self.cfg0_fault_clear_proxy
    }
    #[doc = "0x3100 - CFG0_CLAIMREG_P0_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r0(&self) -> &Cfg0ClaimregP0R0 {
        &self.cfg0_claimreg_p0_r0
    }
    #[doc = "0x4084 - CFG0_DBOUNCE_CFG1"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg1(&self) -> &Cfg0DbounceCfg1 {
        &self.cfg0_dbounce_cfg1
    }
    #[doc = "0x4088 - CFG0_DBOUNCE_CFG2"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg2(&self) -> &Cfg0DbounceCfg2 {
        &self.cfg0_dbounce_cfg2
    }
    #[doc = "0x408c - CFG0_DBOUNCE_CFG3"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg3(&self) -> &Cfg0DbounceCfg3 {
        &self.cfg0_dbounce_cfg3
    }
    #[doc = "0x4090 - CFG0_DBOUNCE_CFG4"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg4(&self) -> &Cfg0DbounceCfg4 {
        &self.cfg0_dbounce_cfg4
    }
    #[doc = "0x4094 - CFG0_DBOUNCE_CFG5"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg5(&self) -> &Cfg0DbounceCfg5 {
        &self.cfg0_dbounce_cfg5
    }
    #[doc = "0x4098 - CFG0_DBOUNCE_CFG6"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg6(&self) -> &Cfg0DbounceCfg6 {
        &self.cfg0_dbounce_cfg6
    }
    #[doc = "0x40a0 - CFG0_TEMP_DIODE_TRIM"]
    #[inline(always)]
    pub const fn cfg0_temp_diode_trim(&self) -> &Cfg0TempDiodeTrim {
        &self.cfg0_temp_diode_trim
    }
    #[doc = "0x40b0 - CFG0_IO_VOLTAGE_STAT"]
    #[inline(always)]
    pub const fn cfg0_io_voltage_stat(&self) -> &Cfg0IoVoltageStat {
        &self.cfg0_io_voltage_stat
    }
    #[doc = "0x4204 - CFG0_MCU_TIMER1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer1_ctrl(&self) -> &Cfg0McuTimer1Ctrl {
        &self.cfg0_mcu_timer1_ctrl
    }
    #[doc = "0x420c - CFG0_MCU_TIMER3_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer3_ctrl(&self) -> &Cfg0McuTimer3Ctrl {
        &self.cfg0_mcu_timer3_ctrl
    }
    #[doc = "0x42e0 - CFG0_MCU_I2C0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_i2c0_ctrl(&self) -> &Cfg0McuI2c0Ctrl {
        &self.cfg0_mcu_i2c0_ctrl
    }
    #[doc = "0x4600 - CFG0_MCU_MTOG_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_mtog_ctrl(&self) -> &Cfg0McuMtogCtrl {
        &self.cfg0_mcu_mtog_ctrl
    }
    #[doc = "0x5008 - CFG0_LOCK1_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock1_kick0(&self) -> &Cfg0Lock1Kick0 {
        &self.cfg0_lock1_kick0
    }
    #[doc = "0x500c - CFG0_LOCK1_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock1_kick1(&self) -> &Cfg0Lock1Kick1 {
        &self.cfg0_lock1_kick1
    }
    #[doc = "0x5100 - CFG0_CLAIMREG_P1_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r0_readonly(&self) -> &Cfg0ClaimregP1R0Readonly {
        &self.cfg0_claimreg_p1_r0_readonly
    }
    #[doc = "0x5104 - CFG0_CLAIMREG_P1_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r1_readonly(&self) -> &Cfg0ClaimregP1R1Readonly {
        &self.cfg0_claimreg_p1_r1_readonly
    }
    #[doc = "0x5108 - CFG0_CLAIMREG_P1_R2_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r2_readonly(&self) -> &Cfg0ClaimregP1R2Readonly {
        &self.cfg0_claimreg_p1_r2_readonly
    }
    #[doc = "0x510c - CFG0_CLAIMREG_P1_R3_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r3_readonly(&self) -> &Cfg0ClaimregP1R3Readonly {
        &self.cfg0_claimreg_p1_r3_readonly
    }
    #[doc = "0x5110 - CFG0_CLAIMREG_P1_R4_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r4_readonly(&self) -> &Cfg0ClaimregP1R4Readonly {
        &self.cfg0_claimreg_p1_r4_readonly
    }
    #[doc = "0x5114 - CFG0_CLAIMREG_P1_R5_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r5_readonly(&self) -> &Cfg0ClaimregP1R5Readonly {
        &self.cfg0_claimreg_p1_r5_readonly
    }
    #[doc = "0x5118 - CFG0_CLAIMREG_P1_R6_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r6_readonly(&self) -> &Cfg0ClaimregP1R6Readonly {
        &self.cfg0_claimreg_p1_r6_readonly
    }
    #[doc = "0x511c - CFG0_CLAIMREG_P1_R7_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r7_readonly(&self) -> &Cfg0ClaimregP1R7Readonly {
        &self.cfg0_claimreg_p1_r7_readonly
    }
    #[doc = "0x5120 - CFG0_CLAIMREG_P1_R8_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r8_readonly(&self) -> &Cfg0ClaimregP1R8Readonly {
        &self.cfg0_claimreg_p1_r8_readonly
    }
    #[doc = "0x5124 - CFG0_CLAIMREG_P1_R9_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r9_readonly(&self) -> &Cfg0ClaimregP1R9Readonly {
        &self.cfg0_claimreg_p1_r9_readonly
    }
    #[doc = "0x5128 - CFG0_CLAIMREG_P1_R10_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r10_readonly(&self) -> &Cfg0ClaimregP1R10Readonly {
        &self.cfg0_claimreg_p1_r10_readonly
    }
    #[doc = "0x512c - CFG0_CLAIMREG_P1_R11_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r11_readonly(&self) -> &Cfg0ClaimregP1R11Readonly {
        &self.cfg0_claimreg_p1_r11_readonly
    }
    #[doc = "0x5130 - CFG0_CLAIMREG_P1_R12_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r12_readonly(&self) -> &Cfg0ClaimregP1R12Readonly {
        &self.cfg0_claimreg_p1_r12_readonly
    }
    #[doc = "0x6084 - CFG0_DBOUNCE_CFG1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg1_proxy(&self) -> &Cfg0DbounceCfg1Proxy {
        &self.cfg0_dbounce_cfg1_proxy
    }
    #[doc = "0x6088 - CFG0_DBOUNCE_CFG2_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg2_proxy(&self) -> &Cfg0DbounceCfg2Proxy {
        &self.cfg0_dbounce_cfg2_proxy
    }
    #[doc = "0x608c - CFG0_DBOUNCE_CFG3_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg3_proxy(&self) -> &Cfg0DbounceCfg3Proxy {
        &self.cfg0_dbounce_cfg3_proxy
    }
    #[doc = "0x6090 - CFG0_DBOUNCE_CFG4_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg4_proxy(&self) -> &Cfg0DbounceCfg4Proxy {
        &self.cfg0_dbounce_cfg4_proxy
    }
    #[doc = "0x6094 - CFG0_DBOUNCE_CFG5_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg5_proxy(&self) -> &Cfg0DbounceCfg5Proxy {
        &self.cfg0_dbounce_cfg5_proxy
    }
    #[doc = "0x6098 - CFG0_DBOUNCE_CFG6_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dbounce_cfg6_proxy(&self) -> &Cfg0DbounceCfg6Proxy {
        &self.cfg0_dbounce_cfg6_proxy
    }
    #[doc = "0x60a0 - CFG0_TEMP_DIODE_TRIM_PROXY"]
    #[inline(always)]
    pub const fn cfg0_temp_diode_trim_proxy(&self) -> &Cfg0TempDiodeTrimProxy {
        &self.cfg0_temp_diode_trim_proxy
    }
    #[doc = "0x60b0 - CFG0_IO_VOLTAGE_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_io_voltage_stat_proxy(&self) -> &Cfg0IoVoltageStatProxy {
        &self.cfg0_io_voltage_stat_proxy
    }
    #[doc = "0x6204 - CFG0_MCU_TIMER1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer1_ctrl_proxy(&self) -> &Cfg0McuTimer1CtrlProxy {
        &self.cfg0_mcu_timer1_ctrl_proxy
    }
    #[doc = "0x620c - CFG0_MCU_TIMER3_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer3_ctrl_proxy(&self) -> &Cfg0McuTimer3CtrlProxy {
        &self.cfg0_mcu_timer3_ctrl_proxy
    }
    #[doc = "0x62e0 - CFG0_MCU_I2C0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_i2c0_ctrl_proxy(&self) -> &Cfg0McuI2c0CtrlProxy {
        &self.cfg0_mcu_i2c0_ctrl_proxy
    }
    #[doc = "0x6600 - CFG0_MCU_MTOG_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_mtog_ctrl_proxy(&self) -> &Cfg0McuMtogCtrlProxy {
        &self.cfg0_mcu_mtog_ctrl_proxy
    }
    #[doc = "0x7008 - CFG0_LOCK1_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock1_kick0_proxy(&self) -> &Cfg0Lock1Kick0Proxy {
        &self.cfg0_lock1_kick0_proxy
    }
    #[doc = "0x700c - CFG0_LOCK1_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock1_kick1_proxy(&self) -> &Cfg0Lock1Kick1Proxy {
        &self.cfg0_lock1_kick1_proxy
    }
    #[doc = "0x7100 - CFG0_CLAIMREG_P1_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r0(&self) -> &Cfg0ClaimregP1R0 {
        &self.cfg0_claimreg_p1_r0
    }
    #[doc = "0x7104 - CFG0_CLAIMREG_P1_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r1(&self) -> &Cfg0ClaimregP1R1 {
        &self.cfg0_claimreg_p1_r1
    }
    #[doc = "0x7108 - CFG0_CLAIMREG_P1_R2"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r2(&self) -> &Cfg0ClaimregP1R2 {
        &self.cfg0_claimreg_p1_r2
    }
    #[doc = "0x710c - CFG0_CLAIMREG_P1_R3"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r3(&self) -> &Cfg0ClaimregP1R3 {
        &self.cfg0_claimreg_p1_r3
    }
    #[doc = "0x7110 - CFG0_CLAIMREG_P1_R4"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r4(&self) -> &Cfg0ClaimregP1R4 {
        &self.cfg0_claimreg_p1_r4
    }
    #[doc = "0x7114 - CFG0_CLAIMREG_P1_R5"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r5(&self) -> &Cfg0ClaimregP1R5 {
        &self.cfg0_claimreg_p1_r5
    }
    #[doc = "0x7118 - CFG0_CLAIMREG_P1_R6"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r6(&self) -> &Cfg0ClaimregP1R6 {
        &self.cfg0_claimreg_p1_r6
    }
    #[doc = "0x711c - CFG0_CLAIMREG_P1_R7"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r7(&self) -> &Cfg0ClaimregP1R7 {
        &self.cfg0_claimreg_p1_r7
    }
    #[doc = "0x7120 - CFG0_CLAIMREG_P1_R8"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r8(&self) -> &Cfg0ClaimregP1R8 {
        &self.cfg0_claimreg_p1_r8
    }
    #[doc = "0x7124 - CFG0_CLAIMREG_P1_R9"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r9(&self) -> &Cfg0ClaimregP1R9 {
        &self.cfg0_claimreg_p1_r9
    }
    #[doc = "0x7128 - CFG0_CLAIMREG_P1_R10"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r10(&self) -> &Cfg0ClaimregP1R10 {
        &self.cfg0_claimreg_p1_r10
    }
    #[doc = "0x712c - CFG0_CLAIMREG_P1_R11"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r11(&self) -> &Cfg0ClaimregP1R11 {
        &self.cfg0_claimreg_p1_r11
    }
    #[doc = "0x7130 - CFG0_CLAIMREG_P1_R12"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r12(&self) -> &Cfg0ClaimregP1R12 {
        &self.cfg0_claimreg_p1_r12
    }
    #[doc = "0x8000 - CFG0_MCU_OBSCLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_obsclk_ctrl(&self) -> &Cfg0McuObsclkCtrl {
        &self.cfg0_mcu_obsclk_ctrl
    }
    #[doc = "0x8010 - CFG0_HFOSC0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_hfosc0_ctrl(&self) -> &Cfg0Hfosc0Ctrl {
        &self.cfg0_hfosc0_ctrl
    }
    #[doc = "0x8018 - CFG0_HFOSC0_TRIM"]
    #[inline(always)]
    pub const fn cfg0_hfosc0_trim(&self) -> &Cfg0Hfosc0Trim {
        &self.cfg0_hfosc0_trim
    }
    #[doc = "0x8024 - CFG0_RC12M_OSC_TRIM"]
    #[inline(always)]
    pub const fn cfg0_rc12m_osc_trim(&self) -> &Cfg0Rc12mOscTrim {
        &self.cfg0_rc12m_osc_trim
    }
    #[doc = "0x8030 - CFG0_HFOSC0_CLKOUT_32K_CTRL"]
    #[inline(always)]
    pub const fn cfg0_hfosc0_clkout_32k_ctrl(&self) -> &Cfg0Hfosc0Clkout32kCtrl {
        &self.cfg0_hfosc0_clkout_32k_ctrl
    }
    #[doc = "0x8040 - CFG0_MCU_M4FSS_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss_clksel(&self) -> &Cfg0McuM4fssClksel {
        &self.cfg0_mcu_m4fss_clksel
    }
    #[doc = "0x8044 - CFG0_MCU_M4FSS_SYSTICK"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss_systick(&self) -> &Cfg0McuM4fssSystick {
        &self.cfg0_mcu_m4fss_systick
    }
    #[doc = "0x8050 - CFG0_MCU_PLL_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_pll_clksel(&self) -> &Cfg0McuPllClksel {
        &self.cfg0_mcu_pll_clksel
    }
    #[doc = "0x8060 - CFG0_MCU_TIMER0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer0_clksel(&self) -> &Cfg0McuTimer0Clksel {
        &self.cfg0_mcu_timer0_clksel
    }
    #[doc = "0x8064 - CFG0_MCU_TIMER1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer1_clksel(&self) -> &Cfg0McuTimer1Clksel {
        &self.cfg0_mcu_timer1_clksel
    }
    #[doc = "0x8068 - CFG0_MCU_TIMER2_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer2_clksel(&self) -> &Cfg0McuTimer2Clksel {
        &self.cfg0_mcu_timer2_clksel
    }
    #[doc = "0x806c - CFG0_MCU_TIMER3_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer3_clksel(&self) -> &Cfg0McuTimer3Clksel {
        &self.cfg0_mcu_timer3_clksel
    }
    #[doc = "0x80a0 - CFG0_MCU_SPI0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_spi0_clksel(&self) -> &Cfg0McuSpi0Clksel {
        &self.cfg0_mcu_spi0_clksel
    }
    #[doc = "0x80a4 - CFG0_MCU_SPI1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_spi1_clksel(&self) -> &Cfg0McuSpi1Clksel {
        &self.cfg0_mcu_spi1_clksel
    }
    #[doc = "0x80b0 - CFG0_MCU_WWD0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcu_wwd0_clksel(&self) -> &Cfg0McuWwd0Clksel {
        &self.cfg0_mcu_wwd0_clksel
    }
    #[doc = "0x80d0 - CFG0_DDR16SS_PMCTRL"]
    #[inline(always)]
    pub const fn cfg0_ddr16ss_pmctrl(&self) -> &Cfg0Ddr16ssPmctrl {
        &self.cfg0_ddr16ss_pmctrl
    }
    #[doc = "0x9008 - CFG0_LOCK2_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock2_kick0(&self) -> &Cfg0Lock2Kick0 {
        &self.cfg0_lock2_kick0
    }
    #[doc = "0x900c - CFG0_LOCK2_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock2_kick1(&self) -> &Cfg0Lock2Kick1 {
        &self.cfg0_lock2_kick1
    }
    #[doc = "0x9100 - CFG0_CLAIMREG_P2_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r0_readonly(&self) -> &Cfg0ClaimregP2R0Readonly {
        &self.cfg0_claimreg_p2_r0_readonly
    }
    #[doc = "0x9104 - CFG0_CLAIMREG_P2_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r1_readonly(&self) -> &Cfg0ClaimregP2R1Readonly {
        &self.cfg0_claimreg_p2_r1_readonly
    }
    #[doc = "0xa000 - CFG0_MCU_OBSCLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_obsclk_ctrl_proxy(&self) -> &Cfg0McuObsclkCtrlProxy {
        &self.cfg0_mcu_obsclk_ctrl_proxy
    }
    #[doc = "0xa010 - CFG0_HFOSC0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_hfosc0_ctrl_proxy(&self) -> &Cfg0Hfosc0CtrlProxy {
        &self.cfg0_hfosc0_ctrl_proxy
    }
    #[doc = "0xa018 - CFG0_HFOSC0_TRIM_PROXY"]
    #[inline(always)]
    pub const fn cfg0_hfosc0_trim_proxy(&self) -> &Cfg0Hfosc0TrimProxy {
        &self.cfg0_hfosc0_trim_proxy
    }
    #[doc = "0xa024 - CFG0_RC12M_OSC_TRIM_PROXY"]
    #[inline(always)]
    pub const fn cfg0_rc12m_osc_trim_proxy(&self) -> &Cfg0Rc12mOscTrimProxy {
        &self.cfg0_rc12m_osc_trim_proxy
    }
    #[doc = "0xa030 - CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_hfosc0_clkout_32k_ctrl_proxy(&self) -> &Cfg0Hfosc0Clkout32kCtrlProxy {
        &self.cfg0_hfosc0_clkout_32k_ctrl_proxy
    }
    #[doc = "0xa040 - CFG0_MCU_M4FSS_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss_clksel_proxy(&self) -> &Cfg0McuM4fssClkselProxy {
        &self.cfg0_mcu_m4fss_clksel_proxy
    }
    #[doc = "0xa044 - CFG0_MCU_M4FSS_SYSTICK_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss_systick_proxy(&self) -> &Cfg0McuM4fssSystickProxy {
        &self.cfg0_mcu_m4fss_systick_proxy
    }
    #[doc = "0xa050 - CFG0_MCU_PLL_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_pll_clksel_proxy(&self) -> &Cfg0McuPllClkselProxy {
        &self.cfg0_mcu_pll_clksel_proxy
    }
    #[doc = "0xa060 - CFG0_MCU_TIMER0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer0_clksel_proxy(&self) -> &Cfg0McuTimer0ClkselProxy {
        &self.cfg0_mcu_timer0_clksel_proxy
    }
    #[doc = "0xa064 - CFG0_MCU_TIMER1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer1_clksel_proxy(&self) -> &Cfg0McuTimer1ClkselProxy {
        &self.cfg0_mcu_timer1_clksel_proxy
    }
    #[doc = "0xa068 - CFG0_MCU_TIMER2_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer2_clksel_proxy(&self) -> &Cfg0McuTimer2ClkselProxy {
        &self.cfg0_mcu_timer2_clksel_proxy
    }
    #[doc = "0xa06c - CFG0_MCU_TIMER3_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_timer3_clksel_proxy(&self) -> &Cfg0McuTimer3ClkselProxy {
        &self.cfg0_mcu_timer3_clksel_proxy
    }
    #[doc = "0xa0a0 - CFG0_MCU_SPI0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_spi0_clksel_proxy(&self) -> &Cfg0McuSpi0ClkselProxy {
        &self.cfg0_mcu_spi0_clksel_proxy
    }
    #[doc = "0xa0a4 - CFG0_MCU_SPI1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_spi1_clksel_proxy(&self) -> &Cfg0McuSpi1ClkselProxy {
        &self.cfg0_mcu_spi1_clksel_proxy
    }
    #[doc = "0xa0b0 - CFG0_MCU_WWD0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_wwd0_clksel_proxy(&self) -> &Cfg0McuWwd0ClkselProxy {
        &self.cfg0_mcu_wwd0_clksel_proxy
    }
    #[doc = "0xa0d0 - CFG0_DDR16SS_PMCTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_ddr16ss_pmctrl_proxy(&self) -> &Cfg0Ddr16ssPmctrlProxy {
        &self.cfg0_ddr16ss_pmctrl_proxy
    }
    #[doc = "0xb008 - CFG0_LOCK2_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock2_kick0_proxy(&self) -> &Cfg0Lock2Kick0Proxy {
        &self.cfg0_lock2_kick0_proxy
    }
    #[doc = "0xb00c - CFG0_LOCK2_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock2_kick1_proxy(&self) -> &Cfg0Lock2Kick1Proxy {
        &self.cfg0_lock2_kick1_proxy
    }
    #[doc = "0xb100 - CFG0_CLAIMREG_P2_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r0(&self) -> &Cfg0ClaimregP2R0 {
        &self.cfg0_claimreg_p2_r0
    }
    #[doc = "0xb104 - CFG0_CLAIMREG_P2_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r1(&self) -> &Cfg0ClaimregP2R1 {
        &self.cfg0_claimreg_p2_r1
    }
    #[doc = "0xc020 - CFG0_MCU_M4FSS0_LBIST_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_ctrl(&self) -> &Cfg0McuM4fss0LbistCtrl {
        &self.cfg0_mcu_m4fss0_lbist_ctrl
    }
    #[doc = "0xc024 - CFG0_MCU_M4FSS0_LBIST_PATCOUNT"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_patcount(&self) -> &Cfg0McuM4fss0LbistPatcount {
        &self.cfg0_mcu_m4fss0_lbist_patcount
    }
    #[doc = "0xc028 - CFG0_MCU_M4FSS0_LBIST_SEED0"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_seed0(&self) -> &Cfg0McuM4fss0LbistSeed0 {
        &self.cfg0_mcu_m4fss0_lbist_seed0
    }
    #[doc = "0xc02c - CFG0_MCU_M4FSS0_LBIST_SEED1"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_seed1(&self) -> &Cfg0McuM4fss0LbistSeed1 {
        &self.cfg0_mcu_m4fss0_lbist_seed1
    }
    #[doc = "0xc030 - CFG0_MCU_M4FSS0_LBIST_SPARE0"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_spare0(&self) -> &Cfg0McuM4fss0LbistSpare0 {
        &self.cfg0_mcu_m4fss0_lbist_spare0
    }
    #[doc = "0xc034 - CFG0_MCU_M4FSS0_LBIST_SPARE1"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_spare1(&self) -> &Cfg0McuM4fss0LbistSpare1 {
        &self.cfg0_mcu_m4fss0_lbist_spare1
    }
    #[doc = "0xc038 - CFG0_MCU_M4FSS0_LBIST_STAT"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_stat(&self) -> &Cfg0McuM4fss0LbistStat {
        &self.cfg0_mcu_m4fss0_lbist_stat
    }
    #[doc = "0xc03c - CFG0_MCU_M4FSS0_LBIST_MISR"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_misr(&self) -> &Cfg0McuM4fss0LbistMisr {
        &self.cfg0_mcu_m4fss0_lbist_misr
    }
    #[doc = "0xd008 - CFG0_LOCK3_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock3_kick0(&self) -> &Cfg0Lock3Kick0 {
        &self.cfg0_lock3_kick0
    }
    #[doc = "0xd00c - CFG0_LOCK3_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock3_kick1(&self) -> &Cfg0Lock3Kick1 {
        &self.cfg0_lock3_kick1
    }
    #[doc = "0xd100 - CFG0_CLAIMREG_P3_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r0_readonly(&self) -> &Cfg0ClaimregP3R0Readonly {
        &self.cfg0_claimreg_p3_r0_readonly
    }
    #[doc = "0xe020 - CFG0_MCU_M4FSS0_LBIST_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_ctrl_proxy(&self) -> &Cfg0McuM4fss0LbistCtrlProxy {
        &self.cfg0_mcu_m4fss0_lbist_ctrl_proxy
    }
    #[doc = "0xe024 - CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_patcount_proxy(&self) -> &Cfg0McuM4fss0LbistPatcountProxy {
        &self.cfg0_mcu_m4fss0_lbist_patcount_proxy
    }
    #[doc = "0xe028 - CFG0_MCU_M4FSS0_LBIST_SEED0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_seed0_proxy(&self) -> &Cfg0McuM4fss0LbistSeed0Proxy {
        &self.cfg0_mcu_m4fss0_lbist_seed0_proxy
    }
    #[doc = "0xe02c - CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_seed1_proxy(&self) -> &Cfg0McuM4fss0LbistSeed1Proxy {
        &self.cfg0_mcu_m4fss0_lbist_seed1_proxy
    }
    #[doc = "0xe030 - CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_spare0_proxy(&self) -> &Cfg0McuM4fss0LbistSpare0Proxy {
        &self.cfg0_mcu_m4fss0_lbist_spare0_proxy
    }
    #[doc = "0xe034 - CFG0_MCU_M4FSS0_LBIST_SPARE1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_spare1_proxy(&self) -> &Cfg0McuM4fss0LbistSpare1Proxy {
        &self.cfg0_mcu_m4fss0_lbist_spare1_proxy
    }
    #[doc = "0xe038 - CFG0_MCU_M4FSS0_LBIST_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_stat_proxy(&self) -> &Cfg0McuM4fss0LbistStatProxy {
        &self.cfg0_mcu_m4fss0_lbist_stat_proxy
    }
    #[doc = "0xe03c - CFG0_MCU_M4FSS0_LBIST_MISR_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_m4fss0_lbist_misr_proxy(&self) -> &Cfg0McuM4fss0LbistMisrProxy {
        &self.cfg0_mcu_m4fss0_lbist_misr_proxy
    }
    #[doc = "0xf008 - CFG0_LOCK3_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock3_kick0_proxy(&self) -> &Cfg0Lock3Kick0Proxy {
        &self.cfg0_lock3_kick0_proxy
    }
    #[doc = "0xf00c - CFG0_LOCK3_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock3_kick1_proxy(&self) -> &Cfg0Lock3Kick1Proxy {
        &self.cfg0_lock3_kick1_proxy
    }
    #[doc = "0xf100 - CFG0_CLAIMREG_P3_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r0(&self) -> &Cfg0ClaimregP3R0 {
        &self.cfg0_claimreg_p3_r0
    }
    #[doc = "0x11008 - CFG0_LOCK4_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock4_kick0(&self) -> &Cfg0Lock4Kick0 {
        &self.cfg0_lock4_kick0
    }
    #[doc = "0x1100c - CFG0_LOCK4_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock4_kick1(&self) -> &Cfg0Lock4Kick1 {
        &self.cfg0_lock4_kick1
    }
    #[doc = "0x11100 - CFG0_CLAIMREG_P4_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r0_readonly(&self) -> &Cfg0ClaimregP4R0Readonly {
        &self.cfg0_claimreg_p4_r0_readonly
    }
    #[doc = "0x11104 - CFG0_CLAIMREG_P4_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r1_readonly(&self) -> &Cfg0ClaimregP4R1Readonly {
        &self.cfg0_claimreg_p4_r1_readonly
    }
    #[doc = "0x11108 - CFG0_CLAIMREG_P4_R2_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r2_readonly(&self) -> &Cfg0ClaimregP4R2Readonly {
        &self.cfg0_claimreg_p4_r2_readonly
    }
    #[doc = "0x1110c - CFG0_CLAIMREG_P4_R3_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r3_readonly(&self) -> &Cfg0ClaimregP4R3Readonly {
        &self.cfg0_claimreg_p4_r3_readonly
    }
    #[doc = "0x11110 - CFG0_CLAIMREG_P4_R4_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r4_readonly(&self) -> &Cfg0ClaimregP4R4Readonly {
        &self.cfg0_claimreg_p4_r4_readonly
    }
    #[doc = "0x11114 - CFG0_CLAIMREG_P4_R5_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r5_readonly(&self) -> &Cfg0ClaimregP4R5Readonly {
        &self.cfg0_claimreg_p4_r5_readonly
    }
    #[doc = "0x11118 - CFG0_CLAIMREG_P4_R6_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r6_readonly(&self) -> &Cfg0ClaimregP4R6Readonly {
        &self.cfg0_claimreg_p4_r6_readonly
    }
    #[doc = "0x1111c - CFG0_CLAIMREG_P4_R7_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r7_readonly(&self) -> &Cfg0ClaimregP4R7Readonly {
        &self.cfg0_claimreg_p4_r7_readonly
    }
    #[doc = "0x11120 - CFG0_CLAIMREG_P4_R8_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r8_readonly(&self) -> &Cfg0ClaimregP4R8Readonly {
        &self.cfg0_claimreg_p4_r8_readonly
    }
    #[doc = "0x11124 - CFG0_CLAIMREG_P4_R9_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r9_readonly(&self) -> &Cfg0ClaimregP4R9Readonly {
        &self.cfg0_claimreg_p4_r9_readonly
    }
    #[doc = "0x11128 - CFG0_CLAIMREG_P4_R10_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r10_readonly(&self) -> &Cfg0ClaimregP4R10Readonly {
        &self.cfg0_claimreg_p4_r10_readonly
    }
    #[doc = "0x1112c - CFG0_CLAIMREG_P4_R11_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r11_readonly(&self) -> &Cfg0ClaimregP4R11Readonly {
        &self.cfg0_claimreg_p4_r11_readonly
    }
    #[doc = "0x11130 - CFG0_CLAIMREG_P4_R12_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r12_readonly(&self) -> &Cfg0ClaimregP4R12Readonly {
        &self.cfg0_claimreg_p4_r12_readonly
    }
    #[doc = "0x11134 - CFG0_CLAIMREG_P4_R13_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r13_readonly(&self) -> &Cfg0ClaimregP4R13Readonly {
        &self.cfg0_claimreg_p4_r13_readonly
    }
    #[doc = "0x11138 - CFG0_CLAIMREG_P4_R14_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r14_readonly(&self) -> &Cfg0ClaimregP4R14Readonly {
        &self.cfg0_claimreg_p4_r14_readonly
    }
    #[doc = "0x13008 - CFG0_LOCK4_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock4_kick0_proxy(&self) -> &Cfg0Lock4Kick0Proxy {
        &self.cfg0_lock4_kick0_proxy
    }
    #[doc = "0x1300c - CFG0_LOCK4_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock4_kick1_proxy(&self) -> &Cfg0Lock4Kick1Proxy {
        &self.cfg0_lock4_kick1_proxy
    }
    #[doc = "0x13100 - CFG0_CLAIMREG_P4_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r0(&self) -> &Cfg0ClaimregP4R0 {
        &self.cfg0_claimreg_p4_r0
    }
    #[doc = "0x13104 - CFG0_CLAIMREG_P4_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r1(&self) -> &Cfg0ClaimregP4R1 {
        &self.cfg0_claimreg_p4_r1
    }
    #[doc = "0x13108 - CFG0_CLAIMREG_P4_R2"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r2(&self) -> &Cfg0ClaimregP4R2 {
        &self.cfg0_claimreg_p4_r2
    }
    #[doc = "0x1310c - CFG0_CLAIMREG_P4_R3"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r3(&self) -> &Cfg0ClaimregP4R3 {
        &self.cfg0_claimreg_p4_r3
    }
    #[doc = "0x13110 - CFG0_CLAIMREG_P4_R4"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r4(&self) -> &Cfg0ClaimregP4R4 {
        &self.cfg0_claimreg_p4_r4
    }
    #[doc = "0x13114 - CFG0_CLAIMREG_P4_R5"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r5(&self) -> &Cfg0ClaimregP4R5 {
        &self.cfg0_claimreg_p4_r5
    }
    #[doc = "0x13118 - CFG0_CLAIMREG_P4_R6"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r6(&self) -> &Cfg0ClaimregP4R6 {
        &self.cfg0_claimreg_p4_r6
    }
    #[doc = "0x1311c - CFG0_CLAIMREG_P4_R7"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r7(&self) -> &Cfg0ClaimregP4R7 {
        &self.cfg0_claimreg_p4_r7
    }
    #[doc = "0x13120 - CFG0_CLAIMREG_P4_R8"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r8(&self) -> &Cfg0ClaimregP4R8 {
        &self.cfg0_claimreg_p4_r8
    }
    #[doc = "0x13124 - CFG0_CLAIMREG_P4_R9"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r9(&self) -> &Cfg0ClaimregP4R9 {
        &self.cfg0_claimreg_p4_r9
    }
    #[doc = "0x13128 - CFG0_CLAIMREG_P4_R10"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r10(&self) -> &Cfg0ClaimregP4R10 {
        &self.cfg0_claimreg_p4_r10
    }
    #[doc = "0x1312c - CFG0_CLAIMREG_P4_R11"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r11(&self) -> &Cfg0ClaimregP4R11 {
        &self.cfg0_claimreg_p4_r11
    }
    #[doc = "0x13130 - CFG0_CLAIMREG_P4_R12"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r12(&self) -> &Cfg0ClaimregP4R12 {
        &self.cfg0_claimreg_p4_r12
    }
    #[doc = "0x13134 - CFG0_CLAIMREG_P4_R13"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r13(&self) -> &Cfg0ClaimregP4R13 {
        &self.cfg0_claimreg_p4_r13
    }
    #[doc = "0x13138 - CFG0_CLAIMREG_P4_R14"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p4_r14(&self) -> &Cfg0ClaimregP4R14 {
        &self.cfg0_claimreg_p4_r14
    }
    #[doc = "0x18000 - CFG0_POR_CTRL"]
    #[inline(always)]
    pub const fn cfg0_por_ctrl(&self) -> &Cfg0PorCtrl {
        &self.cfg0_por_ctrl
    }
    #[doc = "0x18004 - CFG0_POR_STAT"]
    #[inline(always)]
    pub const fn cfg0_por_stat(&self) -> &Cfg0PorStat {
        &self.cfg0_por_stat
    }
    #[doc = "0x18100 - CFG0_POR_BANDGAP_CTRL"]
    #[inline(always)]
    pub const fn cfg0_por_bandgap_ctrl(&self) -> &Cfg0PorBandgapCtrl {
        &self.cfg0_por_bandgap_ctrl
    }
    #[doc = "0x18110 - CFG0_POK_VDDA_MCU_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdda_mcu_uv_ctrl(&self) -> &Cfg0PokVddaMcuUvCtrl {
        &self.cfg0_pok_vdda_mcu_uv_ctrl
    }
    #[doc = "0x18114 - CFG0_POK_VDDA_MCU_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdda_mcu_ov_ctrl(&self) -> &Cfg0PokVddaMcuOvCtrl {
        &self.cfg0_pok_vdda_mcu_ov_ctrl
    }
    #[doc = "0x18118 - CFG0_POK_VDD_CORE_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdd_core_uv_ctrl(&self) -> &Cfg0PokVddCoreUvCtrl {
        &self.cfg0_pok_vdd_core_uv_ctrl
    }
    #[doc = "0x1811c - CFG0_POK_VDD_CORE_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdd_core_ov_ctrl(&self) -> &Cfg0PokVddCoreOvCtrl {
        &self.cfg0_pok_vdd_core_ov_ctrl
    }
    #[doc = "0x18120 - CFG0_POK_VDDR_CORE_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddr_core_uv_ctrl(&self) -> &Cfg0PokVddrCoreUvCtrl {
        &self.cfg0_pok_vddr_core_uv_ctrl
    }
    #[doc = "0x18124 - CFG0_POK_VDDR_CORE_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddr_core_ov_ctrl(&self) -> &Cfg0PokVddrCoreOvCtrl {
        &self.cfg0_pok_vddr_core_ov_ctrl
    }
    #[doc = "0x18128 - CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_1p8_uv_ctrl(&self) -> &Cfg0PokVddshvMcu1p8UvCtrl {
        &self.cfg0_pok_vddshv_mcu_1p8_uv_ctrl
    }
    #[doc = "0x1812c - CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_1p8_ov_ctrl(&self) -> &Cfg0PokVddshvMcu1p8OvCtrl {
        &self.cfg0_pok_vddshv_mcu_1p8_ov_ctrl
    }
    #[doc = "0x18130 - CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_3p3_uv_ctrl(&self) -> &Cfg0PokVddshvMcu3p3UvCtrl {
        &self.cfg0_pok_vddshv_mcu_3p3_uv_ctrl
    }
    #[doc = "0x18134 - CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_3p3_ov_ctrl(&self) -> &Cfg0PokVddshvMcu3p3OvCtrl {
        &self.cfg0_pok_vddshv_mcu_3p3_ov_ctrl
    }
    #[doc = "0x18138 - CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vmon_cap_mcu_general_uv_ctrl(&self) -> &Cfg0PokVmonCapMcuGeneralUvCtrl {
        &self.cfg0_pok_vmon_cap_mcu_general_uv_ctrl
    }
    #[doc = "0x1813c - CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vmon_cap_mcu_general_ov_ctrl(&self) -> &Cfg0PokVmonCapMcuGeneralOvCtrl {
        &self.cfg0_pok_vmon_cap_mcu_general_ov_ctrl
    }
    #[doc = "0x18140 - CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_1p8_uv_ctrl(&self) -> &Cfg0PokVddshvMain1p8UvCtrl {
        &self.cfg0_pok_vddshv_main_1p8_uv_ctrl
    }
    #[doc = "0x18144 - CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_1p8_ov_ctrl(&self) -> &Cfg0PokVddshvMain1p8OvCtrl {
        &self.cfg0_pok_vddshv_main_1p8_ov_ctrl
    }
    #[doc = "0x18148 - CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_3p3_uv_ctrl(&self) -> &Cfg0PokVddshvMain3p3UvCtrl {
        &self.cfg0_pok_vddshv_main_3p3_uv_ctrl
    }
    #[doc = "0x1814c - CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_3p3_ov_ctrl(&self) -> &Cfg0PokVddshvMain3p3OvCtrl {
        &self.cfg0_pok_vddshv_main_3p3_ov_ctrl
    }
    #[doc = "0x18150 - CFG0_POK_VDDS_DDRIO_UV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdds_ddrio_uv_ctrl(&self) -> &Cfg0PokVddsDdrioUvCtrl {
        &self.cfg0_pok_vdds_ddrio_uv_ctrl
    }
    #[doc = "0x18154 - CFG0_POK_VDDS_DDRIO_OV_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdds_ddrio_ov_ctrl(&self) -> &Cfg0PokVddsDdrioOvCtrl {
        &self.cfg0_pok_vdds_ddrio_ov_ctrl
    }
    #[doc = "0x18160 - CFG0_POK_VDDA_PMIC_IN_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pok_vdda_pmic_in_ctrl(&self) -> &Cfg0PokVddaPmicInCtrl {
        &self.cfg0_pok_vdda_pmic_in_ctrl
    }
    #[doc = "0x18170 - CFG0_RST_CTRL"]
    #[inline(always)]
    pub const fn cfg0_rst_ctrl(&self) -> &Cfg0RstCtrl {
        &self.cfg0_rst_ctrl
    }
    #[doc = "0x18174 - CFG0_RST_STAT"]
    #[inline(always)]
    pub const fn cfg0_rst_stat(&self) -> &Cfg0RstStat {
        &self.cfg0_rst_stat
    }
    #[doc = "0x18178 - CFG0_RST_SRC"]
    #[inline(always)]
    pub const fn cfg0_rst_src(&self) -> &Cfg0RstSrc {
        &self.cfg0_rst_src
    }
    #[doc = "0x1817c - CFG0_RST_MAGIC_WORD"]
    #[inline(always)]
    pub const fn cfg0_rst_magic_word(&self) -> &Cfg0RstMagicWord {
        &self.cfg0_rst_magic_word
    }
    #[doc = "0x18180 - CFG0_ISO_CTRL"]
    #[inline(always)]
    pub const fn cfg0_iso_ctrl(&self) -> &Cfg0IsoCtrl {
        &self.cfg0_iso_ctrl
    }
    #[doc = "0x18190 - CFG0_VDD_CORE_GLDTC_CTRL"]
    #[inline(always)]
    pub const fn cfg0_vdd_core_gldtc_ctrl(&self) -> &Cfg0VddCoreGldtcCtrl {
        &self.cfg0_vdd_core_gldtc_ctrl
    }
    #[doc = "0x181b0 - CFG0_VDD_CORE_GLDTC_STAT"]
    #[inline(always)]
    pub const fn cfg0_vdd_core_gldtc_stat(&self) -> &Cfg0VddCoreGldtcStat {
        &self.cfg0_vdd_core_gldtc_stat
    }
    #[doc = "0x18200 - CFG0_PRG_PP_0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_prg_pp_0_ctrl(&self) -> &Cfg0PrgPp0Ctrl {
        &self.cfg0_prg_pp_0_ctrl
    }
    #[doc = "0x18208 - CFG0_PRG_PP_1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_prg_pp_1_ctrl(&self) -> &Cfg0PrgPp1Ctrl {
        &self.cfg0_prg_pp_1_ctrl
    }
    #[doc = "0x18284 - CFG0_MCU_CLKGATE_CTRL"]
    #[inline(always)]
    pub const fn cfg0_mcu_clkgate_ctrl(&self) -> &Cfg0McuClkgateCtrl {
        &self.cfg0_mcu_clkgate_ctrl
    }
    #[doc = "0x18288 - CFG0_MAIN_CLKGATE_CTRL0"]
    #[inline(always)]
    pub const fn cfg0_main_clkgate_ctrl0(&self) -> &Cfg0MainClkgateCtrl0 {
        &self.cfg0_main_clkgate_ctrl0
    }
    #[doc = "0x19008 - CFG0_LOCK6_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock6_kick0(&self) -> &Cfg0Lock6Kick0 {
        &self.cfg0_lock6_kick0
    }
    #[doc = "0x1900c - CFG0_LOCK6_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock6_kick1(&self) -> &Cfg0Lock6Kick1 {
        &self.cfg0_lock6_kick1
    }
    #[doc = "0x19100 - CFG0_CLAIMREG_P6_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r0_readonly(&self) -> &Cfg0ClaimregP6R0Readonly {
        &self.cfg0_claimreg_p6_r0_readonly
    }
    #[doc = "0x19104 - CFG0_CLAIMREG_P6_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r1_readonly(&self) -> &Cfg0ClaimregP6R1Readonly {
        &self.cfg0_claimreg_p6_r1_readonly
    }
    #[doc = "0x19108 - CFG0_CLAIMREG_P6_R2_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r2_readonly(&self) -> &Cfg0ClaimregP6R2Readonly {
        &self.cfg0_claimreg_p6_r2_readonly
    }
    #[doc = "0x1910c - CFG0_CLAIMREG_P6_R3_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r3_readonly(&self) -> &Cfg0ClaimregP6R3Readonly {
        &self.cfg0_claimreg_p6_r3_readonly
    }
    #[doc = "0x19110 - CFG0_CLAIMREG_P6_R4_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r4_readonly(&self) -> &Cfg0ClaimregP6R4Readonly {
        &self.cfg0_claimreg_p6_r4_readonly
    }
    #[doc = "0x19114 - CFG0_CLAIMREG_P6_R5_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r5_readonly(&self) -> &Cfg0ClaimregP6R5Readonly {
        &self.cfg0_claimreg_p6_r5_readonly
    }
    #[doc = "0x1a000 - CFG0_POR_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_por_ctrl_proxy(&self) -> &Cfg0PorCtrlProxy {
        &self.cfg0_por_ctrl_proxy
    }
    #[doc = "0x1a004 - CFG0_POR_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_por_stat_proxy(&self) -> &Cfg0PorStatProxy {
        &self.cfg0_por_stat_proxy
    }
    #[doc = "0x1a100 - CFG0_POR_BANDGAP_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_por_bandgap_ctrl_proxy(&self) -> &Cfg0PorBandgapCtrlProxy {
        &self.cfg0_por_bandgap_ctrl_proxy
    }
    #[doc = "0x1a110 - CFG0_POK_VDDA_MCU_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdda_mcu_uv_ctrl_proxy(&self) -> &Cfg0PokVddaMcuUvCtrlProxy {
        &self.cfg0_pok_vdda_mcu_uv_ctrl_proxy
    }
    #[doc = "0x1a114 - CFG0_POK_VDDA_MCU_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdda_mcu_ov_ctrl_proxy(&self) -> &Cfg0PokVddaMcuOvCtrlProxy {
        &self.cfg0_pok_vdda_mcu_ov_ctrl_proxy
    }
    #[doc = "0x1a118 - CFG0_POK_VDD_CORE_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdd_core_uv_ctrl_proxy(&self) -> &Cfg0PokVddCoreUvCtrlProxy {
        &self.cfg0_pok_vdd_core_uv_ctrl_proxy
    }
    #[doc = "0x1a11c - CFG0_POK_VDD_CORE_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdd_core_ov_ctrl_proxy(&self) -> &Cfg0PokVddCoreOvCtrlProxy {
        &self.cfg0_pok_vdd_core_ov_ctrl_proxy
    }
    #[doc = "0x1a120 - CFG0_POK_VDDR_CORE_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddr_core_uv_ctrl_proxy(&self) -> &Cfg0PokVddrCoreUvCtrlProxy {
        &self.cfg0_pok_vddr_core_uv_ctrl_proxy
    }
    #[doc = "0x1a124 - CFG0_POK_VDDR_CORE_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddr_core_ov_ctrl_proxy(&self) -> &Cfg0PokVddrCoreOvCtrlProxy {
        &self.cfg0_pok_vddr_core_ov_ctrl_proxy
    }
    #[doc = "0x1a128 - CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy(&self) -> &Cfg0PokVddshvMcu1p8UvCtrlProxy {
        &self.cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy
    }
    #[doc = "0x1a12c - CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy(&self) -> &Cfg0PokVddshvMcu1p8OvCtrlProxy {
        &self.cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy
    }
    #[doc = "0x1a130 - CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy(&self) -> &Cfg0PokVddshvMcu3p3UvCtrlProxy {
        &self.cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy
    }
    #[doc = "0x1a134 - CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy(&self) -> &Cfg0PokVddshvMcu3p3OvCtrlProxy {
        &self.cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy
    }
    #[doc = "0x1a138 - CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy(
        &self,
    ) -> &Cfg0PokVmonCapMcuGeneralUvCtrlProxy {
        &self.cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy
    }
    #[doc = "0x1a13c - CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy(
        &self,
    ) -> &Cfg0PokVmonCapMcuGeneralOvCtrlProxy {
        &self.cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy
    }
    #[doc = "0x1a140 - CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy(&self) -> &Cfg0PokVddshvMain1p8UvCtrlProxy {
        &self.cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy
    }
    #[doc = "0x1a144 - CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy(&self) -> &Cfg0PokVddshvMain1p8OvCtrlProxy {
        &self.cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy
    }
    #[doc = "0x1a148 - CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy(&self) -> &Cfg0PokVddshvMain3p3UvCtrlProxy {
        &self.cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy
    }
    #[doc = "0x1a14c - CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy(&self) -> &Cfg0PokVddshvMain3p3OvCtrlProxy {
        &self.cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy
    }
    #[doc = "0x1a150 - CFG0_POK_VDDS_DDRIO_UV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdds_ddrio_uv_ctrl_proxy(&self) -> &Cfg0PokVddsDdrioUvCtrlProxy {
        &self.cfg0_pok_vdds_ddrio_uv_ctrl_proxy
    }
    #[doc = "0x1a154 - CFG0_POK_VDDS_DDRIO_OV_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdds_ddrio_ov_ctrl_proxy(&self) -> &Cfg0PokVddsDdrioOvCtrlProxy {
        &self.cfg0_pok_vdds_ddrio_ov_ctrl_proxy
    }
    #[doc = "0x1a160 - CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pok_vdda_pmic_in_ctrl_proxy(&self) -> &Cfg0PokVddaPmicInCtrlProxy {
        &self.cfg0_pok_vdda_pmic_in_ctrl_proxy
    }
    #[doc = "0x1a170 - CFG0_RST_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_rst_ctrl_proxy(&self) -> &Cfg0RstCtrlProxy {
        &self.cfg0_rst_ctrl_proxy
    }
    #[doc = "0x1a174 - CFG0_RST_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_rst_stat_proxy(&self) -> &Cfg0RstStatProxy {
        &self.cfg0_rst_stat_proxy
    }
    #[doc = "0x1a178 - CFG0_RST_SRC_PROXY"]
    #[inline(always)]
    pub const fn cfg0_rst_src_proxy(&self) -> &Cfg0RstSrcProxy {
        &self.cfg0_rst_src_proxy
    }
    #[doc = "0x1a17c - CFG0_RST_MAGIC_WORD_PROXY"]
    #[inline(always)]
    pub const fn cfg0_rst_magic_word_proxy(&self) -> &Cfg0RstMagicWordProxy {
        &self.cfg0_rst_magic_word_proxy
    }
    #[doc = "0x1a180 - CFG0_ISO_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_iso_ctrl_proxy(&self) -> &Cfg0IsoCtrlProxy {
        &self.cfg0_iso_ctrl_proxy
    }
    #[doc = "0x1a190 - CFG0_VDD_CORE_GLDTC_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_vdd_core_gldtc_ctrl_proxy(&self) -> &Cfg0VddCoreGldtcCtrlProxy {
        &self.cfg0_vdd_core_gldtc_ctrl_proxy
    }
    #[doc = "0x1a1b0 - CFG0_VDD_CORE_GLDTC_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_vdd_core_gldtc_stat_proxy(&self) -> &Cfg0VddCoreGldtcStatProxy {
        &self.cfg0_vdd_core_gldtc_stat_proxy
    }
    #[doc = "0x1a200 - CFG0_PRG_PP_0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_prg_pp_0_ctrl_proxy(&self) -> &Cfg0PrgPp0CtrlProxy {
        &self.cfg0_prg_pp_0_ctrl_proxy
    }
    #[doc = "0x1a208 - CFG0_PRG_PP_1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_prg_pp_1_ctrl_proxy(&self) -> &Cfg0PrgPp1CtrlProxy {
        &self.cfg0_prg_pp_1_ctrl_proxy
    }
    #[doc = "0x1a284 - CFG0_MCU_CLKGATE_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcu_clkgate_ctrl_proxy(&self) -> &Cfg0McuClkgateCtrlProxy {
        &self.cfg0_mcu_clkgate_ctrl_proxy
    }
    #[doc = "0x1a288 - CFG0_MAIN_CLKGATE_CTRL0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_clkgate_ctrl0_proxy(&self) -> &Cfg0MainClkgateCtrl0Proxy {
        &self.cfg0_main_clkgate_ctrl0_proxy
    }
    #[doc = "0x1b008 - CFG0_LOCK6_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock6_kick0_proxy(&self) -> &Cfg0Lock6Kick0Proxy {
        &self.cfg0_lock6_kick0_proxy
    }
    #[doc = "0x1b00c - CFG0_LOCK6_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock6_kick1_proxy(&self) -> &Cfg0Lock6Kick1Proxy {
        &self.cfg0_lock6_kick1_proxy
    }
    #[doc = "0x1b100 - CFG0_CLAIMREG_P6_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r0(&self) -> &Cfg0ClaimregP6R0 {
        &self.cfg0_claimreg_p6_r0
    }
    #[doc = "0x1b104 - CFG0_CLAIMREG_P6_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r1(&self) -> &Cfg0ClaimregP6R1 {
        &self.cfg0_claimreg_p6_r1
    }
    #[doc = "0x1b108 - CFG0_CLAIMREG_P6_R2"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r2(&self) -> &Cfg0ClaimregP6R2 {
        &self.cfg0_claimreg_p6_r2
    }
    #[doc = "0x1b10c - CFG0_CLAIMREG_P6_R3"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r3(&self) -> &Cfg0ClaimregP6R3 {
        &self.cfg0_claimreg_p6_r3
    }
    #[doc = "0x1b110 - CFG0_CLAIMREG_P6_R4"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r4(&self) -> &Cfg0ClaimregP6R4 {
        &self.cfg0_claimreg_p6_r4
    }
    #[doc = "0x1b114 - CFG0_CLAIMREG_P6_R5"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p6_r5(&self) -> &Cfg0ClaimregP6R5 {
        &self.cfg0_claimreg_p6_r5
    }
}
#[doc = "CFG0_PID (rw) register accessor: CFG0_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pid`]
module"]
#[doc(alias = "CFG0_PID")]
pub type Cfg0Pid = crate::Reg<cfg0_pid::Cfg0PidSpec>;
#[doc = "CFG0_PID"]
pub mod cfg0_pid;
#[doc = "CFG0_MMR_CFG1 (rw) register accessor: CFG0_MMR_CFG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mmr_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mmr_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mmr_cfg1`]
module"]
#[doc(alias = "CFG0_MMR_CFG1")]
pub type Cfg0MmrCfg1 = crate::Reg<cfg0_mmr_cfg1::Cfg0MmrCfg1Spec>;
#[doc = "CFG0_MMR_CFG1"]
pub mod cfg0_mmr_cfg1;
#[doc = "CFG0_LOCK0_KICK0 (rw) register accessor: CFG0_LOCK0_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock0_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock0_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock0_kick0`]
module"]
#[doc(alias = "CFG0_LOCK0_KICK0")]
pub type Cfg0Lock0Kick0 = crate::Reg<cfg0_lock0_kick0::Cfg0Lock0Kick0Spec>;
#[doc = "CFG0_LOCK0_KICK0"]
pub mod cfg0_lock0_kick0;
#[doc = "CFG0_LOCK0_KICK1 (rw) register accessor: CFG0_LOCK0_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock0_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock0_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock0_kick1`]
module"]
#[doc(alias = "CFG0_LOCK0_KICK1")]
pub type Cfg0Lock0Kick1 = crate::Reg<cfg0_lock0_kick1::Cfg0Lock0Kick1Spec>;
#[doc = "CFG0_LOCK0_KICK1"]
pub mod cfg0_lock0_kick1;
#[doc = "CFG0_intr_raw_status (rw) register accessor: CFG0_intr_raw_status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_raw_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_raw_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_raw_status`]
module"]
#[doc(alias = "CFG0_intr_raw_status")]
pub type Cfg0IntrRawStatus = crate::Reg<cfg0_intr_raw_status::Cfg0IntrRawStatusSpec>;
#[doc = "CFG0_intr_raw_status"]
pub mod cfg0_intr_raw_status;
#[doc = "CFG0_intr_enabled_status_clear (rw) register accessor: CFG0_intr_enabled_status_clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enabled_status_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enabled_status_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_enabled_status_clear`]
module"]
#[doc(alias = "CFG0_intr_enabled_status_clear")]
pub type Cfg0IntrEnabledStatusClear =
    crate::Reg<cfg0_intr_enabled_status_clear::Cfg0IntrEnabledStatusClearSpec>;
#[doc = "CFG0_intr_enabled_status_clear"]
pub mod cfg0_intr_enabled_status_clear;
#[doc = "CFG0_intr_enable (rw) register accessor: CFG0_intr_enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_enable`]
module"]
#[doc(alias = "CFG0_intr_enable")]
pub type Cfg0IntrEnable = crate::Reg<cfg0_intr_enable::Cfg0IntrEnableSpec>;
#[doc = "CFG0_intr_enable"]
pub mod cfg0_intr_enable;
#[doc = "CFG0_intr_enable_clear (rw) register accessor: CFG0_intr_enable_clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_enable_clear`]
module"]
#[doc(alias = "CFG0_intr_enable_clear")]
pub type Cfg0IntrEnableClear = crate::Reg<cfg0_intr_enable_clear::Cfg0IntrEnableClearSpec>;
#[doc = "CFG0_intr_enable_clear"]
pub mod cfg0_intr_enable_clear;
#[doc = "CFG0_eoi (rw) register accessor: CFG0_eoi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eoi`]
module"]
#[doc(alias = "CFG0_eoi")]
pub type Cfg0Eoi = crate::Reg<cfg0_eoi::Cfg0EoiSpec>;
#[doc = "CFG0_eoi"]
pub mod cfg0_eoi;
#[doc = "CFG0_fault_address (rw) register accessor: CFG0_fault_address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_address`]
module"]
#[doc(alias = "CFG0_fault_address")]
pub type Cfg0FaultAddress = crate::Reg<cfg0_fault_address::Cfg0FaultAddressSpec>;
#[doc = "CFG0_fault_address"]
pub mod cfg0_fault_address;
#[doc = "CFG0_fault_type_status (rw) register accessor: CFG0_fault_type_status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_type_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_type_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_type_status`]
module"]
#[doc(alias = "CFG0_fault_type_status")]
pub type Cfg0FaultTypeStatus = crate::Reg<cfg0_fault_type_status::Cfg0FaultTypeStatusSpec>;
#[doc = "CFG0_fault_type_status"]
pub mod cfg0_fault_type_status;
#[doc = "CFG0_fault_attr_status (rw) register accessor: CFG0_fault_attr_status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_attr_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_attr_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_attr_status`]
module"]
#[doc(alias = "CFG0_fault_attr_status")]
pub type Cfg0FaultAttrStatus = crate::Reg<cfg0_fault_attr_status::Cfg0FaultAttrStatusSpec>;
#[doc = "CFG0_fault_attr_status"]
pub mod cfg0_fault_attr_status;
#[doc = "CFG0_fault_clear (rw) register accessor: CFG0_fault_clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_clear`]
module"]
#[doc(alias = "CFG0_fault_clear")]
pub type Cfg0FaultClear = crate::Reg<cfg0_fault_clear::Cfg0FaultClearSpec>;
#[doc = "CFG0_fault_clear"]
pub mod cfg0_fault_clear;
#[doc = "CFG0_CLAIMREG_P0_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R0_READONLY")]
pub type Cfg0ClaimregP0R0Readonly =
    crate::Reg<cfg0_claimreg_p0_r0_readonly::Cfg0ClaimregP0R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R0_READONLY"]
pub mod cfg0_claimreg_p0_r0_readonly;
#[doc = "CFG0_PID_PROXY (rw) register accessor: CFG0_PID_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pid_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pid_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pid_proxy`]
module"]
#[doc(alias = "CFG0_PID_PROXY")]
pub type Cfg0PidProxy = crate::Reg<cfg0_pid_proxy::Cfg0PidProxySpec>;
#[doc = "CFG0_PID_PROXY"]
pub mod cfg0_pid_proxy;
#[doc = "CFG0_MMR_CFG1_PROXY (rw) register accessor: CFG0_MMR_CFG1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mmr_cfg1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mmr_cfg1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mmr_cfg1_proxy`]
module"]
#[doc(alias = "CFG0_MMR_CFG1_PROXY")]
pub type Cfg0MmrCfg1Proxy = crate::Reg<cfg0_mmr_cfg1_proxy::Cfg0MmrCfg1ProxySpec>;
#[doc = "CFG0_MMR_CFG1_PROXY"]
pub mod cfg0_mmr_cfg1_proxy;
#[doc = "CFG0_LOCK0_KICK0_PROXY (rw) register accessor: CFG0_LOCK0_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock0_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock0_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock0_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK0_KICK0_PROXY")]
pub type Cfg0Lock0Kick0Proxy = crate::Reg<cfg0_lock0_kick0_proxy::Cfg0Lock0Kick0ProxySpec>;
#[doc = "CFG0_LOCK0_KICK0_PROXY"]
pub mod cfg0_lock0_kick0_proxy;
#[doc = "CFG0_LOCK0_KICK1_PROXY (rw) register accessor: CFG0_LOCK0_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock0_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock0_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock0_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK0_KICK1_PROXY")]
pub type Cfg0Lock0Kick1Proxy = crate::Reg<cfg0_lock0_kick1_proxy::Cfg0Lock0Kick1ProxySpec>;
#[doc = "CFG0_LOCK0_KICK1_PROXY"]
pub mod cfg0_lock0_kick1_proxy;
#[doc = "CFG0_intr_raw_status_PROXY (rw) register accessor: CFG0_intr_raw_status_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_raw_status_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_raw_status_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_raw_status_proxy`]
module"]
#[doc(alias = "CFG0_intr_raw_status_PROXY")]
pub type Cfg0IntrRawStatusProxy =
    crate::Reg<cfg0_intr_raw_status_proxy::Cfg0IntrRawStatusProxySpec>;
#[doc = "CFG0_intr_raw_status_PROXY"]
pub mod cfg0_intr_raw_status_proxy;
#[doc = "CFG0_intr_enabled_status_clear_PROXY (rw) register accessor: CFG0_intr_enabled_status_clear_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enabled_status_clear_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enabled_status_clear_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_enabled_status_clear_proxy`]
module"]
#[doc(alias = "CFG0_intr_enabled_status_clear_PROXY")]
pub type Cfg0IntrEnabledStatusClearProxy =
    crate::Reg<cfg0_intr_enabled_status_clear_proxy::Cfg0IntrEnabledStatusClearProxySpec>;
#[doc = "CFG0_intr_enabled_status_clear_PROXY"]
pub mod cfg0_intr_enabled_status_clear_proxy;
#[doc = "CFG0_intr_enable_PROXY (rw) register accessor: CFG0_intr_enable_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_enable_proxy`]
module"]
#[doc(alias = "CFG0_intr_enable_PROXY")]
pub type Cfg0IntrEnableProxy = crate::Reg<cfg0_intr_enable_proxy::Cfg0IntrEnableProxySpec>;
#[doc = "CFG0_intr_enable_PROXY"]
pub mod cfg0_intr_enable_proxy;
#[doc = "CFG0_intr_enable_clear_PROXY (rw) register accessor: CFG0_intr_enable_clear_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable_clear_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable_clear_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_intr_enable_clear_proxy`]
module"]
#[doc(alias = "CFG0_intr_enable_clear_PROXY")]
pub type Cfg0IntrEnableClearProxy =
    crate::Reg<cfg0_intr_enable_clear_proxy::Cfg0IntrEnableClearProxySpec>;
#[doc = "CFG0_intr_enable_clear_PROXY"]
pub mod cfg0_intr_enable_clear_proxy;
#[doc = "CFG0_eoi_PROXY (rw) register accessor: CFG0_eoi_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eoi_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eoi_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eoi_proxy`]
module"]
#[doc(alias = "CFG0_eoi_PROXY")]
pub type Cfg0EoiProxy = crate::Reg<cfg0_eoi_proxy::Cfg0EoiProxySpec>;
#[doc = "CFG0_eoi_PROXY"]
pub mod cfg0_eoi_proxy;
#[doc = "CFG0_fault_address_PROXY (rw) register accessor: CFG0_fault_address_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_address_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_address_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_address_proxy`]
module"]
#[doc(alias = "CFG0_fault_address_PROXY")]
pub type Cfg0FaultAddressProxy = crate::Reg<cfg0_fault_address_proxy::Cfg0FaultAddressProxySpec>;
#[doc = "CFG0_fault_address_PROXY"]
pub mod cfg0_fault_address_proxy;
#[doc = "CFG0_fault_type_status_PROXY (rw) register accessor: CFG0_fault_type_status_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_type_status_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_type_status_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_type_status_proxy`]
module"]
#[doc(alias = "CFG0_fault_type_status_PROXY")]
pub type Cfg0FaultTypeStatusProxy =
    crate::Reg<cfg0_fault_type_status_proxy::Cfg0FaultTypeStatusProxySpec>;
#[doc = "CFG0_fault_type_status_PROXY"]
pub mod cfg0_fault_type_status_proxy;
#[doc = "CFG0_fault_attr_status_PROXY (rw) register accessor: CFG0_fault_attr_status_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_attr_status_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_attr_status_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_attr_status_proxy`]
module"]
#[doc(alias = "CFG0_fault_attr_status_PROXY")]
pub type Cfg0FaultAttrStatusProxy =
    crate::Reg<cfg0_fault_attr_status_proxy::Cfg0FaultAttrStatusProxySpec>;
#[doc = "CFG0_fault_attr_status_PROXY"]
pub mod cfg0_fault_attr_status_proxy;
#[doc = "CFG0_fault_clear_PROXY (rw) register accessor: CFG0_fault_clear_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_clear_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_clear_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fault_clear_proxy`]
module"]
#[doc(alias = "CFG0_fault_clear_PROXY")]
pub type Cfg0FaultClearProxy = crate::Reg<cfg0_fault_clear_proxy::Cfg0FaultClearProxySpec>;
#[doc = "CFG0_fault_clear_PROXY"]
pub mod cfg0_fault_clear_proxy;
#[doc = "CFG0_CLAIMREG_P0_R0 (rw) register accessor: CFG0_CLAIMREG_P0_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R0")]
pub type Cfg0ClaimregP0R0 = crate::Reg<cfg0_claimreg_p0_r0::Cfg0ClaimregP0R0Spec>;
#[doc = "CFG0_CLAIMREG_P0_R0"]
pub mod cfg0_claimreg_p0_r0;
#[doc = "CFG0_DBOUNCE_CFG1 (rw) register accessor: CFG0_DBOUNCE_CFG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg1`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG1")]
pub type Cfg0DbounceCfg1 = crate::Reg<cfg0_dbounce_cfg1::Cfg0DbounceCfg1Spec>;
#[doc = "CFG0_DBOUNCE_CFG1"]
pub mod cfg0_dbounce_cfg1;
#[doc = "CFG0_DBOUNCE_CFG2 (rw) register accessor: CFG0_DBOUNCE_CFG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg2`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG2")]
pub type Cfg0DbounceCfg2 = crate::Reg<cfg0_dbounce_cfg2::Cfg0DbounceCfg2Spec>;
#[doc = "CFG0_DBOUNCE_CFG2"]
pub mod cfg0_dbounce_cfg2;
#[doc = "CFG0_DBOUNCE_CFG3 (rw) register accessor: CFG0_DBOUNCE_CFG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg3`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG3")]
pub type Cfg0DbounceCfg3 = crate::Reg<cfg0_dbounce_cfg3::Cfg0DbounceCfg3Spec>;
#[doc = "CFG0_DBOUNCE_CFG3"]
pub mod cfg0_dbounce_cfg3;
#[doc = "CFG0_DBOUNCE_CFG4 (rw) register accessor: CFG0_DBOUNCE_CFG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg4`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG4")]
pub type Cfg0DbounceCfg4 = crate::Reg<cfg0_dbounce_cfg4::Cfg0DbounceCfg4Spec>;
#[doc = "CFG0_DBOUNCE_CFG4"]
pub mod cfg0_dbounce_cfg4;
#[doc = "CFG0_DBOUNCE_CFG5 (rw) register accessor: CFG0_DBOUNCE_CFG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg5`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG5")]
pub type Cfg0DbounceCfg5 = crate::Reg<cfg0_dbounce_cfg5::Cfg0DbounceCfg5Spec>;
#[doc = "CFG0_DBOUNCE_CFG5"]
pub mod cfg0_dbounce_cfg5;
#[doc = "CFG0_DBOUNCE_CFG6 (rw) register accessor: CFG0_DBOUNCE_CFG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg6`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG6")]
pub type Cfg0DbounceCfg6 = crate::Reg<cfg0_dbounce_cfg6::Cfg0DbounceCfg6Spec>;
#[doc = "CFG0_DBOUNCE_CFG6"]
pub mod cfg0_dbounce_cfg6;
#[doc = "CFG0_TEMP_DIODE_TRIM (rw) register accessor: CFG0_TEMP_DIODE_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_temp_diode_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_temp_diode_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_temp_diode_trim`]
module"]
#[doc(alias = "CFG0_TEMP_DIODE_TRIM")]
pub type Cfg0TempDiodeTrim = crate::Reg<cfg0_temp_diode_trim::Cfg0TempDiodeTrimSpec>;
#[doc = "CFG0_TEMP_DIODE_TRIM"]
pub mod cfg0_temp_diode_trim;
#[doc = "CFG0_IO_VOLTAGE_STAT (rw) register accessor: CFG0_IO_VOLTAGE_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_io_voltage_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_io_voltage_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_io_voltage_stat`]
module"]
#[doc(alias = "CFG0_IO_VOLTAGE_STAT")]
pub type Cfg0IoVoltageStat = crate::Reg<cfg0_io_voltage_stat::Cfg0IoVoltageStatSpec>;
#[doc = "CFG0_IO_VOLTAGE_STAT"]
pub mod cfg0_io_voltage_stat;
#[doc = "CFG0_MCU_TIMER1_CTRL (rw) register accessor: CFG0_MCU_TIMER1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer1_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_TIMER1_CTRL")]
pub type Cfg0McuTimer1Ctrl = crate::Reg<cfg0_mcu_timer1_ctrl::Cfg0McuTimer1CtrlSpec>;
#[doc = "CFG0_MCU_TIMER1_CTRL"]
pub mod cfg0_mcu_timer1_ctrl;
#[doc = "CFG0_MCU_TIMER3_CTRL (rw) register accessor: CFG0_MCU_TIMER3_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer3_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_TIMER3_CTRL")]
pub type Cfg0McuTimer3Ctrl = crate::Reg<cfg0_mcu_timer3_ctrl::Cfg0McuTimer3CtrlSpec>;
#[doc = "CFG0_MCU_TIMER3_CTRL"]
pub mod cfg0_mcu_timer3_ctrl;
#[doc = "CFG0_MCU_I2C0_CTRL (rw) register accessor: CFG0_MCU_I2C0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_i2c0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_i2c0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_i2c0_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_I2C0_CTRL")]
pub type Cfg0McuI2c0Ctrl = crate::Reg<cfg0_mcu_i2c0_ctrl::Cfg0McuI2c0CtrlSpec>;
#[doc = "CFG0_MCU_I2C0_CTRL"]
pub mod cfg0_mcu_i2c0_ctrl;
#[doc = "CFG0_MCU_MTOG_CTRL (rw) register accessor: CFG0_MCU_MTOG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_mtog_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_mtog_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_mtog_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_MTOG_CTRL")]
pub type Cfg0McuMtogCtrl = crate::Reg<cfg0_mcu_mtog_ctrl::Cfg0McuMtogCtrlSpec>;
#[doc = "CFG0_MCU_MTOG_CTRL"]
pub mod cfg0_mcu_mtog_ctrl;
#[doc = "CFG0_LOCK1_KICK0 (rw) register accessor: CFG0_LOCK1_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock1_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock1_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock1_kick0`]
module"]
#[doc(alias = "CFG0_LOCK1_KICK0")]
pub type Cfg0Lock1Kick0 = crate::Reg<cfg0_lock1_kick0::Cfg0Lock1Kick0Spec>;
#[doc = "CFG0_LOCK1_KICK0"]
pub mod cfg0_lock1_kick0;
#[doc = "CFG0_LOCK1_KICK1 (rw) register accessor: CFG0_LOCK1_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock1_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock1_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock1_kick1`]
module"]
#[doc(alias = "CFG0_LOCK1_KICK1")]
pub type Cfg0Lock1Kick1 = crate::Reg<cfg0_lock1_kick1::Cfg0Lock1Kick1Spec>;
#[doc = "CFG0_LOCK1_KICK1"]
pub mod cfg0_lock1_kick1;
#[doc = "CFG0_CLAIMREG_P1_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R0_READONLY")]
pub type Cfg0ClaimregP1R0Readonly =
    crate::Reg<cfg0_claimreg_p1_r0_readonly::Cfg0ClaimregP1R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R0_READONLY"]
pub mod cfg0_claimreg_p1_r0_readonly;
#[doc = "CFG0_CLAIMREG_P1_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R1_READONLY")]
pub type Cfg0ClaimregP1R1Readonly =
    crate::Reg<cfg0_claimreg_p1_r1_readonly::Cfg0ClaimregP1R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R1_READONLY"]
pub mod cfg0_claimreg_p1_r1_readonly;
#[doc = "CFG0_CLAIMREG_P1_R2_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R2_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r2_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r2_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r2_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R2_READONLY")]
pub type Cfg0ClaimregP1R2Readonly =
    crate::Reg<cfg0_claimreg_p1_r2_readonly::Cfg0ClaimregP1R2ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R2_READONLY"]
pub mod cfg0_claimreg_p1_r2_readonly;
#[doc = "CFG0_CLAIMREG_P1_R3_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R3_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r3_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r3_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r3_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R3_READONLY")]
pub type Cfg0ClaimregP1R3Readonly =
    crate::Reg<cfg0_claimreg_p1_r3_readonly::Cfg0ClaimregP1R3ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R3_READONLY"]
pub mod cfg0_claimreg_p1_r3_readonly;
#[doc = "CFG0_CLAIMREG_P1_R4_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r4_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r4_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r4_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R4_READONLY")]
pub type Cfg0ClaimregP1R4Readonly =
    crate::Reg<cfg0_claimreg_p1_r4_readonly::Cfg0ClaimregP1R4ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R4_READONLY"]
pub mod cfg0_claimreg_p1_r4_readonly;
#[doc = "CFG0_CLAIMREG_P1_R5_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r5_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r5_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r5_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R5_READONLY")]
pub type Cfg0ClaimregP1R5Readonly =
    crate::Reg<cfg0_claimreg_p1_r5_readonly::Cfg0ClaimregP1R5ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R5_READONLY"]
pub mod cfg0_claimreg_p1_r5_readonly;
#[doc = "CFG0_CLAIMREG_P1_R6_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R6_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r6_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r6_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r6_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R6_READONLY")]
pub type Cfg0ClaimregP1R6Readonly =
    crate::Reg<cfg0_claimreg_p1_r6_readonly::Cfg0ClaimregP1R6ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R6_READONLY"]
pub mod cfg0_claimreg_p1_r6_readonly;
#[doc = "CFG0_CLAIMREG_P1_R7_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R7_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r7_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r7_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r7_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R7_READONLY")]
pub type Cfg0ClaimregP1R7Readonly =
    crate::Reg<cfg0_claimreg_p1_r7_readonly::Cfg0ClaimregP1R7ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R7_READONLY"]
pub mod cfg0_claimreg_p1_r7_readonly;
#[doc = "CFG0_CLAIMREG_P1_R8_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R8_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r8_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r8_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r8_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R8_READONLY")]
pub type Cfg0ClaimregP1R8Readonly =
    crate::Reg<cfg0_claimreg_p1_r8_readonly::Cfg0ClaimregP1R8ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R8_READONLY"]
pub mod cfg0_claimreg_p1_r8_readonly;
#[doc = "CFG0_CLAIMREG_P1_R9_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R9_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r9_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r9_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r9_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R9_READONLY")]
pub type Cfg0ClaimregP1R9Readonly =
    crate::Reg<cfg0_claimreg_p1_r9_readonly::Cfg0ClaimregP1R9ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R9_READONLY"]
pub mod cfg0_claimreg_p1_r9_readonly;
#[doc = "CFG0_CLAIMREG_P1_R10_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R10_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r10_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r10_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r10_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R10_READONLY")]
pub type Cfg0ClaimregP1R10Readonly =
    crate::Reg<cfg0_claimreg_p1_r10_readonly::Cfg0ClaimregP1R10ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R10_READONLY"]
pub mod cfg0_claimreg_p1_r10_readonly;
#[doc = "CFG0_CLAIMREG_P1_R11_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R11_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r11_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r11_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r11_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R11_READONLY")]
pub type Cfg0ClaimregP1R11Readonly =
    crate::Reg<cfg0_claimreg_p1_r11_readonly::Cfg0ClaimregP1R11ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R11_READONLY"]
pub mod cfg0_claimreg_p1_r11_readonly;
#[doc = "CFG0_CLAIMREG_P1_R12_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R12_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r12_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r12_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r12_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R12_READONLY")]
pub type Cfg0ClaimregP1R12Readonly =
    crate::Reg<cfg0_claimreg_p1_r12_readonly::Cfg0ClaimregP1R12ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R12_READONLY"]
pub mod cfg0_claimreg_p1_r12_readonly;
#[doc = "CFG0_DBOUNCE_CFG1_PROXY (rw) register accessor: CFG0_DBOUNCE_CFG1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg1_proxy`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG1_PROXY")]
pub type Cfg0DbounceCfg1Proxy = crate::Reg<cfg0_dbounce_cfg1_proxy::Cfg0DbounceCfg1ProxySpec>;
#[doc = "CFG0_DBOUNCE_CFG1_PROXY"]
pub mod cfg0_dbounce_cfg1_proxy;
#[doc = "CFG0_DBOUNCE_CFG2_PROXY (rw) register accessor: CFG0_DBOUNCE_CFG2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg2_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg2_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg2_proxy`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG2_PROXY")]
pub type Cfg0DbounceCfg2Proxy = crate::Reg<cfg0_dbounce_cfg2_proxy::Cfg0DbounceCfg2ProxySpec>;
#[doc = "CFG0_DBOUNCE_CFG2_PROXY"]
pub mod cfg0_dbounce_cfg2_proxy;
#[doc = "CFG0_DBOUNCE_CFG3_PROXY (rw) register accessor: CFG0_DBOUNCE_CFG3_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg3_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg3_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg3_proxy`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG3_PROXY")]
pub type Cfg0DbounceCfg3Proxy = crate::Reg<cfg0_dbounce_cfg3_proxy::Cfg0DbounceCfg3ProxySpec>;
#[doc = "CFG0_DBOUNCE_CFG3_PROXY"]
pub mod cfg0_dbounce_cfg3_proxy;
#[doc = "CFG0_DBOUNCE_CFG4_PROXY (rw) register accessor: CFG0_DBOUNCE_CFG4_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg4_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg4_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg4_proxy`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG4_PROXY")]
pub type Cfg0DbounceCfg4Proxy = crate::Reg<cfg0_dbounce_cfg4_proxy::Cfg0DbounceCfg4ProxySpec>;
#[doc = "CFG0_DBOUNCE_CFG4_PROXY"]
pub mod cfg0_dbounce_cfg4_proxy;
#[doc = "CFG0_DBOUNCE_CFG5_PROXY (rw) register accessor: CFG0_DBOUNCE_CFG5_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg5_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg5_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg5_proxy`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG5_PROXY")]
pub type Cfg0DbounceCfg5Proxy = crate::Reg<cfg0_dbounce_cfg5_proxy::Cfg0DbounceCfg5ProxySpec>;
#[doc = "CFG0_DBOUNCE_CFG5_PROXY"]
pub mod cfg0_dbounce_cfg5_proxy;
#[doc = "CFG0_DBOUNCE_CFG6_PROXY (rw) register accessor: CFG0_DBOUNCE_CFG6_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg6_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg6_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dbounce_cfg6_proxy`]
module"]
#[doc(alias = "CFG0_DBOUNCE_CFG6_PROXY")]
pub type Cfg0DbounceCfg6Proxy = crate::Reg<cfg0_dbounce_cfg6_proxy::Cfg0DbounceCfg6ProxySpec>;
#[doc = "CFG0_DBOUNCE_CFG6_PROXY"]
pub mod cfg0_dbounce_cfg6_proxy;
#[doc = "CFG0_TEMP_DIODE_TRIM_PROXY (rw) register accessor: CFG0_TEMP_DIODE_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_temp_diode_trim_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_temp_diode_trim_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_temp_diode_trim_proxy`]
module"]
#[doc(alias = "CFG0_TEMP_DIODE_TRIM_PROXY")]
pub type Cfg0TempDiodeTrimProxy =
    crate::Reg<cfg0_temp_diode_trim_proxy::Cfg0TempDiodeTrimProxySpec>;
#[doc = "CFG0_TEMP_DIODE_TRIM_PROXY"]
pub mod cfg0_temp_diode_trim_proxy;
#[doc = "CFG0_IO_VOLTAGE_STAT_PROXY (rw) register accessor: CFG0_IO_VOLTAGE_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_io_voltage_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_io_voltage_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_io_voltage_stat_proxy`]
module"]
#[doc(alias = "CFG0_IO_VOLTAGE_STAT_PROXY")]
pub type Cfg0IoVoltageStatProxy =
    crate::Reg<cfg0_io_voltage_stat_proxy::Cfg0IoVoltageStatProxySpec>;
#[doc = "CFG0_IO_VOLTAGE_STAT_PROXY"]
pub mod cfg0_io_voltage_stat_proxy;
#[doc = "CFG0_MCU_TIMER1_CTRL_PROXY (rw) register accessor: CFG0_MCU_TIMER1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_TIMER1_CTRL_PROXY")]
pub type Cfg0McuTimer1CtrlProxy =
    crate::Reg<cfg0_mcu_timer1_ctrl_proxy::Cfg0McuTimer1CtrlProxySpec>;
#[doc = "CFG0_MCU_TIMER1_CTRL_PROXY"]
pub mod cfg0_mcu_timer1_ctrl_proxy;
#[doc = "CFG0_MCU_TIMER3_CTRL_PROXY (rw) register accessor: CFG0_MCU_TIMER3_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer3_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer3_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer3_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_TIMER3_CTRL_PROXY")]
pub type Cfg0McuTimer3CtrlProxy =
    crate::Reg<cfg0_mcu_timer3_ctrl_proxy::Cfg0McuTimer3CtrlProxySpec>;
#[doc = "CFG0_MCU_TIMER3_CTRL_PROXY"]
pub mod cfg0_mcu_timer3_ctrl_proxy;
#[doc = "CFG0_MCU_I2C0_CTRL_PROXY (rw) register accessor: CFG0_MCU_I2C0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_i2c0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_i2c0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_i2c0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_I2C0_CTRL_PROXY")]
pub type Cfg0McuI2c0CtrlProxy = crate::Reg<cfg0_mcu_i2c0_ctrl_proxy::Cfg0McuI2c0CtrlProxySpec>;
#[doc = "CFG0_MCU_I2C0_CTRL_PROXY"]
pub mod cfg0_mcu_i2c0_ctrl_proxy;
#[doc = "CFG0_MCU_MTOG_CTRL_PROXY (rw) register accessor: CFG0_MCU_MTOG_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_mtog_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_mtog_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_mtog_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_MTOG_CTRL_PROXY")]
pub type Cfg0McuMtogCtrlProxy = crate::Reg<cfg0_mcu_mtog_ctrl_proxy::Cfg0McuMtogCtrlProxySpec>;
#[doc = "CFG0_MCU_MTOG_CTRL_PROXY"]
pub mod cfg0_mcu_mtog_ctrl_proxy;
#[doc = "CFG0_LOCK1_KICK0_PROXY (rw) register accessor: CFG0_LOCK1_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock1_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock1_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock1_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK1_KICK0_PROXY")]
pub type Cfg0Lock1Kick0Proxy = crate::Reg<cfg0_lock1_kick0_proxy::Cfg0Lock1Kick0ProxySpec>;
#[doc = "CFG0_LOCK1_KICK0_PROXY"]
pub mod cfg0_lock1_kick0_proxy;
#[doc = "CFG0_LOCK1_KICK1_PROXY (rw) register accessor: CFG0_LOCK1_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock1_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock1_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock1_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK1_KICK1_PROXY")]
pub type Cfg0Lock1Kick1Proxy = crate::Reg<cfg0_lock1_kick1_proxy::Cfg0Lock1Kick1ProxySpec>;
#[doc = "CFG0_LOCK1_KICK1_PROXY"]
pub mod cfg0_lock1_kick1_proxy;
#[doc = "CFG0_CLAIMREG_P1_R0 (rw) register accessor: CFG0_CLAIMREG_P1_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R0")]
pub type Cfg0ClaimregP1R0 = crate::Reg<cfg0_claimreg_p1_r0::Cfg0ClaimregP1R0Spec>;
#[doc = "CFG0_CLAIMREG_P1_R0"]
pub mod cfg0_claimreg_p1_r0;
#[doc = "CFG0_CLAIMREG_P1_R1 (rw) register accessor: CFG0_CLAIMREG_P1_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R1")]
pub type Cfg0ClaimregP1R1 = crate::Reg<cfg0_claimreg_p1_r1::Cfg0ClaimregP1R1Spec>;
#[doc = "CFG0_CLAIMREG_P1_R1"]
pub mod cfg0_claimreg_p1_r1;
#[doc = "CFG0_CLAIMREG_P1_R2 (rw) register accessor: CFG0_CLAIMREG_P1_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r2`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R2")]
pub type Cfg0ClaimregP1R2 = crate::Reg<cfg0_claimreg_p1_r2::Cfg0ClaimregP1R2Spec>;
#[doc = "CFG0_CLAIMREG_P1_R2"]
pub mod cfg0_claimreg_p1_r2;
#[doc = "CFG0_CLAIMREG_P1_R3 (rw) register accessor: CFG0_CLAIMREG_P1_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r3`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R3")]
pub type Cfg0ClaimregP1R3 = crate::Reg<cfg0_claimreg_p1_r3::Cfg0ClaimregP1R3Spec>;
#[doc = "CFG0_CLAIMREG_P1_R3"]
pub mod cfg0_claimreg_p1_r3;
#[doc = "CFG0_CLAIMREG_P1_R4 (rw) register accessor: CFG0_CLAIMREG_P1_R4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r4`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R4")]
pub type Cfg0ClaimregP1R4 = crate::Reg<cfg0_claimreg_p1_r4::Cfg0ClaimregP1R4Spec>;
#[doc = "CFG0_CLAIMREG_P1_R4"]
pub mod cfg0_claimreg_p1_r4;
#[doc = "CFG0_CLAIMREG_P1_R5 (rw) register accessor: CFG0_CLAIMREG_P1_R5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r5`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R5")]
pub type Cfg0ClaimregP1R5 = crate::Reg<cfg0_claimreg_p1_r5::Cfg0ClaimregP1R5Spec>;
#[doc = "CFG0_CLAIMREG_P1_R5"]
pub mod cfg0_claimreg_p1_r5;
#[doc = "CFG0_CLAIMREG_P1_R6 (rw) register accessor: CFG0_CLAIMREG_P1_R6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r6`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R6")]
pub type Cfg0ClaimregP1R6 = crate::Reg<cfg0_claimreg_p1_r6::Cfg0ClaimregP1R6Spec>;
#[doc = "CFG0_CLAIMREG_P1_R6"]
pub mod cfg0_claimreg_p1_r6;
#[doc = "CFG0_CLAIMREG_P1_R7 (rw) register accessor: CFG0_CLAIMREG_P1_R7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r7`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R7")]
pub type Cfg0ClaimregP1R7 = crate::Reg<cfg0_claimreg_p1_r7::Cfg0ClaimregP1R7Spec>;
#[doc = "CFG0_CLAIMREG_P1_R7"]
pub mod cfg0_claimreg_p1_r7;
#[doc = "CFG0_CLAIMREG_P1_R8 (rw) register accessor: CFG0_CLAIMREG_P1_R8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r8`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R8")]
pub type Cfg0ClaimregP1R8 = crate::Reg<cfg0_claimreg_p1_r8::Cfg0ClaimregP1R8Spec>;
#[doc = "CFG0_CLAIMREG_P1_R8"]
pub mod cfg0_claimreg_p1_r8;
#[doc = "CFG0_CLAIMREG_P1_R9 (rw) register accessor: CFG0_CLAIMREG_P1_R9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r9`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R9")]
pub type Cfg0ClaimregP1R9 = crate::Reg<cfg0_claimreg_p1_r9::Cfg0ClaimregP1R9Spec>;
#[doc = "CFG0_CLAIMREG_P1_R9"]
pub mod cfg0_claimreg_p1_r9;
#[doc = "CFG0_CLAIMREG_P1_R10 (rw) register accessor: CFG0_CLAIMREG_P1_R10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r10`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R10")]
pub type Cfg0ClaimregP1R10 = crate::Reg<cfg0_claimreg_p1_r10::Cfg0ClaimregP1R10Spec>;
#[doc = "CFG0_CLAIMREG_P1_R10"]
pub mod cfg0_claimreg_p1_r10;
#[doc = "CFG0_CLAIMREG_P1_R11 (rw) register accessor: CFG0_CLAIMREG_P1_R11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r11`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R11")]
pub type Cfg0ClaimregP1R11 = crate::Reg<cfg0_claimreg_p1_r11::Cfg0ClaimregP1R11Spec>;
#[doc = "CFG0_CLAIMREG_P1_R11"]
pub mod cfg0_claimreg_p1_r11;
#[doc = "CFG0_CLAIMREG_P1_R12 (rw) register accessor: CFG0_CLAIMREG_P1_R12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r12`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R12")]
pub type Cfg0ClaimregP1R12 = crate::Reg<cfg0_claimreg_p1_r12::Cfg0ClaimregP1R12Spec>;
#[doc = "CFG0_CLAIMREG_P1_R12"]
pub mod cfg0_claimreg_p1_r12;
#[doc = "CFG0_MCU_OBSCLK_CTRL (rw) register accessor: CFG0_MCU_OBSCLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_obsclk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_obsclk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_obsclk_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_OBSCLK_CTRL")]
pub type Cfg0McuObsclkCtrl = crate::Reg<cfg0_mcu_obsclk_ctrl::Cfg0McuObsclkCtrlSpec>;
#[doc = "CFG0_MCU_OBSCLK_CTRL"]
pub mod cfg0_mcu_obsclk_ctrl;
#[doc = "CFG0_HFOSC0_CTRL (rw) register accessor: CFG0_HFOSC0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_hfosc0_ctrl`]
module"]
#[doc(alias = "CFG0_HFOSC0_CTRL")]
pub type Cfg0Hfosc0Ctrl = crate::Reg<cfg0_hfosc0_ctrl::Cfg0Hfosc0CtrlSpec>;
#[doc = "CFG0_HFOSC0_CTRL"]
pub mod cfg0_hfosc0_ctrl;
#[doc = "CFG0_HFOSC0_TRIM (rw) register accessor: CFG0_HFOSC0_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_hfosc0_trim`]
module"]
#[doc(alias = "CFG0_HFOSC0_TRIM")]
pub type Cfg0Hfosc0Trim = crate::Reg<cfg0_hfosc0_trim::Cfg0Hfosc0TrimSpec>;
#[doc = "CFG0_HFOSC0_TRIM"]
pub mod cfg0_hfosc0_trim;
#[doc = "CFG0_RC12M_OSC_TRIM (rw) register accessor: CFG0_RC12M_OSC_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rc12m_osc_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rc12m_osc_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rc12m_osc_trim`]
module"]
#[doc(alias = "CFG0_RC12M_OSC_TRIM")]
pub type Cfg0Rc12mOscTrim = crate::Reg<cfg0_rc12m_osc_trim::Cfg0Rc12mOscTrimSpec>;
#[doc = "CFG0_RC12M_OSC_TRIM"]
pub mod cfg0_rc12m_osc_trim;
#[doc = "CFG0_HFOSC0_CLKOUT_32K_CTRL (rw) register accessor: CFG0_HFOSC0_CLKOUT_32K_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_clkout_32k_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_clkout_32k_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_hfosc0_clkout_32k_ctrl`]
module"]
#[doc(alias = "CFG0_HFOSC0_CLKOUT_32K_CTRL")]
pub type Cfg0Hfosc0Clkout32kCtrl =
    crate::Reg<cfg0_hfosc0_clkout_32k_ctrl::Cfg0Hfosc0Clkout32kCtrlSpec>;
#[doc = "CFG0_HFOSC0_CLKOUT_32K_CTRL"]
pub mod cfg0_hfosc0_clkout_32k_ctrl;
#[doc = "CFG0_MCU_M4FSS_CLKSEL (rw) register accessor: CFG0_MCU_M4FSS_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss_clksel`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS_CLKSEL")]
pub type Cfg0McuM4fssClksel = crate::Reg<cfg0_mcu_m4fss_clksel::Cfg0McuM4fssClkselSpec>;
#[doc = "CFG0_MCU_M4FSS_CLKSEL"]
pub mod cfg0_mcu_m4fss_clksel;
#[doc = "CFG0_MCU_M4FSS_SYSTICK (rw) register accessor: CFG0_MCU_M4FSS_SYSTICK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_systick::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_systick::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss_systick`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS_SYSTICK")]
pub type Cfg0McuM4fssSystick = crate::Reg<cfg0_mcu_m4fss_systick::Cfg0McuM4fssSystickSpec>;
#[doc = "CFG0_MCU_M4FSS_SYSTICK"]
pub mod cfg0_mcu_m4fss_systick;
#[doc = "CFG0_MCU_PLL_CLKSEL (rw) register accessor: CFG0_MCU_PLL_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_pll_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_pll_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_pll_clksel`]
module"]
#[doc(alias = "CFG0_MCU_PLL_CLKSEL")]
pub type Cfg0McuPllClksel = crate::Reg<cfg0_mcu_pll_clksel::Cfg0McuPllClkselSpec>;
#[doc = "CFG0_MCU_PLL_CLKSEL"]
pub mod cfg0_mcu_pll_clksel;
#[doc = "CFG0_MCU_TIMER0_CLKSEL (rw) register accessor: CFG0_MCU_TIMER0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer0_clksel`]
module"]
#[doc(alias = "CFG0_MCU_TIMER0_CLKSEL")]
pub type Cfg0McuTimer0Clksel = crate::Reg<cfg0_mcu_timer0_clksel::Cfg0McuTimer0ClkselSpec>;
#[doc = "CFG0_MCU_TIMER0_CLKSEL"]
pub mod cfg0_mcu_timer0_clksel;
#[doc = "CFG0_MCU_TIMER1_CLKSEL (rw) register accessor: CFG0_MCU_TIMER1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer1_clksel`]
module"]
#[doc(alias = "CFG0_MCU_TIMER1_CLKSEL")]
pub type Cfg0McuTimer1Clksel = crate::Reg<cfg0_mcu_timer1_clksel::Cfg0McuTimer1ClkselSpec>;
#[doc = "CFG0_MCU_TIMER1_CLKSEL"]
pub mod cfg0_mcu_timer1_clksel;
#[doc = "CFG0_MCU_TIMER2_CLKSEL (rw) register accessor: CFG0_MCU_TIMER2_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer2_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer2_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer2_clksel`]
module"]
#[doc(alias = "CFG0_MCU_TIMER2_CLKSEL")]
pub type Cfg0McuTimer2Clksel = crate::Reg<cfg0_mcu_timer2_clksel::Cfg0McuTimer2ClkselSpec>;
#[doc = "CFG0_MCU_TIMER2_CLKSEL"]
pub mod cfg0_mcu_timer2_clksel;
#[doc = "CFG0_MCU_TIMER3_CLKSEL (rw) register accessor: CFG0_MCU_TIMER3_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer3_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer3_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer3_clksel`]
module"]
#[doc(alias = "CFG0_MCU_TIMER3_CLKSEL")]
pub type Cfg0McuTimer3Clksel = crate::Reg<cfg0_mcu_timer3_clksel::Cfg0McuTimer3ClkselSpec>;
#[doc = "CFG0_MCU_TIMER3_CLKSEL"]
pub mod cfg0_mcu_timer3_clksel;
#[doc = "CFG0_MCU_SPI0_CLKSEL (rw) register accessor: CFG0_MCU_SPI0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_spi0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_spi0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_spi0_clksel`]
module"]
#[doc(alias = "CFG0_MCU_SPI0_CLKSEL")]
pub type Cfg0McuSpi0Clksel = crate::Reg<cfg0_mcu_spi0_clksel::Cfg0McuSpi0ClkselSpec>;
#[doc = "CFG0_MCU_SPI0_CLKSEL"]
pub mod cfg0_mcu_spi0_clksel;
#[doc = "CFG0_MCU_SPI1_CLKSEL (rw) register accessor: CFG0_MCU_SPI1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_spi1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_spi1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_spi1_clksel`]
module"]
#[doc(alias = "CFG0_MCU_SPI1_CLKSEL")]
pub type Cfg0McuSpi1Clksel = crate::Reg<cfg0_mcu_spi1_clksel::Cfg0McuSpi1ClkselSpec>;
#[doc = "CFG0_MCU_SPI1_CLKSEL"]
pub mod cfg0_mcu_spi1_clksel;
#[doc = "CFG0_MCU_WWD0_CLKSEL (rw) register accessor: CFG0_MCU_WWD0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_wwd0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_wwd0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_wwd0_clksel`]
module"]
#[doc(alias = "CFG0_MCU_WWD0_CLKSEL")]
pub type Cfg0McuWwd0Clksel = crate::Reg<cfg0_mcu_wwd0_clksel::Cfg0McuWwd0ClkselSpec>;
#[doc = "CFG0_MCU_WWD0_CLKSEL"]
pub mod cfg0_mcu_wwd0_clksel;
#[doc = "CFG0_DDR16SS_PMCTRL (rw) register accessor: CFG0_DDR16SS_PMCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr16ss_pmctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr16ss_pmctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ddr16ss_pmctrl`]
module"]
#[doc(alias = "CFG0_DDR16SS_PMCTRL")]
pub type Cfg0Ddr16ssPmctrl = crate::Reg<cfg0_ddr16ss_pmctrl::Cfg0Ddr16ssPmctrlSpec>;
#[doc = "CFG0_DDR16SS_PMCTRL"]
pub mod cfg0_ddr16ss_pmctrl;
#[doc = "CFG0_LOCK2_KICK0 (rw) register accessor: CFG0_LOCK2_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock2_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock2_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock2_kick0`]
module"]
#[doc(alias = "CFG0_LOCK2_KICK0")]
pub type Cfg0Lock2Kick0 = crate::Reg<cfg0_lock2_kick0::Cfg0Lock2Kick0Spec>;
#[doc = "CFG0_LOCK2_KICK0"]
pub mod cfg0_lock2_kick0;
#[doc = "CFG0_LOCK2_KICK1 (rw) register accessor: CFG0_LOCK2_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock2_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock2_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock2_kick1`]
module"]
#[doc(alias = "CFG0_LOCK2_KICK1")]
pub type Cfg0Lock2Kick1 = crate::Reg<cfg0_lock2_kick1::Cfg0Lock2Kick1Spec>;
#[doc = "CFG0_LOCK2_KICK1"]
pub mod cfg0_lock2_kick1;
#[doc = "CFG0_CLAIMREG_P2_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R0_READONLY")]
pub type Cfg0ClaimregP2R0Readonly =
    crate::Reg<cfg0_claimreg_p2_r0_readonly::Cfg0ClaimregP2R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R0_READONLY"]
pub mod cfg0_claimreg_p2_r0_readonly;
#[doc = "CFG0_CLAIMREG_P2_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R1_READONLY")]
pub type Cfg0ClaimregP2R1Readonly =
    crate::Reg<cfg0_claimreg_p2_r1_readonly::Cfg0ClaimregP2R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R1_READONLY"]
pub mod cfg0_claimreg_p2_r1_readonly;
#[doc = "CFG0_MCU_OBSCLK_CTRL_PROXY (rw) register accessor: CFG0_MCU_OBSCLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_obsclk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_obsclk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_obsclk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_OBSCLK_CTRL_PROXY")]
pub type Cfg0McuObsclkCtrlProxy =
    crate::Reg<cfg0_mcu_obsclk_ctrl_proxy::Cfg0McuObsclkCtrlProxySpec>;
#[doc = "CFG0_MCU_OBSCLK_CTRL_PROXY"]
pub mod cfg0_mcu_obsclk_ctrl_proxy;
#[doc = "CFG0_HFOSC0_CTRL_PROXY (rw) register accessor: CFG0_HFOSC0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_hfosc0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_HFOSC0_CTRL_PROXY")]
pub type Cfg0Hfosc0CtrlProxy = crate::Reg<cfg0_hfosc0_ctrl_proxy::Cfg0Hfosc0CtrlProxySpec>;
#[doc = "CFG0_HFOSC0_CTRL_PROXY"]
pub mod cfg0_hfosc0_ctrl_proxy;
#[doc = "CFG0_HFOSC0_TRIM_PROXY (rw) register accessor: CFG0_HFOSC0_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_trim_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_trim_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_hfosc0_trim_proxy`]
module"]
#[doc(alias = "CFG0_HFOSC0_TRIM_PROXY")]
pub type Cfg0Hfosc0TrimProxy = crate::Reg<cfg0_hfosc0_trim_proxy::Cfg0Hfosc0TrimProxySpec>;
#[doc = "CFG0_HFOSC0_TRIM_PROXY"]
pub mod cfg0_hfosc0_trim_proxy;
#[doc = "CFG0_RC12M_OSC_TRIM_PROXY (rw) register accessor: CFG0_RC12M_OSC_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rc12m_osc_trim_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rc12m_osc_trim_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rc12m_osc_trim_proxy`]
module"]
#[doc(alias = "CFG0_RC12M_OSC_TRIM_PROXY")]
pub type Cfg0Rc12mOscTrimProxy = crate::Reg<cfg0_rc12m_osc_trim_proxy::Cfg0Rc12mOscTrimProxySpec>;
#[doc = "CFG0_RC12M_OSC_TRIM_PROXY"]
pub mod cfg0_rc12m_osc_trim_proxy;
#[doc = "CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY (rw) register accessor: CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_hfosc0_clkout_32k_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_hfosc0_clkout_32k_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_hfosc0_clkout_32k_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY")]
pub type Cfg0Hfosc0Clkout32kCtrlProxy =
    crate::Reg<cfg0_hfosc0_clkout_32k_ctrl_proxy::Cfg0Hfosc0Clkout32kCtrlProxySpec>;
#[doc = "CFG0_HFOSC0_CLKOUT_32K_CTRL_PROXY"]
pub mod cfg0_hfosc0_clkout_32k_ctrl_proxy;
#[doc = "CFG0_MCU_M4FSS_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_M4FSS_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS_CLKSEL_PROXY")]
pub type Cfg0McuM4fssClkselProxy =
    crate::Reg<cfg0_mcu_m4fss_clksel_proxy::Cfg0McuM4fssClkselProxySpec>;
#[doc = "CFG0_MCU_M4FSS_CLKSEL_PROXY"]
pub mod cfg0_mcu_m4fss_clksel_proxy;
#[doc = "CFG0_MCU_M4FSS_SYSTICK_PROXY (rw) register accessor: CFG0_MCU_M4FSS_SYSTICK_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_systick_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_systick_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss_systick_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS_SYSTICK_PROXY")]
pub type Cfg0McuM4fssSystickProxy =
    crate::Reg<cfg0_mcu_m4fss_systick_proxy::Cfg0McuM4fssSystickProxySpec>;
#[doc = "CFG0_MCU_M4FSS_SYSTICK_PROXY"]
pub mod cfg0_mcu_m4fss_systick_proxy;
#[doc = "CFG0_MCU_PLL_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_PLL_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_pll_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_pll_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_pll_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_PLL_CLKSEL_PROXY")]
pub type Cfg0McuPllClkselProxy = crate::Reg<cfg0_mcu_pll_clksel_proxy::Cfg0McuPllClkselProxySpec>;
#[doc = "CFG0_MCU_PLL_CLKSEL_PROXY"]
pub mod cfg0_mcu_pll_clksel_proxy;
#[doc = "CFG0_MCU_TIMER0_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_TIMER0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_TIMER0_CLKSEL_PROXY")]
pub type Cfg0McuTimer0ClkselProxy =
    crate::Reg<cfg0_mcu_timer0_clksel_proxy::Cfg0McuTimer0ClkselProxySpec>;
#[doc = "CFG0_MCU_TIMER0_CLKSEL_PROXY"]
pub mod cfg0_mcu_timer0_clksel_proxy;
#[doc = "CFG0_MCU_TIMER1_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_TIMER1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_TIMER1_CLKSEL_PROXY")]
pub type Cfg0McuTimer1ClkselProxy =
    crate::Reg<cfg0_mcu_timer1_clksel_proxy::Cfg0McuTimer1ClkselProxySpec>;
#[doc = "CFG0_MCU_TIMER1_CLKSEL_PROXY"]
pub mod cfg0_mcu_timer1_clksel_proxy;
#[doc = "CFG0_MCU_TIMER2_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_TIMER2_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer2_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer2_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer2_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_TIMER2_CLKSEL_PROXY")]
pub type Cfg0McuTimer2ClkselProxy =
    crate::Reg<cfg0_mcu_timer2_clksel_proxy::Cfg0McuTimer2ClkselProxySpec>;
#[doc = "CFG0_MCU_TIMER2_CLKSEL_PROXY"]
pub mod cfg0_mcu_timer2_clksel_proxy;
#[doc = "CFG0_MCU_TIMER3_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_TIMER3_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer3_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer3_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_timer3_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_TIMER3_CLKSEL_PROXY")]
pub type Cfg0McuTimer3ClkselProxy =
    crate::Reg<cfg0_mcu_timer3_clksel_proxy::Cfg0McuTimer3ClkselProxySpec>;
#[doc = "CFG0_MCU_TIMER3_CLKSEL_PROXY"]
pub mod cfg0_mcu_timer3_clksel_proxy;
#[doc = "CFG0_MCU_SPI0_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_SPI0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_spi0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_spi0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_spi0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_SPI0_CLKSEL_PROXY")]
pub type Cfg0McuSpi0ClkselProxy =
    crate::Reg<cfg0_mcu_spi0_clksel_proxy::Cfg0McuSpi0ClkselProxySpec>;
#[doc = "CFG0_MCU_SPI0_CLKSEL_PROXY"]
pub mod cfg0_mcu_spi0_clksel_proxy;
#[doc = "CFG0_MCU_SPI1_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_SPI1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_spi1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_spi1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_spi1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_SPI1_CLKSEL_PROXY")]
pub type Cfg0McuSpi1ClkselProxy =
    crate::Reg<cfg0_mcu_spi1_clksel_proxy::Cfg0McuSpi1ClkselProxySpec>;
#[doc = "CFG0_MCU_SPI1_CLKSEL_PROXY"]
pub mod cfg0_mcu_spi1_clksel_proxy;
#[doc = "CFG0_MCU_WWD0_CLKSEL_PROXY (rw) register accessor: CFG0_MCU_WWD0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_wwd0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_wwd0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_wwd0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCU_WWD0_CLKSEL_PROXY")]
pub type Cfg0McuWwd0ClkselProxy =
    crate::Reg<cfg0_mcu_wwd0_clksel_proxy::Cfg0McuWwd0ClkselProxySpec>;
#[doc = "CFG0_MCU_WWD0_CLKSEL_PROXY"]
pub mod cfg0_mcu_wwd0_clksel_proxy;
#[doc = "CFG0_DDR16SS_PMCTRL_PROXY (rw) register accessor: CFG0_DDR16SS_PMCTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr16ss_pmctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr16ss_pmctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ddr16ss_pmctrl_proxy`]
module"]
#[doc(alias = "CFG0_DDR16SS_PMCTRL_PROXY")]
pub type Cfg0Ddr16ssPmctrlProxy = crate::Reg<cfg0_ddr16ss_pmctrl_proxy::Cfg0Ddr16ssPmctrlProxySpec>;
#[doc = "CFG0_DDR16SS_PMCTRL_PROXY"]
pub mod cfg0_ddr16ss_pmctrl_proxy;
#[doc = "CFG0_LOCK2_KICK0_PROXY (rw) register accessor: CFG0_LOCK2_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock2_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock2_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock2_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK2_KICK0_PROXY")]
pub type Cfg0Lock2Kick0Proxy = crate::Reg<cfg0_lock2_kick0_proxy::Cfg0Lock2Kick0ProxySpec>;
#[doc = "CFG0_LOCK2_KICK0_PROXY"]
pub mod cfg0_lock2_kick0_proxy;
#[doc = "CFG0_LOCK2_KICK1_PROXY (rw) register accessor: CFG0_LOCK2_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock2_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock2_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock2_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK2_KICK1_PROXY")]
pub type Cfg0Lock2Kick1Proxy = crate::Reg<cfg0_lock2_kick1_proxy::Cfg0Lock2Kick1ProxySpec>;
#[doc = "CFG0_LOCK2_KICK1_PROXY"]
pub mod cfg0_lock2_kick1_proxy;
#[doc = "CFG0_CLAIMREG_P2_R0 (rw) register accessor: CFG0_CLAIMREG_P2_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R0")]
pub type Cfg0ClaimregP2R0 = crate::Reg<cfg0_claimreg_p2_r0::Cfg0ClaimregP2R0Spec>;
#[doc = "CFG0_CLAIMREG_P2_R0"]
pub mod cfg0_claimreg_p2_r0;
#[doc = "CFG0_CLAIMREG_P2_R1 (rw) register accessor: CFG0_CLAIMREG_P2_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R1")]
pub type Cfg0ClaimregP2R1 = crate::Reg<cfg0_claimreg_p2_r1::Cfg0ClaimregP2R1Spec>;
#[doc = "CFG0_CLAIMREG_P2_R1"]
pub mod cfg0_claimreg_p2_r1;
#[doc = "CFG0_MCU_M4FSS0_LBIST_CTRL (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_CTRL")]
pub type Cfg0McuM4fss0LbistCtrl =
    crate::Reg<cfg0_mcu_m4fss0_lbist_ctrl::Cfg0McuM4fss0LbistCtrlSpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_CTRL"]
pub mod cfg0_mcu_m4fss0_lbist_ctrl;
#[doc = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_PATCOUNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_patcount::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_patcount::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_patcount`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT")]
pub type Cfg0McuM4fss0LbistPatcount =
    crate::Reg<cfg0_mcu_m4fss0_lbist_patcount::Cfg0McuM4fss0LbistPatcountSpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT"]
pub mod cfg0_mcu_m4fss0_lbist_patcount;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED0 (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SEED0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_seed0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_seed0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_seed0`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SEED0")]
pub type Cfg0McuM4fss0LbistSeed0 =
    crate::Reg<cfg0_mcu_m4fss0_lbist_seed0::Cfg0McuM4fss0LbistSeed0Spec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED0"]
pub mod cfg0_mcu_m4fss0_lbist_seed0;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED1 (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SEED1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_seed1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_seed1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_seed1`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SEED1")]
pub type Cfg0McuM4fss0LbistSeed1 =
    crate::Reg<cfg0_mcu_m4fss0_lbist_seed1::Cfg0McuM4fss0LbistSeed1Spec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED1"]
pub mod cfg0_mcu_m4fss0_lbist_seed1;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE0 (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SPARE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_spare0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_spare0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_spare0`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SPARE0")]
pub type Cfg0McuM4fss0LbistSpare0 =
    crate::Reg<cfg0_mcu_m4fss0_lbist_spare0::Cfg0McuM4fss0LbistSpare0Spec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE0"]
pub mod cfg0_mcu_m4fss0_lbist_spare0;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE1 (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SPARE1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_spare1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_spare1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_spare1`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SPARE1")]
pub type Cfg0McuM4fss0LbistSpare1 =
    crate::Reg<cfg0_mcu_m4fss0_lbist_spare1::Cfg0McuM4fss0LbistSpare1Spec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE1"]
pub mod cfg0_mcu_m4fss0_lbist_spare1;
#[doc = "CFG0_MCU_M4FSS0_LBIST_STAT (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_stat`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_STAT")]
pub type Cfg0McuM4fss0LbistStat =
    crate::Reg<cfg0_mcu_m4fss0_lbist_stat::Cfg0McuM4fss0LbistStatSpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_STAT"]
pub mod cfg0_mcu_m4fss0_lbist_stat;
#[doc = "CFG0_MCU_M4FSS0_LBIST_MISR (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_MISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_misr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_misr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_misr`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_MISR")]
pub type Cfg0McuM4fss0LbistMisr =
    crate::Reg<cfg0_mcu_m4fss0_lbist_misr::Cfg0McuM4fss0LbistMisrSpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_MISR"]
pub mod cfg0_mcu_m4fss0_lbist_misr;
#[doc = "CFG0_LOCK3_KICK0 (rw) register accessor: CFG0_LOCK3_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock3_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock3_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock3_kick0`]
module"]
#[doc(alias = "CFG0_LOCK3_KICK0")]
pub type Cfg0Lock3Kick0 = crate::Reg<cfg0_lock3_kick0::Cfg0Lock3Kick0Spec>;
#[doc = "CFG0_LOCK3_KICK0"]
pub mod cfg0_lock3_kick0;
#[doc = "CFG0_LOCK3_KICK1 (rw) register accessor: CFG0_LOCK3_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock3_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock3_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock3_kick1`]
module"]
#[doc(alias = "CFG0_LOCK3_KICK1")]
pub type Cfg0Lock3Kick1 = crate::Reg<cfg0_lock3_kick1::Cfg0Lock3Kick1Spec>;
#[doc = "CFG0_LOCK3_KICK1"]
pub mod cfg0_lock3_kick1;
#[doc = "CFG0_CLAIMREG_P3_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R0_READONLY")]
pub type Cfg0ClaimregP3R0Readonly =
    crate::Reg<cfg0_claimreg_p3_r0_readonly::Cfg0ClaimregP3R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R0_READONLY"]
pub mod cfg0_claimreg_p3_r0_readonly;
#[doc = "CFG0_MCU_M4FSS0_LBIST_CTRL_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_CTRL_PROXY")]
pub type Cfg0McuM4fss0LbistCtrlProxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_ctrl_proxy::Cfg0McuM4fss0LbistCtrlProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_CTRL_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_ctrl_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_patcount_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_patcount_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_patcount_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY")]
pub type Cfg0McuM4fss0LbistPatcountProxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_patcount_proxy::Cfg0McuM4fss0LbistPatcountProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_PATCOUNT_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_patcount_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED0_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SEED0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_seed0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_seed0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_seed0_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SEED0_PROXY")]
pub type Cfg0McuM4fss0LbistSeed0Proxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_seed0_proxy::Cfg0McuM4fss0LbistSeed0ProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED0_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_seed0_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_seed1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_seed1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_seed1_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY")]
pub type Cfg0McuM4fss0LbistSeed1Proxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_seed1_proxy::Cfg0McuM4fss0LbistSeed1ProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SEED1_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_seed1_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_spare0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_spare0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_spare0_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY")]
pub type Cfg0McuM4fss0LbistSpare0Proxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_spare0_proxy::Cfg0McuM4fss0LbistSpare0ProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE0_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_spare0_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE1_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_SPARE1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_spare1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_spare1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_spare1_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_SPARE1_PROXY")]
pub type Cfg0McuM4fss0LbistSpare1Proxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_spare1_proxy::Cfg0McuM4fss0LbistSpare1ProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_SPARE1_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_spare1_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_STAT_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_stat_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_STAT_PROXY")]
pub type Cfg0McuM4fss0LbistStatProxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_stat_proxy::Cfg0McuM4fss0LbistStatProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_STAT_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_stat_proxy;
#[doc = "CFG0_MCU_M4FSS0_LBIST_MISR_PROXY (rw) register accessor: CFG0_MCU_M4FSS0_LBIST_MISR_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_misr_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_misr_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_m4fss0_lbist_misr_proxy`]
module"]
#[doc(alias = "CFG0_MCU_M4FSS0_LBIST_MISR_PROXY")]
pub type Cfg0McuM4fss0LbistMisrProxy =
    crate::Reg<cfg0_mcu_m4fss0_lbist_misr_proxy::Cfg0McuM4fss0LbistMisrProxySpec>;
#[doc = "CFG0_MCU_M4FSS0_LBIST_MISR_PROXY"]
pub mod cfg0_mcu_m4fss0_lbist_misr_proxy;
#[doc = "CFG0_LOCK3_KICK0_PROXY (rw) register accessor: CFG0_LOCK3_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock3_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock3_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock3_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK3_KICK0_PROXY")]
pub type Cfg0Lock3Kick0Proxy = crate::Reg<cfg0_lock3_kick0_proxy::Cfg0Lock3Kick0ProxySpec>;
#[doc = "CFG0_LOCK3_KICK0_PROXY"]
pub mod cfg0_lock3_kick0_proxy;
#[doc = "CFG0_LOCK3_KICK1_PROXY (rw) register accessor: CFG0_LOCK3_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock3_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock3_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock3_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK3_KICK1_PROXY")]
pub type Cfg0Lock3Kick1Proxy = crate::Reg<cfg0_lock3_kick1_proxy::Cfg0Lock3Kick1ProxySpec>;
#[doc = "CFG0_LOCK3_KICK1_PROXY"]
pub mod cfg0_lock3_kick1_proxy;
#[doc = "CFG0_CLAIMREG_P3_R0 (rw) register accessor: CFG0_CLAIMREG_P3_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R0")]
pub type Cfg0ClaimregP3R0 = crate::Reg<cfg0_claimreg_p3_r0::Cfg0ClaimregP3R0Spec>;
#[doc = "CFG0_CLAIMREG_P3_R0"]
pub mod cfg0_claimreg_p3_r0;
#[doc = "CFG0_LOCK4_KICK0 (rw) register accessor: CFG0_LOCK4_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock4_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock4_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock4_kick0`]
module"]
#[doc(alias = "CFG0_LOCK4_KICK0")]
pub type Cfg0Lock4Kick0 = crate::Reg<cfg0_lock4_kick0::Cfg0Lock4Kick0Spec>;
#[doc = "CFG0_LOCK4_KICK0"]
pub mod cfg0_lock4_kick0;
#[doc = "CFG0_LOCK4_KICK1 (rw) register accessor: CFG0_LOCK4_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock4_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock4_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock4_kick1`]
module"]
#[doc(alias = "CFG0_LOCK4_KICK1")]
pub type Cfg0Lock4Kick1 = crate::Reg<cfg0_lock4_kick1::Cfg0Lock4Kick1Spec>;
#[doc = "CFG0_LOCK4_KICK1"]
pub mod cfg0_lock4_kick1;
#[doc = "CFG0_CLAIMREG_P4_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R0_READONLY")]
pub type Cfg0ClaimregP4R0Readonly =
    crate::Reg<cfg0_claimreg_p4_r0_readonly::Cfg0ClaimregP4R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R0_READONLY"]
pub mod cfg0_claimreg_p4_r0_readonly;
#[doc = "CFG0_CLAIMREG_P4_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R1_READONLY")]
pub type Cfg0ClaimregP4R1Readonly =
    crate::Reg<cfg0_claimreg_p4_r1_readonly::Cfg0ClaimregP4R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R1_READONLY"]
pub mod cfg0_claimreg_p4_r1_readonly;
#[doc = "CFG0_CLAIMREG_P4_R2_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R2_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r2_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r2_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r2_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R2_READONLY")]
pub type Cfg0ClaimregP4R2Readonly =
    crate::Reg<cfg0_claimreg_p4_r2_readonly::Cfg0ClaimregP4R2ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R2_READONLY"]
pub mod cfg0_claimreg_p4_r2_readonly;
#[doc = "CFG0_CLAIMREG_P4_R3_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R3_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r3_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r3_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r3_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R3_READONLY")]
pub type Cfg0ClaimregP4R3Readonly =
    crate::Reg<cfg0_claimreg_p4_r3_readonly::Cfg0ClaimregP4R3ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R3_READONLY"]
pub mod cfg0_claimreg_p4_r3_readonly;
#[doc = "CFG0_CLAIMREG_P4_R4_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r4_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r4_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r4_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R4_READONLY")]
pub type Cfg0ClaimregP4R4Readonly =
    crate::Reg<cfg0_claimreg_p4_r4_readonly::Cfg0ClaimregP4R4ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R4_READONLY"]
pub mod cfg0_claimreg_p4_r4_readonly;
#[doc = "CFG0_CLAIMREG_P4_R5_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r5_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r5_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r5_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R5_READONLY")]
pub type Cfg0ClaimregP4R5Readonly =
    crate::Reg<cfg0_claimreg_p4_r5_readonly::Cfg0ClaimregP4R5ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R5_READONLY"]
pub mod cfg0_claimreg_p4_r5_readonly;
#[doc = "CFG0_CLAIMREG_P4_R6_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R6_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r6_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r6_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r6_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R6_READONLY")]
pub type Cfg0ClaimregP4R6Readonly =
    crate::Reg<cfg0_claimreg_p4_r6_readonly::Cfg0ClaimregP4R6ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R6_READONLY"]
pub mod cfg0_claimreg_p4_r6_readonly;
#[doc = "CFG0_CLAIMREG_P4_R7_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R7_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r7_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r7_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r7_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R7_READONLY")]
pub type Cfg0ClaimregP4R7Readonly =
    crate::Reg<cfg0_claimreg_p4_r7_readonly::Cfg0ClaimregP4R7ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R7_READONLY"]
pub mod cfg0_claimreg_p4_r7_readonly;
#[doc = "CFG0_CLAIMREG_P4_R8_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R8_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r8_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r8_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r8_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R8_READONLY")]
pub type Cfg0ClaimregP4R8Readonly =
    crate::Reg<cfg0_claimreg_p4_r8_readonly::Cfg0ClaimregP4R8ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R8_READONLY"]
pub mod cfg0_claimreg_p4_r8_readonly;
#[doc = "CFG0_CLAIMREG_P4_R9_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R9_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r9_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r9_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r9_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R9_READONLY")]
pub type Cfg0ClaimregP4R9Readonly =
    crate::Reg<cfg0_claimreg_p4_r9_readonly::Cfg0ClaimregP4R9ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R9_READONLY"]
pub mod cfg0_claimreg_p4_r9_readonly;
#[doc = "CFG0_CLAIMREG_P4_R10_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R10_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r10_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r10_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r10_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R10_READONLY")]
pub type Cfg0ClaimregP4R10Readonly =
    crate::Reg<cfg0_claimreg_p4_r10_readonly::Cfg0ClaimregP4R10ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R10_READONLY"]
pub mod cfg0_claimreg_p4_r10_readonly;
#[doc = "CFG0_CLAIMREG_P4_R11_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R11_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r11_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r11_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r11_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R11_READONLY")]
pub type Cfg0ClaimregP4R11Readonly =
    crate::Reg<cfg0_claimreg_p4_r11_readonly::Cfg0ClaimregP4R11ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R11_READONLY"]
pub mod cfg0_claimreg_p4_r11_readonly;
#[doc = "CFG0_CLAIMREG_P4_R12_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R12_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r12_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r12_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r12_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R12_READONLY")]
pub type Cfg0ClaimregP4R12Readonly =
    crate::Reg<cfg0_claimreg_p4_r12_readonly::Cfg0ClaimregP4R12ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R12_READONLY"]
pub mod cfg0_claimreg_p4_r12_readonly;
#[doc = "CFG0_CLAIMREG_P4_R13_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R13_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r13_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r13_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r13_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R13_READONLY")]
pub type Cfg0ClaimregP4R13Readonly =
    crate::Reg<cfg0_claimreg_p4_r13_readonly::Cfg0ClaimregP4R13ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R13_READONLY"]
pub mod cfg0_claimreg_p4_r13_readonly;
#[doc = "CFG0_CLAIMREG_P4_R14_READONLY (rw) register accessor: CFG0_CLAIMREG_P4_R14_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r14_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r14_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r14_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R14_READONLY")]
pub type Cfg0ClaimregP4R14Readonly =
    crate::Reg<cfg0_claimreg_p4_r14_readonly::Cfg0ClaimregP4R14ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P4_R14_READONLY"]
pub mod cfg0_claimreg_p4_r14_readonly;
#[doc = "CFG0_LOCK4_KICK0_PROXY (rw) register accessor: CFG0_LOCK4_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock4_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock4_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock4_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK4_KICK0_PROXY")]
pub type Cfg0Lock4Kick0Proxy = crate::Reg<cfg0_lock4_kick0_proxy::Cfg0Lock4Kick0ProxySpec>;
#[doc = "CFG0_LOCK4_KICK0_PROXY"]
pub mod cfg0_lock4_kick0_proxy;
#[doc = "CFG0_LOCK4_KICK1_PROXY (rw) register accessor: CFG0_LOCK4_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock4_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock4_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock4_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK4_KICK1_PROXY")]
pub type Cfg0Lock4Kick1Proxy = crate::Reg<cfg0_lock4_kick1_proxy::Cfg0Lock4Kick1ProxySpec>;
#[doc = "CFG0_LOCK4_KICK1_PROXY"]
pub mod cfg0_lock4_kick1_proxy;
#[doc = "CFG0_CLAIMREG_P4_R0 (rw) register accessor: CFG0_CLAIMREG_P4_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R0")]
pub type Cfg0ClaimregP4R0 = crate::Reg<cfg0_claimreg_p4_r0::Cfg0ClaimregP4R0Spec>;
#[doc = "CFG0_CLAIMREG_P4_R0"]
pub mod cfg0_claimreg_p4_r0;
#[doc = "CFG0_CLAIMREG_P4_R1 (rw) register accessor: CFG0_CLAIMREG_P4_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R1")]
pub type Cfg0ClaimregP4R1 = crate::Reg<cfg0_claimreg_p4_r1::Cfg0ClaimregP4R1Spec>;
#[doc = "CFG0_CLAIMREG_P4_R1"]
pub mod cfg0_claimreg_p4_r1;
#[doc = "CFG0_CLAIMREG_P4_R2 (rw) register accessor: CFG0_CLAIMREG_P4_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r2`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R2")]
pub type Cfg0ClaimregP4R2 = crate::Reg<cfg0_claimreg_p4_r2::Cfg0ClaimregP4R2Spec>;
#[doc = "CFG0_CLAIMREG_P4_R2"]
pub mod cfg0_claimreg_p4_r2;
#[doc = "CFG0_CLAIMREG_P4_R3 (rw) register accessor: CFG0_CLAIMREG_P4_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r3`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R3")]
pub type Cfg0ClaimregP4R3 = crate::Reg<cfg0_claimreg_p4_r3::Cfg0ClaimregP4R3Spec>;
#[doc = "CFG0_CLAIMREG_P4_R3"]
pub mod cfg0_claimreg_p4_r3;
#[doc = "CFG0_CLAIMREG_P4_R4 (rw) register accessor: CFG0_CLAIMREG_P4_R4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r4`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R4")]
pub type Cfg0ClaimregP4R4 = crate::Reg<cfg0_claimreg_p4_r4::Cfg0ClaimregP4R4Spec>;
#[doc = "CFG0_CLAIMREG_P4_R4"]
pub mod cfg0_claimreg_p4_r4;
#[doc = "CFG0_CLAIMREG_P4_R5 (rw) register accessor: CFG0_CLAIMREG_P4_R5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r5`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R5")]
pub type Cfg0ClaimregP4R5 = crate::Reg<cfg0_claimreg_p4_r5::Cfg0ClaimregP4R5Spec>;
#[doc = "CFG0_CLAIMREG_P4_R5"]
pub mod cfg0_claimreg_p4_r5;
#[doc = "CFG0_CLAIMREG_P4_R6 (rw) register accessor: CFG0_CLAIMREG_P4_R6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r6`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R6")]
pub type Cfg0ClaimregP4R6 = crate::Reg<cfg0_claimreg_p4_r6::Cfg0ClaimregP4R6Spec>;
#[doc = "CFG0_CLAIMREG_P4_R6"]
pub mod cfg0_claimreg_p4_r6;
#[doc = "CFG0_CLAIMREG_P4_R7 (rw) register accessor: CFG0_CLAIMREG_P4_R7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r7`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R7")]
pub type Cfg0ClaimregP4R7 = crate::Reg<cfg0_claimreg_p4_r7::Cfg0ClaimregP4R7Spec>;
#[doc = "CFG0_CLAIMREG_P4_R7"]
pub mod cfg0_claimreg_p4_r7;
#[doc = "CFG0_CLAIMREG_P4_R8 (rw) register accessor: CFG0_CLAIMREG_P4_R8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r8`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R8")]
pub type Cfg0ClaimregP4R8 = crate::Reg<cfg0_claimreg_p4_r8::Cfg0ClaimregP4R8Spec>;
#[doc = "CFG0_CLAIMREG_P4_R8"]
pub mod cfg0_claimreg_p4_r8;
#[doc = "CFG0_CLAIMREG_P4_R9 (rw) register accessor: CFG0_CLAIMREG_P4_R9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r9`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R9")]
pub type Cfg0ClaimregP4R9 = crate::Reg<cfg0_claimreg_p4_r9::Cfg0ClaimregP4R9Spec>;
#[doc = "CFG0_CLAIMREG_P4_R9"]
pub mod cfg0_claimreg_p4_r9;
#[doc = "CFG0_CLAIMREG_P4_R10 (rw) register accessor: CFG0_CLAIMREG_P4_R10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r10`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R10")]
pub type Cfg0ClaimregP4R10 = crate::Reg<cfg0_claimreg_p4_r10::Cfg0ClaimregP4R10Spec>;
#[doc = "CFG0_CLAIMREG_P4_R10"]
pub mod cfg0_claimreg_p4_r10;
#[doc = "CFG0_CLAIMREG_P4_R11 (rw) register accessor: CFG0_CLAIMREG_P4_R11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r11`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R11")]
pub type Cfg0ClaimregP4R11 = crate::Reg<cfg0_claimreg_p4_r11::Cfg0ClaimregP4R11Spec>;
#[doc = "CFG0_CLAIMREG_P4_R11"]
pub mod cfg0_claimreg_p4_r11;
#[doc = "CFG0_CLAIMREG_P4_R12 (rw) register accessor: CFG0_CLAIMREG_P4_R12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r12`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R12")]
pub type Cfg0ClaimregP4R12 = crate::Reg<cfg0_claimreg_p4_r12::Cfg0ClaimregP4R12Spec>;
#[doc = "CFG0_CLAIMREG_P4_R12"]
pub mod cfg0_claimreg_p4_r12;
#[doc = "CFG0_CLAIMREG_P4_R13 (rw) register accessor: CFG0_CLAIMREG_P4_R13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r13`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R13")]
pub type Cfg0ClaimregP4R13 = crate::Reg<cfg0_claimreg_p4_r13::Cfg0ClaimregP4R13Spec>;
#[doc = "CFG0_CLAIMREG_P4_R13"]
pub mod cfg0_claimreg_p4_r13;
#[doc = "CFG0_CLAIMREG_P4_R14 (rw) register accessor: CFG0_CLAIMREG_P4_R14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p4_r14`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P4_R14")]
pub type Cfg0ClaimregP4R14 = crate::Reg<cfg0_claimreg_p4_r14::Cfg0ClaimregP4R14Spec>;
#[doc = "CFG0_CLAIMREG_P4_R14"]
pub mod cfg0_claimreg_p4_r14;
#[doc = "CFG0_POR_CTRL (rw) register accessor: CFG0_POR_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_por_ctrl`]
module"]
#[doc(alias = "CFG0_POR_CTRL")]
pub type Cfg0PorCtrl = crate::Reg<cfg0_por_ctrl::Cfg0PorCtrlSpec>;
#[doc = "CFG0_POR_CTRL"]
pub mod cfg0_por_ctrl;
#[doc = "CFG0_POR_STAT (rw) register accessor: CFG0_POR_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_por_stat`]
module"]
#[doc(alias = "CFG0_POR_STAT")]
pub type Cfg0PorStat = crate::Reg<cfg0_por_stat::Cfg0PorStatSpec>;
#[doc = "CFG0_POR_STAT"]
pub mod cfg0_por_stat;
#[doc = "CFG0_POR_BANDGAP_CTRL (rw) register accessor: CFG0_POR_BANDGAP_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_bandgap_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_bandgap_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_por_bandgap_ctrl`]
module"]
#[doc(alias = "CFG0_POR_BANDGAP_CTRL")]
pub type Cfg0PorBandgapCtrl = crate::Reg<cfg0_por_bandgap_ctrl::Cfg0PorBandgapCtrlSpec>;
#[doc = "CFG0_POR_BANDGAP_CTRL"]
pub mod cfg0_por_bandgap_ctrl;
#[doc = "CFG0_POK_VDDA_MCU_UV_CTRL (rw) register accessor: CFG0_POK_VDDA_MCU_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_mcu_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_mcu_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdda_mcu_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDA_MCU_UV_CTRL")]
pub type Cfg0PokVddaMcuUvCtrl = crate::Reg<cfg0_pok_vdda_mcu_uv_ctrl::Cfg0PokVddaMcuUvCtrlSpec>;
#[doc = "CFG0_POK_VDDA_MCU_UV_CTRL"]
pub mod cfg0_pok_vdda_mcu_uv_ctrl;
#[doc = "CFG0_POK_VDDA_MCU_OV_CTRL (rw) register accessor: CFG0_POK_VDDA_MCU_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_mcu_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_mcu_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdda_mcu_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDA_MCU_OV_CTRL")]
pub type Cfg0PokVddaMcuOvCtrl = crate::Reg<cfg0_pok_vdda_mcu_ov_ctrl::Cfg0PokVddaMcuOvCtrlSpec>;
#[doc = "CFG0_POK_VDDA_MCU_OV_CTRL"]
pub mod cfg0_pok_vdda_mcu_ov_ctrl;
#[doc = "CFG0_POK_VDD_CORE_UV_CTRL (rw) register accessor: CFG0_POK_VDD_CORE_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdd_core_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdd_core_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdd_core_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDD_CORE_UV_CTRL")]
pub type Cfg0PokVddCoreUvCtrl = crate::Reg<cfg0_pok_vdd_core_uv_ctrl::Cfg0PokVddCoreUvCtrlSpec>;
#[doc = "CFG0_POK_VDD_CORE_UV_CTRL"]
pub mod cfg0_pok_vdd_core_uv_ctrl;
#[doc = "CFG0_POK_VDD_CORE_OV_CTRL (rw) register accessor: CFG0_POK_VDD_CORE_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdd_core_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdd_core_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdd_core_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDD_CORE_OV_CTRL")]
pub type Cfg0PokVddCoreOvCtrl = crate::Reg<cfg0_pok_vdd_core_ov_ctrl::Cfg0PokVddCoreOvCtrlSpec>;
#[doc = "CFG0_POK_VDD_CORE_OV_CTRL"]
pub mod cfg0_pok_vdd_core_ov_ctrl;
#[doc = "CFG0_POK_VDDR_CORE_UV_CTRL (rw) register accessor: CFG0_POK_VDDR_CORE_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddr_core_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddr_core_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddr_core_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDR_CORE_UV_CTRL")]
pub type Cfg0PokVddrCoreUvCtrl = crate::Reg<cfg0_pok_vddr_core_uv_ctrl::Cfg0PokVddrCoreUvCtrlSpec>;
#[doc = "CFG0_POK_VDDR_CORE_UV_CTRL"]
pub mod cfg0_pok_vddr_core_uv_ctrl;
#[doc = "CFG0_POK_VDDR_CORE_OV_CTRL (rw) register accessor: CFG0_POK_VDDR_CORE_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddr_core_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddr_core_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddr_core_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDR_CORE_OV_CTRL")]
pub type Cfg0PokVddrCoreOvCtrl = crate::Reg<cfg0_pok_vddr_core_ov_ctrl::Cfg0PokVddrCoreOvCtrlSpec>;
#[doc = "CFG0_POK_VDDR_CORE_OV_CTRL"]
pub mod cfg0_pok_vddr_core_ov_ctrl;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_1p8_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL")]
pub type Cfg0PokVddshvMcu1p8UvCtrl =
    crate::Reg<cfg0_pok_vddshv_mcu_1p8_uv_ctrl::Cfg0PokVddshvMcu1p8UvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL"]
pub mod cfg0_pok_vddshv_mcu_1p8_uv_ctrl;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_1p8_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_1p8_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_1p8_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL")]
pub type Cfg0PokVddshvMcu1p8OvCtrl =
    crate::Reg<cfg0_pok_vddshv_mcu_1p8_ov_ctrl::Cfg0PokVddshvMcu1p8OvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL"]
pub mod cfg0_pok_vddshv_mcu_1p8_ov_ctrl;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_3p3_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_3p3_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_3p3_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL")]
pub type Cfg0PokVddshvMcu3p3UvCtrl =
    crate::Reg<cfg0_pok_vddshv_mcu_3p3_uv_ctrl::Cfg0PokVddshvMcu3p3UvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL"]
pub mod cfg0_pok_vddshv_mcu_3p3_uv_ctrl;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_3p3_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_3p3_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_3p3_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL")]
pub type Cfg0PokVddshvMcu3p3OvCtrl =
    crate::Reg<cfg0_pok_vddshv_mcu_3p3_ov_ctrl::Cfg0PokVddshvMcu3p3OvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL"]
pub mod cfg0_pok_vddshv_mcu_3p3_ov_ctrl;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL (rw) register accessor: CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vmon_cap_mcu_general_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vmon_cap_mcu_general_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vmon_cap_mcu_general_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL")]
pub type Cfg0PokVmonCapMcuGeneralUvCtrl =
    crate::Reg<cfg0_pok_vmon_cap_mcu_general_uv_ctrl::Cfg0PokVmonCapMcuGeneralUvCtrlSpec>;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL"]
pub mod cfg0_pok_vmon_cap_mcu_general_uv_ctrl;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL (rw) register accessor: CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vmon_cap_mcu_general_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vmon_cap_mcu_general_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vmon_cap_mcu_general_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL")]
pub type Cfg0PokVmonCapMcuGeneralOvCtrl =
    crate::Reg<cfg0_pok_vmon_cap_mcu_general_ov_ctrl::Cfg0PokVmonCapMcuGeneralOvCtrlSpec>;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL"]
pub mod cfg0_pok_vmon_cap_mcu_general_ov_ctrl;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_1p8_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_1p8_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_1p8_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL")]
pub type Cfg0PokVddshvMain1p8UvCtrl =
    crate::Reg<cfg0_pok_vddshv_main_1p8_uv_ctrl::Cfg0PokVddshvMain1p8UvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL"]
pub mod cfg0_pok_vddshv_main_1p8_uv_ctrl;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_1p8_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_1p8_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_1p8_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL")]
pub type Cfg0PokVddshvMain1p8OvCtrl =
    crate::Reg<cfg0_pok_vddshv_main_1p8_ov_ctrl::Cfg0PokVddshvMain1p8OvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL"]
pub mod cfg0_pok_vddshv_main_1p8_ov_ctrl;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_3p3_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_3p3_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_3p3_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL")]
pub type Cfg0PokVddshvMain3p3UvCtrl =
    crate::Reg<cfg0_pok_vddshv_main_3p3_uv_ctrl::Cfg0PokVddshvMain3p3UvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL"]
pub mod cfg0_pok_vddshv_main_3p3_uv_ctrl;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL (rw) register accessor: CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_3p3_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_3p3_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_3p3_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL")]
pub type Cfg0PokVddshvMain3p3OvCtrl =
    crate::Reg<cfg0_pok_vddshv_main_3p3_ov_ctrl::Cfg0PokVddshvMain3p3OvCtrlSpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL"]
pub mod cfg0_pok_vddshv_main_3p3_ov_ctrl;
#[doc = "CFG0_POK_VDDS_DDRIO_UV_CTRL (rw) register accessor: CFG0_POK_VDDS_DDRIO_UV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdds_ddrio_uv_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdds_ddrio_uv_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdds_ddrio_uv_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDS_DDRIO_UV_CTRL")]
pub type Cfg0PokVddsDdrioUvCtrl =
    crate::Reg<cfg0_pok_vdds_ddrio_uv_ctrl::Cfg0PokVddsDdrioUvCtrlSpec>;
#[doc = "CFG0_POK_VDDS_DDRIO_UV_CTRL"]
pub mod cfg0_pok_vdds_ddrio_uv_ctrl;
#[doc = "CFG0_POK_VDDS_DDRIO_OV_CTRL (rw) register accessor: CFG0_POK_VDDS_DDRIO_OV_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdds_ddrio_ov_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdds_ddrio_ov_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdds_ddrio_ov_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDS_DDRIO_OV_CTRL")]
pub type Cfg0PokVddsDdrioOvCtrl =
    crate::Reg<cfg0_pok_vdds_ddrio_ov_ctrl::Cfg0PokVddsDdrioOvCtrlSpec>;
#[doc = "CFG0_POK_VDDS_DDRIO_OV_CTRL"]
pub mod cfg0_pok_vdds_ddrio_ov_ctrl;
#[doc = "CFG0_POK_VDDA_PMIC_IN_CTRL (rw) register accessor: CFG0_POK_VDDA_PMIC_IN_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_pmic_in_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_pmic_in_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdda_pmic_in_ctrl`]
module"]
#[doc(alias = "CFG0_POK_VDDA_PMIC_IN_CTRL")]
pub type Cfg0PokVddaPmicInCtrl = crate::Reg<cfg0_pok_vdda_pmic_in_ctrl::Cfg0PokVddaPmicInCtrlSpec>;
#[doc = "CFG0_POK_VDDA_PMIC_IN_CTRL"]
pub mod cfg0_pok_vdda_pmic_in_ctrl;
#[doc = "CFG0_RST_CTRL (rw) register accessor: CFG0_RST_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_ctrl`]
module"]
#[doc(alias = "CFG0_RST_CTRL")]
pub type Cfg0RstCtrl = crate::Reg<cfg0_rst_ctrl::Cfg0RstCtrlSpec>;
#[doc = "CFG0_RST_CTRL"]
pub mod cfg0_rst_ctrl;
#[doc = "CFG0_RST_STAT (rw) register accessor: CFG0_RST_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_stat`]
module"]
#[doc(alias = "CFG0_RST_STAT")]
pub type Cfg0RstStat = crate::Reg<cfg0_rst_stat::Cfg0RstStatSpec>;
#[doc = "CFG0_RST_STAT"]
pub mod cfg0_rst_stat;
#[doc = "CFG0_RST_SRC (rw) register accessor: CFG0_RST_SRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_src`]
module"]
#[doc(alias = "CFG0_RST_SRC")]
pub type Cfg0RstSrc = crate::Reg<cfg0_rst_src::Cfg0RstSrcSpec>;
#[doc = "CFG0_RST_SRC"]
pub mod cfg0_rst_src;
#[doc = "CFG0_RST_MAGIC_WORD (rw) register accessor: CFG0_RST_MAGIC_WORD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_magic_word::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_magic_word::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_magic_word`]
module"]
#[doc(alias = "CFG0_RST_MAGIC_WORD")]
pub type Cfg0RstMagicWord = crate::Reg<cfg0_rst_magic_word::Cfg0RstMagicWordSpec>;
#[doc = "CFG0_RST_MAGIC_WORD"]
pub mod cfg0_rst_magic_word;
#[doc = "CFG0_ISO_CTRL (rw) register accessor: CFG0_ISO_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_iso_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_iso_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_iso_ctrl`]
module"]
#[doc(alias = "CFG0_ISO_CTRL")]
pub type Cfg0IsoCtrl = crate::Reg<cfg0_iso_ctrl::Cfg0IsoCtrlSpec>;
#[doc = "CFG0_ISO_CTRL"]
pub mod cfg0_iso_ctrl;
#[doc = "CFG0_VDD_CORE_GLDTC_CTRL (rw) register accessor: CFG0_VDD_CORE_GLDTC_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_vdd_core_gldtc_ctrl`]
module"]
#[doc(alias = "CFG0_VDD_CORE_GLDTC_CTRL")]
pub type Cfg0VddCoreGldtcCtrl = crate::Reg<cfg0_vdd_core_gldtc_ctrl::Cfg0VddCoreGldtcCtrlSpec>;
#[doc = "CFG0_VDD_CORE_GLDTC_CTRL"]
pub mod cfg0_vdd_core_gldtc_ctrl;
#[doc = "CFG0_VDD_CORE_GLDTC_STAT (rw) register accessor: CFG0_VDD_CORE_GLDTC_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_vdd_core_gldtc_stat`]
module"]
#[doc(alias = "CFG0_VDD_CORE_GLDTC_STAT")]
pub type Cfg0VddCoreGldtcStat = crate::Reg<cfg0_vdd_core_gldtc_stat::Cfg0VddCoreGldtcStatSpec>;
#[doc = "CFG0_VDD_CORE_GLDTC_STAT"]
pub mod cfg0_vdd_core_gldtc_stat;
#[doc = "CFG0_PRG_PP_0_CTRL (rw) register accessor: CFG0_PRG_PP_0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_prg_pp_0_ctrl`]
module"]
#[doc(alias = "CFG0_PRG_PP_0_CTRL")]
pub type Cfg0PrgPp0Ctrl = crate::Reg<cfg0_prg_pp_0_ctrl::Cfg0PrgPp0CtrlSpec>;
#[doc = "CFG0_PRG_PP_0_CTRL"]
pub mod cfg0_prg_pp_0_ctrl;
#[doc = "CFG0_PRG_PP_1_CTRL (rw) register accessor: CFG0_PRG_PP_1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_prg_pp_1_ctrl`]
module"]
#[doc(alias = "CFG0_PRG_PP_1_CTRL")]
pub type Cfg0PrgPp1Ctrl = crate::Reg<cfg0_prg_pp_1_ctrl::Cfg0PrgPp1CtrlSpec>;
#[doc = "CFG0_PRG_PP_1_CTRL"]
pub mod cfg0_prg_pp_1_ctrl;
#[doc = "CFG0_MCU_CLKGATE_CTRL (rw) register accessor: CFG0_MCU_CLKGATE_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_clkgate_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_clkgate_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_clkgate_ctrl`]
module"]
#[doc(alias = "CFG0_MCU_CLKGATE_CTRL")]
pub type Cfg0McuClkgateCtrl = crate::Reg<cfg0_mcu_clkgate_ctrl::Cfg0McuClkgateCtrlSpec>;
#[doc = "CFG0_MCU_CLKGATE_CTRL"]
pub mod cfg0_mcu_clkgate_ctrl;
#[doc = "CFG0_MAIN_CLKGATE_CTRL0 (rw) register accessor: CFG0_MAIN_CLKGATE_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_clkgate_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_clkgate_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_clkgate_ctrl0`]
module"]
#[doc(alias = "CFG0_MAIN_CLKGATE_CTRL0")]
pub type Cfg0MainClkgateCtrl0 = crate::Reg<cfg0_main_clkgate_ctrl0::Cfg0MainClkgateCtrl0Spec>;
#[doc = "CFG0_MAIN_CLKGATE_CTRL0"]
pub mod cfg0_main_clkgate_ctrl0;
#[doc = "CFG0_LOCK6_KICK0 (rw) register accessor: CFG0_LOCK6_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock6_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock6_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock6_kick0`]
module"]
#[doc(alias = "CFG0_LOCK6_KICK0")]
pub type Cfg0Lock6Kick0 = crate::Reg<cfg0_lock6_kick0::Cfg0Lock6Kick0Spec>;
#[doc = "CFG0_LOCK6_KICK0"]
pub mod cfg0_lock6_kick0;
#[doc = "CFG0_LOCK6_KICK1 (rw) register accessor: CFG0_LOCK6_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock6_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock6_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock6_kick1`]
module"]
#[doc(alias = "CFG0_LOCK6_KICK1")]
pub type Cfg0Lock6Kick1 = crate::Reg<cfg0_lock6_kick1::Cfg0Lock6Kick1Spec>;
#[doc = "CFG0_LOCK6_KICK1"]
pub mod cfg0_lock6_kick1;
#[doc = "CFG0_CLAIMREG_P6_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P6_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R0_READONLY")]
pub type Cfg0ClaimregP6R0Readonly =
    crate::Reg<cfg0_claimreg_p6_r0_readonly::Cfg0ClaimregP6R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P6_R0_READONLY"]
pub mod cfg0_claimreg_p6_r0_readonly;
#[doc = "CFG0_CLAIMREG_P6_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P6_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R1_READONLY")]
pub type Cfg0ClaimregP6R1Readonly =
    crate::Reg<cfg0_claimreg_p6_r1_readonly::Cfg0ClaimregP6R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P6_R1_READONLY"]
pub mod cfg0_claimreg_p6_r1_readonly;
#[doc = "CFG0_CLAIMREG_P6_R2_READONLY (rw) register accessor: CFG0_CLAIMREG_P6_R2_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r2_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r2_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r2_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R2_READONLY")]
pub type Cfg0ClaimregP6R2Readonly =
    crate::Reg<cfg0_claimreg_p6_r2_readonly::Cfg0ClaimregP6R2ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P6_R2_READONLY"]
pub mod cfg0_claimreg_p6_r2_readonly;
#[doc = "CFG0_CLAIMREG_P6_R3_READONLY (rw) register accessor: CFG0_CLAIMREG_P6_R3_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r3_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r3_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r3_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R3_READONLY")]
pub type Cfg0ClaimregP6R3Readonly =
    crate::Reg<cfg0_claimreg_p6_r3_readonly::Cfg0ClaimregP6R3ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P6_R3_READONLY"]
pub mod cfg0_claimreg_p6_r3_readonly;
#[doc = "CFG0_CLAIMREG_P6_R4_READONLY (rw) register accessor: CFG0_CLAIMREG_P6_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r4_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r4_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r4_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R4_READONLY")]
pub type Cfg0ClaimregP6R4Readonly =
    crate::Reg<cfg0_claimreg_p6_r4_readonly::Cfg0ClaimregP6R4ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P6_R4_READONLY"]
pub mod cfg0_claimreg_p6_r4_readonly;
#[doc = "CFG0_CLAIMREG_P6_R5_READONLY (rw) register accessor: CFG0_CLAIMREG_P6_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r5_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r5_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r5_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R5_READONLY")]
pub type Cfg0ClaimregP6R5Readonly =
    crate::Reg<cfg0_claimreg_p6_r5_readonly::Cfg0ClaimregP6R5ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P6_R5_READONLY"]
pub mod cfg0_claimreg_p6_r5_readonly;
#[doc = "CFG0_POR_CTRL_PROXY (rw) register accessor: CFG0_POR_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_por_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POR_CTRL_PROXY")]
pub type Cfg0PorCtrlProxy = crate::Reg<cfg0_por_ctrl_proxy::Cfg0PorCtrlProxySpec>;
#[doc = "CFG0_POR_CTRL_PROXY"]
pub mod cfg0_por_ctrl_proxy;
#[doc = "CFG0_POR_STAT_PROXY (rw) register accessor: CFG0_POR_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_por_stat_proxy`]
module"]
#[doc(alias = "CFG0_POR_STAT_PROXY")]
pub type Cfg0PorStatProxy = crate::Reg<cfg0_por_stat_proxy::Cfg0PorStatProxySpec>;
#[doc = "CFG0_POR_STAT_PROXY"]
pub mod cfg0_por_stat_proxy;
#[doc = "CFG0_POR_BANDGAP_CTRL_PROXY (rw) register accessor: CFG0_POR_BANDGAP_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_bandgap_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_bandgap_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_por_bandgap_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POR_BANDGAP_CTRL_PROXY")]
pub type Cfg0PorBandgapCtrlProxy =
    crate::Reg<cfg0_por_bandgap_ctrl_proxy::Cfg0PorBandgapCtrlProxySpec>;
#[doc = "CFG0_POR_BANDGAP_CTRL_PROXY"]
pub mod cfg0_por_bandgap_ctrl_proxy;
#[doc = "CFG0_POK_VDDA_MCU_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDA_MCU_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_mcu_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_mcu_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdda_mcu_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDA_MCU_UV_CTRL_PROXY")]
pub type Cfg0PokVddaMcuUvCtrlProxy =
    crate::Reg<cfg0_pok_vdda_mcu_uv_ctrl_proxy::Cfg0PokVddaMcuUvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDA_MCU_UV_CTRL_PROXY"]
pub mod cfg0_pok_vdda_mcu_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDA_MCU_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDA_MCU_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_mcu_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_mcu_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdda_mcu_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDA_MCU_OV_CTRL_PROXY")]
pub type Cfg0PokVddaMcuOvCtrlProxy =
    crate::Reg<cfg0_pok_vdda_mcu_ov_ctrl_proxy::Cfg0PokVddaMcuOvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDA_MCU_OV_CTRL_PROXY"]
pub mod cfg0_pok_vdda_mcu_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDD_CORE_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDD_CORE_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdd_core_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdd_core_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdd_core_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDD_CORE_UV_CTRL_PROXY")]
pub type Cfg0PokVddCoreUvCtrlProxy =
    crate::Reg<cfg0_pok_vdd_core_uv_ctrl_proxy::Cfg0PokVddCoreUvCtrlProxySpec>;
#[doc = "CFG0_POK_VDD_CORE_UV_CTRL_PROXY"]
pub mod cfg0_pok_vdd_core_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDD_CORE_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDD_CORE_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdd_core_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdd_core_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdd_core_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDD_CORE_OV_CTRL_PROXY")]
pub type Cfg0PokVddCoreOvCtrlProxy =
    crate::Reg<cfg0_pok_vdd_core_ov_ctrl_proxy::Cfg0PokVddCoreOvCtrlProxySpec>;
#[doc = "CFG0_POK_VDD_CORE_OV_CTRL_PROXY"]
pub mod cfg0_pok_vdd_core_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDR_CORE_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDR_CORE_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddr_core_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddr_core_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddr_core_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDR_CORE_UV_CTRL_PROXY")]
pub type Cfg0PokVddrCoreUvCtrlProxy =
    crate::Reg<cfg0_pok_vddr_core_uv_ctrl_proxy::Cfg0PokVddrCoreUvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDR_CORE_UV_CTRL_PROXY"]
pub mod cfg0_pok_vddr_core_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDR_CORE_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDR_CORE_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddr_core_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddr_core_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddr_core_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDR_CORE_OV_CTRL_PROXY")]
pub type Cfg0PokVddrCoreOvCtrlProxy =
    crate::Reg<cfg0_pok_vddr_core_ov_ctrl_proxy::Cfg0PokVddrCoreOvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDR_CORE_OV_CTRL_PROXY"]
pub mod cfg0_pok_vddr_core_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY")]
pub type Cfg0PokVddshvMcu1p8UvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy::Cfg0PokVddshvMcu1p8UvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_UV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_mcu_1p8_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL_PROXY")]
pub type Cfg0PokVddshvMcu1p8OvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy::Cfg0PokVddshvMcu1p8OvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_1P8_OV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_mcu_1p8_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL_PROXY")]
pub type Cfg0PokVddshvMcu3p3UvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy::Cfg0PokVddshvMcu3p3UvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_UV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_mcu_3p3_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL_PROXY")]
pub type Cfg0PokVddshvMcu3p3OvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy::Cfg0PokVddshvMcu3p3OvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MCU_3P3_OV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_mcu_3p3_ov_ctrl_proxy;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL_PROXY")]
pub type Cfg0PokVmonCapMcuGeneralUvCtrlProxy = crate::Reg<
    cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy::Cfg0PokVmonCapMcuGeneralUvCtrlProxySpec,
>;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_UV_CTRL_PROXY"]
pub mod cfg0_pok_vmon_cap_mcu_general_uv_ctrl_proxy;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL_PROXY")]
pub type Cfg0PokVmonCapMcuGeneralOvCtrlProxy = crate::Reg<
    cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy::Cfg0PokVmonCapMcuGeneralOvCtrlProxySpec,
>;
#[doc = "CFG0_POK_VMON_CAP_MCU_GENERAL_OV_CTRL_PROXY"]
pub mod cfg0_pok_vmon_cap_mcu_general_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL_PROXY")]
pub type Cfg0PokVddshvMain1p8UvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy::Cfg0PokVddshvMain1p8UvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_UV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_main_1p8_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL_PROXY")]
pub type Cfg0PokVddshvMain1p8OvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy::Cfg0PokVddshvMain1p8OvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_1P8_OV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_main_1p8_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL_PROXY")]
pub type Cfg0PokVddshvMain3p3UvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy::Cfg0PokVddshvMain3p3UvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_UV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_main_3p3_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY")]
pub type Cfg0PokVddshvMain3p3OvCtrlProxy =
    crate::Reg<cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy::Cfg0PokVddshvMain3p3OvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDSHV_MAIN_3P3_OV_CTRL_PROXY"]
pub mod cfg0_pok_vddshv_main_3p3_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDS_DDRIO_UV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDS_DDRIO_UV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdds_ddrio_uv_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdds_ddrio_uv_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdds_ddrio_uv_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDS_DDRIO_UV_CTRL_PROXY")]
pub type Cfg0PokVddsDdrioUvCtrlProxy =
    crate::Reg<cfg0_pok_vdds_ddrio_uv_ctrl_proxy::Cfg0PokVddsDdrioUvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDS_DDRIO_UV_CTRL_PROXY"]
pub mod cfg0_pok_vdds_ddrio_uv_ctrl_proxy;
#[doc = "CFG0_POK_VDDS_DDRIO_OV_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDS_DDRIO_OV_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdds_ddrio_ov_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdds_ddrio_ov_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdds_ddrio_ov_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDS_DDRIO_OV_CTRL_PROXY")]
pub type Cfg0PokVddsDdrioOvCtrlProxy =
    crate::Reg<cfg0_pok_vdds_ddrio_ov_ctrl_proxy::Cfg0PokVddsDdrioOvCtrlProxySpec>;
#[doc = "CFG0_POK_VDDS_DDRIO_OV_CTRL_PROXY"]
pub mod cfg0_pok_vdds_ddrio_ov_ctrl_proxy;
#[doc = "CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY (rw) register accessor: CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pok_vdda_pmic_in_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pok_vdda_pmic_in_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pok_vdda_pmic_in_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY")]
pub type Cfg0PokVddaPmicInCtrlProxy =
    crate::Reg<cfg0_pok_vdda_pmic_in_ctrl_proxy::Cfg0PokVddaPmicInCtrlProxySpec>;
#[doc = "CFG0_POK_VDDA_PMIC_IN_CTRL_PROXY"]
pub mod cfg0_pok_vdda_pmic_in_ctrl_proxy;
#[doc = "CFG0_RST_CTRL_PROXY (rw) register accessor: CFG0_RST_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_RST_CTRL_PROXY")]
pub type Cfg0RstCtrlProxy = crate::Reg<cfg0_rst_ctrl_proxy::Cfg0RstCtrlProxySpec>;
#[doc = "CFG0_RST_CTRL_PROXY"]
pub mod cfg0_rst_ctrl_proxy;
#[doc = "CFG0_RST_STAT_PROXY (rw) register accessor: CFG0_RST_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_stat_proxy`]
module"]
#[doc(alias = "CFG0_RST_STAT_PROXY")]
pub type Cfg0RstStatProxy = crate::Reg<cfg0_rst_stat_proxy::Cfg0RstStatProxySpec>;
#[doc = "CFG0_RST_STAT_PROXY"]
pub mod cfg0_rst_stat_proxy;
#[doc = "CFG0_RST_SRC_PROXY (rw) register accessor: CFG0_RST_SRC_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_src_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_src_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_src_proxy`]
module"]
#[doc(alias = "CFG0_RST_SRC_PROXY")]
pub type Cfg0RstSrcProxy = crate::Reg<cfg0_rst_src_proxy::Cfg0RstSrcProxySpec>;
#[doc = "CFG0_RST_SRC_PROXY"]
pub mod cfg0_rst_src_proxy;
#[doc = "CFG0_RST_MAGIC_WORD_PROXY (rw) register accessor: CFG0_RST_MAGIC_WORD_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_magic_word_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_magic_word_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_rst_magic_word_proxy`]
module"]
#[doc(alias = "CFG0_RST_MAGIC_WORD_PROXY")]
pub type Cfg0RstMagicWordProxy = crate::Reg<cfg0_rst_magic_word_proxy::Cfg0RstMagicWordProxySpec>;
#[doc = "CFG0_RST_MAGIC_WORD_PROXY"]
pub mod cfg0_rst_magic_word_proxy;
#[doc = "CFG0_ISO_CTRL_PROXY (rw) register accessor: CFG0_ISO_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_iso_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_iso_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_iso_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_ISO_CTRL_PROXY")]
pub type Cfg0IsoCtrlProxy = crate::Reg<cfg0_iso_ctrl_proxy::Cfg0IsoCtrlProxySpec>;
#[doc = "CFG0_ISO_CTRL_PROXY"]
pub mod cfg0_iso_ctrl_proxy;
#[doc = "CFG0_VDD_CORE_GLDTC_CTRL_PROXY (rw) register accessor: CFG0_VDD_CORE_GLDTC_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_vdd_core_gldtc_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_VDD_CORE_GLDTC_CTRL_PROXY")]
pub type Cfg0VddCoreGldtcCtrlProxy =
    crate::Reg<cfg0_vdd_core_gldtc_ctrl_proxy::Cfg0VddCoreGldtcCtrlProxySpec>;
#[doc = "CFG0_VDD_CORE_GLDTC_CTRL_PROXY"]
pub mod cfg0_vdd_core_gldtc_ctrl_proxy;
#[doc = "CFG0_VDD_CORE_GLDTC_STAT_PROXY (rw) register accessor: CFG0_VDD_CORE_GLDTC_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_vdd_core_gldtc_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_vdd_core_gldtc_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_vdd_core_gldtc_stat_proxy`]
module"]
#[doc(alias = "CFG0_VDD_CORE_GLDTC_STAT_PROXY")]
pub type Cfg0VddCoreGldtcStatProxy =
    crate::Reg<cfg0_vdd_core_gldtc_stat_proxy::Cfg0VddCoreGldtcStatProxySpec>;
#[doc = "CFG0_VDD_CORE_GLDTC_STAT_PROXY"]
pub mod cfg0_vdd_core_gldtc_stat_proxy;
#[doc = "CFG0_PRG_PP_0_CTRL_PROXY (rw) register accessor: CFG0_PRG_PP_0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_prg_pp_0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_PRG_PP_0_CTRL_PROXY")]
pub type Cfg0PrgPp0CtrlProxy = crate::Reg<cfg0_prg_pp_0_ctrl_proxy::Cfg0PrgPp0CtrlProxySpec>;
#[doc = "CFG0_PRG_PP_0_CTRL_PROXY"]
pub mod cfg0_prg_pp_0_ctrl_proxy;
#[doc = "CFG0_PRG_PP_1_CTRL_PROXY (rw) register accessor: CFG0_PRG_PP_1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_prg_pp_1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_prg_pp_1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_prg_pp_1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_PRG_PP_1_CTRL_PROXY")]
pub type Cfg0PrgPp1CtrlProxy = crate::Reg<cfg0_prg_pp_1_ctrl_proxy::Cfg0PrgPp1CtrlProxySpec>;
#[doc = "CFG0_PRG_PP_1_CTRL_PROXY"]
pub mod cfg0_prg_pp_1_ctrl_proxy;
#[doc = "CFG0_MCU_CLKGATE_CTRL_PROXY (rw) register accessor: CFG0_MCU_CLKGATE_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_clkgate_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_clkgate_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcu_clkgate_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_MCU_CLKGATE_CTRL_PROXY")]
pub type Cfg0McuClkgateCtrlProxy =
    crate::Reg<cfg0_mcu_clkgate_ctrl_proxy::Cfg0McuClkgateCtrlProxySpec>;
#[doc = "CFG0_MCU_CLKGATE_CTRL_PROXY"]
pub mod cfg0_mcu_clkgate_ctrl_proxy;
#[doc = "CFG0_MAIN_CLKGATE_CTRL0_PROXY (rw) register accessor: CFG0_MAIN_CLKGATE_CTRL0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_clkgate_ctrl0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_clkgate_ctrl0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_clkgate_ctrl0_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_CLKGATE_CTRL0_PROXY")]
pub type Cfg0MainClkgateCtrl0Proxy =
    crate::Reg<cfg0_main_clkgate_ctrl0_proxy::Cfg0MainClkgateCtrl0ProxySpec>;
#[doc = "CFG0_MAIN_CLKGATE_CTRL0_PROXY"]
pub mod cfg0_main_clkgate_ctrl0_proxy;
#[doc = "CFG0_LOCK6_KICK0_PROXY (rw) register accessor: CFG0_LOCK6_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock6_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock6_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock6_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK6_KICK0_PROXY")]
pub type Cfg0Lock6Kick0Proxy = crate::Reg<cfg0_lock6_kick0_proxy::Cfg0Lock6Kick0ProxySpec>;
#[doc = "CFG0_LOCK6_KICK0_PROXY"]
pub mod cfg0_lock6_kick0_proxy;
#[doc = "CFG0_LOCK6_KICK1_PROXY (rw) register accessor: CFG0_LOCK6_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock6_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock6_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock6_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK6_KICK1_PROXY")]
pub type Cfg0Lock6Kick1Proxy = crate::Reg<cfg0_lock6_kick1_proxy::Cfg0Lock6Kick1ProxySpec>;
#[doc = "CFG0_LOCK6_KICK1_PROXY"]
pub mod cfg0_lock6_kick1_proxy;
#[doc = "CFG0_CLAIMREG_P6_R0 (rw) register accessor: CFG0_CLAIMREG_P6_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R0")]
pub type Cfg0ClaimregP6R0 = crate::Reg<cfg0_claimreg_p6_r0::Cfg0ClaimregP6R0Spec>;
#[doc = "CFG0_CLAIMREG_P6_R0"]
pub mod cfg0_claimreg_p6_r0;
#[doc = "CFG0_CLAIMREG_P6_R1 (rw) register accessor: CFG0_CLAIMREG_P6_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R1")]
pub type Cfg0ClaimregP6R1 = crate::Reg<cfg0_claimreg_p6_r1::Cfg0ClaimregP6R1Spec>;
#[doc = "CFG0_CLAIMREG_P6_R1"]
pub mod cfg0_claimreg_p6_r1;
#[doc = "CFG0_CLAIMREG_P6_R2 (rw) register accessor: CFG0_CLAIMREG_P6_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r2`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R2")]
pub type Cfg0ClaimregP6R2 = crate::Reg<cfg0_claimreg_p6_r2::Cfg0ClaimregP6R2Spec>;
#[doc = "CFG0_CLAIMREG_P6_R2"]
pub mod cfg0_claimreg_p6_r2;
#[doc = "CFG0_CLAIMREG_P6_R3 (rw) register accessor: CFG0_CLAIMREG_P6_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r3`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R3")]
pub type Cfg0ClaimregP6R3 = crate::Reg<cfg0_claimreg_p6_r3::Cfg0ClaimregP6R3Spec>;
#[doc = "CFG0_CLAIMREG_P6_R3"]
pub mod cfg0_claimreg_p6_r3;
#[doc = "CFG0_CLAIMREG_P6_R4 (rw) register accessor: CFG0_CLAIMREG_P6_R4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r4`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R4")]
pub type Cfg0ClaimregP6R4 = crate::Reg<cfg0_claimreg_p6_r4::Cfg0ClaimregP6R4Spec>;
#[doc = "CFG0_CLAIMREG_P6_R4"]
pub mod cfg0_claimreg_p6_r4;
#[doc = "CFG0_CLAIMREG_P6_R5 (rw) register accessor: CFG0_CLAIMREG_P6_R5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p6_r5`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P6_R5")]
pub type Cfg0ClaimregP6R5 = crate::Reg<cfg0_claimreg_p6_r5::Cfg0ClaimregP6R5Spec>;
#[doc = "CFG0_CLAIMREG_P6_R5"]
pub mod cfg0_claimreg_p6_r5;
