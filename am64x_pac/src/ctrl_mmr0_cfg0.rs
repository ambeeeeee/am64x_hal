#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg0_pid: Cfg0Pid,
    _reserved1: [u8; 0x04],
    cfg0_mmr_cfg1: Cfg0MmrCfg1,
    _reserved2: [u8; 0x08],
    cfg0_jtagid: Cfg0Jtagid,
    cfg0_jtag_user_id: Cfg0JtagUserId,
    _reserved4: [u8; 0x14],
    cfg0_main_devstat: Cfg0MainDevstat,
    cfg0_main_bootcfg: Cfg0MainBootcfg,
    _reserved6: [u8; 0x0c],
    cfg0_boot_progress: Cfg0BootProgress,
    _reserved7: [u8; 0x18],
    cfg0_device_feature0: Cfg0DeviceFeature0,
    _reserved8: [u8; 0x04],
    cfg0_device_feature2: Cfg0DeviceFeature2,
    _reserved9: [u8; 0x0c],
    cfg0_device_feature6: Cfg0DeviceFeature6,
    _reserved10: [u8; 0x0184],
    cfg0_mac_id0: Cfg0MacId0,
    cfg0_mac_id1: Cfg0MacId1,
    _reserved12: [u8; 0x08],
    cfg0_pci_device_id0: Cfg0PciDeviceId0,
    cfg0_pci_device_id1: Cfg0PciDeviceId1,
    _reserved14: [u8; 0x08],
    cfg0_usb_device_id0: Cfg0UsbDeviceId0,
    cfg0_usb_device_id1: Cfg0UsbDeviceId1,
    _reserved16: [u8; 0x08],
    cfg0_gp_sw0: Cfg0GpSw0,
    cfg0_gp_sw1: Cfg0GpSw1,
    cfg0_gp_sw2: Cfg0GpSw2,
    cfg0_gp_sw3: Cfg0GpSw3,
    _reserved20: [u8; 0x30],
    cfg0_cba_err_stat: Cfg0CbaErrStat,
    _reserved21: [u8; 0x0d94],
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
    _reserved32: [u8; 0xcc],
    cfg0_claimreg_p0_r0_readonly: Cfg0ClaimregP0R0Readonly,
    cfg0_claimreg_p0_r1_readonly: Cfg0ClaimregP0R1Readonly,
    cfg0_claimreg_p0_r2_readonly: Cfg0ClaimregP0R2Readonly,
    cfg0_claimreg_p0_r3_readonly: Cfg0ClaimregP0R3Readonly,
    cfg0_claimreg_p0_r4_readonly: Cfg0ClaimregP0R4Readonly,
    cfg0_claimreg_p0_r5_readonly: Cfg0ClaimregP0R5Readonly,
    cfg0_claimreg_p0_r6_readonly: Cfg0ClaimregP0R6Readonly,
    _reserved39: [u8; 0x0ee4],
    cfg0_pid_proxy: Cfg0PidProxy,
    _reserved40: [u8; 0x04],
    cfg0_mmr_cfg1_proxy: Cfg0MmrCfg1Proxy,
    _reserved41: [u8; 0x08],
    cfg0_jtagid_proxy: Cfg0JtagidProxy,
    cfg0_jtag_user_id_proxy: Cfg0JtagUserIdProxy,
    _reserved43: [u8; 0x14],
    cfg0_main_devstat_proxy: Cfg0MainDevstatProxy,
    cfg0_main_bootcfg_proxy: Cfg0MainBootcfgProxy,
    _reserved45: [u8; 0x0c],
    cfg0_boot_progress_proxy: Cfg0BootProgressProxy,
    _reserved46: [u8; 0x18],
    cfg0_device_feature0_proxy: Cfg0DeviceFeature0Proxy,
    _reserved47: [u8; 0x04],
    cfg0_device_feature2_proxy: Cfg0DeviceFeature2Proxy,
    _reserved48: [u8; 0x0c],
    cfg0_device_feature6_proxy: Cfg0DeviceFeature6Proxy,
    _reserved49: [u8; 0x0184],
    cfg0_mac_id0_proxy: Cfg0MacId0Proxy,
    cfg0_mac_id1_proxy: Cfg0MacId1Proxy,
    _reserved51: [u8; 0x08],
    cfg0_pci_device_id0_proxy: Cfg0PciDeviceId0Proxy,
    cfg0_pci_device_id1_proxy: Cfg0PciDeviceId1Proxy,
    _reserved53: [u8; 0x08],
    cfg0_usb_device_id0_proxy: Cfg0UsbDeviceId0Proxy,
    cfg0_usb_device_id1_proxy: Cfg0UsbDeviceId1Proxy,
    _reserved55: [u8; 0x08],
    cfg0_gp_sw0_proxy: Cfg0GpSw0Proxy,
    cfg0_gp_sw1_proxy: Cfg0GpSw1Proxy,
    cfg0_gp_sw2_proxy: Cfg0GpSw2Proxy,
    cfg0_gp_sw3_proxy: Cfg0GpSw3Proxy,
    _reserved59: [u8; 0x30],
    cfg0_cba_err_stat_proxy: Cfg0CbaErrStatProxy,
    _reserved60: [u8; 0x0d94],
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
    _reserved71: [u8; 0xcc],
    cfg0_claimreg_p0_r0: Cfg0ClaimregP0R0,
    cfg0_claimreg_p0_r1: Cfg0ClaimregP0R1,
    cfg0_claimreg_p0_r2: Cfg0ClaimregP0R2,
    cfg0_claimreg_p0_r3: Cfg0ClaimregP0R3,
    cfg0_claimreg_p0_r4: Cfg0ClaimregP0R4,
    cfg0_claimreg_p0_r5: Cfg0ClaimregP0R5,
    cfg0_claimreg_p0_r6: Cfg0ClaimregP0R6,
    _reserved78: [u8; 0x0eec],
    cfg0_usb0_phy_ctrl: Cfg0Usb0PhyCtrl,
    _reserved79: [u8; 0x38],
    cfg0_enet1_ctrl: Cfg0Enet1Ctrl,
    cfg0_enet2_ctrl: Cfg0Enet2Ctrl,
    _reserved81: [u8; 0x24],
    cfg0_pcie0_ctrl: Cfg0Pcie0Ctrl,
    _reserved82: [u8; 0x0c],
    cfg0_serdes0_ln0_ctrl: Cfg0Serdes0Ln0Ctrl,
    _reserved83: [u8; 0x3c],
    cfg0_adc0_trim: Cfg0Adc0Trim,
    _reserved84: [u8; 0x1c],
    cfg0_serdes0_ctrl: Cfg0Serdes0Ctrl,
    _reserved85: [u8; 0x1c],
    cfg0_icssg0_ctrl0: Cfg0Icssg0Ctrl0,
    cfg0_icssg0_ctrl1: Cfg0Icssg0Ctrl1,
    _reserved87: [u8; 0x08],
    cfg0_icssg1_ctrl0: Cfg0Icssg1Ctrl0,
    cfg0_icssg1_ctrl1: Cfg0Icssg1Ctrl1,
    _reserved89: [u8; 0x18],
    cfg0_epwm_tb_clken: Cfg0EpwmTbClken,
    cfg0_epwm_tb_clken_set: Cfg0EpwmTbClkenSet,
    cfg0_epwm_tb_clken_clr: Cfg0EpwmTbClkenClr,
    _reserved92: [u8; 0x04],
    cfg0_epwm0_ctrl: Cfg0Epwm0Ctrl,
    cfg0_epwm1_ctrl: Cfg0Epwm1Ctrl,
    cfg0_epwm2_ctrl: Cfg0Epwm2Ctrl,
    cfg0_epwm3_ctrl: Cfg0Epwm3Ctrl,
    cfg0_epwm4_ctrl: Cfg0Epwm4Ctrl,
    cfg0_epwm5_ctrl: Cfg0Epwm5Ctrl,
    cfg0_epwm6_ctrl: Cfg0Epwm6Ctrl,
    cfg0_epwm7_ctrl: Cfg0Epwm7Ctrl,
    cfg0_epwm8_ctrl: Cfg0Epwm8Ctrl,
    _reserved101: [u8; 0x0c],
    cfg0_soca_sel: Cfg0SocaSel,
    cfg0_socb_sel: Cfg0SocbSel,
    _reserved103: [u8; 0x08],
    cfg0_eqep0_ctrl: Cfg0Eqep0Ctrl,
    cfg0_eqep1_ctrl: Cfg0Eqep1Ctrl,
    cfg0_eqep2_ctrl: Cfg0Eqep2Ctrl,
    _reserved106: [u8; 0x14],
    cfg0_eqep_stat: Cfg0EqepStat,
    _reserved107: [u8; 0x10],
    cfg0_sdio1_ctrl: Cfg0Sdio1Ctrl,
    _reserved108: [u8; 0x4c],
    cfg0_timer1_ctrl: Cfg0Timer1Ctrl,
    _reserved109: [u8; 0x04],
    cfg0_timer3_ctrl: Cfg0Timer3Ctrl,
    _reserved110: [u8; 0x04],
    cfg0_timer5_ctrl: Cfg0Timer5Ctrl,
    _reserved111: [u8; 0x04],
    cfg0_timer7_ctrl: Cfg0Timer7Ctrl,
    _reserved112: [u8; 0x04],
    cfg0_timer9_ctrl: Cfg0Timer9Ctrl,
    _reserved113: [u8; 0x04],
    cfg0_timer11_ctrl: Cfg0Timer11Ctrl,
    _reserved114: [u8; 0xb0],
    cfg0_i2c0_ctrl: Cfg0I2c0Ctrl,
    _reserved115: [u8; 0x041c],
    cfg0_fss_ctrl: Cfg0FssCtrl,
    _reserved116: [u8; 0x0c],
    cfg0_adc0_ctrl: Cfg0Adc0Ctrl,
    _reserved117: [u8; 0x3c],
    cfg0_dcc_stat: Cfg0DccStat,
    _reserved118: [u8; 0x08b4],
    cfg0_lock1_kick0: Cfg0Lock1Kick0,
    cfg0_lock1_kick1: Cfg0Lock1Kick1,
    _reserved120: [u8; 0xf0],
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
    cfg0_claimreg_p1_r13_readonly: Cfg0ClaimregP1R13Readonly,
    cfg0_claimreg_p1_r14_readonly: Cfg0ClaimregP1R14Readonly,
    _reserved135: [u8; 0x0ecc],
    cfg0_usb0_phy_ctrl_proxy: Cfg0Usb0PhyCtrlProxy,
    _reserved136: [u8; 0x38],
    cfg0_enet1_ctrl_proxy: Cfg0Enet1CtrlProxy,
    cfg0_enet2_ctrl_proxy: Cfg0Enet2CtrlProxy,
    _reserved138: [u8; 0x24],
    cfg0_pcie0_ctrl_proxy: Cfg0Pcie0CtrlProxy,
    _reserved139: [u8; 0x0c],
    cfg0_serdes0_ln0_ctrl_proxy: Cfg0Serdes0Ln0CtrlProxy,
    _reserved140: [u8; 0x3c],
    cfg0_adc0_trim_proxy: Cfg0Adc0TrimProxy,
    _reserved141: [u8; 0x1c],
    cfg0_serdes0_ctrl_proxy: Cfg0Serdes0CtrlProxy,
    _reserved142: [u8; 0x1c],
    cfg0_icssg0_ctrl0_proxy: Cfg0Icssg0Ctrl0Proxy,
    cfg0_icssg0_ctrl1_proxy: Cfg0Icssg0Ctrl1Proxy,
    _reserved144: [u8; 0x08],
    cfg0_icssg1_ctrl0_proxy: Cfg0Icssg1Ctrl0Proxy,
    cfg0_icssg1_ctrl1_proxy: Cfg0Icssg1Ctrl1Proxy,
    _reserved146: [u8; 0x18],
    cfg0_epwm_tb_clken_proxy: Cfg0EpwmTbClkenProxy,
    cfg0_epwm_tb_clken_set_proxy: Cfg0EpwmTbClkenSetProxy,
    cfg0_epwm_tb_clken_clr_proxy: Cfg0EpwmTbClkenClrProxy,
    _reserved149: [u8; 0x04],
    cfg0_epwm0_ctrl_proxy: Cfg0Epwm0CtrlProxy,
    cfg0_epwm1_ctrl_proxy: Cfg0Epwm1CtrlProxy,
    cfg0_epwm2_ctrl_proxy: Cfg0Epwm2CtrlProxy,
    cfg0_epwm3_ctrl_proxy: Cfg0Epwm3CtrlProxy,
    cfg0_epwm4_ctrl_proxy: Cfg0Epwm4CtrlProxy,
    cfg0_epwm5_ctrl_proxy: Cfg0Epwm5CtrlProxy,
    cfg0_epwm6_ctrl_proxy: Cfg0Epwm6CtrlProxy,
    cfg0_epwm7_ctrl_proxy: Cfg0Epwm7CtrlProxy,
    cfg0_epwm8_ctrl_proxy: Cfg0Epwm8CtrlProxy,
    _reserved158: [u8; 0x0c],
    cfg0_soca_sel_proxy: Cfg0SocaSelProxy,
    cfg0_socb_sel_proxy: Cfg0SocbSelProxy,
    _reserved160: [u8; 0x08],
    cfg0_eqep0_ctrl_proxy: Cfg0Eqep0CtrlProxy,
    cfg0_eqep1_ctrl_proxy: Cfg0Eqep1CtrlProxy,
    cfg0_eqep2_ctrl_proxy: Cfg0Eqep2CtrlProxy,
    _reserved163: [u8; 0x14],
    cfg0_eqep_stat_proxy: Cfg0EqepStatProxy,
    _reserved164: [u8; 0x10],
    cfg0_sdio1_ctrl_proxy: Cfg0Sdio1CtrlProxy,
    _reserved165: [u8; 0x4c],
    cfg0_timer1_ctrl_proxy: Cfg0Timer1CtrlProxy,
    _reserved166: [u8; 0x04],
    cfg0_timer3_ctrl_proxy: Cfg0Timer3CtrlProxy,
    _reserved167: [u8; 0x04],
    cfg0_timer5_ctrl_proxy: Cfg0Timer5CtrlProxy,
    _reserved168: [u8; 0x04],
    cfg0_timer7_ctrl_proxy: Cfg0Timer7CtrlProxy,
    _reserved169: [u8; 0x04],
    cfg0_timer9_ctrl_proxy: Cfg0Timer9CtrlProxy,
    _reserved170: [u8; 0x04],
    cfg0_timer11_ctrl_proxy: Cfg0Timer11CtrlProxy,
    _reserved171: [u8; 0xb0],
    cfg0_i2c0_ctrl_proxy: Cfg0I2c0CtrlProxy,
    _reserved172: [u8; 0x041c],
    cfg0_fss_ctrl_proxy: Cfg0FssCtrlProxy,
    _reserved173: [u8; 0x0c],
    cfg0_adc0_ctrl_proxy: Cfg0Adc0CtrlProxy,
    _reserved174: [u8; 0x3c],
    cfg0_dcc_stat_proxy: Cfg0DccStatProxy,
    _reserved175: [u8; 0x08b4],
    cfg0_lock1_kick0_proxy: Cfg0Lock1Kick0Proxy,
    cfg0_lock1_kick1_proxy: Cfg0Lock1Kick1Proxy,
    _reserved177: [u8; 0xf0],
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
    cfg0_claimreg_p1_r13: Cfg0ClaimregP1R13,
    cfg0_claimreg_p1_r14: Cfg0ClaimregP1R14,
    _reserved192: [u8; 0x0ec4],
    cfg0_obsclk0_ctrl: Cfg0Obsclk0Ctrl,
    _reserved193: [u8; 0x0c],
    cfg0_clkout_ctrl: Cfg0ClkoutCtrl,
    _reserved194: [u8; 0x1c],
    cfg0_gtc_clksel: Cfg0GtcClksel,
    _reserved195: [u8; 0x08],
    cfg0_efuse_clksel: Cfg0EfuseClksel,
    cfg0_icssg0_clksel: Cfg0Icssg0Clksel,
    cfg0_icssg1_clksel: Cfg0Icssg1Clksel,
    _reserved198: [u8; 0x18],
    cfg0_main_pll0_clksel: Cfg0MainPll0Clksel,
    cfg0_main_pll1_clksel: Cfg0MainPll1Clksel,
    cfg0_main_pll2_clksel: Cfg0MainPll2Clksel,
    _reserved201: [u8; 0x14],
    cfg0_main_pll8_clksel: Cfg0MainPll8Clksel,
    _reserved202: [u8; 0x0c],
    cfg0_main_pll12_clksel: Cfg0MainPll12Clksel,
    _reserved203: [u8; 0x04],
    cfg0_main_pll14_clksel: Cfg0MainPll14Clksel,
    _reserved204: [u8; 0x84],
    cfg0_pcie0_clksel: Cfg0Pcie0Clksel,
    _reserved205: [u8; 0x1c],
    cfg0_cpsw_clksel: Cfg0CpswClksel,
    _reserved206: [u8; 0x0c],
    cfg0_cpts_clksel: Cfg0CptsClksel,
    _reserved207: [u8; 0x0c],
    cfg0_emmc0_clksel: Cfg0Emmc0Clksel,
    _reserved208: [u8; 0x04],
    cfg0_emmc1_clksel: Cfg0Emmc1Clksel,
    _reserved209: [u8; 0x14],
    cfg0_gpmc_clksel: Cfg0GpmcClksel,
    _reserved210: [u8; 0x0c],
    cfg0_usb0_clksel: Cfg0Usb0Clksel,
    _reserved211: [u8; 0x1c],
    cfg0_timer0_clksel: Cfg0Timer0Clksel,
    cfg0_timer1_clksel: Cfg0Timer1Clksel,
    cfg0_timer2_clksel: Cfg0Timer2Clksel,
    cfg0_timer3_clksel: Cfg0Timer3Clksel,
    cfg0_timer4_clksel: Cfg0Timer4Clksel,
    cfg0_timer5_clksel: Cfg0Timer5Clksel,
    cfg0_timer6_clksel: Cfg0Timer6Clksel,
    cfg0_timer7_clksel: Cfg0Timer7Clksel,
    cfg0_timer8_clksel: Cfg0Timer8Clksel,
    cfg0_timer9_clksel: Cfg0Timer9Clksel,
    cfg0_timer10_clksel: Cfg0Timer10Clksel,
    cfg0_timer11_clksel: Cfg0Timer11Clksel,
    _reserved223: [u8; 0x20],
    cfg0_spi0_clksel: Cfg0Spi0Clksel,
    cfg0_spi1_clksel: Cfg0Spi1Clksel,
    cfg0_spi2_clksel: Cfg0Spi2Clksel,
    cfg0_spi3_clksel: Cfg0Spi3Clksel,
    cfg0_spi4_clksel: Cfg0Spi4Clksel,
    _reserved228: [u8; 0x2c],
    cfg0_usart0_clk_ctrl: Cfg0Usart0ClkCtrl,
    cfg0_usart1_clk_ctrl: Cfg0Usart1ClkCtrl,
    cfg0_usart2_clk_ctrl: Cfg0Usart2ClkCtrl,
    cfg0_usart3_clk_ctrl: Cfg0Usart3ClkCtrl,
    cfg0_usart4_clk_ctrl: Cfg0Usart4ClkCtrl,
    cfg0_usart5_clk_ctrl: Cfg0Usart5ClkCtrl,
    cfg0_usart6_clk_ctrl: Cfg0Usart6ClkCtrl,
    _reserved235: [u8; 0x24],
    cfg0_usart0_clksel: Cfg0Usart0Clksel,
    cfg0_usart1_clksel: Cfg0Usart1Clksel,
    cfg0_usart2_clksel: Cfg0Usart2Clksel,
    cfg0_usart3_clksel: Cfg0Usart3Clksel,
    cfg0_usart4_clksel: Cfg0Usart4Clksel,
    cfg0_usart5_clksel: Cfg0Usart5Clksel,
    cfg0_usart6_clksel: Cfg0Usart6Clksel,
    _reserved242: [u8; 0xe4],
    cfg0_wwd0_clksel: Cfg0Wwd0Clksel,
    cfg0_wwd1_clksel: Cfg0Wwd1Clksel,
    _reserved244: [u8; 0x18],
    cfg0_wwd8_clksel: Cfg0Wwd8Clksel,
    cfg0_wwd9_clksel: Cfg0Wwd9Clksel,
    cfg0_wwd10_clksel: Cfg0Wwd10Clksel,
    cfg0_wwd11_clksel: Cfg0Wwd11Clksel,
    _reserved248: [u8; 0x50],
    cfg0_serdes0_clksel: Cfg0Serdes0Clksel,
    _reserved249: [u8; 0x7c],
    cfg0_mcan0_clksel: Cfg0Mcan0Clksel,
    cfg0_mcan1_clksel: Cfg0Mcan1Clksel,
    _reserved251: [u8; 0x78],
    cfg0_ospi0_clksel: Cfg0Ospi0Clksel,
    _reserved252: [u8; 0x0c],
    cfg0_adc0_clksel: Cfg0Adc0Clksel,
    _reserved253: [u8; 0x0af4],
    cfg0_lock2_kick0: Cfg0Lock2Kick0,
    cfg0_lock2_kick1: Cfg0Lock2Kick1,
    _reserved255: [u8; 0xf0],
    cfg0_claimreg_p2_r0_readonly: Cfg0ClaimregP2R0Readonly,
    cfg0_claimreg_p2_r1_readonly: Cfg0ClaimregP2R1Readonly,
    cfg0_claimreg_p2_r2_readonly: Cfg0ClaimregP2R2Readonly,
    cfg0_claimreg_p2_r3_readonly: Cfg0ClaimregP2R3Readonly,
    cfg0_claimreg_p2_r4_readonly: Cfg0ClaimregP2R4Readonly,
    cfg0_claimreg_p2_r5_readonly: Cfg0ClaimregP2R5Readonly,
    cfg0_claimreg_p2_r6_readonly: Cfg0ClaimregP2R6Readonly,
    cfg0_claimreg_p2_r7_readonly: Cfg0ClaimregP2R7Readonly,
    cfg0_claimreg_p2_r8_readonly: Cfg0ClaimregP2R8Readonly,
    cfg0_claimreg_p2_r9_readonly: Cfg0ClaimregP2R9Readonly,
    cfg0_claimreg_p2_r10_readonly: Cfg0ClaimregP2R10Readonly,
    _reserved266: [u8; 0x0ed4],
    cfg0_obsclk0_ctrl_proxy: Cfg0Obsclk0CtrlProxy,
    _reserved267: [u8; 0x0c],
    cfg0_clkout_ctrl_proxy: Cfg0ClkoutCtrlProxy,
    _reserved268: [u8; 0x1c],
    cfg0_gtc_clksel_proxy: Cfg0GtcClkselProxy,
    _reserved269: [u8; 0x08],
    cfg0_efuse_clksel_proxy: Cfg0EfuseClkselProxy,
    cfg0_icssg0_clksel_proxy: Cfg0Icssg0ClkselProxy,
    cfg0_icssg1_clksel_proxy: Cfg0Icssg1ClkselProxy,
    _reserved272: [u8; 0x18],
    cfg0_main_pll0_clksel_proxy: Cfg0MainPll0ClkselProxy,
    cfg0_main_pll1_clksel_proxy: Cfg0MainPll1ClkselProxy,
    cfg0_main_pll2_clksel_proxy: Cfg0MainPll2ClkselProxy,
    _reserved275: [u8; 0x14],
    cfg0_main_pll8_clksel_proxy: Cfg0MainPll8ClkselProxy,
    _reserved276: [u8; 0x0c],
    cfg0_main_pll12_clksel_proxy: Cfg0MainPll12ClkselProxy,
    _reserved277: [u8; 0x04],
    cfg0_main_pll14_clksel_proxy: Cfg0MainPll14ClkselProxy,
    _reserved278: [u8; 0x84],
    cfg0_pcie0_clksel_proxy: Cfg0Pcie0ClkselProxy,
    _reserved279: [u8; 0x1c],
    cfg0_cpsw_clksel_proxy: Cfg0CpswClkselProxy,
    _reserved280: [u8; 0x0c],
    cfg0_cpts_clksel_proxy: Cfg0CptsClkselProxy,
    _reserved281: [u8; 0x0c],
    cfg0_emmc0_clksel_proxy: Cfg0Emmc0ClkselProxy,
    _reserved282: [u8; 0x04],
    cfg0_emmc1_clksel_proxy: Cfg0Emmc1ClkselProxy,
    _reserved283: [u8; 0x14],
    cfg0_gpmc_clksel_proxy: Cfg0GpmcClkselProxy,
    _reserved284: [u8; 0x0c],
    cfg0_usb0_clksel_proxy: Cfg0Usb0ClkselProxy,
    _reserved285: [u8; 0x1c],
    cfg0_timer0_clksel_proxy: Cfg0Timer0ClkselProxy,
    cfg0_timer1_clksel_proxy: Cfg0Timer1ClkselProxy,
    cfg0_timer2_clksel_proxy: Cfg0Timer2ClkselProxy,
    cfg0_timer3_clksel_proxy: Cfg0Timer3ClkselProxy,
    cfg0_timer4_clksel_proxy: Cfg0Timer4ClkselProxy,
    cfg0_timer5_clksel_proxy: Cfg0Timer5ClkselProxy,
    cfg0_timer6_clksel_proxy: Cfg0Timer6ClkselProxy,
    cfg0_timer7_clksel_proxy: Cfg0Timer7ClkselProxy,
    cfg0_timer8_clksel_proxy: Cfg0Timer8ClkselProxy,
    cfg0_timer9_clksel_proxy: Cfg0Timer9ClkselProxy,
    cfg0_timer10_clksel_proxy: Cfg0Timer10ClkselProxy,
    cfg0_timer11_clksel_proxy: Cfg0Timer11ClkselProxy,
    _reserved297: [u8; 0x20],
    cfg0_spi0_clksel_proxy: Cfg0Spi0ClkselProxy,
    cfg0_spi1_clksel_proxy: Cfg0Spi1ClkselProxy,
    cfg0_spi2_clksel_proxy: Cfg0Spi2ClkselProxy,
    cfg0_spi3_clksel_proxy: Cfg0Spi3ClkselProxy,
    cfg0_spi4_clksel_proxy: Cfg0Spi4ClkselProxy,
    _reserved302: [u8; 0x2c],
    cfg0_usart0_clk_ctrl_proxy: Cfg0Usart0ClkCtrlProxy,
    cfg0_usart1_clk_ctrl_proxy: Cfg0Usart1ClkCtrlProxy,
    cfg0_usart2_clk_ctrl_proxy: Cfg0Usart2ClkCtrlProxy,
    cfg0_usart3_clk_ctrl_proxy: Cfg0Usart3ClkCtrlProxy,
    cfg0_usart4_clk_ctrl_proxy: Cfg0Usart4ClkCtrlProxy,
    cfg0_usart5_clk_ctrl_proxy: Cfg0Usart5ClkCtrlProxy,
    cfg0_usart6_clk_ctrl_proxy: Cfg0Usart6ClkCtrlProxy,
    _reserved309: [u8; 0x24],
    cfg0_usart0_clksel_proxy: Cfg0Usart0ClkselProxy,
    cfg0_usart1_clksel_proxy: Cfg0Usart1ClkselProxy,
    cfg0_usart2_clksel_proxy: Cfg0Usart2ClkselProxy,
    cfg0_usart3_clksel_proxy: Cfg0Usart3ClkselProxy,
    cfg0_usart4_clksel_proxy: Cfg0Usart4ClkselProxy,
    cfg0_usart5_clksel_proxy: Cfg0Usart5ClkselProxy,
    cfg0_usart6_clksel_proxy: Cfg0Usart6ClkselProxy,
    _reserved316: [u8; 0xe4],
    cfg0_wwd0_clksel_proxy: Cfg0Wwd0ClkselProxy,
    cfg0_wwd1_clksel_proxy: Cfg0Wwd1ClkselProxy,
    _reserved318: [u8; 0x18],
    cfg0_wwd8_clksel_proxy: Cfg0Wwd8ClkselProxy,
    cfg0_wwd9_clksel_proxy: Cfg0Wwd9ClkselProxy,
    cfg0_wwd10_clksel_proxy: Cfg0Wwd10ClkselProxy,
    cfg0_wwd11_clksel_proxy: Cfg0Wwd11ClkselProxy,
    _reserved322: [u8; 0x50],
    cfg0_serdes0_clksel_proxy: Cfg0Serdes0ClkselProxy,
    _reserved323: [u8; 0x7c],
    cfg0_mcan0_clksel_proxy: Cfg0Mcan0ClkselProxy,
    cfg0_mcan1_clksel_proxy: Cfg0Mcan1ClkselProxy,
    _reserved325: [u8; 0x78],
    cfg0_ospi0_clksel_proxy: Cfg0Ospi0ClkselProxy,
    _reserved326: [u8; 0x0c],
    cfg0_adc0_clksel_proxy: Cfg0Adc0ClkselProxy,
    _reserved327: [u8; 0x0af4],
    cfg0_lock2_kick0_proxy: Cfg0Lock2Kick0Proxy,
    cfg0_lock2_kick1_proxy: Cfg0Lock2Kick1Proxy,
    _reserved329: [u8; 0xf0],
    cfg0_claimreg_p2_r0: Cfg0ClaimregP2R0,
    cfg0_claimreg_p2_r1: Cfg0ClaimregP2R1,
    cfg0_claimreg_p2_r2: Cfg0ClaimregP2R2,
    cfg0_claimreg_p2_r3: Cfg0ClaimregP2R3,
    cfg0_claimreg_p2_r4: Cfg0ClaimregP2R4,
    cfg0_claimreg_p2_r5: Cfg0ClaimregP2R5,
    cfg0_claimreg_p2_r6: Cfg0ClaimregP2R6,
    cfg0_claimreg_p2_r7: Cfg0ClaimregP2R7,
    cfg0_claimreg_p2_r8: Cfg0ClaimregP2R8,
    cfg0_claimreg_p2_r9: Cfg0ClaimregP2R9,
    cfg0_claimreg_p2_r10: Cfg0ClaimregP2R10,
    _reserved340: [u8; 0x11f4],
    cfg0_fuse_crc_stat: Cfg0FuseCrcStat,
    _reserved341: [u8; 0xdc],
    cfg0_pbist_en: Cfg0PbistEn,
    _reserved342: [u8; 0x0c04],
    cfg0_lock3_kick0: Cfg0Lock3Kick0,
    cfg0_lock3_kick1: Cfg0Lock3Kick1,
    _reserved344: [u8; 0xf0],
    cfg0_claimreg_p3_r0_readonly: Cfg0ClaimregP3R0Readonly,
    cfg0_claimreg_p3_r1_readonly: Cfg0ClaimregP3R1Readonly,
    cfg0_claimreg_p3_r2_readonly: Cfg0ClaimregP3R2Readonly,
    cfg0_claimreg_p3_r3_readonly: Cfg0ClaimregP3R3Readonly,
    cfg0_claimreg_p3_r4_readonly: Cfg0ClaimregP3R4Readonly,
    cfg0_claimreg_p3_r5_readonly: Cfg0ClaimregP3R5Readonly,
    cfg0_claimreg_p3_r6_readonly: Cfg0ClaimregP3R6Readonly,
    cfg0_claimreg_p3_r7_readonly: Cfg0ClaimregP3R7Readonly,
    cfg0_claimreg_p3_r8_readonly: Cfg0ClaimregP3R8Readonly,
    _reserved353: [u8; 0x11fc],
    cfg0_fuse_crc_stat_proxy: Cfg0FuseCrcStatProxy,
    _reserved354: [u8; 0xdc],
    cfg0_pbist_en_proxy: Cfg0PbistEnProxy,
    _reserved355: [u8; 0x0c04],
    cfg0_lock3_kick0_proxy: Cfg0Lock3Kick0Proxy,
    cfg0_lock3_kick1_proxy: Cfg0Lock3Kick1Proxy,
    _reserved357: [u8; 0xf0],
    cfg0_claimreg_p3_r0: Cfg0ClaimregP3R0,
    cfg0_claimreg_p3_r1: Cfg0ClaimregP3R1,
    cfg0_claimreg_p3_r2: Cfg0ClaimregP3R2,
    cfg0_claimreg_p3_r3: Cfg0ClaimregP3R3,
    cfg0_claimreg_p3_r4: Cfg0ClaimregP3R4,
    cfg0_claimreg_p3_r5: Cfg0ClaimregP3R5,
    cfg0_claimreg_p3_r6: Cfg0ClaimregP3R6,
    cfg0_claimreg_p3_r7: Cfg0ClaimregP3R7,
    cfg0_claimreg_p3_r8: Cfg0ClaimregP3R8,
    _reserved366: [u8; 0x1ee4],
    cfg0_lock4_kick0: Cfg0Lock4Kick0,
    cfg0_lock4_kick1: Cfg0Lock4Kick1,
    _reserved368: [u8; 0xf0],
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
    _reserved381: [u8; 0x1ed4],
    cfg0_lock4_kick0_proxy: Cfg0Lock4Kick0Proxy,
    cfg0_lock4_kick1_proxy: Cfg0Lock4Kick1Proxy,
    _reserved383: [u8; 0xf0],
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
    _reserved396: [u8; 0x0ecc],
    cfg0_chng_ddr4_fsp_req: Cfg0ChngDdr4FspReq,
    cfg0_chng_ddr4_fsp_ack: Cfg0ChngDdr4FspAck,
    _reserved398: [u8; 0x78],
    cfg0_ddr4_fsp_clkchng_req: Cfg0Ddr4FspClkchngReq,
    _reserved399: [u8; 0x3c],
    cfg0_ddr4_fsp_clkchng_ack: Cfg0Ddr4FspClkchngAck,
    _reserved400: [u8; 0x0f44],
    cfg0_lock5_kick0: Cfg0Lock5Kick0,
    cfg0_lock5_kick1: Cfg0Lock5Kick1,
    _reserved402: [u8; 0xf0],
    cfg0_claimreg_p5_r0_readonly: Cfg0ClaimregP5R0Readonly,
    cfg0_claimreg_p5_r1_readonly: Cfg0ClaimregP5R1Readonly,
    _reserved404: [u8; 0x0ef8],
    cfg0_chng_ddr4_fsp_req_proxy: Cfg0ChngDdr4FspReqProxy,
    cfg0_chng_ddr4_fsp_ack_proxy: Cfg0ChngDdr4FspAckProxy,
    _reserved406: [u8; 0x78],
    cfg0_ddr4_fsp_clkchng_req_proxy: Cfg0Ddr4FspClkchngReqProxy,
    _reserved407: [u8; 0x3c],
    cfg0_ddr4_fsp_clkchng_ack_proxy: Cfg0Ddr4FspClkchngAckProxy,
    _reserved408: [u8; 0x0f44],
    cfg0_lock5_kick0_proxy: Cfg0Lock5Kick0Proxy,
    cfg0_lock5_kick1_proxy: Cfg0Lock5Kick1Proxy,
    _reserved410: [u8; 0xf0],
    cfg0_claimreg_p5_r0: Cfg0ClaimregP5R0,
    cfg0_claimreg_p5_r1: Cfg0ClaimregP5R1,
    _reserved412: [u8; 0x1068],
    cfg0_rst_ctrl: Cfg0RstCtrl,
    cfg0_rst_stat: Cfg0RstStat,
    cfg0_rst_src: Cfg0RstSrc,
    cfg0_rst_magic_word: Cfg0RstMagicWord,
    _reserved416: [u8; 0x0e88],
    cfg0_lock6_kick0: Cfg0Lock6Kick0,
    cfg0_lock6_kick1: Cfg0Lock6Kick1,
    _reserved418: [u8; 0xf0],
    cfg0_claimreg_p6_r0_readonly: Cfg0ClaimregP6R0Readonly,
    cfg0_claimreg_p6_r1_readonly: Cfg0ClaimregP6R1Readonly,
    cfg0_claimreg_p6_r2_readonly: Cfg0ClaimregP6R2Readonly,
    _reserved421: [u8; 0x1064],
    cfg0_rst_ctrl_proxy: Cfg0RstCtrlProxy,
    cfg0_rst_stat_proxy: Cfg0RstStatProxy,
    cfg0_rst_src_proxy: Cfg0RstSrcProxy,
    cfg0_rst_magic_word_proxy: Cfg0RstMagicWordProxy,
    _reserved425: [u8; 0x0e88],
    cfg0_lock6_kick0_proxy: Cfg0Lock6Kick0Proxy,
    cfg0_lock6_kick1_proxy: Cfg0Lock6Kick1Proxy,
    _reserved427: [u8; 0xf0],
    cfg0_claimreg_p6_r0: Cfg0ClaimregP6R0,
    cfg0_claimreg_p6_r1: Cfg0ClaimregP6R1,
    cfg0_claimreg_p6_r2: Cfg0ClaimregP6R2,
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
    #[doc = "0x14 - CFG0_JTAGID"]
    #[inline(always)]
    pub const fn cfg0_jtagid(&self) -> &Cfg0Jtagid {
        &self.cfg0_jtagid
    }
    #[doc = "0x18 - CFG0_JTAG_USER_ID"]
    #[inline(always)]
    pub const fn cfg0_jtag_user_id(&self) -> &Cfg0JtagUserId {
        &self.cfg0_jtag_user_id
    }
    #[doc = "0x30 - CFG0_MAIN_DEVSTAT"]
    #[inline(always)]
    pub const fn cfg0_main_devstat(&self) -> &Cfg0MainDevstat {
        &self.cfg0_main_devstat
    }
    #[doc = "0x34 - CFG0_MAIN_BOOTCFG"]
    #[inline(always)]
    pub const fn cfg0_main_bootcfg(&self) -> &Cfg0MainBootcfg {
        &self.cfg0_main_bootcfg
    }
    #[doc = "0x44 - CFG0_BOOT_PROGRESS"]
    #[inline(always)]
    pub const fn cfg0_boot_progress(&self) -> &Cfg0BootProgress {
        &self.cfg0_boot_progress
    }
    #[doc = "0x60 - CFG0_DEVICE_FEATURE0"]
    #[inline(always)]
    pub const fn cfg0_device_feature0(&self) -> &Cfg0DeviceFeature0 {
        &self.cfg0_device_feature0
    }
    #[doc = "0x68 - CFG0_DEVICE_FEATURE2"]
    #[inline(always)]
    pub const fn cfg0_device_feature2(&self) -> &Cfg0DeviceFeature2 {
        &self.cfg0_device_feature2
    }
    #[doc = "0x78 - CFG0_DEVICE_FEATURE6"]
    #[inline(always)]
    pub const fn cfg0_device_feature6(&self) -> &Cfg0DeviceFeature6 {
        &self.cfg0_device_feature6
    }
    #[doc = "0x200 - CFG0_MAC_ID0"]
    #[inline(always)]
    pub const fn cfg0_mac_id0(&self) -> &Cfg0MacId0 {
        &self.cfg0_mac_id0
    }
    #[doc = "0x204 - CFG0_MAC_ID1"]
    #[inline(always)]
    pub const fn cfg0_mac_id1(&self) -> &Cfg0MacId1 {
        &self.cfg0_mac_id1
    }
    #[doc = "0x210 - CFG0_PCI_DEVICE_ID0"]
    #[inline(always)]
    pub const fn cfg0_pci_device_id0(&self) -> &Cfg0PciDeviceId0 {
        &self.cfg0_pci_device_id0
    }
    #[doc = "0x214 - CFG0_PCI_DEVICE_ID1"]
    #[inline(always)]
    pub const fn cfg0_pci_device_id1(&self) -> &Cfg0PciDeviceId1 {
        &self.cfg0_pci_device_id1
    }
    #[doc = "0x220 - CFG0_USB_DEVICE_ID0"]
    #[inline(always)]
    pub const fn cfg0_usb_device_id0(&self) -> &Cfg0UsbDeviceId0 {
        &self.cfg0_usb_device_id0
    }
    #[doc = "0x224 - CFG0_USB_DEVICE_ID1"]
    #[inline(always)]
    pub const fn cfg0_usb_device_id1(&self) -> &Cfg0UsbDeviceId1 {
        &self.cfg0_usb_device_id1
    }
    #[doc = "0x230 - CFG0_GP_SW0"]
    #[inline(always)]
    pub const fn cfg0_gp_sw0(&self) -> &Cfg0GpSw0 {
        &self.cfg0_gp_sw0
    }
    #[doc = "0x234 - CFG0_GP_SW1"]
    #[inline(always)]
    pub const fn cfg0_gp_sw1(&self) -> &Cfg0GpSw1 {
        &self.cfg0_gp_sw1
    }
    #[doc = "0x238 - CFG0_GP_SW2"]
    #[inline(always)]
    pub const fn cfg0_gp_sw2(&self) -> &Cfg0GpSw2 {
        &self.cfg0_gp_sw2
    }
    #[doc = "0x23c - CFG0_GP_SW3"]
    #[inline(always)]
    pub const fn cfg0_gp_sw3(&self) -> &Cfg0GpSw3 {
        &self.cfg0_gp_sw3
    }
    #[doc = "0x270 - CFG0_CBA_ERR_STAT"]
    #[inline(always)]
    pub const fn cfg0_cba_err_stat(&self) -> &Cfg0CbaErrStat {
        &self.cfg0_cba_err_stat
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
    #[doc = "0x1104 - CFG0_CLAIMREG_P0_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r1_readonly(&self) -> &Cfg0ClaimregP0R1Readonly {
        &self.cfg0_claimreg_p0_r1_readonly
    }
    #[doc = "0x1108 - CFG0_CLAIMREG_P0_R2_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r2_readonly(&self) -> &Cfg0ClaimregP0R2Readonly {
        &self.cfg0_claimreg_p0_r2_readonly
    }
    #[doc = "0x110c - CFG0_CLAIMREG_P0_R3_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r3_readonly(&self) -> &Cfg0ClaimregP0R3Readonly {
        &self.cfg0_claimreg_p0_r3_readonly
    }
    #[doc = "0x1110 - CFG0_CLAIMREG_P0_R4_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r4_readonly(&self) -> &Cfg0ClaimregP0R4Readonly {
        &self.cfg0_claimreg_p0_r4_readonly
    }
    #[doc = "0x1114 - CFG0_CLAIMREG_P0_R5_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r5_readonly(&self) -> &Cfg0ClaimregP0R5Readonly {
        &self.cfg0_claimreg_p0_r5_readonly
    }
    #[doc = "0x1118 - CFG0_CLAIMREG_P0_R6_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r6_readonly(&self) -> &Cfg0ClaimregP0R6Readonly {
        &self.cfg0_claimreg_p0_r6_readonly
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
    #[doc = "0x2014 - CFG0_JTAGID_PROXY"]
    #[inline(always)]
    pub const fn cfg0_jtagid_proxy(&self) -> &Cfg0JtagidProxy {
        &self.cfg0_jtagid_proxy
    }
    #[doc = "0x2018 - CFG0_JTAG_USER_ID_PROXY"]
    #[inline(always)]
    pub const fn cfg0_jtag_user_id_proxy(&self) -> &Cfg0JtagUserIdProxy {
        &self.cfg0_jtag_user_id_proxy
    }
    #[doc = "0x2030 - CFG0_MAIN_DEVSTAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_devstat_proxy(&self) -> &Cfg0MainDevstatProxy {
        &self.cfg0_main_devstat_proxy
    }
    #[doc = "0x2034 - CFG0_MAIN_BOOTCFG_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_bootcfg_proxy(&self) -> &Cfg0MainBootcfgProxy {
        &self.cfg0_main_bootcfg_proxy
    }
    #[doc = "0x2044 - CFG0_BOOT_PROGRESS_PROXY"]
    #[inline(always)]
    pub const fn cfg0_boot_progress_proxy(&self) -> &Cfg0BootProgressProxy {
        &self.cfg0_boot_progress_proxy
    }
    #[doc = "0x2060 - CFG0_DEVICE_FEATURE0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_device_feature0_proxy(&self) -> &Cfg0DeviceFeature0Proxy {
        &self.cfg0_device_feature0_proxy
    }
    #[doc = "0x2068 - CFG0_DEVICE_FEATURE2_PROXY"]
    #[inline(always)]
    pub const fn cfg0_device_feature2_proxy(&self) -> &Cfg0DeviceFeature2Proxy {
        &self.cfg0_device_feature2_proxy
    }
    #[doc = "0x2078 - CFG0_DEVICE_FEATURE6_PROXY"]
    #[inline(always)]
    pub const fn cfg0_device_feature6_proxy(&self) -> &Cfg0DeviceFeature6Proxy {
        &self.cfg0_device_feature6_proxy
    }
    #[doc = "0x2200 - CFG0_MAC_ID0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mac_id0_proxy(&self) -> &Cfg0MacId0Proxy {
        &self.cfg0_mac_id0_proxy
    }
    #[doc = "0x2204 - CFG0_MAC_ID1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mac_id1_proxy(&self) -> &Cfg0MacId1Proxy {
        &self.cfg0_mac_id1_proxy
    }
    #[doc = "0x2210 - CFG0_PCI_DEVICE_ID0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pci_device_id0_proxy(&self) -> &Cfg0PciDeviceId0Proxy {
        &self.cfg0_pci_device_id0_proxy
    }
    #[doc = "0x2214 - CFG0_PCI_DEVICE_ID1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pci_device_id1_proxy(&self) -> &Cfg0PciDeviceId1Proxy {
        &self.cfg0_pci_device_id1_proxy
    }
    #[doc = "0x2220 - CFG0_USB_DEVICE_ID0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usb_device_id0_proxy(&self) -> &Cfg0UsbDeviceId0Proxy {
        &self.cfg0_usb_device_id0_proxy
    }
    #[doc = "0x2224 - CFG0_USB_DEVICE_ID1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usb_device_id1_proxy(&self) -> &Cfg0UsbDeviceId1Proxy {
        &self.cfg0_usb_device_id1_proxy
    }
    #[doc = "0x2230 - CFG0_GP_SW0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_gp_sw0_proxy(&self) -> &Cfg0GpSw0Proxy {
        &self.cfg0_gp_sw0_proxy
    }
    #[doc = "0x2234 - CFG0_GP_SW1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_gp_sw1_proxy(&self) -> &Cfg0GpSw1Proxy {
        &self.cfg0_gp_sw1_proxy
    }
    #[doc = "0x2238 - CFG0_GP_SW2_PROXY"]
    #[inline(always)]
    pub const fn cfg0_gp_sw2_proxy(&self) -> &Cfg0GpSw2Proxy {
        &self.cfg0_gp_sw2_proxy
    }
    #[doc = "0x223c - CFG0_GP_SW3_PROXY"]
    #[inline(always)]
    pub const fn cfg0_gp_sw3_proxy(&self) -> &Cfg0GpSw3Proxy {
        &self.cfg0_gp_sw3_proxy
    }
    #[doc = "0x2270 - CFG0_CBA_ERR_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_cba_err_stat_proxy(&self) -> &Cfg0CbaErrStatProxy {
        &self.cfg0_cba_err_stat_proxy
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
    #[doc = "0x3104 - CFG0_CLAIMREG_P0_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r1(&self) -> &Cfg0ClaimregP0R1 {
        &self.cfg0_claimreg_p0_r1
    }
    #[doc = "0x3108 - CFG0_CLAIMREG_P0_R2"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r2(&self) -> &Cfg0ClaimregP0R2 {
        &self.cfg0_claimreg_p0_r2
    }
    #[doc = "0x310c - CFG0_CLAIMREG_P0_R3"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r3(&self) -> &Cfg0ClaimregP0R3 {
        &self.cfg0_claimreg_p0_r3
    }
    #[doc = "0x3110 - CFG0_CLAIMREG_P0_R4"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r4(&self) -> &Cfg0ClaimregP0R4 {
        &self.cfg0_claimreg_p0_r4
    }
    #[doc = "0x3114 - CFG0_CLAIMREG_P0_R5"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r5(&self) -> &Cfg0ClaimregP0R5 {
        &self.cfg0_claimreg_p0_r5
    }
    #[doc = "0x3118 - CFG0_CLAIMREG_P0_R6"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p0_r6(&self) -> &Cfg0ClaimregP0R6 {
        &self.cfg0_claimreg_p0_r6
    }
    #[doc = "0x4008 - CFG0_USB0_PHY_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usb0_phy_ctrl(&self) -> &Cfg0Usb0PhyCtrl {
        &self.cfg0_usb0_phy_ctrl
    }
    #[doc = "0x4044 - CFG0_ENET1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_enet1_ctrl(&self) -> &Cfg0Enet1Ctrl {
        &self.cfg0_enet1_ctrl
    }
    #[doc = "0x4048 - CFG0_ENET2_CTRL"]
    #[inline(always)]
    pub const fn cfg0_enet2_ctrl(&self) -> &Cfg0Enet2Ctrl {
        &self.cfg0_enet2_ctrl
    }
    #[doc = "0x4070 - CFG0_PCIE0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_pcie0_ctrl(&self) -> &Cfg0Pcie0Ctrl {
        &self.cfg0_pcie0_ctrl
    }
    #[doc = "0x4080 - CFG0_SERDES0_LN0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_serdes0_ln0_ctrl(&self) -> &Cfg0Serdes0Ln0Ctrl {
        &self.cfg0_serdes0_ln0_ctrl
    }
    #[doc = "0x40c0 - CFG0_ADC0_TRIM"]
    #[inline(always)]
    pub const fn cfg0_adc0_trim(&self) -> &Cfg0Adc0Trim {
        &self.cfg0_adc0_trim
    }
    #[doc = "0x40e0 - CFG0_SERDES0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_serdes0_ctrl(&self) -> &Cfg0Serdes0Ctrl {
        &self.cfg0_serdes0_ctrl
    }
    #[doc = "0x4100 - CFG0_ICSSG0_CTRL0"]
    #[inline(always)]
    pub const fn cfg0_icssg0_ctrl0(&self) -> &Cfg0Icssg0Ctrl0 {
        &self.cfg0_icssg0_ctrl0
    }
    #[doc = "0x4104 - CFG0_ICSSG0_CTRL1"]
    #[inline(always)]
    pub const fn cfg0_icssg0_ctrl1(&self) -> &Cfg0Icssg0Ctrl1 {
        &self.cfg0_icssg0_ctrl1
    }
    #[doc = "0x4110 - CFG0_ICSSG1_CTRL0"]
    #[inline(always)]
    pub const fn cfg0_icssg1_ctrl0(&self) -> &Cfg0Icssg1Ctrl0 {
        &self.cfg0_icssg1_ctrl0
    }
    #[doc = "0x4114 - CFG0_ICSSG1_CTRL1"]
    #[inline(always)]
    pub const fn cfg0_icssg1_ctrl1(&self) -> &Cfg0Icssg1Ctrl1 {
        &self.cfg0_icssg1_ctrl1
    }
    #[doc = "0x4130 - CFG0_EPWM_TB_CLKEN"]
    #[inline(always)]
    pub const fn cfg0_epwm_tb_clken(&self) -> &Cfg0EpwmTbClken {
        &self.cfg0_epwm_tb_clken
    }
    #[doc = "0x4134 - CFG0_EPWM_TB_CLKEN_SET"]
    #[inline(always)]
    pub const fn cfg0_epwm_tb_clken_set(&self) -> &Cfg0EpwmTbClkenSet {
        &self.cfg0_epwm_tb_clken_set
    }
    #[doc = "0x4138 - CFG0_EPWM_TB_CLKEN_CLR"]
    #[inline(always)]
    pub const fn cfg0_epwm_tb_clken_clr(&self) -> &Cfg0EpwmTbClkenClr {
        &self.cfg0_epwm_tb_clken_clr
    }
    #[doc = "0x4140 - CFG0_EPWM0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm0_ctrl(&self) -> &Cfg0Epwm0Ctrl {
        &self.cfg0_epwm0_ctrl
    }
    #[doc = "0x4144 - CFG0_EPWM1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm1_ctrl(&self) -> &Cfg0Epwm1Ctrl {
        &self.cfg0_epwm1_ctrl
    }
    #[doc = "0x4148 - CFG0_EPWM2_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm2_ctrl(&self) -> &Cfg0Epwm2Ctrl {
        &self.cfg0_epwm2_ctrl
    }
    #[doc = "0x414c - CFG0_EPWM3_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm3_ctrl(&self) -> &Cfg0Epwm3Ctrl {
        &self.cfg0_epwm3_ctrl
    }
    #[doc = "0x4150 - CFG0_EPWM4_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm4_ctrl(&self) -> &Cfg0Epwm4Ctrl {
        &self.cfg0_epwm4_ctrl
    }
    #[doc = "0x4154 - CFG0_EPWM5_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm5_ctrl(&self) -> &Cfg0Epwm5Ctrl {
        &self.cfg0_epwm5_ctrl
    }
    #[doc = "0x4158 - CFG0_EPWM6_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm6_ctrl(&self) -> &Cfg0Epwm6Ctrl {
        &self.cfg0_epwm6_ctrl
    }
    #[doc = "0x415c - CFG0_EPWM7_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm7_ctrl(&self) -> &Cfg0Epwm7Ctrl {
        &self.cfg0_epwm7_ctrl
    }
    #[doc = "0x4160 - CFG0_EPWM8_CTRL"]
    #[inline(always)]
    pub const fn cfg0_epwm8_ctrl(&self) -> &Cfg0Epwm8Ctrl {
        &self.cfg0_epwm8_ctrl
    }
    #[doc = "0x4170 - CFG0_SOCA_SEL"]
    #[inline(always)]
    pub const fn cfg0_soca_sel(&self) -> &Cfg0SocaSel {
        &self.cfg0_soca_sel
    }
    #[doc = "0x4174 - CFG0_SOCB_SEL"]
    #[inline(always)]
    pub const fn cfg0_socb_sel(&self) -> &Cfg0SocbSel {
        &self.cfg0_socb_sel
    }
    #[doc = "0x4180 - CFG0_EQEP0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_eqep0_ctrl(&self) -> &Cfg0Eqep0Ctrl {
        &self.cfg0_eqep0_ctrl
    }
    #[doc = "0x4184 - CFG0_EQEP1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_eqep1_ctrl(&self) -> &Cfg0Eqep1Ctrl {
        &self.cfg0_eqep1_ctrl
    }
    #[doc = "0x4188 - CFG0_EQEP2_CTRL"]
    #[inline(always)]
    pub const fn cfg0_eqep2_ctrl(&self) -> &Cfg0Eqep2Ctrl {
        &self.cfg0_eqep2_ctrl
    }
    #[doc = "0x41a0 - CFG0_EQEP_STAT"]
    #[inline(always)]
    pub const fn cfg0_eqep_stat(&self) -> &Cfg0EqepStat {
        &self.cfg0_eqep_stat
    }
    #[doc = "0x41b4 - CFG0_SDIO1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_sdio1_ctrl(&self) -> &Cfg0Sdio1Ctrl {
        &self.cfg0_sdio1_ctrl
    }
    #[doc = "0x4204 - CFG0_TIMER1_CTRL"]
    #[inline(always)]
    pub const fn cfg0_timer1_ctrl(&self) -> &Cfg0Timer1Ctrl {
        &self.cfg0_timer1_ctrl
    }
    #[doc = "0x420c - CFG0_TIMER3_CTRL"]
    #[inline(always)]
    pub const fn cfg0_timer3_ctrl(&self) -> &Cfg0Timer3Ctrl {
        &self.cfg0_timer3_ctrl
    }
    #[doc = "0x4214 - CFG0_TIMER5_CTRL"]
    #[inline(always)]
    pub const fn cfg0_timer5_ctrl(&self) -> &Cfg0Timer5Ctrl {
        &self.cfg0_timer5_ctrl
    }
    #[doc = "0x421c - CFG0_TIMER7_CTRL"]
    #[inline(always)]
    pub const fn cfg0_timer7_ctrl(&self) -> &Cfg0Timer7Ctrl {
        &self.cfg0_timer7_ctrl
    }
    #[doc = "0x4224 - CFG0_TIMER9_CTRL"]
    #[inline(always)]
    pub const fn cfg0_timer9_ctrl(&self) -> &Cfg0Timer9Ctrl {
        &self.cfg0_timer9_ctrl
    }
    #[doc = "0x422c - CFG0_TIMER11_CTRL"]
    #[inline(always)]
    pub const fn cfg0_timer11_ctrl(&self) -> &Cfg0Timer11Ctrl {
        &self.cfg0_timer11_ctrl
    }
    #[doc = "0x42e0 - CFG0_I2C0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_i2c0_ctrl(&self) -> &Cfg0I2c0Ctrl {
        &self.cfg0_i2c0_ctrl
    }
    #[doc = "0x4700 - CFG0_FSS_CTRL"]
    #[inline(always)]
    pub const fn cfg0_fss_ctrl(&self) -> &Cfg0FssCtrl {
        &self.cfg0_fss_ctrl
    }
    #[doc = "0x4710 - CFG0_ADC0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_adc0_ctrl(&self) -> &Cfg0Adc0Ctrl {
        &self.cfg0_adc0_ctrl
    }
    #[doc = "0x4750 - CFG0_DCC_STAT"]
    #[inline(always)]
    pub const fn cfg0_dcc_stat(&self) -> &Cfg0DccStat {
        &self.cfg0_dcc_stat
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
    #[doc = "0x5134 - CFG0_CLAIMREG_P1_R13_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r13_readonly(&self) -> &Cfg0ClaimregP1R13Readonly {
        &self.cfg0_claimreg_p1_r13_readonly
    }
    #[doc = "0x5138 - CFG0_CLAIMREG_P1_R14_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r14_readonly(&self) -> &Cfg0ClaimregP1R14Readonly {
        &self.cfg0_claimreg_p1_r14_readonly
    }
    #[doc = "0x6008 - CFG0_USB0_PHY_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usb0_phy_ctrl_proxy(&self) -> &Cfg0Usb0PhyCtrlProxy {
        &self.cfg0_usb0_phy_ctrl_proxy
    }
    #[doc = "0x6044 - CFG0_ENET1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_enet1_ctrl_proxy(&self) -> &Cfg0Enet1CtrlProxy {
        &self.cfg0_enet1_ctrl_proxy
    }
    #[doc = "0x6048 - CFG0_ENET2_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_enet2_ctrl_proxy(&self) -> &Cfg0Enet2CtrlProxy {
        &self.cfg0_enet2_ctrl_proxy
    }
    #[doc = "0x6070 - CFG0_PCIE0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pcie0_ctrl_proxy(&self) -> &Cfg0Pcie0CtrlProxy {
        &self.cfg0_pcie0_ctrl_proxy
    }
    #[doc = "0x6080 - CFG0_SERDES0_LN0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_serdes0_ln0_ctrl_proxy(&self) -> &Cfg0Serdes0Ln0CtrlProxy {
        &self.cfg0_serdes0_ln0_ctrl_proxy
    }
    #[doc = "0x60c0 - CFG0_ADC0_TRIM_PROXY"]
    #[inline(always)]
    pub const fn cfg0_adc0_trim_proxy(&self) -> &Cfg0Adc0TrimProxy {
        &self.cfg0_adc0_trim_proxy
    }
    #[doc = "0x60e0 - CFG0_SERDES0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_serdes0_ctrl_proxy(&self) -> &Cfg0Serdes0CtrlProxy {
        &self.cfg0_serdes0_ctrl_proxy
    }
    #[doc = "0x6100 - CFG0_ICSSG0_CTRL0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_icssg0_ctrl0_proxy(&self) -> &Cfg0Icssg0Ctrl0Proxy {
        &self.cfg0_icssg0_ctrl0_proxy
    }
    #[doc = "0x6104 - CFG0_ICSSG0_CTRL1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_icssg0_ctrl1_proxy(&self) -> &Cfg0Icssg0Ctrl1Proxy {
        &self.cfg0_icssg0_ctrl1_proxy
    }
    #[doc = "0x6110 - CFG0_ICSSG1_CTRL0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_icssg1_ctrl0_proxy(&self) -> &Cfg0Icssg1Ctrl0Proxy {
        &self.cfg0_icssg1_ctrl0_proxy
    }
    #[doc = "0x6114 - CFG0_ICSSG1_CTRL1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_icssg1_ctrl1_proxy(&self) -> &Cfg0Icssg1Ctrl1Proxy {
        &self.cfg0_icssg1_ctrl1_proxy
    }
    #[doc = "0x6130 - CFG0_EPWM_TB_CLKEN_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm_tb_clken_proxy(&self) -> &Cfg0EpwmTbClkenProxy {
        &self.cfg0_epwm_tb_clken_proxy
    }
    #[doc = "0x6134 - CFG0_EPWM_TB_CLKEN_SET_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm_tb_clken_set_proxy(&self) -> &Cfg0EpwmTbClkenSetProxy {
        &self.cfg0_epwm_tb_clken_set_proxy
    }
    #[doc = "0x6138 - CFG0_EPWM_TB_CLKEN_CLR_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm_tb_clken_clr_proxy(&self) -> &Cfg0EpwmTbClkenClrProxy {
        &self.cfg0_epwm_tb_clken_clr_proxy
    }
    #[doc = "0x6140 - CFG0_EPWM0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm0_ctrl_proxy(&self) -> &Cfg0Epwm0CtrlProxy {
        &self.cfg0_epwm0_ctrl_proxy
    }
    #[doc = "0x6144 - CFG0_EPWM1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm1_ctrl_proxy(&self) -> &Cfg0Epwm1CtrlProxy {
        &self.cfg0_epwm1_ctrl_proxy
    }
    #[doc = "0x6148 - CFG0_EPWM2_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm2_ctrl_proxy(&self) -> &Cfg0Epwm2CtrlProxy {
        &self.cfg0_epwm2_ctrl_proxy
    }
    #[doc = "0x614c - CFG0_EPWM3_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm3_ctrl_proxy(&self) -> &Cfg0Epwm3CtrlProxy {
        &self.cfg0_epwm3_ctrl_proxy
    }
    #[doc = "0x6150 - CFG0_EPWM4_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm4_ctrl_proxy(&self) -> &Cfg0Epwm4CtrlProxy {
        &self.cfg0_epwm4_ctrl_proxy
    }
    #[doc = "0x6154 - CFG0_EPWM5_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm5_ctrl_proxy(&self) -> &Cfg0Epwm5CtrlProxy {
        &self.cfg0_epwm5_ctrl_proxy
    }
    #[doc = "0x6158 - CFG0_EPWM6_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm6_ctrl_proxy(&self) -> &Cfg0Epwm6CtrlProxy {
        &self.cfg0_epwm6_ctrl_proxy
    }
    #[doc = "0x615c - CFG0_EPWM7_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm7_ctrl_proxy(&self) -> &Cfg0Epwm7CtrlProxy {
        &self.cfg0_epwm7_ctrl_proxy
    }
    #[doc = "0x6160 - CFG0_EPWM8_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_epwm8_ctrl_proxy(&self) -> &Cfg0Epwm8CtrlProxy {
        &self.cfg0_epwm8_ctrl_proxy
    }
    #[doc = "0x6170 - CFG0_SOCA_SEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_soca_sel_proxy(&self) -> &Cfg0SocaSelProxy {
        &self.cfg0_soca_sel_proxy
    }
    #[doc = "0x6174 - CFG0_SOCB_SEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_socb_sel_proxy(&self) -> &Cfg0SocbSelProxy {
        &self.cfg0_socb_sel_proxy
    }
    #[doc = "0x6180 - CFG0_EQEP0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_eqep0_ctrl_proxy(&self) -> &Cfg0Eqep0CtrlProxy {
        &self.cfg0_eqep0_ctrl_proxy
    }
    #[doc = "0x6184 - CFG0_EQEP1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_eqep1_ctrl_proxy(&self) -> &Cfg0Eqep1CtrlProxy {
        &self.cfg0_eqep1_ctrl_proxy
    }
    #[doc = "0x6188 - CFG0_EQEP2_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_eqep2_ctrl_proxy(&self) -> &Cfg0Eqep2CtrlProxy {
        &self.cfg0_eqep2_ctrl_proxy
    }
    #[doc = "0x61a0 - CFG0_EQEP_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_eqep_stat_proxy(&self) -> &Cfg0EqepStatProxy {
        &self.cfg0_eqep_stat_proxy
    }
    #[doc = "0x61b4 - CFG0_SDIO1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_sdio1_ctrl_proxy(&self) -> &Cfg0Sdio1CtrlProxy {
        &self.cfg0_sdio1_ctrl_proxy
    }
    #[doc = "0x6204 - CFG0_TIMER1_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer1_ctrl_proxy(&self) -> &Cfg0Timer1CtrlProxy {
        &self.cfg0_timer1_ctrl_proxy
    }
    #[doc = "0x620c - CFG0_TIMER3_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer3_ctrl_proxy(&self) -> &Cfg0Timer3CtrlProxy {
        &self.cfg0_timer3_ctrl_proxy
    }
    #[doc = "0x6214 - CFG0_TIMER5_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer5_ctrl_proxy(&self) -> &Cfg0Timer5CtrlProxy {
        &self.cfg0_timer5_ctrl_proxy
    }
    #[doc = "0x621c - CFG0_TIMER7_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer7_ctrl_proxy(&self) -> &Cfg0Timer7CtrlProxy {
        &self.cfg0_timer7_ctrl_proxy
    }
    #[doc = "0x6224 - CFG0_TIMER9_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer9_ctrl_proxy(&self) -> &Cfg0Timer9CtrlProxy {
        &self.cfg0_timer9_ctrl_proxy
    }
    #[doc = "0x622c - CFG0_TIMER11_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer11_ctrl_proxy(&self) -> &Cfg0Timer11CtrlProxy {
        &self.cfg0_timer11_ctrl_proxy
    }
    #[doc = "0x62e0 - CFG0_I2C0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_i2c0_ctrl_proxy(&self) -> &Cfg0I2c0CtrlProxy {
        &self.cfg0_i2c0_ctrl_proxy
    }
    #[doc = "0x6700 - CFG0_FSS_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_fss_ctrl_proxy(&self) -> &Cfg0FssCtrlProxy {
        &self.cfg0_fss_ctrl_proxy
    }
    #[doc = "0x6710 - CFG0_ADC0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_adc0_ctrl_proxy(&self) -> &Cfg0Adc0CtrlProxy {
        &self.cfg0_adc0_ctrl_proxy
    }
    #[doc = "0x6750 - CFG0_DCC_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_dcc_stat_proxy(&self) -> &Cfg0DccStatProxy {
        &self.cfg0_dcc_stat_proxy
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
    #[doc = "0x7134 - CFG0_CLAIMREG_P1_R13"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r13(&self) -> &Cfg0ClaimregP1R13 {
        &self.cfg0_claimreg_p1_r13
    }
    #[doc = "0x7138 - CFG0_CLAIMREG_P1_R14"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p1_r14(&self) -> &Cfg0ClaimregP1R14 {
        &self.cfg0_claimreg_p1_r14
    }
    #[doc = "0x8000 - CFG0_OBSCLK0_CTRL"]
    #[inline(always)]
    pub const fn cfg0_obsclk0_ctrl(&self) -> &Cfg0Obsclk0Ctrl {
        &self.cfg0_obsclk0_ctrl
    }
    #[doc = "0x8010 - CFG0_CLKOUT_CTRL"]
    #[inline(always)]
    pub const fn cfg0_clkout_ctrl(&self) -> &Cfg0ClkoutCtrl {
        &self.cfg0_clkout_ctrl
    }
    #[doc = "0x8030 - CFG0_GTC_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_gtc_clksel(&self) -> &Cfg0GtcClksel {
        &self.cfg0_gtc_clksel
    }
    #[doc = "0x803c - CFG0_EFUSE_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_efuse_clksel(&self) -> &Cfg0EfuseClksel {
        &self.cfg0_efuse_clksel
    }
    #[doc = "0x8040 - CFG0_ICSSG0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_icssg0_clksel(&self) -> &Cfg0Icssg0Clksel {
        &self.cfg0_icssg0_clksel
    }
    #[doc = "0x8044 - CFG0_ICSSG1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_icssg1_clksel(&self) -> &Cfg0Icssg1Clksel {
        &self.cfg0_icssg1_clksel
    }
    #[doc = "0x8060 - CFG0_MAIN_PLL0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_main_pll0_clksel(&self) -> &Cfg0MainPll0Clksel {
        &self.cfg0_main_pll0_clksel
    }
    #[doc = "0x8064 - CFG0_MAIN_PLL1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_main_pll1_clksel(&self) -> &Cfg0MainPll1Clksel {
        &self.cfg0_main_pll1_clksel
    }
    #[doc = "0x8068 - CFG0_MAIN_PLL2_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_main_pll2_clksel(&self) -> &Cfg0MainPll2Clksel {
        &self.cfg0_main_pll2_clksel
    }
    #[doc = "0x8080 - CFG0_MAIN_PLL8_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_main_pll8_clksel(&self) -> &Cfg0MainPll8Clksel {
        &self.cfg0_main_pll8_clksel
    }
    #[doc = "0x8090 - CFG0_MAIN_PLL12_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_main_pll12_clksel(&self) -> &Cfg0MainPll12Clksel {
        &self.cfg0_main_pll12_clksel
    }
    #[doc = "0x8098 - CFG0_MAIN_PLL14_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_main_pll14_clksel(&self) -> &Cfg0MainPll14Clksel {
        &self.cfg0_main_pll14_clksel
    }
    #[doc = "0x8120 - CFG0_PCIE0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_pcie0_clksel(&self) -> &Cfg0Pcie0Clksel {
        &self.cfg0_pcie0_clksel
    }
    #[doc = "0x8140 - CFG0_CPSW_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_cpsw_clksel(&self) -> &Cfg0CpswClksel {
        &self.cfg0_cpsw_clksel
    }
    #[doc = "0x8150 - CFG0_CPTS_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_cpts_clksel(&self) -> &Cfg0CptsClksel {
        &self.cfg0_cpts_clksel
    }
    #[doc = "0x8160 - CFG0_EMMC0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_emmc0_clksel(&self) -> &Cfg0Emmc0Clksel {
        &self.cfg0_emmc0_clksel
    }
    #[doc = "0x8168 - CFG0_EMMC1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_emmc1_clksel(&self) -> &Cfg0Emmc1Clksel {
        &self.cfg0_emmc1_clksel
    }
    #[doc = "0x8180 - CFG0_GPMC_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_gpmc_clksel(&self) -> &Cfg0GpmcClksel {
        &self.cfg0_gpmc_clksel
    }
    #[doc = "0x8190 - CFG0_USB0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usb0_clksel(&self) -> &Cfg0Usb0Clksel {
        &self.cfg0_usb0_clksel
    }
    #[doc = "0x81b0 - CFG0_TIMER0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer0_clksel(&self) -> &Cfg0Timer0Clksel {
        &self.cfg0_timer0_clksel
    }
    #[doc = "0x81b4 - CFG0_TIMER1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer1_clksel(&self) -> &Cfg0Timer1Clksel {
        &self.cfg0_timer1_clksel
    }
    #[doc = "0x81b8 - CFG0_TIMER2_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer2_clksel(&self) -> &Cfg0Timer2Clksel {
        &self.cfg0_timer2_clksel
    }
    #[doc = "0x81bc - CFG0_TIMER3_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer3_clksel(&self) -> &Cfg0Timer3Clksel {
        &self.cfg0_timer3_clksel
    }
    #[doc = "0x81c0 - CFG0_TIMER4_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer4_clksel(&self) -> &Cfg0Timer4Clksel {
        &self.cfg0_timer4_clksel
    }
    #[doc = "0x81c4 - CFG0_TIMER5_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer5_clksel(&self) -> &Cfg0Timer5Clksel {
        &self.cfg0_timer5_clksel
    }
    #[doc = "0x81c8 - CFG0_TIMER6_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer6_clksel(&self) -> &Cfg0Timer6Clksel {
        &self.cfg0_timer6_clksel
    }
    #[doc = "0x81cc - CFG0_TIMER7_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer7_clksel(&self) -> &Cfg0Timer7Clksel {
        &self.cfg0_timer7_clksel
    }
    #[doc = "0x81d0 - CFG0_TIMER8_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer8_clksel(&self) -> &Cfg0Timer8Clksel {
        &self.cfg0_timer8_clksel
    }
    #[doc = "0x81d4 - CFG0_TIMER9_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer9_clksel(&self) -> &Cfg0Timer9Clksel {
        &self.cfg0_timer9_clksel
    }
    #[doc = "0x81d8 - CFG0_TIMER10_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer10_clksel(&self) -> &Cfg0Timer10Clksel {
        &self.cfg0_timer10_clksel
    }
    #[doc = "0x81dc - CFG0_TIMER11_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_timer11_clksel(&self) -> &Cfg0Timer11Clksel {
        &self.cfg0_timer11_clksel
    }
    #[doc = "0x8200 - CFG0_SPI0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_spi0_clksel(&self) -> &Cfg0Spi0Clksel {
        &self.cfg0_spi0_clksel
    }
    #[doc = "0x8204 - CFG0_SPI1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_spi1_clksel(&self) -> &Cfg0Spi1Clksel {
        &self.cfg0_spi1_clksel
    }
    #[doc = "0x8208 - CFG0_SPI2_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_spi2_clksel(&self) -> &Cfg0Spi2Clksel {
        &self.cfg0_spi2_clksel
    }
    #[doc = "0x820c - CFG0_SPI3_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_spi3_clksel(&self) -> &Cfg0Spi3Clksel {
        &self.cfg0_spi3_clksel
    }
    #[doc = "0x8210 - CFG0_SPI4_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_spi4_clksel(&self) -> &Cfg0Spi4Clksel {
        &self.cfg0_spi4_clksel
    }
    #[doc = "0x8240 - CFG0_USART0_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart0_clk_ctrl(&self) -> &Cfg0Usart0ClkCtrl {
        &self.cfg0_usart0_clk_ctrl
    }
    #[doc = "0x8244 - CFG0_USART1_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart1_clk_ctrl(&self) -> &Cfg0Usart1ClkCtrl {
        &self.cfg0_usart1_clk_ctrl
    }
    #[doc = "0x8248 - CFG0_USART2_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart2_clk_ctrl(&self) -> &Cfg0Usart2ClkCtrl {
        &self.cfg0_usart2_clk_ctrl
    }
    #[doc = "0x824c - CFG0_USART3_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart3_clk_ctrl(&self) -> &Cfg0Usart3ClkCtrl {
        &self.cfg0_usart3_clk_ctrl
    }
    #[doc = "0x8250 - CFG0_USART4_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart4_clk_ctrl(&self) -> &Cfg0Usart4ClkCtrl {
        &self.cfg0_usart4_clk_ctrl
    }
    #[doc = "0x8254 - CFG0_USART5_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart5_clk_ctrl(&self) -> &Cfg0Usart5ClkCtrl {
        &self.cfg0_usart5_clk_ctrl
    }
    #[doc = "0x8258 - CFG0_USART6_CLK_CTRL"]
    #[inline(always)]
    pub const fn cfg0_usart6_clk_ctrl(&self) -> &Cfg0Usart6ClkCtrl {
        &self.cfg0_usart6_clk_ctrl
    }
    #[doc = "0x8280 - CFG0_USART0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart0_clksel(&self) -> &Cfg0Usart0Clksel {
        &self.cfg0_usart0_clksel
    }
    #[doc = "0x8284 - CFG0_USART1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart1_clksel(&self) -> &Cfg0Usart1Clksel {
        &self.cfg0_usart1_clksel
    }
    #[doc = "0x8288 - CFG0_USART2_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart2_clksel(&self) -> &Cfg0Usart2Clksel {
        &self.cfg0_usart2_clksel
    }
    #[doc = "0x828c - CFG0_USART3_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart3_clksel(&self) -> &Cfg0Usart3Clksel {
        &self.cfg0_usart3_clksel
    }
    #[doc = "0x8290 - CFG0_USART4_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart4_clksel(&self) -> &Cfg0Usart4Clksel {
        &self.cfg0_usart4_clksel
    }
    #[doc = "0x8294 - CFG0_USART5_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart5_clksel(&self) -> &Cfg0Usart5Clksel {
        &self.cfg0_usart5_clksel
    }
    #[doc = "0x8298 - CFG0_USART6_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_usart6_clksel(&self) -> &Cfg0Usart6Clksel {
        &self.cfg0_usart6_clksel
    }
    #[doc = "0x8380 - CFG0_WWD0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_wwd0_clksel(&self) -> &Cfg0Wwd0Clksel {
        &self.cfg0_wwd0_clksel
    }
    #[doc = "0x8384 - CFG0_WWD1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_wwd1_clksel(&self) -> &Cfg0Wwd1Clksel {
        &self.cfg0_wwd1_clksel
    }
    #[doc = "0x83a0 - CFG0_WWD8_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_wwd8_clksel(&self) -> &Cfg0Wwd8Clksel {
        &self.cfg0_wwd8_clksel
    }
    #[doc = "0x83a4 - CFG0_WWD9_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_wwd9_clksel(&self) -> &Cfg0Wwd9Clksel {
        &self.cfg0_wwd9_clksel
    }
    #[doc = "0x83a8 - CFG0_WWD10_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_wwd10_clksel(&self) -> &Cfg0Wwd10Clksel {
        &self.cfg0_wwd10_clksel
    }
    #[doc = "0x83ac - CFG0_WWD11_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_wwd11_clksel(&self) -> &Cfg0Wwd11Clksel {
        &self.cfg0_wwd11_clksel
    }
    #[doc = "0x8400 - CFG0_SERDES0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_serdes0_clksel(&self) -> &Cfg0Serdes0Clksel {
        &self.cfg0_serdes0_clksel
    }
    #[doc = "0x8480 - CFG0_MCAN0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcan0_clksel(&self) -> &Cfg0Mcan0Clksel {
        &self.cfg0_mcan0_clksel
    }
    #[doc = "0x8484 - CFG0_MCAN1_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_mcan1_clksel(&self) -> &Cfg0Mcan1Clksel {
        &self.cfg0_mcan1_clksel
    }
    #[doc = "0x8500 - CFG0_OSPI0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_ospi0_clksel(&self) -> &Cfg0Ospi0Clksel {
        &self.cfg0_ospi0_clksel
    }
    #[doc = "0x8510 - CFG0_ADC0_CLKSEL"]
    #[inline(always)]
    pub const fn cfg0_adc0_clksel(&self) -> &Cfg0Adc0Clksel {
        &self.cfg0_adc0_clksel
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
    #[doc = "0x9108 - CFG0_CLAIMREG_P2_R2_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r2_readonly(&self) -> &Cfg0ClaimregP2R2Readonly {
        &self.cfg0_claimreg_p2_r2_readonly
    }
    #[doc = "0x910c - CFG0_CLAIMREG_P2_R3_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r3_readonly(&self) -> &Cfg0ClaimregP2R3Readonly {
        &self.cfg0_claimreg_p2_r3_readonly
    }
    #[doc = "0x9110 - CFG0_CLAIMREG_P2_R4_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r4_readonly(&self) -> &Cfg0ClaimregP2R4Readonly {
        &self.cfg0_claimreg_p2_r4_readonly
    }
    #[doc = "0x9114 - CFG0_CLAIMREG_P2_R5_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r5_readonly(&self) -> &Cfg0ClaimregP2R5Readonly {
        &self.cfg0_claimreg_p2_r5_readonly
    }
    #[doc = "0x9118 - CFG0_CLAIMREG_P2_R6_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r6_readonly(&self) -> &Cfg0ClaimregP2R6Readonly {
        &self.cfg0_claimreg_p2_r6_readonly
    }
    #[doc = "0x911c - CFG0_CLAIMREG_P2_R7_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r7_readonly(&self) -> &Cfg0ClaimregP2R7Readonly {
        &self.cfg0_claimreg_p2_r7_readonly
    }
    #[doc = "0x9120 - CFG0_CLAIMREG_P2_R8_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r8_readonly(&self) -> &Cfg0ClaimregP2R8Readonly {
        &self.cfg0_claimreg_p2_r8_readonly
    }
    #[doc = "0x9124 - CFG0_CLAIMREG_P2_R9_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r9_readonly(&self) -> &Cfg0ClaimregP2R9Readonly {
        &self.cfg0_claimreg_p2_r9_readonly
    }
    #[doc = "0x9128 - CFG0_CLAIMREG_P2_R10_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r10_readonly(&self) -> &Cfg0ClaimregP2R10Readonly {
        &self.cfg0_claimreg_p2_r10_readonly
    }
    #[doc = "0xa000 - CFG0_OBSCLK0_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_obsclk0_ctrl_proxy(&self) -> &Cfg0Obsclk0CtrlProxy {
        &self.cfg0_obsclk0_ctrl_proxy
    }
    #[doc = "0xa010 - CFG0_CLKOUT_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_clkout_ctrl_proxy(&self) -> &Cfg0ClkoutCtrlProxy {
        &self.cfg0_clkout_ctrl_proxy
    }
    #[doc = "0xa030 - CFG0_GTC_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_gtc_clksel_proxy(&self) -> &Cfg0GtcClkselProxy {
        &self.cfg0_gtc_clksel_proxy
    }
    #[doc = "0xa03c - CFG0_EFUSE_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_efuse_clksel_proxy(&self) -> &Cfg0EfuseClkselProxy {
        &self.cfg0_efuse_clksel_proxy
    }
    #[doc = "0xa040 - CFG0_ICSSG0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_icssg0_clksel_proxy(&self) -> &Cfg0Icssg0ClkselProxy {
        &self.cfg0_icssg0_clksel_proxy
    }
    #[doc = "0xa044 - CFG0_ICSSG1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_icssg1_clksel_proxy(&self) -> &Cfg0Icssg1ClkselProxy {
        &self.cfg0_icssg1_clksel_proxy
    }
    #[doc = "0xa060 - CFG0_MAIN_PLL0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_pll0_clksel_proxy(&self) -> &Cfg0MainPll0ClkselProxy {
        &self.cfg0_main_pll0_clksel_proxy
    }
    #[doc = "0xa064 - CFG0_MAIN_PLL1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_pll1_clksel_proxy(&self) -> &Cfg0MainPll1ClkselProxy {
        &self.cfg0_main_pll1_clksel_proxy
    }
    #[doc = "0xa068 - CFG0_MAIN_PLL2_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_pll2_clksel_proxy(&self) -> &Cfg0MainPll2ClkselProxy {
        &self.cfg0_main_pll2_clksel_proxy
    }
    #[doc = "0xa080 - CFG0_MAIN_PLL8_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_pll8_clksel_proxy(&self) -> &Cfg0MainPll8ClkselProxy {
        &self.cfg0_main_pll8_clksel_proxy
    }
    #[doc = "0xa090 - CFG0_MAIN_PLL12_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_pll12_clksel_proxy(&self) -> &Cfg0MainPll12ClkselProxy {
        &self.cfg0_main_pll12_clksel_proxy
    }
    #[doc = "0xa098 - CFG0_MAIN_PLL14_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_main_pll14_clksel_proxy(&self) -> &Cfg0MainPll14ClkselProxy {
        &self.cfg0_main_pll14_clksel_proxy
    }
    #[doc = "0xa120 - CFG0_PCIE0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pcie0_clksel_proxy(&self) -> &Cfg0Pcie0ClkselProxy {
        &self.cfg0_pcie0_clksel_proxy
    }
    #[doc = "0xa140 - CFG0_CPSW_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_cpsw_clksel_proxy(&self) -> &Cfg0CpswClkselProxy {
        &self.cfg0_cpsw_clksel_proxy
    }
    #[doc = "0xa150 - CFG0_CPTS_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_cpts_clksel_proxy(&self) -> &Cfg0CptsClkselProxy {
        &self.cfg0_cpts_clksel_proxy
    }
    #[doc = "0xa160 - CFG0_EMMC0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_emmc0_clksel_proxy(&self) -> &Cfg0Emmc0ClkselProxy {
        &self.cfg0_emmc0_clksel_proxy
    }
    #[doc = "0xa168 - CFG0_EMMC1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_emmc1_clksel_proxy(&self) -> &Cfg0Emmc1ClkselProxy {
        &self.cfg0_emmc1_clksel_proxy
    }
    #[doc = "0xa180 - CFG0_GPMC_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_gpmc_clksel_proxy(&self) -> &Cfg0GpmcClkselProxy {
        &self.cfg0_gpmc_clksel_proxy
    }
    #[doc = "0xa190 - CFG0_USB0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usb0_clksel_proxy(&self) -> &Cfg0Usb0ClkselProxy {
        &self.cfg0_usb0_clksel_proxy
    }
    #[doc = "0xa1b0 - CFG0_TIMER0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer0_clksel_proxy(&self) -> &Cfg0Timer0ClkselProxy {
        &self.cfg0_timer0_clksel_proxy
    }
    #[doc = "0xa1b4 - CFG0_TIMER1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer1_clksel_proxy(&self) -> &Cfg0Timer1ClkselProxy {
        &self.cfg0_timer1_clksel_proxy
    }
    #[doc = "0xa1b8 - CFG0_TIMER2_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer2_clksel_proxy(&self) -> &Cfg0Timer2ClkselProxy {
        &self.cfg0_timer2_clksel_proxy
    }
    #[doc = "0xa1bc - CFG0_TIMER3_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer3_clksel_proxy(&self) -> &Cfg0Timer3ClkselProxy {
        &self.cfg0_timer3_clksel_proxy
    }
    #[doc = "0xa1c0 - CFG0_TIMER4_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer4_clksel_proxy(&self) -> &Cfg0Timer4ClkselProxy {
        &self.cfg0_timer4_clksel_proxy
    }
    #[doc = "0xa1c4 - CFG0_TIMER5_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer5_clksel_proxy(&self) -> &Cfg0Timer5ClkselProxy {
        &self.cfg0_timer5_clksel_proxy
    }
    #[doc = "0xa1c8 - CFG0_TIMER6_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer6_clksel_proxy(&self) -> &Cfg0Timer6ClkselProxy {
        &self.cfg0_timer6_clksel_proxy
    }
    #[doc = "0xa1cc - CFG0_TIMER7_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer7_clksel_proxy(&self) -> &Cfg0Timer7ClkselProxy {
        &self.cfg0_timer7_clksel_proxy
    }
    #[doc = "0xa1d0 - CFG0_TIMER8_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer8_clksel_proxy(&self) -> &Cfg0Timer8ClkselProxy {
        &self.cfg0_timer8_clksel_proxy
    }
    #[doc = "0xa1d4 - CFG0_TIMER9_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer9_clksel_proxy(&self) -> &Cfg0Timer9ClkselProxy {
        &self.cfg0_timer9_clksel_proxy
    }
    #[doc = "0xa1d8 - CFG0_TIMER10_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer10_clksel_proxy(&self) -> &Cfg0Timer10ClkselProxy {
        &self.cfg0_timer10_clksel_proxy
    }
    #[doc = "0xa1dc - CFG0_TIMER11_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_timer11_clksel_proxy(&self) -> &Cfg0Timer11ClkselProxy {
        &self.cfg0_timer11_clksel_proxy
    }
    #[doc = "0xa200 - CFG0_SPI0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_spi0_clksel_proxy(&self) -> &Cfg0Spi0ClkselProxy {
        &self.cfg0_spi0_clksel_proxy
    }
    #[doc = "0xa204 - CFG0_SPI1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_spi1_clksel_proxy(&self) -> &Cfg0Spi1ClkselProxy {
        &self.cfg0_spi1_clksel_proxy
    }
    #[doc = "0xa208 - CFG0_SPI2_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_spi2_clksel_proxy(&self) -> &Cfg0Spi2ClkselProxy {
        &self.cfg0_spi2_clksel_proxy
    }
    #[doc = "0xa20c - CFG0_SPI3_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_spi3_clksel_proxy(&self) -> &Cfg0Spi3ClkselProxy {
        &self.cfg0_spi3_clksel_proxy
    }
    #[doc = "0xa210 - CFG0_SPI4_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_spi4_clksel_proxy(&self) -> &Cfg0Spi4ClkselProxy {
        &self.cfg0_spi4_clksel_proxy
    }
    #[doc = "0xa240 - CFG0_USART0_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart0_clk_ctrl_proxy(&self) -> &Cfg0Usart0ClkCtrlProxy {
        &self.cfg0_usart0_clk_ctrl_proxy
    }
    #[doc = "0xa244 - CFG0_USART1_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart1_clk_ctrl_proxy(&self) -> &Cfg0Usart1ClkCtrlProxy {
        &self.cfg0_usart1_clk_ctrl_proxy
    }
    #[doc = "0xa248 - CFG0_USART2_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart2_clk_ctrl_proxy(&self) -> &Cfg0Usart2ClkCtrlProxy {
        &self.cfg0_usart2_clk_ctrl_proxy
    }
    #[doc = "0xa24c - CFG0_USART3_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart3_clk_ctrl_proxy(&self) -> &Cfg0Usart3ClkCtrlProxy {
        &self.cfg0_usart3_clk_ctrl_proxy
    }
    #[doc = "0xa250 - CFG0_USART4_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart4_clk_ctrl_proxy(&self) -> &Cfg0Usart4ClkCtrlProxy {
        &self.cfg0_usart4_clk_ctrl_proxy
    }
    #[doc = "0xa254 - CFG0_USART5_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart5_clk_ctrl_proxy(&self) -> &Cfg0Usart5ClkCtrlProxy {
        &self.cfg0_usart5_clk_ctrl_proxy
    }
    #[doc = "0xa258 - CFG0_USART6_CLK_CTRL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart6_clk_ctrl_proxy(&self) -> &Cfg0Usart6ClkCtrlProxy {
        &self.cfg0_usart6_clk_ctrl_proxy
    }
    #[doc = "0xa280 - CFG0_USART0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart0_clksel_proxy(&self) -> &Cfg0Usart0ClkselProxy {
        &self.cfg0_usart0_clksel_proxy
    }
    #[doc = "0xa284 - CFG0_USART1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart1_clksel_proxy(&self) -> &Cfg0Usart1ClkselProxy {
        &self.cfg0_usart1_clksel_proxy
    }
    #[doc = "0xa288 - CFG0_USART2_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart2_clksel_proxy(&self) -> &Cfg0Usart2ClkselProxy {
        &self.cfg0_usart2_clksel_proxy
    }
    #[doc = "0xa28c - CFG0_USART3_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart3_clksel_proxy(&self) -> &Cfg0Usart3ClkselProxy {
        &self.cfg0_usart3_clksel_proxy
    }
    #[doc = "0xa290 - CFG0_USART4_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart4_clksel_proxy(&self) -> &Cfg0Usart4ClkselProxy {
        &self.cfg0_usart4_clksel_proxy
    }
    #[doc = "0xa294 - CFG0_USART5_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart5_clksel_proxy(&self) -> &Cfg0Usart5ClkselProxy {
        &self.cfg0_usart5_clksel_proxy
    }
    #[doc = "0xa298 - CFG0_USART6_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_usart6_clksel_proxy(&self) -> &Cfg0Usart6ClkselProxy {
        &self.cfg0_usart6_clksel_proxy
    }
    #[doc = "0xa380 - CFG0_WWD0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_wwd0_clksel_proxy(&self) -> &Cfg0Wwd0ClkselProxy {
        &self.cfg0_wwd0_clksel_proxy
    }
    #[doc = "0xa384 - CFG0_WWD1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_wwd1_clksel_proxy(&self) -> &Cfg0Wwd1ClkselProxy {
        &self.cfg0_wwd1_clksel_proxy
    }
    #[doc = "0xa3a0 - CFG0_WWD8_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_wwd8_clksel_proxy(&self) -> &Cfg0Wwd8ClkselProxy {
        &self.cfg0_wwd8_clksel_proxy
    }
    #[doc = "0xa3a4 - CFG0_WWD9_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_wwd9_clksel_proxy(&self) -> &Cfg0Wwd9ClkselProxy {
        &self.cfg0_wwd9_clksel_proxy
    }
    #[doc = "0xa3a8 - CFG0_WWD10_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_wwd10_clksel_proxy(&self) -> &Cfg0Wwd10ClkselProxy {
        &self.cfg0_wwd10_clksel_proxy
    }
    #[doc = "0xa3ac - CFG0_WWD11_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_wwd11_clksel_proxy(&self) -> &Cfg0Wwd11ClkselProxy {
        &self.cfg0_wwd11_clksel_proxy
    }
    #[doc = "0xa400 - CFG0_SERDES0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_serdes0_clksel_proxy(&self) -> &Cfg0Serdes0ClkselProxy {
        &self.cfg0_serdes0_clksel_proxy
    }
    #[doc = "0xa480 - CFG0_MCAN0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcan0_clksel_proxy(&self) -> &Cfg0Mcan0ClkselProxy {
        &self.cfg0_mcan0_clksel_proxy
    }
    #[doc = "0xa484 - CFG0_MCAN1_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_mcan1_clksel_proxy(&self) -> &Cfg0Mcan1ClkselProxy {
        &self.cfg0_mcan1_clksel_proxy
    }
    #[doc = "0xa500 - CFG0_OSPI0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_ospi0_clksel_proxy(&self) -> &Cfg0Ospi0ClkselProxy {
        &self.cfg0_ospi0_clksel_proxy
    }
    #[doc = "0xa510 - CFG0_ADC0_CLKSEL_PROXY"]
    #[inline(always)]
    pub const fn cfg0_adc0_clksel_proxy(&self) -> &Cfg0Adc0ClkselProxy {
        &self.cfg0_adc0_clksel_proxy
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
    #[doc = "0xb108 - CFG0_CLAIMREG_P2_R2"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r2(&self) -> &Cfg0ClaimregP2R2 {
        &self.cfg0_claimreg_p2_r2
    }
    #[doc = "0xb10c - CFG0_CLAIMREG_P2_R3"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r3(&self) -> &Cfg0ClaimregP2R3 {
        &self.cfg0_claimreg_p2_r3
    }
    #[doc = "0xb110 - CFG0_CLAIMREG_P2_R4"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r4(&self) -> &Cfg0ClaimregP2R4 {
        &self.cfg0_claimreg_p2_r4
    }
    #[doc = "0xb114 - CFG0_CLAIMREG_P2_R5"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r5(&self) -> &Cfg0ClaimregP2R5 {
        &self.cfg0_claimreg_p2_r5
    }
    #[doc = "0xb118 - CFG0_CLAIMREG_P2_R6"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r6(&self) -> &Cfg0ClaimregP2R6 {
        &self.cfg0_claimreg_p2_r6
    }
    #[doc = "0xb11c - CFG0_CLAIMREG_P2_R7"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r7(&self) -> &Cfg0ClaimregP2R7 {
        &self.cfg0_claimreg_p2_r7
    }
    #[doc = "0xb120 - CFG0_CLAIMREG_P2_R8"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r8(&self) -> &Cfg0ClaimregP2R8 {
        &self.cfg0_claimreg_p2_r8
    }
    #[doc = "0xb124 - CFG0_CLAIMREG_P2_R9"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r9(&self) -> &Cfg0ClaimregP2R9 {
        &self.cfg0_claimreg_p2_r9
    }
    #[doc = "0xb128 - CFG0_CLAIMREG_P2_R10"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p2_r10(&self) -> &Cfg0ClaimregP2R10 {
        &self.cfg0_claimreg_p2_r10
    }
    #[doc = "0xc320 - CFG0_FUSE_CRC_STAT"]
    #[inline(always)]
    pub const fn cfg0_fuse_crc_stat(&self) -> &Cfg0FuseCrcStat {
        &self.cfg0_fuse_crc_stat
    }
    #[doc = "0xc400 - CFG0_PBIST_EN"]
    #[inline(always)]
    pub const fn cfg0_pbist_en(&self) -> &Cfg0PbistEn {
        &self.cfg0_pbist_en
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
    #[doc = "0xd104 - CFG0_CLAIMREG_P3_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r1_readonly(&self) -> &Cfg0ClaimregP3R1Readonly {
        &self.cfg0_claimreg_p3_r1_readonly
    }
    #[doc = "0xd108 - CFG0_CLAIMREG_P3_R2_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r2_readonly(&self) -> &Cfg0ClaimregP3R2Readonly {
        &self.cfg0_claimreg_p3_r2_readonly
    }
    #[doc = "0xd10c - CFG0_CLAIMREG_P3_R3_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r3_readonly(&self) -> &Cfg0ClaimregP3R3Readonly {
        &self.cfg0_claimreg_p3_r3_readonly
    }
    #[doc = "0xd110 - CFG0_CLAIMREG_P3_R4_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r4_readonly(&self) -> &Cfg0ClaimregP3R4Readonly {
        &self.cfg0_claimreg_p3_r4_readonly
    }
    #[doc = "0xd114 - CFG0_CLAIMREG_P3_R5_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r5_readonly(&self) -> &Cfg0ClaimregP3R5Readonly {
        &self.cfg0_claimreg_p3_r5_readonly
    }
    #[doc = "0xd118 - CFG0_CLAIMREG_P3_R6_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r6_readonly(&self) -> &Cfg0ClaimregP3R6Readonly {
        &self.cfg0_claimreg_p3_r6_readonly
    }
    #[doc = "0xd11c - CFG0_CLAIMREG_P3_R7_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r7_readonly(&self) -> &Cfg0ClaimregP3R7Readonly {
        &self.cfg0_claimreg_p3_r7_readonly
    }
    #[doc = "0xd120 - CFG0_CLAIMREG_P3_R8_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r8_readonly(&self) -> &Cfg0ClaimregP3R8Readonly {
        &self.cfg0_claimreg_p3_r8_readonly
    }
    #[doc = "0xe320 - CFG0_FUSE_CRC_STAT_PROXY"]
    #[inline(always)]
    pub const fn cfg0_fuse_crc_stat_proxy(&self) -> &Cfg0FuseCrcStatProxy {
        &self.cfg0_fuse_crc_stat_proxy
    }
    #[doc = "0xe400 - CFG0_PBIST_EN_PROXY"]
    #[inline(always)]
    pub const fn cfg0_pbist_en_proxy(&self) -> &Cfg0PbistEnProxy {
        &self.cfg0_pbist_en_proxy
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
    #[doc = "0xf104 - CFG0_CLAIMREG_P3_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r1(&self) -> &Cfg0ClaimregP3R1 {
        &self.cfg0_claimreg_p3_r1
    }
    #[doc = "0xf108 - CFG0_CLAIMREG_P3_R2"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r2(&self) -> &Cfg0ClaimregP3R2 {
        &self.cfg0_claimreg_p3_r2
    }
    #[doc = "0xf10c - CFG0_CLAIMREG_P3_R3"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r3(&self) -> &Cfg0ClaimregP3R3 {
        &self.cfg0_claimreg_p3_r3
    }
    #[doc = "0xf110 - CFG0_CLAIMREG_P3_R4"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r4(&self) -> &Cfg0ClaimregP3R4 {
        &self.cfg0_claimreg_p3_r4
    }
    #[doc = "0xf114 - CFG0_CLAIMREG_P3_R5"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r5(&self) -> &Cfg0ClaimregP3R5 {
        &self.cfg0_claimreg_p3_r5
    }
    #[doc = "0xf118 - CFG0_CLAIMREG_P3_R6"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r6(&self) -> &Cfg0ClaimregP3R6 {
        &self.cfg0_claimreg_p3_r6
    }
    #[doc = "0xf11c - CFG0_CLAIMREG_P3_R7"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r7(&self) -> &Cfg0ClaimregP3R7 {
        &self.cfg0_claimreg_p3_r7
    }
    #[doc = "0xf120 - CFG0_CLAIMREG_P3_R8"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p3_r8(&self) -> &Cfg0ClaimregP3R8 {
        &self.cfg0_claimreg_p3_r8
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
    #[doc = "0x14000 - CFG0_CHNG_DDR4_FSP_REQ"]
    #[inline(always)]
    pub const fn cfg0_chng_ddr4_fsp_req(&self) -> &Cfg0ChngDdr4FspReq {
        &self.cfg0_chng_ddr4_fsp_req
    }
    #[doc = "0x14004 - CFG0_CHNG_DDR4_FSP_ACK"]
    #[inline(always)]
    pub const fn cfg0_chng_ddr4_fsp_ack(&self) -> &Cfg0ChngDdr4FspAck {
        &self.cfg0_chng_ddr4_fsp_ack
    }
    #[doc = "0x14080 - CFG0_DDR4_FSP_CLKCHNG_REQ"]
    #[inline(always)]
    pub const fn cfg0_ddr4_fsp_clkchng_req(&self) -> &Cfg0Ddr4FspClkchngReq {
        &self.cfg0_ddr4_fsp_clkchng_req
    }
    #[doc = "0x140c0 - CFG0_DDR4_FSP_CLKCHNG_ACK"]
    #[inline(always)]
    pub const fn cfg0_ddr4_fsp_clkchng_ack(&self) -> &Cfg0Ddr4FspClkchngAck {
        &self.cfg0_ddr4_fsp_clkchng_ack
    }
    #[doc = "0x15008 - CFG0_LOCK5_KICK0"]
    #[inline(always)]
    pub const fn cfg0_lock5_kick0(&self) -> &Cfg0Lock5Kick0 {
        &self.cfg0_lock5_kick0
    }
    #[doc = "0x1500c - CFG0_LOCK5_KICK1"]
    #[inline(always)]
    pub const fn cfg0_lock5_kick1(&self) -> &Cfg0Lock5Kick1 {
        &self.cfg0_lock5_kick1
    }
    #[doc = "0x15100 - CFG0_CLAIMREG_P5_R0_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p5_r0_readonly(&self) -> &Cfg0ClaimregP5R0Readonly {
        &self.cfg0_claimreg_p5_r0_readonly
    }
    #[doc = "0x15104 - CFG0_CLAIMREG_P5_R1_READONLY"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p5_r1_readonly(&self) -> &Cfg0ClaimregP5R1Readonly {
        &self.cfg0_claimreg_p5_r1_readonly
    }
    #[doc = "0x16000 - CFG0_CHNG_DDR4_FSP_REQ_PROXY"]
    #[inline(always)]
    pub const fn cfg0_chng_ddr4_fsp_req_proxy(&self) -> &Cfg0ChngDdr4FspReqProxy {
        &self.cfg0_chng_ddr4_fsp_req_proxy
    }
    #[doc = "0x16004 - CFG0_CHNG_DDR4_FSP_ACK_PROXY"]
    #[inline(always)]
    pub const fn cfg0_chng_ddr4_fsp_ack_proxy(&self) -> &Cfg0ChngDdr4FspAckProxy {
        &self.cfg0_chng_ddr4_fsp_ack_proxy
    }
    #[doc = "0x16080 - CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY"]
    #[inline(always)]
    pub const fn cfg0_ddr4_fsp_clkchng_req_proxy(&self) -> &Cfg0Ddr4FspClkchngReqProxy {
        &self.cfg0_ddr4_fsp_clkchng_req_proxy
    }
    #[doc = "0x160c0 - CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY"]
    #[inline(always)]
    pub const fn cfg0_ddr4_fsp_clkchng_ack_proxy(&self) -> &Cfg0Ddr4FspClkchngAckProxy {
        &self.cfg0_ddr4_fsp_clkchng_ack_proxy
    }
    #[doc = "0x17008 - CFG0_LOCK5_KICK0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock5_kick0_proxy(&self) -> &Cfg0Lock5Kick0Proxy {
        &self.cfg0_lock5_kick0_proxy
    }
    #[doc = "0x1700c - CFG0_LOCK5_KICK1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_lock5_kick1_proxy(&self) -> &Cfg0Lock5Kick1Proxy {
        &self.cfg0_lock5_kick1_proxy
    }
    #[doc = "0x17100 - CFG0_CLAIMREG_P5_R0"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p5_r0(&self) -> &Cfg0ClaimregP5R0 {
        &self.cfg0_claimreg_p5_r0
    }
    #[doc = "0x17104 - CFG0_CLAIMREG_P5_R1"]
    #[inline(always)]
    pub const fn cfg0_claimreg_p5_r1(&self) -> &Cfg0ClaimregP5R1 {
        &self.cfg0_claimreg_p5_r1
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
#[doc = "CFG0_JTAGID (rw) register accessor: CFG0_JTAGID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtagid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtagid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_jtagid`]
module"]
#[doc(alias = "CFG0_JTAGID")]
pub type Cfg0Jtagid = crate::Reg<cfg0_jtagid::Cfg0JtagidSpec>;
#[doc = "CFG0_JTAGID"]
pub mod cfg0_jtagid;
#[doc = "CFG0_JTAG_USER_ID (rw) register accessor: CFG0_JTAG_USER_ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtag_user_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtag_user_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_jtag_user_id`]
module"]
#[doc(alias = "CFG0_JTAG_USER_ID")]
pub type Cfg0JtagUserId = crate::Reg<cfg0_jtag_user_id::Cfg0JtagUserIdSpec>;
#[doc = "CFG0_JTAG_USER_ID"]
pub mod cfg0_jtag_user_id;
#[doc = "CFG0_MAIN_DEVSTAT (rw) register accessor: CFG0_MAIN_DEVSTAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_devstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_devstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_devstat`]
module"]
#[doc(alias = "CFG0_MAIN_DEVSTAT")]
pub type Cfg0MainDevstat = crate::Reg<cfg0_main_devstat::Cfg0MainDevstatSpec>;
#[doc = "CFG0_MAIN_DEVSTAT"]
pub mod cfg0_main_devstat;
#[doc = "CFG0_MAIN_BOOTCFG (rw) register accessor: CFG0_MAIN_BOOTCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_bootcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_bootcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_bootcfg`]
module"]
#[doc(alias = "CFG0_MAIN_BOOTCFG")]
pub type Cfg0MainBootcfg = crate::Reg<cfg0_main_bootcfg::Cfg0MainBootcfgSpec>;
#[doc = "CFG0_MAIN_BOOTCFG"]
pub mod cfg0_main_bootcfg;
#[doc = "CFG0_BOOT_PROGRESS (rw) register accessor: CFG0_BOOT_PROGRESS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_boot_progress::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_boot_progress::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_boot_progress`]
module"]
#[doc(alias = "CFG0_BOOT_PROGRESS")]
pub type Cfg0BootProgress = crate::Reg<cfg0_boot_progress::Cfg0BootProgressSpec>;
#[doc = "CFG0_BOOT_PROGRESS"]
pub mod cfg0_boot_progress;
#[doc = "CFG0_DEVICE_FEATURE0 (rw) register accessor: CFG0_DEVICE_FEATURE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_device_feature0`]
module"]
#[doc(alias = "CFG0_DEVICE_FEATURE0")]
pub type Cfg0DeviceFeature0 = crate::Reg<cfg0_device_feature0::Cfg0DeviceFeature0Spec>;
#[doc = "CFG0_DEVICE_FEATURE0"]
pub mod cfg0_device_feature0;
#[doc = "CFG0_DEVICE_FEATURE2 (rw) register accessor: CFG0_DEVICE_FEATURE2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_device_feature2`]
module"]
#[doc(alias = "CFG0_DEVICE_FEATURE2")]
pub type Cfg0DeviceFeature2 = crate::Reg<cfg0_device_feature2::Cfg0DeviceFeature2Spec>;
#[doc = "CFG0_DEVICE_FEATURE2"]
pub mod cfg0_device_feature2;
#[doc = "CFG0_DEVICE_FEATURE6 (rw) register accessor: CFG0_DEVICE_FEATURE6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_device_feature6`]
module"]
#[doc(alias = "CFG0_DEVICE_FEATURE6")]
pub type Cfg0DeviceFeature6 = crate::Reg<cfg0_device_feature6::Cfg0DeviceFeature6Spec>;
#[doc = "CFG0_DEVICE_FEATURE6"]
pub mod cfg0_device_feature6;
#[doc = "CFG0_MAC_ID0 (rw) register accessor: CFG0_MAC_ID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mac_id0`]
module"]
#[doc(alias = "CFG0_MAC_ID0")]
pub type Cfg0MacId0 = crate::Reg<cfg0_mac_id0::Cfg0MacId0Spec>;
#[doc = "CFG0_MAC_ID0"]
pub mod cfg0_mac_id0;
#[doc = "CFG0_MAC_ID1 (rw) register accessor: CFG0_MAC_ID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mac_id1`]
module"]
#[doc(alias = "CFG0_MAC_ID1")]
pub type Cfg0MacId1 = crate::Reg<cfg0_mac_id1::Cfg0MacId1Spec>;
#[doc = "CFG0_MAC_ID1"]
pub mod cfg0_mac_id1;
#[doc = "CFG0_PCI_DEVICE_ID0 (rw) register accessor: CFG0_PCI_DEVICE_ID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pci_device_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pci_device_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pci_device_id0`]
module"]
#[doc(alias = "CFG0_PCI_DEVICE_ID0")]
pub type Cfg0PciDeviceId0 = crate::Reg<cfg0_pci_device_id0::Cfg0PciDeviceId0Spec>;
#[doc = "CFG0_PCI_DEVICE_ID0"]
pub mod cfg0_pci_device_id0;
#[doc = "CFG0_PCI_DEVICE_ID1 (rw) register accessor: CFG0_PCI_DEVICE_ID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pci_device_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pci_device_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pci_device_id1`]
module"]
#[doc(alias = "CFG0_PCI_DEVICE_ID1")]
pub type Cfg0PciDeviceId1 = crate::Reg<cfg0_pci_device_id1::Cfg0PciDeviceId1Spec>;
#[doc = "CFG0_PCI_DEVICE_ID1"]
pub mod cfg0_pci_device_id1;
#[doc = "CFG0_USB_DEVICE_ID0 (rw) register accessor: CFG0_USB_DEVICE_ID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb_device_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb_device_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb_device_id0`]
module"]
#[doc(alias = "CFG0_USB_DEVICE_ID0")]
pub type Cfg0UsbDeviceId0 = crate::Reg<cfg0_usb_device_id0::Cfg0UsbDeviceId0Spec>;
#[doc = "CFG0_USB_DEVICE_ID0"]
pub mod cfg0_usb_device_id0;
#[doc = "CFG0_USB_DEVICE_ID1 (rw) register accessor: CFG0_USB_DEVICE_ID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb_device_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb_device_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb_device_id1`]
module"]
#[doc(alias = "CFG0_USB_DEVICE_ID1")]
pub type Cfg0UsbDeviceId1 = crate::Reg<cfg0_usb_device_id1::Cfg0UsbDeviceId1Spec>;
#[doc = "CFG0_USB_DEVICE_ID1"]
pub mod cfg0_usb_device_id1;
#[doc = "CFG0_GP_SW0 (rw) register accessor: CFG0_GP_SW0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw0`]
module"]
#[doc(alias = "CFG0_GP_SW0")]
pub type Cfg0GpSw0 = crate::Reg<cfg0_gp_sw0::Cfg0GpSw0Spec>;
#[doc = "CFG0_GP_SW0"]
pub mod cfg0_gp_sw0;
#[doc = "CFG0_GP_SW1 (rw) register accessor: CFG0_GP_SW1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw1`]
module"]
#[doc(alias = "CFG0_GP_SW1")]
pub type Cfg0GpSw1 = crate::Reg<cfg0_gp_sw1::Cfg0GpSw1Spec>;
#[doc = "CFG0_GP_SW1"]
pub mod cfg0_gp_sw1;
#[doc = "CFG0_GP_SW2 (rw) register accessor: CFG0_GP_SW2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw2`]
module"]
#[doc(alias = "CFG0_GP_SW2")]
pub type Cfg0GpSw2 = crate::Reg<cfg0_gp_sw2::Cfg0GpSw2Spec>;
#[doc = "CFG0_GP_SW2"]
pub mod cfg0_gp_sw2;
#[doc = "CFG0_GP_SW3 (rw) register accessor: CFG0_GP_SW3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw3`]
module"]
#[doc(alias = "CFG0_GP_SW3")]
pub type Cfg0GpSw3 = crate::Reg<cfg0_gp_sw3::Cfg0GpSw3Spec>;
#[doc = "CFG0_GP_SW3"]
pub mod cfg0_gp_sw3;
#[doc = "CFG0_CBA_ERR_STAT (rw) register accessor: CFG0_CBA_ERR_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cba_err_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cba_err_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_cba_err_stat`]
module"]
#[doc(alias = "CFG0_CBA_ERR_STAT")]
pub type Cfg0CbaErrStat = crate::Reg<cfg0_cba_err_stat::Cfg0CbaErrStatSpec>;
#[doc = "CFG0_CBA_ERR_STAT"]
pub mod cfg0_cba_err_stat;
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
#[doc = "CFG0_CLAIMREG_P0_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R1_READONLY")]
pub type Cfg0ClaimregP0R1Readonly =
    crate::Reg<cfg0_claimreg_p0_r1_readonly::Cfg0ClaimregP0R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R1_READONLY"]
pub mod cfg0_claimreg_p0_r1_readonly;
#[doc = "CFG0_CLAIMREG_P0_R2_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R2_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r2_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r2_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r2_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R2_READONLY")]
pub type Cfg0ClaimregP0R2Readonly =
    crate::Reg<cfg0_claimreg_p0_r2_readonly::Cfg0ClaimregP0R2ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R2_READONLY"]
pub mod cfg0_claimreg_p0_r2_readonly;
#[doc = "CFG0_CLAIMREG_P0_R3_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R3_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r3_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r3_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r3_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R3_READONLY")]
pub type Cfg0ClaimregP0R3Readonly =
    crate::Reg<cfg0_claimreg_p0_r3_readonly::Cfg0ClaimregP0R3ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R3_READONLY"]
pub mod cfg0_claimreg_p0_r3_readonly;
#[doc = "CFG0_CLAIMREG_P0_R4_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r4_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r4_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r4_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R4_READONLY")]
pub type Cfg0ClaimregP0R4Readonly =
    crate::Reg<cfg0_claimreg_p0_r4_readonly::Cfg0ClaimregP0R4ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R4_READONLY"]
pub mod cfg0_claimreg_p0_r4_readonly;
#[doc = "CFG0_CLAIMREG_P0_R5_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r5_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r5_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r5_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R5_READONLY")]
pub type Cfg0ClaimregP0R5Readonly =
    crate::Reg<cfg0_claimreg_p0_r5_readonly::Cfg0ClaimregP0R5ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R5_READONLY"]
pub mod cfg0_claimreg_p0_r5_readonly;
#[doc = "CFG0_CLAIMREG_P0_R6_READONLY (rw) register accessor: CFG0_CLAIMREG_P0_R6_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r6_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r6_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r6_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R6_READONLY")]
pub type Cfg0ClaimregP0R6Readonly =
    crate::Reg<cfg0_claimreg_p0_r6_readonly::Cfg0ClaimregP0R6ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P0_R6_READONLY"]
pub mod cfg0_claimreg_p0_r6_readonly;
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
#[doc = "CFG0_JTAGID_PROXY (rw) register accessor: CFG0_JTAGID_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtagid_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtagid_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_jtagid_proxy`]
module"]
#[doc(alias = "CFG0_JTAGID_PROXY")]
pub type Cfg0JtagidProxy = crate::Reg<cfg0_jtagid_proxy::Cfg0JtagidProxySpec>;
#[doc = "CFG0_JTAGID_PROXY"]
pub mod cfg0_jtagid_proxy;
#[doc = "CFG0_JTAG_USER_ID_PROXY (rw) register accessor: CFG0_JTAG_USER_ID_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_jtag_user_id_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_jtag_user_id_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_jtag_user_id_proxy`]
module"]
#[doc(alias = "CFG0_JTAG_USER_ID_PROXY")]
pub type Cfg0JtagUserIdProxy = crate::Reg<cfg0_jtag_user_id_proxy::Cfg0JtagUserIdProxySpec>;
#[doc = "CFG0_JTAG_USER_ID_PROXY"]
pub mod cfg0_jtag_user_id_proxy;
#[doc = "CFG0_MAIN_DEVSTAT_PROXY (rw) register accessor: CFG0_MAIN_DEVSTAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_devstat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_devstat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_devstat_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_DEVSTAT_PROXY")]
pub type Cfg0MainDevstatProxy = crate::Reg<cfg0_main_devstat_proxy::Cfg0MainDevstatProxySpec>;
#[doc = "CFG0_MAIN_DEVSTAT_PROXY"]
pub mod cfg0_main_devstat_proxy;
#[doc = "CFG0_MAIN_BOOTCFG_PROXY (rw) register accessor: CFG0_MAIN_BOOTCFG_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_bootcfg_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_bootcfg_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_bootcfg_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_BOOTCFG_PROXY")]
pub type Cfg0MainBootcfgProxy = crate::Reg<cfg0_main_bootcfg_proxy::Cfg0MainBootcfgProxySpec>;
#[doc = "CFG0_MAIN_BOOTCFG_PROXY"]
pub mod cfg0_main_bootcfg_proxy;
#[doc = "CFG0_BOOT_PROGRESS_PROXY (rw) register accessor: CFG0_BOOT_PROGRESS_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_boot_progress_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_boot_progress_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_boot_progress_proxy`]
module"]
#[doc(alias = "CFG0_BOOT_PROGRESS_PROXY")]
pub type Cfg0BootProgressProxy = crate::Reg<cfg0_boot_progress_proxy::Cfg0BootProgressProxySpec>;
#[doc = "CFG0_BOOT_PROGRESS_PROXY"]
pub mod cfg0_boot_progress_proxy;
#[doc = "CFG0_DEVICE_FEATURE0_PROXY (rw) register accessor: CFG0_DEVICE_FEATURE0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_device_feature0_proxy`]
module"]
#[doc(alias = "CFG0_DEVICE_FEATURE0_PROXY")]
pub type Cfg0DeviceFeature0Proxy =
    crate::Reg<cfg0_device_feature0_proxy::Cfg0DeviceFeature0ProxySpec>;
#[doc = "CFG0_DEVICE_FEATURE0_PROXY"]
pub mod cfg0_device_feature0_proxy;
#[doc = "CFG0_DEVICE_FEATURE2_PROXY (rw) register accessor: CFG0_DEVICE_FEATURE2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature2_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature2_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_device_feature2_proxy`]
module"]
#[doc(alias = "CFG0_DEVICE_FEATURE2_PROXY")]
pub type Cfg0DeviceFeature2Proxy =
    crate::Reg<cfg0_device_feature2_proxy::Cfg0DeviceFeature2ProxySpec>;
#[doc = "CFG0_DEVICE_FEATURE2_PROXY"]
pub mod cfg0_device_feature2_proxy;
#[doc = "CFG0_DEVICE_FEATURE6_PROXY (rw) register accessor: CFG0_DEVICE_FEATURE6_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature6_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature6_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_device_feature6_proxy`]
module"]
#[doc(alias = "CFG0_DEVICE_FEATURE6_PROXY")]
pub type Cfg0DeviceFeature6Proxy =
    crate::Reg<cfg0_device_feature6_proxy::Cfg0DeviceFeature6ProxySpec>;
#[doc = "CFG0_DEVICE_FEATURE6_PROXY"]
pub mod cfg0_device_feature6_proxy;
#[doc = "CFG0_MAC_ID0_PROXY (rw) register accessor: CFG0_MAC_ID0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mac_id0_proxy`]
module"]
#[doc(alias = "CFG0_MAC_ID0_PROXY")]
pub type Cfg0MacId0Proxy = crate::Reg<cfg0_mac_id0_proxy::Cfg0MacId0ProxySpec>;
#[doc = "CFG0_MAC_ID0_PROXY"]
pub mod cfg0_mac_id0_proxy;
#[doc = "CFG0_MAC_ID1_PROXY (rw) register accessor: CFG0_MAC_ID1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mac_id1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mac_id1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mac_id1_proxy`]
module"]
#[doc(alias = "CFG0_MAC_ID1_PROXY")]
pub type Cfg0MacId1Proxy = crate::Reg<cfg0_mac_id1_proxy::Cfg0MacId1ProxySpec>;
#[doc = "CFG0_MAC_ID1_PROXY"]
pub mod cfg0_mac_id1_proxy;
#[doc = "CFG0_PCI_DEVICE_ID0_PROXY (rw) register accessor: CFG0_PCI_DEVICE_ID0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pci_device_id0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pci_device_id0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pci_device_id0_proxy`]
module"]
#[doc(alias = "CFG0_PCI_DEVICE_ID0_PROXY")]
pub type Cfg0PciDeviceId0Proxy = crate::Reg<cfg0_pci_device_id0_proxy::Cfg0PciDeviceId0ProxySpec>;
#[doc = "CFG0_PCI_DEVICE_ID0_PROXY"]
pub mod cfg0_pci_device_id0_proxy;
#[doc = "CFG0_PCI_DEVICE_ID1_PROXY (rw) register accessor: CFG0_PCI_DEVICE_ID1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pci_device_id1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pci_device_id1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pci_device_id1_proxy`]
module"]
#[doc(alias = "CFG0_PCI_DEVICE_ID1_PROXY")]
pub type Cfg0PciDeviceId1Proxy = crate::Reg<cfg0_pci_device_id1_proxy::Cfg0PciDeviceId1ProxySpec>;
#[doc = "CFG0_PCI_DEVICE_ID1_PROXY"]
pub mod cfg0_pci_device_id1_proxy;
#[doc = "CFG0_USB_DEVICE_ID0_PROXY (rw) register accessor: CFG0_USB_DEVICE_ID0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb_device_id0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb_device_id0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb_device_id0_proxy`]
module"]
#[doc(alias = "CFG0_USB_DEVICE_ID0_PROXY")]
pub type Cfg0UsbDeviceId0Proxy = crate::Reg<cfg0_usb_device_id0_proxy::Cfg0UsbDeviceId0ProxySpec>;
#[doc = "CFG0_USB_DEVICE_ID0_PROXY"]
pub mod cfg0_usb_device_id0_proxy;
#[doc = "CFG0_USB_DEVICE_ID1_PROXY (rw) register accessor: CFG0_USB_DEVICE_ID1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb_device_id1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb_device_id1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb_device_id1_proxy`]
module"]
#[doc(alias = "CFG0_USB_DEVICE_ID1_PROXY")]
pub type Cfg0UsbDeviceId1Proxy = crate::Reg<cfg0_usb_device_id1_proxy::Cfg0UsbDeviceId1ProxySpec>;
#[doc = "CFG0_USB_DEVICE_ID1_PROXY"]
pub mod cfg0_usb_device_id1_proxy;
#[doc = "CFG0_GP_SW0_PROXY (rw) register accessor: CFG0_GP_SW0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw0_proxy`]
module"]
#[doc(alias = "CFG0_GP_SW0_PROXY")]
pub type Cfg0GpSw0Proxy = crate::Reg<cfg0_gp_sw0_proxy::Cfg0GpSw0ProxySpec>;
#[doc = "CFG0_GP_SW0_PROXY"]
pub mod cfg0_gp_sw0_proxy;
#[doc = "CFG0_GP_SW1_PROXY (rw) register accessor: CFG0_GP_SW1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw1_proxy`]
module"]
#[doc(alias = "CFG0_GP_SW1_PROXY")]
pub type Cfg0GpSw1Proxy = crate::Reg<cfg0_gp_sw1_proxy::Cfg0GpSw1ProxySpec>;
#[doc = "CFG0_GP_SW1_PROXY"]
pub mod cfg0_gp_sw1_proxy;
#[doc = "CFG0_GP_SW2_PROXY (rw) register accessor: CFG0_GP_SW2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw2_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw2_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw2_proxy`]
module"]
#[doc(alias = "CFG0_GP_SW2_PROXY")]
pub type Cfg0GpSw2Proxy = crate::Reg<cfg0_gp_sw2_proxy::Cfg0GpSw2ProxySpec>;
#[doc = "CFG0_GP_SW2_PROXY"]
pub mod cfg0_gp_sw2_proxy;
#[doc = "CFG0_GP_SW3_PROXY (rw) register accessor: CFG0_GP_SW3_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gp_sw3_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gp_sw3_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gp_sw3_proxy`]
module"]
#[doc(alias = "CFG0_GP_SW3_PROXY")]
pub type Cfg0GpSw3Proxy = crate::Reg<cfg0_gp_sw3_proxy::Cfg0GpSw3ProxySpec>;
#[doc = "CFG0_GP_SW3_PROXY"]
pub mod cfg0_gp_sw3_proxy;
#[doc = "CFG0_CBA_ERR_STAT_PROXY (rw) register accessor: CFG0_CBA_ERR_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cba_err_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cba_err_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_cba_err_stat_proxy`]
module"]
#[doc(alias = "CFG0_CBA_ERR_STAT_PROXY")]
pub type Cfg0CbaErrStatProxy = crate::Reg<cfg0_cba_err_stat_proxy::Cfg0CbaErrStatProxySpec>;
#[doc = "CFG0_CBA_ERR_STAT_PROXY"]
pub mod cfg0_cba_err_stat_proxy;
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
#[doc = "CFG0_CLAIMREG_P0_R1 (rw) register accessor: CFG0_CLAIMREG_P0_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R1")]
pub type Cfg0ClaimregP0R1 = crate::Reg<cfg0_claimreg_p0_r1::Cfg0ClaimregP0R1Spec>;
#[doc = "CFG0_CLAIMREG_P0_R1"]
pub mod cfg0_claimreg_p0_r1;
#[doc = "CFG0_CLAIMREG_P0_R2 (rw) register accessor: CFG0_CLAIMREG_P0_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r2`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R2")]
pub type Cfg0ClaimregP0R2 = crate::Reg<cfg0_claimreg_p0_r2::Cfg0ClaimregP0R2Spec>;
#[doc = "CFG0_CLAIMREG_P0_R2"]
pub mod cfg0_claimreg_p0_r2;
#[doc = "CFG0_CLAIMREG_P0_R3 (rw) register accessor: CFG0_CLAIMREG_P0_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r3`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R3")]
pub type Cfg0ClaimregP0R3 = crate::Reg<cfg0_claimreg_p0_r3::Cfg0ClaimregP0R3Spec>;
#[doc = "CFG0_CLAIMREG_P0_R3"]
pub mod cfg0_claimreg_p0_r3;
#[doc = "CFG0_CLAIMREG_P0_R4 (rw) register accessor: CFG0_CLAIMREG_P0_R4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r4`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R4")]
pub type Cfg0ClaimregP0R4 = crate::Reg<cfg0_claimreg_p0_r4::Cfg0ClaimregP0R4Spec>;
#[doc = "CFG0_CLAIMREG_P0_R4"]
pub mod cfg0_claimreg_p0_r4;
#[doc = "CFG0_CLAIMREG_P0_R5 (rw) register accessor: CFG0_CLAIMREG_P0_R5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r5`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R5")]
pub type Cfg0ClaimregP0R5 = crate::Reg<cfg0_claimreg_p0_r5::Cfg0ClaimregP0R5Spec>;
#[doc = "CFG0_CLAIMREG_P0_R5"]
pub mod cfg0_claimreg_p0_r5;
#[doc = "CFG0_CLAIMREG_P0_R6 (rw) register accessor: CFG0_CLAIMREG_P0_R6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p0_r6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p0_r6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p0_r6`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P0_R6")]
pub type Cfg0ClaimregP0R6 = crate::Reg<cfg0_claimreg_p0_r6::Cfg0ClaimregP0R6Spec>;
#[doc = "CFG0_CLAIMREG_P0_R6"]
pub mod cfg0_claimreg_p0_r6;
#[doc = "CFG0_USB0_PHY_CTRL (rw) register accessor: CFG0_USB0_PHY_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_phy_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_phy_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb0_phy_ctrl`]
module"]
#[doc(alias = "CFG0_USB0_PHY_CTRL")]
pub type Cfg0Usb0PhyCtrl = crate::Reg<cfg0_usb0_phy_ctrl::Cfg0Usb0PhyCtrlSpec>;
#[doc = "CFG0_USB0_PHY_CTRL"]
pub mod cfg0_usb0_phy_ctrl;
#[doc = "CFG0_ENET1_CTRL (rw) register accessor: CFG0_ENET1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_enet1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_enet1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_enet1_ctrl`]
module"]
#[doc(alias = "CFG0_ENET1_CTRL")]
pub type Cfg0Enet1Ctrl = crate::Reg<cfg0_enet1_ctrl::Cfg0Enet1CtrlSpec>;
#[doc = "CFG0_ENET1_CTRL"]
pub mod cfg0_enet1_ctrl;
#[doc = "CFG0_ENET2_CTRL (rw) register accessor: CFG0_ENET2_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_enet2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_enet2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_enet2_ctrl`]
module"]
#[doc(alias = "CFG0_ENET2_CTRL")]
pub type Cfg0Enet2Ctrl = crate::Reg<cfg0_enet2_ctrl::Cfg0Enet2CtrlSpec>;
#[doc = "CFG0_ENET2_CTRL"]
pub mod cfg0_enet2_ctrl;
#[doc = "CFG0_PCIE0_CTRL (rw) register accessor: CFG0_PCIE0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pcie0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pcie0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pcie0_ctrl`]
module"]
#[doc(alias = "CFG0_PCIE0_CTRL")]
pub type Cfg0Pcie0Ctrl = crate::Reg<cfg0_pcie0_ctrl::Cfg0Pcie0CtrlSpec>;
#[doc = "CFG0_PCIE0_CTRL"]
pub mod cfg0_pcie0_ctrl;
#[doc = "CFG0_SERDES0_LN0_CTRL (rw) register accessor: CFG0_SERDES0_LN0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_ln0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_ln0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_serdes0_ln0_ctrl`]
module"]
#[doc(alias = "CFG0_SERDES0_LN0_CTRL")]
pub type Cfg0Serdes0Ln0Ctrl = crate::Reg<cfg0_serdes0_ln0_ctrl::Cfg0Serdes0Ln0CtrlSpec>;
#[doc = "CFG0_SERDES0_LN0_CTRL"]
pub mod cfg0_serdes0_ln0_ctrl;
#[doc = "CFG0_ADC0_TRIM (rw) register accessor: CFG0_ADC0_TRIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_trim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_trim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_adc0_trim`]
module"]
#[doc(alias = "CFG0_ADC0_TRIM")]
pub type Cfg0Adc0Trim = crate::Reg<cfg0_adc0_trim::Cfg0Adc0TrimSpec>;
#[doc = "CFG0_ADC0_TRIM"]
pub mod cfg0_adc0_trim;
#[doc = "CFG0_SERDES0_CTRL (rw) register accessor: CFG0_SERDES0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_serdes0_ctrl`]
module"]
#[doc(alias = "CFG0_SERDES0_CTRL")]
pub type Cfg0Serdes0Ctrl = crate::Reg<cfg0_serdes0_ctrl::Cfg0Serdes0CtrlSpec>;
#[doc = "CFG0_SERDES0_CTRL"]
pub mod cfg0_serdes0_ctrl;
#[doc = "CFG0_ICSSG0_CTRL0 (rw) register accessor: CFG0_ICSSG0_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg0_ctrl0`]
module"]
#[doc(alias = "CFG0_ICSSG0_CTRL0")]
pub type Cfg0Icssg0Ctrl0 = crate::Reg<cfg0_icssg0_ctrl0::Cfg0Icssg0Ctrl0Spec>;
#[doc = "CFG0_ICSSG0_CTRL0"]
pub mod cfg0_icssg0_ctrl0;
#[doc = "CFG0_ICSSG0_CTRL1 (rw) register accessor: CFG0_ICSSG0_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg0_ctrl1`]
module"]
#[doc(alias = "CFG0_ICSSG0_CTRL1")]
pub type Cfg0Icssg0Ctrl1 = crate::Reg<cfg0_icssg0_ctrl1::Cfg0Icssg0Ctrl1Spec>;
#[doc = "CFG0_ICSSG0_CTRL1"]
pub mod cfg0_icssg0_ctrl1;
#[doc = "CFG0_ICSSG1_CTRL0 (rw) register accessor: CFG0_ICSSG1_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg1_ctrl0`]
module"]
#[doc(alias = "CFG0_ICSSG1_CTRL0")]
pub type Cfg0Icssg1Ctrl0 = crate::Reg<cfg0_icssg1_ctrl0::Cfg0Icssg1Ctrl0Spec>;
#[doc = "CFG0_ICSSG1_CTRL0"]
pub mod cfg0_icssg1_ctrl0;
#[doc = "CFG0_ICSSG1_CTRL1 (rw) register accessor: CFG0_ICSSG1_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg1_ctrl1`]
module"]
#[doc(alias = "CFG0_ICSSG1_CTRL1")]
pub type Cfg0Icssg1Ctrl1 = crate::Reg<cfg0_icssg1_ctrl1::Cfg0Icssg1Ctrl1Spec>;
#[doc = "CFG0_ICSSG1_CTRL1"]
pub mod cfg0_icssg1_ctrl1;
#[doc = "CFG0_EPWM_TB_CLKEN (rw) register accessor: CFG0_EPWM_TB_CLKEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm_tb_clken`]
module"]
#[doc(alias = "CFG0_EPWM_TB_CLKEN")]
pub type Cfg0EpwmTbClken = crate::Reg<cfg0_epwm_tb_clken::Cfg0EpwmTbClkenSpec>;
#[doc = "CFG0_EPWM_TB_CLKEN"]
pub mod cfg0_epwm_tb_clken;
#[doc = "CFG0_EPWM_TB_CLKEN_SET (rw) register accessor: CFG0_EPWM_TB_CLKEN_SET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm_tb_clken_set`]
module"]
#[doc(alias = "CFG0_EPWM_TB_CLKEN_SET")]
pub type Cfg0EpwmTbClkenSet = crate::Reg<cfg0_epwm_tb_clken_set::Cfg0EpwmTbClkenSetSpec>;
#[doc = "CFG0_EPWM_TB_CLKEN_SET"]
pub mod cfg0_epwm_tb_clken_set;
#[doc = "CFG0_EPWM_TB_CLKEN_CLR (rw) register accessor: CFG0_EPWM_TB_CLKEN_CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm_tb_clken_clr`]
module"]
#[doc(alias = "CFG0_EPWM_TB_CLKEN_CLR")]
pub type Cfg0EpwmTbClkenClr = crate::Reg<cfg0_epwm_tb_clken_clr::Cfg0EpwmTbClkenClrSpec>;
#[doc = "CFG0_EPWM_TB_CLKEN_CLR"]
pub mod cfg0_epwm_tb_clken_clr;
#[doc = "CFG0_EPWM0_CTRL (rw) register accessor: CFG0_EPWM0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm0_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM0_CTRL")]
pub type Cfg0Epwm0Ctrl = crate::Reg<cfg0_epwm0_ctrl::Cfg0Epwm0CtrlSpec>;
#[doc = "CFG0_EPWM0_CTRL"]
pub mod cfg0_epwm0_ctrl;
#[doc = "CFG0_EPWM1_CTRL (rw) register accessor: CFG0_EPWM1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm1_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM1_CTRL")]
pub type Cfg0Epwm1Ctrl = crate::Reg<cfg0_epwm1_ctrl::Cfg0Epwm1CtrlSpec>;
#[doc = "CFG0_EPWM1_CTRL"]
pub mod cfg0_epwm1_ctrl;
#[doc = "CFG0_EPWM2_CTRL (rw) register accessor: CFG0_EPWM2_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm2_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM2_CTRL")]
pub type Cfg0Epwm2Ctrl = crate::Reg<cfg0_epwm2_ctrl::Cfg0Epwm2CtrlSpec>;
#[doc = "CFG0_EPWM2_CTRL"]
pub mod cfg0_epwm2_ctrl;
#[doc = "CFG0_EPWM3_CTRL (rw) register accessor: CFG0_EPWM3_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm3_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM3_CTRL")]
pub type Cfg0Epwm3Ctrl = crate::Reg<cfg0_epwm3_ctrl::Cfg0Epwm3CtrlSpec>;
#[doc = "CFG0_EPWM3_CTRL"]
pub mod cfg0_epwm3_ctrl;
#[doc = "CFG0_EPWM4_CTRL (rw) register accessor: CFG0_EPWM4_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm4_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM4_CTRL")]
pub type Cfg0Epwm4Ctrl = crate::Reg<cfg0_epwm4_ctrl::Cfg0Epwm4CtrlSpec>;
#[doc = "CFG0_EPWM4_CTRL"]
pub mod cfg0_epwm4_ctrl;
#[doc = "CFG0_EPWM5_CTRL (rw) register accessor: CFG0_EPWM5_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm5_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM5_CTRL")]
pub type Cfg0Epwm5Ctrl = crate::Reg<cfg0_epwm5_ctrl::Cfg0Epwm5CtrlSpec>;
#[doc = "CFG0_EPWM5_CTRL"]
pub mod cfg0_epwm5_ctrl;
#[doc = "CFG0_EPWM6_CTRL (rw) register accessor: CFG0_EPWM6_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm6_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm6_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm6_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM6_CTRL")]
pub type Cfg0Epwm6Ctrl = crate::Reg<cfg0_epwm6_ctrl::Cfg0Epwm6CtrlSpec>;
#[doc = "CFG0_EPWM6_CTRL"]
pub mod cfg0_epwm6_ctrl;
#[doc = "CFG0_EPWM7_CTRL (rw) register accessor: CFG0_EPWM7_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm7_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM7_CTRL")]
pub type Cfg0Epwm7Ctrl = crate::Reg<cfg0_epwm7_ctrl::Cfg0Epwm7CtrlSpec>;
#[doc = "CFG0_EPWM7_CTRL"]
pub mod cfg0_epwm7_ctrl;
#[doc = "CFG0_EPWM8_CTRL (rw) register accessor: CFG0_EPWM8_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm8_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm8_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm8_ctrl`]
module"]
#[doc(alias = "CFG0_EPWM8_CTRL")]
pub type Cfg0Epwm8Ctrl = crate::Reg<cfg0_epwm8_ctrl::Cfg0Epwm8CtrlSpec>;
#[doc = "CFG0_EPWM8_CTRL"]
pub mod cfg0_epwm8_ctrl;
#[doc = "CFG0_SOCA_SEL (rw) register accessor: CFG0_SOCA_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_soca_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_soca_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_soca_sel`]
module"]
#[doc(alias = "CFG0_SOCA_SEL")]
pub type Cfg0SocaSel = crate::Reg<cfg0_soca_sel::Cfg0SocaSelSpec>;
#[doc = "CFG0_SOCA_SEL"]
pub mod cfg0_soca_sel;
#[doc = "CFG0_SOCB_SEL (rw) register accessor: CFG0_SOCB_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_socb_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_socb_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_socb_sel`]
module"]
#[doc(alias = "CFG0_SOCB_SEL")]
pub type Cfg0SocbSel = crate::Reg<cfg0_socb_sel::Cfg0SocbSelSpec>;
#[doc = "CFG0_SOCB_SEL"]
pub mod cfg0_socb_sel;
#[doc = "CFG0_EQEP0_CTRL (rw) register accessor: CFG0_EQEP0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep0_ctrl`]
module"]
#[doc(alias = "CFG0_EQEP0_CTRL")]
pub type Cfg0Eqep0Ctrl = crate::Reg<cfg0_eqep0_ctrl::Cfg0Eqep0CtrlSpec>;
#[doc = "CFG0_EQEP0_CTRL"]
pub mod cfg0_eqep0_ctrl;
#[doc = "CFG0_EQEP1_CTRL (rw) register accessor: CFG0_EQEP1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep1_ctrl`]
module"]
#[doc(alias = "CFG0_EQEP1_CTRL")]
pub type Cfg0Eqep1Ctrl = crate::Reg<cfg0_eqep1_ctrl::Cfg0Eqep1CtrlSpec>;
#[doc = "CFG0_EQEP1_CTRL"]
pub mod cfg0_eqep1_ctrl;
#[doc = "CFG0_EQEP2_CTRL (rw) register accessor: CFG0_EQEP2_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep2_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep2_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep2_ctrl`]
module"]
#[doc(alias = "CFG0_EQEP2_CTRL")]
pub type Cfg0Eqep2Ctrl = crate::Reg<cfg0_eqep2_ctrl::Cfg0Eqep2CtrlSpec>;
#[doc = "CFG0_EQEP2_CTRL"]
pub mod cfg0_eqep2_ctrl;
#[doc = "CFG0_EQEP_STAT (rw) register accessor: CFG0_EQEP_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep_stat`]
module"]
#[doc(alias = "CFG0_EQEP_STAT")]
pub type Cfg0EqepStat = crate::Reg<cfg0_eqep_stat::Cfg0EqepStatSpec>;
#[doc = "CFG0_EQEP_STAT"]
pub mod cfg0_eqep_stat;
#[doc = "CFG0_SDIO1_CTRL (rw) register accessor: CFG0_SDIO1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_sdio1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_sdio1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_sdio1_ctrl`]
module"]
#[doc(alias = "CFG0_SDIO1_CTRL")]
pub type Cfg0Sdio1Ctrl = crate::Reg<cfg0_sdio1_ctrl::Cfg0Sdio1CtrlSpec>;
#[doc = "CFG0_SDIO1_CTRL"]
pub mod cfg0_sdio1_ctrl;
#[doc = "CFG0_TIMER1_CTRL (rw) register accessor: CFG0_TIMER1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer1_ctrl`]
module"]
#[doc(alias = "CFG0_TIMER1_CTRL")]
pub type Cfg0Timer1Ctrl = crate::Reg<cfg0_timer1_ctrl::Cfg0Timer1CtrlSpec>;
#[doc = "CFG0_TIMER1_CTRL"]
pub mod cfg0_timer1_ctrl;
#[doc = "CFG0_TIMER3_CTRL (rw) register accessor: CFG0_TIMER3_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer3_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer3_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer3_ctrl`]
module"]
#[doc(alias = "CFG0_TIMER3_CTRL")]
pub type Cfg0Timer3Ctrl = crate::Reg<cfg0_timer3_ctrl::Cfg0Timer3CtrlSpec>;
#[doc = "CFG0_TIMER3_CTRL"]
pub mod cfg0_timer3_ctrl;
#[doc = "CFG0_TIMER5_CTRL (rw) register accessor: CFG0_TIMER5_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer5_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer5_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer5_ctrl`]
module"]
#[doc(alias = "CFG0_TIMER5_CTRL")]
pub type Cfg0Timer5Ctrl = crate::Reg<cfg0_timer5_ctrl::Cfg0Timer5CtrlSpec>;
#[doc = "CFG0_TIMER5_CTRL"]
pub mod cfg0_timer5_ctrl;
#[doc = "CFG0_TIMER7_CTRL (rw) register accessor: CFG0_TIMER7_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer7_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer7_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer7_ctrl`]
module"]
#[doc(alias = "CFG0_TIMER7_CTRL")]
pub type Cfg0Timer7Ctrl = crate::Reg<cfg0_timer7_ctrl::Cfg0Timer7CtrlSpec>;
#[doc = "CFG0_TIMER7_CTRL"]
pub mod cfg0_timer7_ctrl;
#[doc = "CFG0_TIMER9_CTRL (rw) register accessor: CFG0_TIMER9_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer9_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer9_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer9_ctrl`]
module"]
#[doc(alias = "CFG0_TIMER9_CTRL")]
pub type Cfg0Timer9Ctrl = crate::Reg<cfg0_timer9_ctrl::Cfg0Timer9CtrlSpec>;
#[doc = "CFG0_TIMER9_CTRL"]
pub mod cfg0_timer9_ctrl;
#[doc = "CFG0_TIMER11_CTRL (rw) register accessor: CFG0_TIMER11_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer11_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer11_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer11_ctrl`]
module"]
#[doc(alias = "CFG0_TIMER11_CTRL")]
pub type Cfg0Timer11Ctrl = crate::Reg<cfg0_timer11_ctrl::Cfg0Timer11CtrlSpec>;
#[doc = "CFG0_TIMER11_CTRL"]
pub mod cfg0_timer11_ctrl;
#[doc = "CFG0_I2C0_CTRL (rw) register accessor: CFG0_I2C0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_i2c0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_i2c0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_i2c0_ctrl`]
module"]
#[doc(alias = "CFG0_I2C0_CTRL")]
pub type Cfg0I2c0Ctrl = crate::Reg<cfg0_i2c0_ctrl::Cfg0I2c0CtrlSpec>;
#[doc = "CFG0_I2C0_CTRL"]
pub mod cfg0_i2c0_ctrl;
#[doc = "CFG0_FSS_CTRL (rw) register accessor: CFG0_FSS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fss_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fss_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fss_ctrl`]
module"]
#[doc(alias = "CFG0_FSS_CTRL")]
pub type Cfg0FssCtrl = crate::Reg<cfg0_fss_ctrl::Cfg0FssCtrlSpec>;
#[doc = "CFG0_FSS_CTRL"]
pub mod cfg0_fss_ctrl;
#[doc = "CFG0_ADC0_CTRL (rw) register accessor: CFG0_ADC0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_adc0_ctrl`]
module"]
#[doc(alias = "CFG0_ADC0_CTRL")]
pub type Cfg0Adc0Ctrl = crate::Reg<cfg0_adc0_ctrl::Cfg0Adc0CtrlSpec>;
#[doc = "CFG0_ADC0_CTRL"]
pub mod cfg0_adc0_ctrl;
#[doc = "CFG0_DCC_STAT (rw) register accessor: CFG0_DCC_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dcc_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dcc_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dcc_stat`]
module"]
#[doc(alias = "CFG0_DCC_STAT")]
pub type Cfg0DccStat = crate::Reg<cfg0_dcc_stat::Cfg0DccStatSpec>;
#[doc = "CFG0_DCC_STAT"]
pub mod cfg0_dcc_stat;
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
#[doc = "CFG0_CLAIMREG_P1_R13_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R13_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r13_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r13_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r13_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R13_READONLY")]
pub type Cfg0ClaimregP1R13Readonly =
    crate::Reg<cfg0_claimreg_p1_r13_readonly::Cfg0ClaimregP1R13ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R13_READONLY"]
pub mod cfg0_claimreg_p1_r13_readonly;
#[doc = "CFG0_CLAIMREG_P1_R14_READONLY (rw) register accessor: CFG0_CLAIMREG_P1_R14_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r14_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r14_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r14_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R14_READONLY")]
pub type Cfg0ClaimregP1R14Readonly =
    crate::Reg<cfg0_claimreg_p1_r14_readonly::Cfg0ClaimregP1R14ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P1_R14_READONLY"]
pub mod cfg0_claimreg_p1_r14_readonly;
#[doc = "CFG0_USB0_PHY_CTRL_PROXY (rw) register accessor: CFG0_USB0_PHY_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_phy_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_phy_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb0_phy_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USB0_PHY_CTRL_PROXY")]
pub type Cfg0Usb0PhyCtrlProxy = crate::Reg<cfg0_usb0_phy_ctrl_proxy::Cfg0Usb0PhyCtrlProxySpec>;
#[doc = "CFG0_USB0_PHY_CTRL_PROXY"]
pub mod cfg0_usb0_phy_ctrl_proxy;
#[doc = "CFG0_ENET1_CTRL_PROXY (rw) register accessor: CFG0_ENET1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_enet1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_enet1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_enet1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_ENET1_CTRL_PROXY")]
pub type Cfg0Enet1CtrlProxy = crate::Reg<cfg0_enet1_ctrl_proxy::Cfg0Enet1CtrlProxySpec>;
#[doc = "CFG0_ENET1_CTRL_PROXY"]
pub mod cfg0_enet1_ctrl_proxy;
#[doc = "CFG0_ENET2_CTRL_PROXY (rw) register accessor: CFG0_ENET2_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_enet2_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_enet2_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_enet2_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_ENET2_CTRL_PROXY")]
pub type Cfg0Enet2CtrlProxy = crate::Reg<cfg0_enet2_ctrl_proxy::Cfg0Enet2CtrlProxySpec>;
#[doc = "CFG0_ENET2_CTRL_PROXY"]
pub mod cfg0_enet2_ctrl_proxy;
#[doc = "CFG0_PCIE0_CTRL_PROXY (rw) register accessor: CFG0_PCIE0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pcie0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pcie0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pcie0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_PCIE0_CTRL_PROXY")]
pub type Cfg0Pcie0CtrlProxy = crate::Reg<cfg0_pcie0_ctrl_proxy::Cfg0Pcie0CtrlProxySpec>;
#[doc = "CFG0_PCIE0_CTRL_PROXY"]
pub mod cfg0_pcie0_ctrl_proxy;
#[doc = "CFG0_SERDES0_LN0_CTRL_PROXY (rw) register accessor: CFG0_SERDES0_LN0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_ln0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_ln0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_serdes0_ln0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_SERDES0_LN0_CTRL_PROXY")]
pub type Cfg0Serdes0Ln0CtrlProxy =
    crate::Reg<cfg0_serdes0_ln0_ctrl_proxy::Cfg0Serdes0Ln0CtrlProxySpec>;
#[doc = "CFG0_SERDES0_LN0_CTRL_PROXY"]
pub mod cfg0_serdes0_ln0_ctrl_proxy;
#[doc = "CFG0_ADC0_TRIM_PROXY (rw) register accessor: CFG0_ADC0_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_trim_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_trim_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_adc0_trim_proxy`]
module"]
#[doc(alias = "CFG0_ADC0_TRIM_PROXY")]
pub type Cfg0Adc0TrimProxy = crate::Reg<cfg0_adc0_trim_proxy::Cfg0Adc0TrimProxySpec>;
#[doc = "CFG0_ADC0_TRIM_PROXY"]
pub mod cfg0_adc0_trim_proxy;
#[doc = "CFG0_SERDES0_CTRL_PROXY (rw) register accessor: CFG0_SERDES0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_serdes0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_SERDES0_CTRL_PROXY")]
pub type Cfg0Serdes0CtrlProxy = crate::Reg<cfg0_serdes0_ctrl_proxy::Cfg0Serdes0CtrlProxySpec>;
#[doc = "CFG0_SERDES0_CTRL_PROXY"]
pub mod cfg0_serdes0_ctrl_proxy;
#[doc = "CFG0_ICSSG0_CTRL0_PROXY (rw) register accessor: CFG0_ICSSG0_CTRL0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_ctrl0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_ctrl0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg0_ctrl0_proxy`]
module"]
#[doc(alias = "CFG0_ICSSG0_CTRL0_PROXY")]
pub type Cfg0Icssg0Ctrl0Proxy = crate::Reg<cfg0_icssg0_ctrl0_proxy::Cfg0Icssg0Ctrl0ProxySpec>;
#[doc = "CFG0_ICSSG0_CTRL0_PROXY"]
pub mod cfg0_icssg0_ctrl0_proxy;
#[doc = "CFG0_ICSSG0_CTRL1_PROXY (rw) register accessor: CFG0_ICSSG0_CTRL1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_ctrl1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_ctrl1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg0_ctrl1_proxy`]
module"]
#[doc(alias = "CFG0_ICSSG0_CTRL1_PROXY")]
pub type Cfg0Icssg0Ctrl1Proxy = crate::Reg<cfg0_icssg0_ctrl1_proxy::Cfg0Icssg0Ctrl1ProxySpec>;
#[doc = "CFG0_ICSSG0_CTRL1_PROXY"]
pub mod cfg0_icssg0_ctrl1_proxy;
#[doc = "CFG0_ICSSG1_CTRL0_PROXY (rw) register accessor: CFG0_ICSSG1_CTRL0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_ctrl0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_ctrl0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg1_ctrl0_proxy`]
module"]
#[doc(alias = "CFG0_ICSSG1_CTRL0_PROXY")]
pub type Cfg0Icssg1Ctrl0Proxy = crate::Reg<cfg0_icssg1_ctrl0_proxy::Cfg0Icssg1Ctrl0ProxySpec>;
#[doc = "CFG0_ICSSG1_CTRL0_PROXY"]
pub mod cfg0_icssg1_ctrl0_proxy;
#[doc = "CFG0_ICSSG1_CTRL1_PROXY (rw) register accessor: CFG0_ICSSG1_CTRL1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_ctrl1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_ctrl1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg1_ctrl1_proxy`]
module"]
#[doc(alias = "CFG0_ICSSG1_CTRL1_PROXY")]
pub type Cfg0Icssg1Ctrl1Proxy = crate::Reg<cfg0_icssg1_ctrl1_proxy::Cfg0Icssg1Ctrl1ProxySpec>;
#[doc = "CFG0_ICSSG1_CTRL1_PROXY"]
pub mod cfg0_icssg1_ctrl1_proxy;
#[doc = "CFG0_EPWM_TB_CLKEN_PROXY (rw) register accessor: CFG0_EPWM_TB_CLKEN_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm_tb_clken_proxy`]
module"]
#[doc(alias = "CFG0_EPWM_TB_CLKEN_PROXY")]
pub type Cfg0EpwmTbClkenProxy = crate::Reg<cfg0_epwm_tb_clken_proxy::Cfg0EpwmTbClkenProxySpec>;
#[doc = "CFG0_EPWM_TB_CLKEN_PROXY"]
pub mod cfg0_epwm_tb_clken_proxy;
#[doc = "CFG0_EPWM_TB_CLKEN_SET_PROXY (rw) register accessor: CFG0_EPWM_TB_CLKEN_SET_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_set_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_set_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm_tb_clken_set_proxy`]
module"]
#[doc(alias = "CFG0_EPWM_TB_CLKEN_SET_PROXY")]
pub type Cfg0EpwmTbClkenSetProxy =
    crate::Reg<cfg0_epwm_tb_clken_set_proxy::Cfg0EpwmTbClkenSetProxySpec>;
#[doc = "CFG0_EPWM_TB_CLKEN_SET_PROXY"]
pub mod cfg0_epwm_tb_clken_set_proxy;
#[doc = "CFG0_EPWM_TB_CLKEN_CLR_PROXY (rw) register accessor: CFG0_EPWM_TB_CLKEN_CLR_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm_tb_clken_clr_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm_tb_clken_clr_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm_tb_clken_clr_proxy`]
module"]
#[doc(alias = "CFG0_EPWM_TB_CLKEN_CLR_PROXY")]
pub type Cfg0EpwmTbClkenClrProxy =
    crate::Reg<cfg0_epwm_tb_clken_clr_proxy::Cfg0EpwmTbClkenClrProxySpec>;
#[doc = "CFG0_EPWM_TB_CLKEN_CLR_PROXY"]
pub mod cfg0_epwm_tb_clken_clr_proxy;
#[doc = "CFG0_EPWM0_CTRL_PROXY (rw) register accessor: CFG0_EPWM0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM0_CTRL_PROXY")]
pub type Cfg0Epwm0CtrlProxy = crate::Reg<cfg0_epwm0_ctrl_proxy::Cfg0Epwm0CtrlProxySpec>;
#[doc = "CFG0_EPWM0_CTRL_PROXY"]
pub mod cfg0_epwm0_ctrl_proxy;
#[doc = "CFG0_EPWM1_CTRL_PROXY (rw) register accessor: CFG0_EPWM1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM1_CTRL_PROXY")]
pub type Cfg0Epwm1CtrlProxy = crate::Reg<cfg0_epwm1_ctrl_proxy::Cfg0Epwm1CtrlProxySpec>;
#[doc = "CFG0_EPWM1_CTRL_PROXY"]
pub mod cfg0_epwm1_ctrl_proxy;
#[doc = "CFG0_EPWM2_CTRL_PROXY (rw) register accessor: CFG0_EPWM2_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm2_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm2_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm2_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM2_CTRL_PROXY")]
pub type Cfg0Epwm2CtrlProxy = crate::Reg<cfg0_epwm2_ctrl_proxy::Cfg0Epwm2CtrlProxySpec>;
#[doc = "CFG0_EPWM2_CTRL_PROXY"]
pub mod cfg0_epwm2_ctrl_proxy;
#[doc = "CFG0_EPWM3_CTRL_PROXY (rw) register accessor: CFG0_EPWM3_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm3_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm3_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm3_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM3_CTRL_PROXY")]
pub type Cfg0Epwm3CtrlProxy = crate::Reg<cfg0_epwm3_ctrl_proxy::Cfg0Epwm3CtrlProxySpec>;
#[doc = "CFG0_EPWM3_CTRL_PROXY"]
pub mod cfg0_epwm3_ctrl_proxy;
#[doc = "CFG0_EPWM4_CTRL_PROXY (rw) register accessor: CFG0_EPWM4_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm4_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm4_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm4_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM4_CTRL_PROXY")]
pub type Cfg0Epwm4CtrlProxy = crate::Reg<cfg0_epwm4_ctrl_proxy::Cfg0Epwm4CtrlProxySpec>;
#[doc = "CFG0_EPWM4_CTRL_PROXY"]
pub mod cfg0_epwm4_ctrl_proxy;
#[doc = "CFG0_EPWM5_CTRL_PROXY (rw) register accessor: CFG0_EPWM5_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm5_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm5_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm5_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM5_CTRL_PROXY")]
pub type Cfg0Epwm5CtrlProxy = crate::Reg<cfg0_epwm5_ctrl_proxy::Cfg0Epwm5CtrlProxySpec>;
#[doc = "CFG0_EPWM5_CTRL_PROXY"]
pub mod cfg0_epwm5_ctrl_proxy;
#[doc = "CFG0_EPWM6_CTRL_PROXY (rw) register accessor: CFG0_EPWM6_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm6_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm6_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm6_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM6_CTRL_PROXY")]
pub type Cfg0Epwm6CtrlProxy = crate::Reg<cfg0_epwm6_ctrl_proxy::Cfg0Epwm6CtrlProxySpec>;
#[doc = "CFG0_EPWM6_CTRL_PROXY"]
pub mod cfg0_epwm6_ctrl_proxy;
#[doc = "CFG0_EPWM7_CTRL_PROXY (rw) register accessor: CFG0_EPWM7_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm7_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm7_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm7_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM7_CTRL_PROXY")]
pub type Cfg0Epwm7CtrlProxy = crate::Reg<cfg0_epwm7_ctrl_proxy::Cfg0Epwm7CtrlProxySpec>;
#[doc = "CFG0_EPWM7_CTRL_PROXY"]
pub mod cfg0_epwm7_ctrl_proxy;
#[doc = "CFG0_EPWM8_CTRL_PROXY (rw) register accessor: CFG0_EPWM8_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_epwm8_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_epwm8_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_epwm8_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EPWM8_CTRL_PROXY")]
pub type Cfg0Epwm8CtrlProxy = crate::Reg<cfg0_epwm8_ctrl_proxy::Cfg0Epwm8CtrlProxySpec>;
#[doc = "CFG0_EPWM8_CTRL_PROXY"]
pub mod cfg0_epwm8_ctrl_proxy;
#[doc = "CFG0_SOCA_SEL_PROXY (rw) register accessor: CFG0_SOCA_SEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_soca_sel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_soca_sel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_soca_sel_proxy`]
module"]
#[doc(alias = "CFG0_SOCA_SEL_PROXY")]
pub type Cfg0SocaSelProxy = crate::Reg<cfg0_soca_sel_proxy::Cfg0SocaSelProxySpec>;
#[doc = "CFG0_SOCA_SEL_PROXY"]
pub mod cfg0_soca_sel_proxy;
#[doc = "CFG0_SOCB_SEL_PROXY (rw) register accessor: CFG0_SOCB_SEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_socb_sel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_socb_sel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_socb_sel_proxy`]
module"]
#[doc(alias = "CFG0_SOCB_SEL_PROXY")]
pub type Cfg0SocbSelProxy = crate::Reg<cfg0_socb_sel_proxy::Cfg0SocbSelProxySpec>;
#[doc = "CFG0_SOCB_SEL_PROXY"]
pub mod cfg0_socb_sel_proxy;
#[doc = "CFG0_EQEP0_CTRL_PROXY (rw) register accessor: CFG0_EQEP0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EQEP0_CTRL_PROXY")]
pub type Cfg0Eqep0CtrlProxy = crate::Reg<cfg0_eqep0_ctrl_proxy::Cfg0Eqep0CtrlProxySpec>;
#[doc = "CFG0_EQEP0_CTRL_PROXY"]
pub mod cfg0_eqep0_ctrl_proxy;
#[doc = "CFG0_EQEP1_CTRL_PROXY (rw) register accessor: CFG0_EQEP1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EQEP1_CTRL_PROXY")]
pub type Cfg0Eqep1CtrlProxy = crate::Reg<cfg0_eqep1_ctrl_proxy::Cfg0Eqep1CtrlProxySpec>;
#[doc = "CFG0_EQEP1_CTRL_PROXY"]
pub mod cfg0_eqep1_ctrl_proxy;
#[doc = "CFG0_EQEP2_CTRL_PROXY (rw) register accessor: CFG0_EQEP2_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep2_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep2_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep2_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_EQEP2_CTRL_PROXY")]
pub type Cfg0Eqep2CtrlProxy = crate::Reg<cfg0_eqep2_ctrl_proxy::Cfg0Eqep2CtrlProxySpec>;
#[doc = "CFG0_EQEP2_CTRL_PROXY"]
pub mod cfg0_eqep2_ctrl_proxy;
#[doc = "CFG0_EQEP_STAT_PROXY (rw) register accessor: CFG0_EQEP_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_eqep_stat_proxy`]
module"]
#[doc(alias = "CFG0_EQEP_STAT_PROXY")]
pub type Cfg0EqepStatProxy = crate::Reg<cfg0_eqep_stat_proxy::Cfg0EqepStatProxySpec>;
#[doc = "CFG0_EQEP_STAT_PROXY"]
pub mod cfg0_eqep_stat_proxy;
#[doc = "CFG0_SDIO1_CTRL_PROXY (rw) register accessor: CFG0_SDIO1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_sdio1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_sdio1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_sdio1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_SDIO1_CTRL_PROXY")]
pub type Cfg0Sdio1CtrlProxy = crate::Reg<cfg0_sdio1_ctrl_proxy::Cfg0Sdio1CtrlProxySpec>;
#[doc = "CFG0_SDIO1_CTRL_PROXY"]
pub mod cfg0_sdio1_ctrl_proxy;
#[doc = "CFG0_TIMER1_CTRL_PROXY (rw) register accessor: CFG0_TIMER1_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer1_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer1_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer1_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_TIMER1_CTRL_PROXY")]
pub type Cfg0Timer1CtrlProxy = crate::Reg<cfg0_timer1_ctrl_proxy::Cfg0Timer1CtrlProxySpec>;
#[doc = "CFG0_TIMER1_CTRL_PROXY"]
pub mod cfg0_timer1_ctrl_proxy;
#[doc = "CFG0_TIMER3_CTRL_PROXY (rw) register accessor: CFG0_TIMER3_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer3_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer3_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer3_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_TIMER3_CTRL_PROXY")]
pub type Cfg0Timer3CtrlProxy = crate::Reg<cfg0_timer3_ctrl_proxy::Cfg0Timer3CtrlProxySpec>;
#[doc = "CFG0_TIMER3_CTRL_PROXY"]
pub mod cfg0_timer3_ctrl_proxy;
#[doc = "CFG0_TIMER5_CTRL_PROXY (rw) register accessor: CFG0_TIMER5_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer5_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer5_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer5_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_TIMER5_CTRL_PROXY")]
pub type Cfg0Timer5CtrlProxy = crate::Reg<cfg0_timer5_ctrl_proxy::Cfg0Timer5CtrlProxySpec>;
#[doc = "CFG0_TIMER5_CTRL_PROXY"]
pub mod cfg0_timer5_ctrl_proxy;
#[doc = "CFG0_TIMER7_CTRL_PROXY (rw) register accessor: CFG0_TIMER7_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer7_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer7_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer7_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_TIMER7_CTRL_PROXY")]
pub type Cfg0Timer7CtrlProxy = crate::Reg<cfg0_timer7_ctrl_proxy::Cfg0Timer7CtrlProxySpec>;
#[doc = "CFG0_TIMER7_CTRL_PROXY"]
pub mod cfg0_timer7_ctrl_proxy;
#[doc = "CFG0_TIMER9_CTRL_PROXY (rw) register accessor: CFG0_TIMER9_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer9_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer9_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer9_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_TIMER9_CTRL_PROXY")]
pub type Cfg0Timer9CtrlProxy = crate::Reg<cfg0_timer9_ctrl_proxy::Cfg0Timer9CtrlProxySpec>;
#[doc = "CFG0_TIMER9_CTRL_PROXY"]
pub mod cfg0_timer9_ctrl_proxy;
#[doc = "CFG0_TIMER11_CTRL_PROXY (rw) register accessor: CFG0_TIMER11_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer11_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer11_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer11_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_TIMER11_CTRL_PROXY")]
pub type Cfg0Timer11CtrlProxy = crate::Reg<cfg0_timer11_ctrl_proxy::Cfg0Timer11CtrlProxySpec>;
#[doc = "CFG0_TIMER11_CTRL_PROXY"]
pub mod cfg0_timer11_ctrl_proxy;
#[doc = "CFG0_I2C0_CTRL_PROXY (rw) register accessor: CFG0_I2C0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_i2c0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_i2c0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_i2c0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_I2C0_CTRL_PROXY")]
pub type Cfg0I2c0CtrlProxy = crate::Reg<cfg0_i2c0_ctrl_proxy::Cfg0I2c0CtrlProxySpec>;
#[doc = "CFG0_I2C0_CTRL_PROXY"]
pub mod cfg0_i2c0_ctrl_proxy;
#[doc = "CFG0_FSS_CTRL_PROXY (rw) register accessor: CFG0_FSS_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fss_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fss_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fss_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_FSS_CTRL_PROXY")]
pub type Cfg0FssCtrlProxy = crate::Reg<cfg0_fss_ctrl_proxy::Cfg0FssCtrlProxySpec>;
#[doc = "CFG0_FSS_CTRL_PROXY"]
pub mod cfg0_fss_ctrl_proxy;
#[doc = "CFG0_ADC0_CTRL_PROXY (rw) register accessor: CFG0_ADC0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_adc0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_ADC0_CTRL_PROXY")]
pub type Cfg0Adc0CtrlProxy = crate::Reg<cfg0_adc0_ctrl_proxy::Cfg0Adc0CtrlProxySpec>;
#[doc = "CFG0_ADC0_CTRL_PROXY"]
pub mod cfg0_adc0_ctrl_proxy;
#[doc = "CFG0_DCC_STAT_PROXY (rw) register accessor: CFG0_DCC_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dcc_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dcc_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_dcc_stat_proxy`]
module"]
#[doc(alias = "CFG0_DCC_STAT_PROXY")]
pub type Cfg0DccStatProxy = crate::Reg<cfg0_dcc_stat_proxy::Cfg0DccStatProxySpec>;
#[doc = "CFG0_DCC_STAT_PROXY"]
pub mod cfg0_dcc_stat_proxy;
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
#[doc = "CFG0_CLAIMREG_P1_R13 (rw) register accessor: CFG0_CLAIMREG_P1_R13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r13`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R13")]
pub type Cfg0ClaimregP1R13 = crate::Reg<cfg0_claimreg_p1_r13::Cfg0ClaimregP1R13Spec>;
#[doc = "CFG0_CLAIMREG_P1_R13"]
pub mod cfg0_claimreg_p1_r13;
#[doc = "CFG0_CLAIMREG_P1_R14 (rw) register accessor: CFG0_CLAIMREG_P1_R14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p1_r14`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P1_R14")]
pub type Cfg0ClaimregP1R14 = crate::Reg<cfg0_claimreg_p1_r14::Cfg0ClaimregP1R14Spec>;
#[doc = "CFG0_CLAIMREG_P1_R14"]
pub mod cfg0_claimreg_p1_r14;
#[doc = "CFG0_OBSCLK0_CTRL (rw) register accessor: CFG0_OBSCLK0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_obsclk0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_obsclk0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_obsclk0_ctrl`]
module"]
#[doc(alias = "CFG0_OBSCLK0_CTRL")]
pub type Cfg0Obsclk0Ctrl = crate::Reg<cfg0_obsclk0_ctrl::Cfg0Obsclk0CtrlSpec>;
#[doc = "CFG0_OBSCLK0_CTRL"]
pub mod cfg0_obsclk0_ctrl;
#[doc = "CFG0_CLKOUT_CTRL (rw) register accessor: CFG0_CLKOUT_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_clkout_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_clkout_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_clkout_ctrl`]
module"]
#[doc(alias = "CFG0_CLKOUT_CTRL")]
pub type Cfg0ClkoutCtrl = crate::Reg<cfg0_clkout_ctrl::Cfg0ClkoutCtrlSpec>;
#[doc = "CFG0_CLKOUT_CTRL"]
pub mod cfg0_clkout_ctrl;
#[doc = "CFG0_GTC_CLKSEL (rw) register accessor: CFG0_GTC_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gtc_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gtc_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gtc_clksel`]
module"]
#[doc(alias = "CFG0_GTC_CLKSEL")]
pub type Cfg0GtcClksel = crate::Reg<cfg0_gtc_clksel::Cfg0GtcClkselSpec>;
#[doc = "CFG0_GTC_CLKSEL"]
pub mod cfg0_gtc_clksel;
#[doc = "CFG0_EFUSE_CLKSEL (rw) register accessor: CFG0_EFUSE_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_efuse_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_efuse_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_efuse_clksel`]
module"]
#[doc(alias = "CFG0_EFUSE_CLKSEL")]
pub type Cfg0EfuseClksel = crate::Reg<cfg0_efuse_clksel::Cfg0EfuseClkselSpec>;
#[doc = "CFG0_EFUSE_CLKSEL"]
pub mod cfg0_efuse_clksel;
#[doc = "CFG0_ICSSG0_CLKSEL (rw) register accessor: CFG0_ICSSG0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg0_clksel`]
module"]
#[doc(alias = "CFG0_ICSSG0_CLKSEL")]
pub type Cfg0Icssg0Clksel = crate::Reg<cfg0_icssg0_clksel::Cfg0Icssg0ClkselSpec>;
#[doc = "CFG0_ICSSG0_CLKSEL"]
pub mod cfg0_icssg0_clksel;
#[doc = "CFG0_ICSSG1_CLKSEL (rw) register accessor: CFG0_ICSSG1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg1_clksel`]
module"]
#[doc(alias = "CFG0_ICSSG1_CLKSEL")]
pub type Cfg0Icssg1Clksel = crate::Reg<cfg0_icssg1_clksel::Cfg0Icssg1ClkselSpec>;
#[doc = "CFG0_ICSSG1_CLKSEL"]
pub mod cfg0_icssg1_clksel;
#[doc = "CFG0_MAIN_PLL0_CLKSEL (rw) register accessor: CFG0_MAIN_PLL0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll0_clksel`]
module"]
#[doc(alias = "CFG0_MAIN_PLL0_CLKSEL")]
pub type Cfg0MainPll0Clksel = crate::Reg<cfg0_main_pll0_clksel::Cfg0MainPll0ClkselSpec>;
#[doc = "CFG0_MAIN_PLL0_CLKSEL"]
pub mod cfg0_main_pll0_clksel;
#[doc = "CFG0_MAIN_PLL1_CLKSEL (rw) register accessor: CFG0_MAIN_PLL1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll1_clksel`]
module"]
#[doc(alias = "CFG0_MAIN_PLL1_CLKSEL")]
pub type Cfg0MainPll1Clksel = crate::Reg<cfg0_main_pll1_clksel::Cfg0MainPll1ClkselSpec>;
#[doc = "CFG0_MAIN_PLL1_CLKSEL"]
pub mod cfg0_main_pll1_clksel;
#[doc = "CFG0_MAIN_PLL2_CLKSEL (rw) register accessor: CFG0_MAIN_PLL2_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll2_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll2_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll2_clksel`]
module"]
#[doc(alias = "CFG0_MAIN_PLL2_CLKSEL")]
pub type Cfg0MainPll2Clksel = crate::Reg<cfg0_main_pll2_clksel::Cfg0MainPll2ClkselSpec>;
#[doc = "CFG0_MAIN_PLL2_CLKSEL"]
pub mod cfg0_main_pll2_clksel;
#[doc = "CFG0_MAIN_PLL8_CLKSEL (rw) register accessor: CFG0_MAIN_PLL8_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll8_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll8_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll8_clksel`]
module"]
#[doc(alias = "CFG0_MAIN_PLL8_CLKSEL")]
pub type Cfg0MainPll8Clksel = crate::Reg<cfg0_main_pll8_clksel::Cfg0MainPll8ClkselSpec>;
#[doc = "CFG0_MAIN_PLL8_CLKSEL"]
pub mod cfg0_main_pll8_clksel;
#[doc = "CFG0_MAIN_PLL12_CLKSEL (rw) register accessor: CFG0_MAIN_PLL12_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll12_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll12_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll12_clksel`]
module"]
#[doc(alias = "CFG0_MAIN_PLL12_CLKSEL")]
pub type Cfg0MainPll12Clksel = crate::Reg<cfg0_main_pll12_clksel::Cfg0MainPll12ClkselSpec>;
#[doc = "CFG0_MAIN_PLL12_CLKSEL"]
pub mod cfg0_main_pll12_clksel;
#[doc = "CFG0_MAIN_PLL14_CLKSEL (rw) register accessor: CFG0_MAIN_PLL14_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll14_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll14_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll14_clksel`]
module"]
#[doc(alias = "CFG0_MAIN_PLL14_CLKSEL")]
pub type Cfg0MainPll14Clksel = crate::Reg<cfg0_main_pll14_clksel::Cfg0MainPll14ClkselSpec>;
#[doc = "CFG0_MAIN_PLL14_CLKSEL"]
pub mod cfg0_main_pll14_clksel;
#[doc = "CFG0_PCIE0_CLKSEL (rw) register accessor: CFG0_PCIE0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pcie0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pcie0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pcie0_clksel`]
module"]
#[doc(alias = "CFG0_PCIE0_CLKSEL")]
pub type Cfg0Pcie0Clksel = crate::Reg<cfg0_pcie0_clksel::Cfg0Pcie0ClkselSpec>;
#[doc = "CFG0_PCIE0_CLKSEL"]
pub mod cfg0_pcie0_clksel;
#[doc = "CFG0_CPSW_CLKSEL (rw) register accessor: CFG0_CPSW_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpsw_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpsw_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_cpsw_clksel`]
module"]
#[doc(alias = "CFG0_CPSW_CLKSEL")]
pub type Cfg0CpswClksel = crate::Reg<cfg0_cpsw_clksel::Cfg0CpswClkselSpec>;
#[doc = "CFG0_CPSW_CLKSEL"]
pub mod cfg0_cpsw_clksel;
#[doc = "CFG0_CPTS_CLKSEL (rw) register accessor: CFG0_CPTS_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpts_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpts_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_cpts_clksel`]
module"]
#[doc(alias = "CFG0_CPTS_CLKSEL")]
pub type Cfg0CptsClksel = crate::Reg<cfg0_cpts_clksel::Cfg0CptsClkselSpec>;
#[doc = "CFG0_CPTS_CLKSEL"]
pub mod cfg0_cpts_clksel;
#[doc = "CFG0_EMMC0_CLKSEL (rw) register accessor: CFG0_EMMC0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_emmc0_clksel`]
module"]
#[doc(alias = "CFG0_EMMC0_CLKSEL")]
pub type Cfg0Emmc0Clksel = crate::Reg<cfg0_emmc0_clksel::Cfg0Emmc0ClkselSpec>;
#[doc = "CFG0_EMMC0_CLKSEL"]
pub mod cfg0_emmc0_clksel;
#[doc = "CFG0_EMMC1_CLKSEL (rw) register accessor: CFG0_EMMC1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_emmc1_clksel`]
module"]
#[doc(alias = "CFG0_EMMC1_CLKSEL")]
pub type Cfg0Emmc1Clksel = crate::Reg<cfg0_emmc1_clksel::Cfg0Emmc1ClkselSpec>;
#[doc = "CFG0_EMMC1_CLKSEL"]
pub mod cfg0_emmc1_clksel;
#[doc = "CFG0_GPMC_CLKSEL (rw) register accessor: CFG0_GPMC_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gpmc_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gpmc_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gpmc_clksel`]
module"]
#[doc(alias = "CFG0_GPMC_CLKSEL")]
pub type Cfg0GpmcClksel = crate::Reg<cfg0_gpmc_clksel::Cfg0GpmcClkselSpec>;
#[doc = "CFG0_GPMC_CLKSEL"]
pub mod cfg0_gpmc_clksel;
#[doc = "CFG0_USB0_CLKSEL (rw) register accessor: CFG0_USB0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb0_clksel`]
module"]
#[doc(alias = "CFG0_USB0_CLKSEL")]
pub type Cfg0Usb0Clksel = crate::Reg<cfg0_usb0_clksel::Cfg0Usb0ClkselSpec>;
#[doc = "CFG0_USB0_CLKSEL"]
pub mod cfg0_usb0_clksel;
#[doc = "CFG0_TIMER0_CLKSEL (rw) register accessor: CFG0_TIMER0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer0_clksel`]
module"]
#[doc(alias = "CFG0_TIMER0_CLKSEL")]
pub type Cfg0Timer0Clksel = crate::Reg<cfg0_timer0_clksel::Cfg0Timer0ClkselSpec>;
#[doc = "CFG0_TIMER0_CLKSEL"]
pub mod cfg0_timer0_clksel;
#[doc = "CFG0_TIMER1_CLKSEL (rw) register accessor: CFG0_TIMER1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer1_clksel`]
module"]
#[doc(alias = "CFG0_TIMER1_CLKSEL")]
pub type Cfg0Timer1Clksel = crate::Reg<cfg0_timer1_clksel::Cfg0Timer1ClkselSpec>;
#[doc = "CFG0_TIMER1_CLKSEL"]
pub mod cfg0_timer1_clksel;
#[doc = "CFG0_TIMER2_CLKSEL (rw) register accessor: CFG0_TIMER2_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer2_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer2_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer2_clksel`]
module"]
#[doc(alias = "CFG0_TIMER2_CLKSEL")]
pub type Cfg0Timer2Clksel = crate::Reg<cfg0_timer2_clksel::Cfg0Timer2ClkselSpec>;
#[doc = "CFG0_TIMER2_CLKSEL"]
pub mod cfg0_timer2_clksel;
#[doc = "CFG0_TIMER3_CLKSEL (rw) register accessor: CFG0_TIMER3_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer3_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer3_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer3_clksel`]
module"]
#[doc(alias = "CFG0_TIMER3_CLKSEL")]
pub type Cfg0Timer3Clksel = crate::Reg<cfg0_timer3_clksel::Cfg0Timer3ClkselSpec>;
#[doc = "CFG0_TIMER3_CLKSEL"]
pub mod cfg0_timer3_clksel;
#[doc = "CFG0_TIMER4_CLKSEL (rw) register accessor: CFG0_TIMER4_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer4_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer4_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer4_clksel`]
module"]
#[doc(alias = "CFG0_TIMER4_CLKSEL")]
pub type Cfg0Timer4Clksel = crate::Reg<cfg0_timer4_clksel::Cfg0Timer4ClkselSpec>;
#[doc = "CFG0_TIMER4_CLKSEL"]
pub mod cfg0_timer4_clksel;
#[doc = "CFG0_TIMER5_CLKSEL (rw) register accessor: CFG0_TIMER5_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer5_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer5_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer5_clksel`]
module"]
#[doc(alias = "CFG0_TIMER5_CLKSEL")]
pub type Cfg0Timer5Clksel = crate::Reg<cfg0_timer5_clksel::Cfg0Timer5ClkselSpec>;
#[doc = "CFG0_TIMER5_CLKSEL"]
pub mod cfg0_timer5_clksel;
#[doc = "CFG0_TIMER6_CLKSEL (rw) register accessor: CFG0_TIMER6_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer6_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer6_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer6_clksel`]
module"]
#[doc(alias = "CFG0_TIMER6_CLKSEL")]
pub type Cfg0Timer6Clksel = crate::Reg<cfg0_timer6_clksel::Cfg0Timer6ClkselSpec>;
#[doc = "CFG0_TIMER6_CLKSEL"]
pub mod cfg0_timer6_clksel;
#[doc = "CFG0_TIMER7_CLKSEL (rw) register accessor: CFG0_TIMER7_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer7_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer7_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer7_clksel`]
module"]
#[doc(alias = "CFG0_TIMER7_CLKSEL")]
pub type Cfg0Timer7Clksel = crate::Reg<cfg0_timer7_clksel::Cfg0Timer7ClkselSpec>;
#[doc = "CFG0_TIMER7_CLKSEL"]
pub mod cfg0_timer7_clksel;
#[doc = "CFG0_TIMER8_CLKSEL (rw) register accessor: CFG0_TIMER8_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer8_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer8_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer8_clksel`]
module"]
#[doc(alias = "CFG0_TIMER8_CLKSEL")]
pub type Cfg0Timer8Clksel = crate::Reg<cfg0_timer8_clksel::Cfg0Timer8ClkselSpec>;
#[doc = "CFG0_TIMER8_CLKSEL"]
pub mod cfg0_timer8_clksel;
#[doc = "CFG0_TIMER9_CLKSEL (rw) register accessor: CFG0_TIMER9_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer9_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer9_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer9_clksel`]
module"]
#[doc(alias = "CFG0_TIMER9_CLKSEL")]
pub type Cfg0Timer9Clksel = crate::Reg<cfg0_timer9_clksel::Cfg0Timer9ClkselSpec>;
#[doc = "CFG0_TIMER9_CLKSEL"]
pub mod cfg0_timer9_clksel;
#[doc = "CFG0_TIMER10_CLKSEL (rw) register accessor: CFG0_TIMER10_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer10_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer10_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer10_clksel`]
module"]
#[doc(alias = "CFG0_TIMER10_CLKSEL")]
pub type Cfg0Timer10Clksel = crate::Reg<cfg0_timer10_clksel::Cfg0Timer10ClkselSpec>;
#[doc = "CFG0_TIMER10_CLKSEL"]
pub mod cfg0_timer10_clksel;
#[doc = "CFG0_TIMER11_CLKSEL (rw) register accessor: CFG0_TIMER11_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer11_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer11_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer11_clksel`]
module"]
#[doc(alias = "CFG0_TIMER11_CLKSEL")]
pub type Cfg0Timer11Clksel = crate::Reg<cfg0_timer11_clksel::Cfg0Timer11ClkselSpec>;
#[doc = "CFG0_TIMER11_CLKSEL"]
pub mod cfg0_timer11_clksel;
#[doc = "CFG0_SPI0_CLKSEL (rw) register accessor: CFG0_SPI0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi0_clksel`]
module"]
#[doc(alias = "CFG0_SPI0_CLKSEL")]
pub type Cfg0Spi0Clksel = crate::Reg<cfg0_spi0_clksel::Cfg0Spi0ClkselSpec>;
#[doc = "CFG0_SPI0_CLKSEL"]
pub mod cfg0_spi0_clksel;
#[doc = "CFG0_SPI1_CLKSEL (rw) register accessor: CFG0_SPI1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi1_clksel`]
module"]
#[doc(alias = "CFG0_SPI1_CLKSEL")]
pub type Cfg0Spi1Clksel = crate::Reg<cfg0_spi1_clksel::Cfg0Spi1ClkselSpec>;
#[doc = "CFG0_SPI1_CLKSEL"]
pub mod cfg0_spi1_clksel;
#[doc = "CFG0_SPI2_CLKSEL (rw) register accessor: CFG0_SPI2_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi2_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi2_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi2_clksel`]
module"]
#[doc(alias = "CFG0_SPI2_CLKSEL")]
pub type Cfg0Spi2Clksel = crate::Reg<cfg0_spi2_clksel::Cfg0Spi2ClkselSpec>;
#[doc = "CFG0_SPI2_CLKSEL"]
pub mod cfg0_spi2_clksel;
#[doc = "CFG0_SPI3_CLKSEL (rw) register accessor: CFG0_SPI3_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi3_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi3_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi3_clksel`]
module"]
#[doc(alias = "CFG0_SPI3_CLKSEL")]
pub type Cfg0Spi3Clksel = crate::Reg<cfg0_spi3_clksel::Cfg0Spi3ClkselSpec>;
#[doc = "CFG0_SPI3_CLKSEL"]
pub mod cfg0_spi3_clksel;
#[doc = "CFG0_SPI4_CLKSEL (rw) register accessor: CFG0_SPI4_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi4_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi4_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi4_clksel`]
module"]
#[doc(alias = "CFG0_SPI4_CLKSEL")]
pub type Cfg0Spi4Clksel = crate::Reg<cfg0_spi4_clksel::Cfg0Spi4ClkselSpec>;
#[doc = "CFG0_SPI4_CLKSEL"]
pub mod cfg0_spi4_clksel;
#[doc = "CFG0_USART0_CLK_CTRL (rw) register accessor: CFG0_USART0_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart0_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart0_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart0_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART0_CLK_CTRL")]
pub type Cfg0Usart0ClkCtrl = crate::Reg<cfg0_usart0_clk_ctrl::Cfg0Usart0ClkCtrlSpec>;
#[doc = "CFG0_USART0_CLK_CTRL"]
pub mod cfg0_usart0_clk_ctrl;
#[doc = "CFG0_USART1_CLK_CTRL (rw) register accessor: CFG0_USART1_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart1_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart1_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart1_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART1_CLK_CTRL")]
pub type Cfg0Usart1ClkCtrl = crate::Reg<cfg0_usart1_clk_ctrl::Cfg0Usart1ClkCtrlSpec>;
#[doc = "CFG0_USART1_CLK_CTRL"]
pub mod cfg0_usart1_clk_ctrl;
#[doc = "CFG0_USART2_CLK_CTRL (rw) register accessor: CFG0_USART2_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart2_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart2_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart2_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART2_CLK_CTRL")]
pub type Cfg0Usart2ClkCtrl = crate::Reg<cfg0_usart2_clk_ctrl::Cfg0Usart2ClkCtrlSpec>;
#[doc = "CFG0_USART2_CLK_CTRL"]
pub mod cfg0_usart2_clk_ctrl;
#[doc = "CFG0_USART3_CLK_CTRL (rw) register accessor: CFG0_USART3_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart3_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart3_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart3_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART3_CLK_CTRL")]
pub type Cfg0Usart3ClkCtrl = crate::Reg<cfg0_usart3_clk_ctrl::Cfg0Usart3ClkCtrlSpec>;
#[doc = "CFG0_USART3_CLK_CTRL"]
pub mod cfg0_usart3_clk_ctrl;
#[doc = "CFG0_USART4_CLK_CTRL (rw) register accessor: CFG0_USART4_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart4_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart4_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart4_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART4_CLK_CTRL")]
pub type Cfg0Usart4ClkCtrl = crate::Reg<cfg0_usart4_clk_ctrl::Cfg0Usart4ClkCtrlSpec>;
#[doc = "CFG0_USART4_CLK_CTRL"]
pub mod cfg0_usart4_clk_ctrl;
#[doc = "CFG0_USART5_CLK_CTRL (rw) register accessor: CFG0_USART5_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart5_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart5_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart5_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART5_CLK_CTRL")]
pub type Cfg0Usart5ClkCtrl = crate::Reg<cfg0_usart5_clk_ctrl::Cfg0Usart5ClkCtrlSpec>;
#[doc = "CFG0_USART5_CLK_CTRL"]
pub mod cfg0_usart5_clk_ctrl;
#[doc = "CFG0_USART6_CLK_CTRL (rw) register accessor: CFG0_USART6_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart6_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart6_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart6_clk_ctrl`]
module"]
#[doc(alias = "CFG0_USART6_CLK_CTRL")]
pub type Cfg0Usart6ClkCtrl = crate::Reg<cfg0_usart6_clk_ctrl::Cfg0Usart6ClkCtrlSpec>;
#[doc = "CFG0_USART6_CLK_CTRL"]
pub mod cfg0_usart6_clk_ctrl;
#[doc = "CFG0_USART0_CLKSEL (rw) register accessor: CFG0_USART0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart0_clksel`]
module"]
#[doc(alias = "CFG0_USART0_CLKSEL")]
pub type Cfg0Usart0Clksel = crate::Reg<cfg0_usart0_clksel::Cfg0Usart0ClkselSpec>;
#[doc = "CFG0_USART0_CLKSEL"]
pub mod cfg0_usart0_clksel;
#[doc = "CFG0_USART1_CLKSEL (rw) register accessor: CFG0_USART1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart1_clksel`]
module"]
#[doc(alias = "CFG0_USART1_CLKSEL")]
pub type Cfg0Usart1Clksel = crate::Reg<cfg0_usart1_clksel::Cfg0Usart1ClkselSpec>;
#[doc = "CFG0_USART1_CLKSEL"]
pub mod cfg0_usart1_clksel;
#[doc = "CFG0_USART2_CLKSEL (rw) register accessor: CFG0_USART2_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart2_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart2_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart2_clksel`]
module"]
#[doc(alias = "CFG0_USART2_CLKSEL")]
pub type Cfg0Usart2Clksel = crate::Reg<cfg0_usart2_clksel::Cfg0Usart2ClkselSpec>;
#[doc = "CFG0_USART2_CLKSEL"]
pub mod cfg0_usart2_clksel;
#[doc = "CFG0_USART3_CLKSEL (rw) register accessor: CFG0_USART3_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart3_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart3_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart3_clksel`]
module"]
#[doc(alias = "CFG0_USART3_CLKSEL")]
pub type Cfg0Usart3Clksel = crate::Reg<cfg0_usart3_clksel::Cfg0Usart3ClkselSpec>;
#[doc = "CFG0_USART3_CLKSEL"]
pub mod cfg0_usart3_clksel;
#[doc = "CFG0_USART4_CLKSEL (rw) register accessor: CFG0_USART4_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart4_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart4_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart4_clksel`]
module"]
#[doc(alias = "CFG0_USART4_CLKSEL")]
pub type Cfg0Usart4Clksel = crate::Reg<cfg0_usart4_clksel::Cfg0Usart4ClkselSpec>;
#[doc = "CFG0_USART4_CLKSEL"]
pub mod cfg0_usart4_clksel;
#[doc = "CFG0_USART5_CLKSEL (rw) register accessor: CFG0_USART5_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart5_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart5_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart5_clksel`]
module"]
#[doc(alias = "CFG0_USART5_CLKSEL")]
pub type Cfg0Usart5Clksel = crate::Reg<cfg0_usart5_clksel::Cfg0Usart5ClkselSpec>;
#[doc = "CFG0_USART5_CLKSEL"]
pub mod cfg0_usart5_clksel;
#[doc = "CFG0_USART6_CLKSEL (rw) register accessor: CFG0_USART6_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart6_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart6_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart6_clksel`]
module"]
#[doc(alias = "CFG0_USART6_CLKSEL")]
pub type Cfg0Usart6Clksel = crate::Reg<cfg0_usart6_clksel::Cfg0Usart6ClkselSpec>;
#[doc = "CFG0_USART6_CLKSEL"]
pub mod cfg0_usart6_clksel;
#[doc = "CFG0_WWD0_CLKSEL (rw) register accessor: CFG0_WWD0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd0_clksel`]
module"]
#[doc(alias = "CFG0_WWD0_CLKSEL")]
pub type Cfg0Wwd0Clksel = crate::Reg<cfg0_wwd0_clksel::Cfg0Wwd0ClkselSpec>;
#[doc = "CFG0_WWD0_CLKSEL"]
pub mod cfg0_wwd0_clksel;
#[doc = "CFG0_WWD1_CLKSEL (rw) register accessor: CFG0_WWD1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd1_clksel`]
module"]
#[doc(alias = "CFG0_WWD1_CLKSEL")]
pub type Cfg0Wwd1Clksel = crate::Reg<cfg0_wwd1_clksel::Cfg0Wwd1ClkselSpec>;
#[doc = "CFG0_WWD1_CLKSEL"]
pub mod cfg0_wwd1_clksel;
#[doc = "CFG0_WWD8_CLKSEL (rw) register accessor: CFG0_WWD8_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd8_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd8_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd8_clksel`]
module"]
#[doc(alias = "CFG0_WWD8_CLKSEL")]
pub type Cfg0Wwd8Clksel = crate::Reg<cfg0_wwd8_clksel::Cfg0Wwd8ClkselSpec>;
#[doc = "CFG0_WWD8_CLKSEL"]
pub mod cfg0_wwd8_clksel;
#[doc = "CFG0_WWD9_CLKSEL (rw) register accessor: CFG0_WWD9_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd9_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd9_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd9_clksel`]
module"]
#[doc(alias = "CFG0_WWD9_CLKSEL")]
pub type Cfg0Wwd9Clksel = crate::Reg<cfg0_wwd9_clksel::Cfg0Wwd9ClkselSpec>;
#[doc = "CFG0_WWD9_CLKSEL"]
pub mod cfg0_wwd9_clksel;
#[doc = "CFG0_WWD10_CLKSEL (rw) register accessor: CFG0_WWD10_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd10_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd10_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd10_clksel`]
module"]
#[doc(alias = "CFG0_WWD10_CLKSEL")]
pub type Cfg0Wwd10Clksel = crate::Reg<cfg0_wwd10_clksel::Cfg0Wwd10ClkselSpec>;
#[doc = "CFG0_WWD10_CLKSEL"]
pub mod cfg0_wwd10_clksel;
#[doc = "CFG0_WWD11_CLKSEL (rw) register accessor: CFG0_WWD11_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd11_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd11_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd11_clksel`]
module"]
#[doc(alias = "CFG0_WWD11_CLKSEL")]
pub type Cfg0Wwd11Clksel = crate::Reg<cfg0_wwd11_clksel::Cfg0Wwd11ClkselSpec>;
#[doc = "CFG0_WWD11_CLKSEL"]
pub mod cfg0_wwd11_clksel;
#[doc = "CFG0_SERDES0_CLKSEL (rw) register accessor: CFG0_SERDES0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_serdes0_clksel`]
module"]
#[doc(alias = "CFG0_SERDES0_CLKSEL")]
pub type Cfg0Serdes0Clksel = crate::Reg<cfg0_serdes0_clksel::Cfg0Serdes0ClkselSpec>;
#[doc = "CFG0_SERDES0_CLKSEL"]
pub mod cfg0_serdes0_clksel;
#[doc = "CFG0_MCAN0_CLKSEL (rw) register accessor: CFG0_MCAN0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcan0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcan0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcan0_clksel`]
module"]
#[doc(alias = "CFG0_MCAN0_CLKSEL")]
pub type Cfg0Mcan0Clksel = crate::Reg<cfg0_mcan0_clksel::Cfg0Mcan0ClkselSpec>;
#[doc = "CFG0_MCAN0_CLKSEL"]
pub mod cfg0_mcan0_clksel;
#[doc = "CFG0_MCAN1_CLKSEL (rw) register accessor: CFG0_MCAN1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcan1_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcan1_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcan1_clksel`]
module"]
#[doc(alias = "CFG0_MCAN1_CLKSEL")]
pub type Cfg0Mcan1Clksel = crate::Reg<cfg0_mcan1_clksel::Cfg0Mcan1ClkselSpec>;
#[doc = "CFG0_MCAN1_CLKSEL"]
pub mod cfg0_mcan1_clksel;
#[doc = "CFG0_OSPI0_CLKSEL (rw) register accessor: CFG0_OSPI0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ospi0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ospi0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ospi0_clksel`]
module"]
#[doc(alias = "CFG0_OSPI0_CLKSEL")]
pub type Cfg0Ospi0Clksel = crate::Reg<cfg0_ospi0_clksel::Cfg0Ospi0ClkselSpec>;
#[doc = "CFG0_OSPI0_CLKSEL"]
pub mod cfg0_ospi0_clksel;
#[doc = "CFG0_ADC0_CLKSEL (rw) register accessor: CFG0_ADC0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_adc0_clksel`]
module"]
#[doc(alias = "CFG0_ADC0_CLKSEL")]
pub type Cfg0Adc0Clksel = crate::Reg<cfg0_adc0_clksel::Cfg0Adc0ClkselSpec>;
#[doc = "CFG0_ADC0_CLKSEL"]
pub mod cfg0_adc0_clksel;
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
#[doc = "CFG0_CLAIMREG_P2_R2_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R2_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r2_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r2_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r2_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R2_READONLY")]
pub type Cfg0ClaimregP2R2Readonly =
    crate::Reg<cfg0_claimreg_p2_r2_readonly::Cfg0ClaimregP2R2ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R2_READONLY"]
pub mod cfg0_claimreg_p2_r2_readonly;
#[doc = "CFG0_CLAIMREG_P2_R3_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R3_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r3_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r3_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r3_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R3_READONLY")]
pub type Cfg0ClaimregP2R3Readonly =
    crate::Reg<cfg0_claimreg_p2_r3_readonly::Cfg0ClaimregP2R3ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R3_READONLY"]
pub mod cfg0_claimreg_p2_r3_readonly;
#[doc = "CFG0_CLAIMREG_P2_R4_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r4_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r4_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r4_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R4_READONLY")]
pub type Cfg0ClaimregP2R4Readonly =
    crate::Reg<cfg0_claimreg_p2_r4_readonly::Cfg0ClaimregP2R4ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R4_READONLY"]
pub mod cfg0_claimreg_p2_r4_readonly;
#[doc = "CFG0_CLAIMREG_P2_R5_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r5_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r5_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r5_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R5_READONLY")]
pub type Cfg0ClaimregP2R5Readonly =
    crate::Reg<cfg0_claimreg_p2_r5_readonly::Cfg0ClaimregP2R5ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R5_READONLY"]
pub mod cfg0_claimreg_p2_r5_readonly;
#[doc = "CFG0_CLAIMREG_P2_R6_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R6_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r6_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r6_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r6_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R6_READONLY")]
pub type Cfg0ClaimregP2R6Readonly =
    crate::Reg<cfg0_claimreg_p2_r6_readonly::Cfg0ClaimregP2R6ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R6_READONLY"]
pub mod cfg0_claimreg_p2_r6_readonly;
#[doc = "CFG0_CLAIMREG_P2_R7_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R7_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r7_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r7_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r7_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R7_READONLY")]
pub type Cfg0ClaimregP2R7Readonly =
    crate::Reg<cfg0_claimreg_p2_r7_readonly::Cfg0ClaimregP2R7ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R7_READONLY"]
pub mod cfg0_claimreg_p2_r7_readonly;
#[doc = "CFG0_CLAIMREG_P2_R8_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R8_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r8_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r8_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r8_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R8_READONLY")]
pub type Cfg0ClaimregP2R8Readonly =
    crate::Reg<cfg0_claimreg_p2_r8_readonly::Cfg0ClaimregP2R8ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R8_READONLY"]
pub mod cfg0_claimreg_p2_r8_readonly;
#[doc = "CFG0_CLAIMREG_P2_R9_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R9_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r9_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r9_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r9_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R9_READONLY")]
pub type Cfg0ClaimregP2R9Readonly =
    crate::Reg<cfg0_claimreg_p2_r9_readonly::Cfg0ClaimregP2R9ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R9_READONLY"]
pub mod cfg0_claimreg_p2_r9_readonly;
#[doc = "CFG0_CLAIMREG_P2_R10_READONLY (rw) register accessor: CFG0_CLAIMREG_P2_R10_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r10_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r10_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r10_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R10_READONLY")]
pub type Cfg0ClaimregP2R10Readonly =
    crate::Reg<cfg0_claimreg_p2_r10_readonly::Cfg0ClaimregP2R10ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P2_R10_READONLY"]
pub mod cfg0_claimreg_p2_r10_readonly;
#[doc = "CFG0_OBSCLK0_CTRL_PROXY (rw) register accessor: CFG0_OBSCLK0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_obsclk0_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_obsclk0_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_obsclk0_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_OBSCLK0_CTRL_PROXY")]
pub type Cfg0Obsclk0CtrlProxy = crate::Reg<cfg0_obsclk0_ctrl_proxy::Cfg0Obsclk0CtrlProxySpec>;
#[doc = "CFG0_OBSCLK0_CTRL_PROXY"]
pub mod cfg0_obsclk0_ctrl_proxy;
#[doc = "CFG0_CLKOUT_CTRL_PROXY (rw) register accessor: CFG0_CLKOUT_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_clkout_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_clkout_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_clkout_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_CLKOUT_CTRL_PROXY")]
pub type Cfg0ClkoutCtrlProxy = crate::Reg<cfg0_clkout_ctrl_proxy::Cfg0ClkoutCtrlProxySpec>;
#[doc = "CFG0_CLKOUT_CTRL_PROXY"]
pub mod cfg0_clkout_ctrl_proxy;
#[doc = "CFG0_GTC_CLKSEL_PROXY (rw) register accessor: CFG0_GTC_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gtc_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gtc_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gtc_clksel_proxy`]
module"]
#[doc(alias = "CFG0_GTC_CLKSEL_PROXY")]
pub type Cfg0GtcClkselProxy = crate::Reg<cfg0_gtc_clksel_proxy::Cfg0GtcClkselProxySpec>;
#[doc = "CFG0_GTC_CLKSEL_PROXY"]
pub mod cfg0_gtc_clksel_proxy;
#[doc = "CFG0_EFUSE_CLKSEL_PROXY (rw) register accessor: CFG0_EFUSE_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_efuse_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_efuse_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_efuse_clksel_proxy`]
module"]
#[doc(alias = "CFG0_EFUSE_CLKSEL_PROXY")]
pub type Cfg0EfuseClkselProxy = crate::Reg<cfg0_efuse_clksel_proxy::Cfg0EfuseClkselProxySpec>;
#[doc = "CFG0_EFUSE_CLKSEL_PROXY"]
pub mod cfg0_efuse_clksel_proxy;
#[doc = "CFG0_ICSSG0_CLKSEL_PROXY (rw) register accessor: CFG0_ICSSG0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_ICSSG0_CLKSEL_PROXY")]
pub type Cfg0Icssg0ClkselProxy = crate::Reg<cfg0_icssg0_clksel_proxy::Cfg0Icssg0ClkselProxySpec>;
#[doc = "CFG0_ICSSG0_CLKSEL_PROXY"]
pub mod cfg0_icssg0_clksel_proxy;
#[doc = "CFG0_ICSSG1_CLKSEL_PROXY (rw) register accessor: CFG0_ICSSG1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_icssg1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_ICSSG1_CLKSEL_PROXY")]
pub type Cfg0Icssg1ClkselProxy = crate::Reg<cfg0_icssg1_clksel_proxy::Cfg0Icssg1ClkselProxySpec>;
#[doc = "CFG0_ICSSG1_CLKSEL_PROXY"]
pub mod cfg0_icssg1_clksel_proxy;
#[doc = "CFG0_MAIN_PLL0_CLKSEL_PROXY (rw) register accessor: CFG0_MAIN_PLL0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_PLL0_CLKSEL_PROXY")]
pub type Cfg0MainPll0ClkselProxy =
    crate::Reg<cfg0_main_pll0_clksel_proxy::Cfg0MainPll0ClkselProxySpec>;
#[doc = "CFG0_MAIN_PLL0_CLKSEL_PROXY"]
pub mod cfg0_main_pll0_clksel_proxy;
#[doc = "CFG0_MAIN_PLL1_CLKSEL_PROXY (rw) register accessor: CFG0_MAIN_PLL1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_PLL1_CLKSEL_PROXY")]
pub type Cfg0MainPll1ClkselProxy =
    crate::Reg<cfg0_main_pll1_clksel_proxy::Cfg0MainPll1ClkselProxySpec>;
#[doc = "CFG0_MAIN_PLL1_CLKSEL_PROXY"]
pub mod cfg0_main_pll1_clksel_proxy;
#[doc = "CFG0_MAIN_PLL2_CLKSEL_PROXY (rw) register accessor: CFG0_MAIN_PLL2_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll2_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll2_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll2_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_PLL2_CLKSEL_PROXY")]
pub type Cfg0MainPll2ClkselProxy =
    crate::Reg<cfg0_main_pll2_clksel_proxy::Cfg0MainPll2ClkselProxySpec>;
#[doc = "CFG0_MAIN_PLL2_CLKSEL_PROXY"]
pub mod cfg0_main_pll2_clksel_proxy;
#[doc = "CFG0_MAIN_PLL8_CLKSEL_PROXY (rw) register accessor: CFG0_MAIN_PLL8_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll8_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll8_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll8_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_PLL8_CLKSEL_PROXY")]
pub type Cfg0MainPll8ClkselProxy =
    crate::Reg<cfg0_main_pll8_clksel_proxy::Cfg0MainPll8ClkselProxySpec>;
#[doc = "CFG0_MAIN_PLL8_CLKSEL_PROXY"]
pub mod cfg0_main_pll8_clksel_proxy;
#[doc = "CFG0_MAIN_PLL12_CLKSEL_PROXY (rw) register accessor: CFG0_MAIN_PLL12_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll12_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll12_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll12_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_PLL12_CLKSEL_PROXY")]
pub type Cfg0MainPll12ClkselProxy =
    crate::Reg<cfg0_main_pll12_clksel_proxy::Cfg0MainPll12ClkselProxySpec>;
#[doc = "CFG0_MAIN_PLL12_CLKSEL_PROXY"]
pub mod cfg0_main_pll12_clksel_proxy;
#[doc = "CFG0_MAIN_PLL14_CLKSEL_PROXY (rw) register accessor: CFG0_MAIN_PLL14_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll14_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll14_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_main_pll14_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MAIN_PLL14_CLKSEL_PROXY")]
pub type Cfg0MainPll14ClkselProxy =
    crate::Reg<cfg0_main_pll14_clksel_proxy::Cfg0MainPll14ClkselProxySpec>;
#[doc = "CFG0_MAIN_PLL14_CLKSEL_PROXY"]
pub mod cfg0_main_pll14_clksel_proxy;
#[doc = "CFG0_PCIE0_CLKSEL_PROXY (rw) register accessor: CFG0_PCIE0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pcie0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pcie0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pcie0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_PCIE0_CLKSEL_PROXY")]
pub type Cfg0Pcie0ClkselProxy = crate::Reg<cfg0_pcie0_clksel_proxy::Cfg0Pcie0ClkselProxySpec>;
#[doc = "CFG0_PCIE0_CLKSEL_PROXY"]
pub mod cfg0_pcie0_clksel_proxy;
#[doc = "CFG0_CPSW_CLKSEL_PROXY (rw) register accessor: CFG0_CPSW_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpsw_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpsw_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_cpsw_clksel_proxy`]
module"]
#[doc(alias = "CFG0_CPSW_CLKSEL_PROXY")]
pub type Cfg0CpswClkselProxy = crate::Reg<cfg0_cpsw_clksel_proxy::Cfg0CpswClkselProxySpec>;
#[doc = "CFG0_CPSW_CLKSEL_PROXY"]
pub mod cfg0_cpsw_clksel_proxy;
#[doc = "CFG0_CPTS_CLKSEL_PROXY (rw) register accessor: CFG0_CPTS_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpts_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpts_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_cpts_clksel_proxy`]
module"]
#[doc(alias = "CFG0_CPTS_CLKSEL_PROXY")]
pub type Cfg0CptsClkselProxy = crate::Reg<cfg0_cpts_clksel_proxy::Cfg0CptsClkselProxySpec>;
#[doc = "CFG0_CPTS_CLKSEL_PROXY"]
pub mod cfg0_cpts_clksel_proxy;
#[doc = "CFG0_EMMC0_CLKSEL_PROXY (rw) register accessor: CFG0_EMMC0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_emmc0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_EMMC0_CLKSEL_PROXY")]
pub type Cfg0Emmc0ClkselProxy = crate::Reg<cfg0_emmc0_clksel_proxy::Cfg0Emmc0ClkselProxySpec>;
#[doc = "CFG0_EMMC0_CLKSEL_PROXY"]
pub mod cfg0_emmc0_clksel_proxy;
#[doc = "CFG0_EMMC1_CLKSEL_PROXY (rw) register accessor: CFG0_EMMC1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_emmc1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_EMMC1_CLKSEL_PROXY")]
pub type Cfg0Emmc1ClkselProxy = crate::Reg<cfg0_emmc1_clksel_proxy::Cfg0Emmc1ClkselProxySpec>;
#[doc = "CFG0_EMMC1_CLKSEL_PROXY"]
pub mod cfg0_emmc1_clksel_proxy;
#[doc = "CFG0_GPMC_CLKSEL_PROXY (rw) register accessor: CFG0_GPMC_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gpmc_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gpmc_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_gpmc_clksel_proxy`]
module"]
#[doc(alias = "CFG0_GPMC_CLKSEL_PROXY")]
pub type Cfg0GpmcClkselProxy = crate::Reg<cfg0_gpmc_clksel_proxy::Cfg0GpmcClkselProxySpec>;
#[doc = "CFG0_GPMC_CLKSEL_PROXY"]
pub mod cfg0_gpmc_clksel_proxy;
#[doc = "CFG0_USB0_CLKSEL_PROXY (rw) register accessor: CFG0_USB0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usb0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USB0_CLKSEL_PROXY")]
pub type Cfg0Usb0ClkselProxy = crate::Reg<cfg0_usb0_clksel_proxy::Cfg0Usb0ClkselProxySpec>;
#[doc = "CFG0_USB0_CLKSEL_PROXY"]
pub mod cfg0_usb0_clksel_proxy;
#[doc = "CFG0_TIMER0_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER0_CLKSEL_PROXY")]
pub type Cfg0Timer0ClkselProxy = crate::Reg<cfg0_timer0_clksel_proxy::Cfg0Timer0ClkselProxySpec>;
#[doc = "CFG0_TIMER0_CLKSEL_PROXY"]
pub mod cfg0_timer0_clksel_proxy;
#[doc = "CFG0_TIMER1_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER1_CLKSEL_PROXY")]
pub type Cfg0Timer1ClkselProxy = crate::Reg<cfg0_timer1_clksel_proxy::Cfg0Timer1ClkselProxySpec>;
#[doc = "CFG0_TIMER1_CLKSEL_PROXY"]
pub mod cfg0_timer1_clksel_proxy;
#[doc = "CFG0_TIMER2_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER2_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer2_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer2_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer2_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER2_CLKSEL_PROXY")]
pub type Cfg0Timer2ClkselProxy = crate::Reg<cfg0_timer2_clksel_proxy::Cfg0Timer2ClkselProxySpec>;
#[doc = "CFG0_TIMER2_CLKSEL_PROXY"]
pub mod cfg0_timer2_clksel_proxy;
#[doc = "CFG0_TIMER3_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER3_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer3_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer3_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer3_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER3_CLKSEL_PROXY")]
pub type Cfg0Timer3ClkselProxy = crate::Reg<cfg0_timer3_clksel_proxy::Cfg0Timer3ClkselProxySpec>;
#[doc = "CFG0_TIMER3_CLKSEL_PROXY"]
pub mod cfg0_timer3_clksel_proxy;
#[doc = "CFG0_TIMER4_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER4_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer4_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer4_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer4_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER4_CLKSEL_PROXY")]
pub type Cfg0Timer4ClkselProxy = crate::Reg<cfg0_timer4_clksel_proxy::Cfg0Timer4ClkselProxySpec>;
#[doc = "CFG0_TIMER4_CLKSEL_PROXY"]
pub mod cfg0_timer4_clksel_proxy;
#[doc = "CFG0_TIMER5_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER5_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer5_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer5_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer5_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER5_CLKSEL_PROXY")]
pub type Cfg0Timer5ClkselProxy = crate::Reg<cfg0_timer5_clksel_proxy::Cfg0Timer5ClkselProxySpec>;
#[doc = "CFG0_TIMER5_CLKSEL_PROXY"]
pub mod cfg0_timer5_clksel_proxy;
#[doc = "CFG0_TIMER6_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER6_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer6_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer6_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer6_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER6_CLKSEL_PROXY")]
pub type Cfg0Timer6ClkselProxy = crate::Reg<cfg0_timer6_clksel_proxy::Cfg0Timer6ClkselProxySpec>;
#[doc = "CFG0_TIMER6_CLKSEL_PROXY"]
pub mod cfg0_timer6_clksel_proxy;
#[doc = "CFG0_TIMER7_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER7_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer7_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer7_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer7_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER7_CLKSEL_PROXY")]
pub type Cfg0Timer7ClkselProxy = crate::Reg<cfg0_timer7_clksel_proxy::Cfg0Timer7ClkselProxySpec>;
#[doc = "CFG0_TIMER7_CLKSEL_PROXY"]
pub mod cfg0_timer7_clksel_proxy;
#[doc = "CFG0_TIMER8_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER8_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer8_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer8_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer8_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER8_CLKSEL_PROXY")]
pub type Cfg0Timer8ClkselProxy = crate::Reg<cfg0_timer8_clksel_proxy::Cfg0Timer8ClkselProxySpec>;
#[doc = "CFG0_TIMER8_CLKSEL_PROXY"]
pub mod cfg0_timer8_clksel_proxy;
#[doc = "CFG0_TIMER9_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER9_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer9_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer9_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer9_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER9_CLKSEL_PROXY")]
pub type Cfg0Timer9ClkselProxy = crate::Reg<cfg0_timer9_clksel_proxy::Cfg0Timer9ClkselProxySpec>;
#[doc = "CFG0_TIMER9_CLKSEL_PROXY"]
pub mod cfg0_timer9_clksel_proxy;
#[doc = "CFG0_TIMER10_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER10_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer10_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer10_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer10_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER10_CLKSEL_PROXY")]
pub type Cfg0Timer10ClkselProxy = crate::Reg<cfg0_timer10_clksel_proxy::Cfg0Timer10ClkselProxySpec>;
#[doc = "CFG0_TIMER10_CLKSEL_PROXY"]
pub mod cfg0_timer10_clksel_proxy;
#[doc = "CFG0_TIMER11_CLKSEL_PROXY (rw) register accessor: CFG0_TIMER11_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer11_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer11_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_timer11_clksel_proxy`]
module"]
#[doc(alias = "CFG0_TIMER11_CLKSEL_PROXY")]
pub type Cfg0Timer11ClkselProxy = crate::Reg<cfg0_timer11_clksel_proxy::Cfg0Timer11ClkselProxySpec>;
#[doc = "CFG0_TIMER11_CLKSEL_PROXY"]
pub mod cfg0_timer11_clksel_proxy;
#[doc = "CFG0_SPI0_CLKSEL_PROXY (rw) register accessor: CFG0_SPI0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_SPI0_CLKSEL_PROXY")]
pub type Cfg0Spi0ClkselProxy = crate::Reg<cfg0_spi0_clksel_proxy::Cfg0Spi0ClkselProxySpec>;
#[doc = "CFG0_SPI0_CLKSEL_PROXY"]
pub mod cfg0_spi0_clksel_proxy;
#[doc = "CFG0_SPI1_CLKSEL_PROXY (rw) register accessor: CFG0_SPI1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_SPI1_CLKSEL_PROXY")]
pub type Cfg0Spi1ClkselProxy = crate::Reg<cfg0_spi1_clksel_proxy::Cfg0Spi1ClkselProxySpec>;
#[doc = "CFG0_SPI1_CLKSEL_PROXY"]
pub mod cfg0_spi1_clksel_proxy;
#[doc = "CFG0_SPI2_CLKSEL_PROXY (rw) register accessor: CFG0_SPI2_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi2_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi2_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi2_clksel_proxy`]
module"]
#[doc(alias = "CFG0_SPI2_CLKSEL_PROXY")]
pub type Cfg0Spi2ClkselProxy = crate::Reg<cfg0_spi2_clksel_proxy::Cfg0Spi2ClkselProxySpec>;
#[doc = "CFG0_SPI2_CLKSEL_PROXY"]
pub mod cfg0_spi2_clksel_proxy;
#[doc = "CFG0_SPI3_CLKSEL_PROXY (rw) register accessor: CFG0_SPI3_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi3_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi3_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi3_clksel_proxy`]
module"]
#[doc(alias = "CFG0_SPI3_CLKSEL_PROXY")]
pub type Cfg0Spi3ClkselProxy = crate::Reg<cfg0_spi3_clksel_proxy::Cfg0Spi3ClkselProxySpec>;
#[doc = "CFG0_SPI3_CLKSEL_PROXY"]
pub mod cfg0_spi3_clksel_proxy;
#[doc = "CFG0_SPI4_CLKSEL_PROXY (rw) register accessor: CFG0_SPI4_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi4_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi4_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_spi4_clksel_proxy`]
module"]
#[doc(alias = "CFG0_SPI4_CLKSEL_PROXY")]
pub type Cfg0Spi4ClkselProxy = crate::Reg<cfg0_spi4_clksel_proxy::Cfg0Spi4ClkselProxySpec>;
#[doc = "CFG0_SPI4_CLKSEL_PROXY"]
pub mod cfg0_spi4_clksel_proxy;
#[doc = "CFG0_USART0_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART0_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart0_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart0_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart0_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART0_CLK_CTRL_PROXY")]
pub type Cfg0Usart0ClkCtrlProxy =
    crate::Reg<cfg0_usart0_clk_ctrl_proxy::Cfg0Usart0ClkCtrlProxySpec>;
#[doc = "CFG0_USART0_CLK_CTRL_PROXY"]
pub mod cfg0_usart0_clk_ctrl_proxy;
#[doc = "CFG0_USART1_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART1_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart1_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart1_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart1_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART1_CLK_CTRL_PROXY")]
pub type Cfg0Usart1ClkCtrlProxy =
    crate::Reg<cfg0_usart1_clk_ctrl_proxy::Cfg0Usart1ClkCtrlProxySpec>;
#[doc = "CFG0_USART1_CLK_CTRL_PROXY"]
pub mod cfg0_usart1_clk_ctrl_proxy;
#[doc = "CFG0_USART2_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART2_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart2_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart2_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart2_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART2_CLK_CTRL_PROXY")]
pub type Cfg0Usart2ClkCtrlProxy =
    crate::Reg<cfg0_usart2_clk_ctrl_proxy::Cfg0Usart2ClkCtrlProxySpec>;
#[doc = "CFG0_USART2_CLK_CTRL_PROXY"]
pub mod cfg0_usart2_clk_ctrl_proxy;
#[doc = "CFG0_USART3_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART3_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart3_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart3_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart3_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART3_CLK_CTRL_PROXY")]
pub type Cfg0Usart3ClkCtrlProxy =
    crate::Reg<cfg0_usart3_clk_ctrl_proxy::Cfg0Usart3ClkCtrlProxySpec>;
#[doc = "CFG0_USART3_CLK_CTRL_PROXY"]
pub mod cfg0_usart3_clk_ctrl_proxy;
#[doc = "CFG0_USART4_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART4_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart4_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart4_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart4_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART4_CLK_CTRL_PROXY")]
pub type Cfg0Usart4ClkCtrlProxy =
    crate::Reg<cfg0_usart4_clk_ctrl_proxy::Cfg0Usart4ClkCtrlProxySpec>;
#[doc = "CFG0_USART4_CLK_CTRL_PROXY"]
pub mod cfg0_usart4_clk_ctrl_proxy;
#[doc = "CFG0_USART5_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART5_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart5_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart5_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart5_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART5_CLK_CTRL_PROXY")]
pub type Cfg0Usart5ClkCtrlProxy =
    crate::Reg<cfg0_usart5_clk_ctrl_proxy::Cfg0Usart5ClkCtrlProxySpec>;
#[doc = "CFG0_USART5_CLK_CTRL_PROXY"]
pub mod cfg0_usart5_clk_ctrl_proxy;
#[doc = "CFG0_USART6_CLK_CTRL_PROXY (rw) register accessor: CFG0_USART6_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart6_clk_ctrl_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart6_clk_ctrl_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart6_clk_ctrl_proxy`]
module"]
#[doc(alias = "CFG0_USART6_CLK_CTRL_PROXY")]
pub type Cfg0Usart6ClkCtrlProxy =
    crate::Reg<cfg0_usart6_clk_ctrl_proxy::Cfg0Usart6ClkCtrlProxySpec>;
#[doc = "CFG0_USART6_CLK_CTRL_PROXY"]
pub mod cfg0_usart6_clk_ctrl_proxy;
#[doc = "CFG0_USART0_CLKSEL_PROXY (rw) register accessor: CFG0_USART0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART0_CLKSEL_PROXY")]
pub type Cfg0Usart0ClkselProxy = crate::Reg<cfg0_usart0_clksel_proxy::Cfg0Usart0ClkselProxySpec>;
#[doc = "CFG0_USART0_CLKSEL_PROXY"]
pub mod cfg0_usart0_clksel_proxy;
#[doc = "CFG0_USART1_CLKSEL_PROXY (rw) register accessor: CFG0_USART1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART1_CLKSEL_PROXY")]
pub type Cfg0Usart1ClkselProxy = crate::Reg<cfg0_usart1_clksel_proxy::Cfg0Usart1ClkselProxySpec>;
#[doc = "CFG0_USART1_CLKSEL_PROXY"]
pub mod cfg0_usart1_clksel_proxy;
#[doc = "CFG0_USART2_CLKSEL_PROXY (rw) register accessor: CFG0_USART2_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart2_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart2_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart2_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART2_CLKSEL_PROXY")]
pub type Cfg0Usart2ClkselProxy = crate::Reg<cfg0_usart2_clksel_proxy::Cfg0Usart2ClkselProxySpec>;
#[doc = "CFG0_USART2_CLKSEL_PROXY"]
pub mod cfg0_usart2_clksel_proxy;
#[doc = "CFG0_USART3_CLKSEL_PROXY (rw) register accessor: CFG0_USART3_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart3_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart3_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart3_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART3_CLKSEL_PROXY")]
pub type Cfg0Usart3ClkselProxy = crate::Reg<cfg0_usart3_clksel_proxy::Cfg0Usart3ClkselProxySpec>;
#[doc = "CFG0_USART3_CLKSEL_PROXY"]
pub mod cfg0_usart3_clksel_proxy;
#[doc = "CFG0_USART4_CLKSEL_PROXY (rw) register accessor: CFG0_USART4_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart4_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart4_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart4_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART4_CLKSEL_PROXY")]
pub type Cfg0Usart4ClkselProxy = crate::Reg<cfg0_usart4_clksel_proxy::Cfg0Usart4ClkselProxySpec>;
#[doc = "CFG0_USART4_CLKSEL_PROXY"]
pub mod cfg0_usart4_clksel_proxy;
#[doc = "CFG0_USART5_CLKSEL_PROXY (rw) register accessor: CFG0_USART5_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart5_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart5_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart5_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART5_CLKSEL_PROXY")]
pub type Cfg0Usart5ClkselProxy = crate::Reg<cfg0_usart5_clksel_proxy::Cfg0Usart5ClkselProxySpec>;
#[doc = "CFG0_USART5_CLKSEL_PROXY"]
pub mod cfg0_usart5_clksel_proxy;
#[doc = "CFG0_USART6_CLKSEL_PROXY (rw) register accessor: CFG0_USART6_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart6_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart6_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_usart6_clksel_proxy`]
module"]
#[doc(alias = "CFG0_USART6_CLKSEL_PROXY")]
pub type Cfg0Usart6ClkselProxy = crate::Reg<cfg0_usart6_clksel_proxy::Cfg0Usart6ClkselProxySpec>;
#[doc = "CFG0_USART6_CLKSEL_PROXY"]
pub mod cfg0_usart6_clksel_proxy;
#[doc = "CFG0_WWD0_CLKSEL_PROXY (rw) register accessor: CFG0_WWD0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_WWD0_CLKSEL_PROXY")]
pub type Cfg0Wwd0ClkselProxy = crate::Reg<cfg0_wwd0_clksel_proxy::Cfg0Wwd0ClkselProxySpec>;
#[doc = "CFG0_WWD0_CLKSEL_PROXY"]
pub mod cfg0_wwd0_clksel_proxy;
#[doc = "CFG0_WWD1_CLKSEL_PROXY (rw) register accessor: CFG0_WWD1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_WWD1_CLKSEL_PROXY")]
pub type Cfg0Wwd1ClkselProxy = crate::Reg<cfg0_wwd1_clksel_proxy::Cfg0Wwd1ClkselProxySpec>;
#[doc = "CFG0_WWD1_CLKSEL_PROXY"]
pub mod cfg0_wwd1_clksel_proxy;
#[doc = "CFG0_WWD8_CLKSEL_PROXY (rw) register accessor: CFG0_WWD8_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd8_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd8_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd8_clksel_proxy`]
module"]
#[doc(alias = "CFG0_WWD8_CLKSEL_PROXY")]
pub type Cfg0Wwd8ClkselProxy = crate::Reg<cfg0_wwd8_clksel_proxy::Cfg0Wwd8ClkselProxySpec>;
#[doc = "CFG0_WWD8_CLKSEL_PROXY"]
pub mod cfg0_wwd8_clksel_proxy;
#[doc = "CFG0_WWD9_CLKSEL_PROXY (rw) register accessor: CFG0_WWD9_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd9_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd9_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd9_clksel_proxy`]
module"]
#[doc(alias = "CFG0_WWD9_CLKSEL_PROXY")]
pub type Cfg0Wwd9ClkselProxy = crate::Reg<cfg0_wwd9_clksel_proxy::Cfg0Wwd9ClkselProxySpec>;
#[doc = "CFG0_WWD9_CLKSEL_PROXY"]
pub mod cfg0_wwd9_clksel_proxy;
#[doc = "CFG0_WWD10_CLKSEL_PROXY (rw) register accessor: CFG0_WWD10_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd10_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd10_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd10_clksel_proxy`]
module"]
#[doc(alias = "CFG0_WWD10_CLKSEL_PROXY")]
pub type Cfg0Wwd10ClkselProxy = crate::Reg<cfg0_wwd10_clksel_proxy::Cfg0Wwd10ClkselProxySpec>;
#[doc = "CFG0_WWD10_CLKSEL_PROXY"]
pub mod cfg0_wwd10_clksel_proxy;
#[doc = "CFG0_WWD11_CLKSEL_PROXY (rw) register accessor: CFG0_WWD11_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd11_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd11_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_wwd11_clksel_proxy`]
module"]
#[doc(alias = "CFG0_WWD11_CLKSEL_PROXY")]
pub type Cfg0Wwd11ClkselProxy = crate::Reg<cfg0_wwd11_clksel_proxy::Cfg0Wwd11ClkselProxySpec>;
#[doc = "CFG0_WWD11_CLKSEL_PROXY"]
pub mod cfg0_wwd11_clksel_proxy;
#[doc = "CFG0_SERDES0_CLKSEL_PROXY (rw) register accessor: CFG0_SERDES0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_serdes0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_serdes0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_serdes0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_SERDES0_CLKSEL_PROXY")]
pub type Cfg0Serdes0ClkselProxy = crate::Reg<cfg0_serdes0_clksel_proxy::Cfg0Serdes0ClkselProxySpec>;
#[doc = "CFG0_SERDES0_CLKSEL_PROXY"]
pub mod cfg0_serdes0_clksel_proxy;
#[doc = "CFG0_MCAN0_CLKSEL_PROXY (rw) register accessor: CFG0_MCAN0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcan0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcan0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcan0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCAN0_CLKSEL_PROXY")]
pub type Cfg0Mcan0ClkselProxy = crate::Reg<cfg0_mcan0_clksel_proxy::Cfg0Mcan0ClkselProxySpec>;
#[doc = "CFG0_MCAN0_CLKSEL_PROXY"]
pub mod cfg0_mcan0_clksel_proxy;
#[doc = "CFG0_MCAN1_CLKSEL_PROXY (rw) register accessor: CFG0_MCAN1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcan1_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcan1_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_mcan1_clksel_proxy`]
module"]
#[doc(alias = "CFG0_MCAN1_CLKSEL_PROXY")]
pub type Cfg0Mcan1ClkselProxy = crate::Reg<cfg0_mcan1_clksel_proxy::Cfg0Mcan1ClkselProxySpec>;
#[doc = "CFG0_MCAN1_CLKSEL_PROXY"]
pub mod cfg0_mcan1_clksel_proxy;
#[doc = "CFG0_OSPI0_CLKSEL_PROXY (rw) register accessor: CFG0_OSPI0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ospi0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ospi0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ospi0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_OSPI0_CLKSEL_PROXY")]
pub type Cfg0Ospi0ClkselProxy = crate::Reg<cfg0_ospi0_clksel_proxy::Cfg0Ospi0ClkselProxySpec>;
#[doc = "CFG0_OSPI0_CLKSEL_PROXY"]
pub mod cfg0_ospi0_clksel_proxy;
#[doc = "CFG0_ADC0_CLKSEL_PROXY (rw) register accessor: CFG0_ADC0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_clksel_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_clksel_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_adc0_clksel_proxy`]
module"]
#[doc(alias = "CFG0_ADC0_CLKSEL_PROXY")]
pub type Cfg0Adc0ClkselProxy = crate::Reg<cfg0_adc0_clksel_proxy::Cfg0Adc0ClkselProxySpec>;
#[doc = "CFG0_ADC0_CLKSEL_PROXY"]
pub mod cfg0_adc0_clksel_proxy;
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
#[doc = "CFG0_CLAIMREG_P2_R2 (rw) register accessor: CFG0_CLAIMREG_P2_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r2`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R2")]
pub type Cfg0ClaimregP2R2 = crate::Reg<cfg0_claimreg_p2_r2::Cfg0ClaimregP2R2Spec>;
#[doc = "CFG0_CLAIMREG_P2_R2"]
pub mod cfg0_claimreg_p2_r2;
#[doc = "CFG0_CLAIMREG_P2_R3 (rw) register accessor: CFG0_CLAIMREG_P2_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r3`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R3")]
pub type Cfg0ClaimregP2R3 = crate::Reg<cfg0_claimreg_p2_r3::Cfg0ClaimregP2R3Spec>;
#[doc = "CFG0_CLAIMREG_P2_R3"]
pub mod cfg0_claimreg_p2_r3;
#[doc = "CFG0_CLAIMREG_P2_R4 (rw) register accessor: CFG0_CLAIMREG_P2_R4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r4`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R4")]
pub type Cfg0ClaimregP2R4 = crate::Reg<cfg0_claimreg_p2_r4::Cfg0ClaimregP2R4Spec>;
#[doc = "CFG0_CLAIMREG_P2_R4"]
pub mod cfg0_claimreg_p2_r4;
#[doc = "CFG0_CLAIMREG_P2_R5 (rw) register accessor: CFG0_CLAIMREG_P2_R5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r5`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R5")]
pub type Cfg0ClaimregP2R5 = crate::Reg<cfg0_claimreg_p2_r5::Cfg0ClaimregP2R5Spec>;
#[doc = "CFG0_CLAIMREG_P2_R5"]
pub mod cfg0_claimreg_p2_r5;
#[doc = "CFG0_CLAIMREG_P2_R6 (rw) register accessor: CFG0_CLAIMREG_P2_R6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r6`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R6")]
pub type Cfg0ClaimregP2R6 = crate::Reg<cfg0_claimreg_p2_r6::Cfg0ClaimregP2R6Spec>;
#[doc = "CFG0_CLAIMREG_P2_R6"]
pub mod cfg0_claimreg_p2_r6;
#[doc = "CFG0_CLAIMREG_P2_R7 (rw) register accessor: CFG0_CLAIMREG_P2_R7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r7`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R7")]
pub type Cfg0ClaimregP2R7 = crate::Reg<cfg0_claimreg_p2_r7::Cfg0ClaimregP2R7Spec>;
#[doc = "CFG0_CLAIMREG_P2_R7"]
pub mod cfg0_claimreg_p2_r7;
#[doc = "CFG0_CLAIMREG_P2_R8 (rw) register accessor: CFG0_CLAIMREG_P2_R8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r8`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R8")]
pub type Cfg0ClaimregP2R8 = crate::Reg<cfg0_claimreg_p2_r8::Cfg0ClaimregP2R8Spec>;
#[doc = "CFG0_CLAIMREG_P2_R8"]
pub mod cfg0_claimreg_p2_r8;
#[doc = "CFG0_CLAIMREG_P2_R9 (rw) register accessor: CFG0_CLAIMREG_P2_R9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r9`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R9")]
pub type Cfg0ClaimregP2R9 = crate::Reg<cfg0_claimreg_p2_r9::Cfg0ClaimregP2R9Spec>;
#[doc = "CFG0_CLAIMREG_P2_R9"]
pub mod cfg0_claimreg_p2_r9;
#[doc = "CFG0_CLAIMREG_P2_R10 (rw) register accessor: CFG0_CLAIMREG_P2_R10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p2_r10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p2_r10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p2_r10`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P2_R10")]
pub type Cfg0ClaimregP2R10 = crate::Reg<cfg0_claimreg_p2_r10::Cfg0ClaimregP2R10Spec>;
#[doc = "CFG0_CLAIMREG_P2_R10"]
pub mod cfg0_claimreg_p2_r10;
#[doc = "CFG0_FUSE_CRC_STAT (rw) register accessor: CFG0_FUSE_CRC_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fuse_crc_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fuse_crc_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fuse_crc_stat`]
module"]
#[doc(alias = "CFG0_FUSE_CRC_STAT")]
pub type Cfg0FuseCrcStat = crate::Reg<cfg0_fuse_crc_stat::Cfg0FuseCrcStatSpec>;
#[doc = "CFG0_FUSE_CRC_STAT"]
pub mod cfg0_fuse_crc_stat;
#[doc = "CFG0_PBIST_EN (rw) register accessor: CFG0_PBIST_EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pbist_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pbist_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pbist_en`]
module"]
#[doc(alias = "CFG0_PBIST_EN")]
pub type Cfg0PbistEn = crate::Reg<cfg0_pbist_en::Cfg0PbistEnSpec>;
#[doc = "CFG0_PBIST_EN"]
pub mod cfg0_pbist_en;
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
#[doc = "CFG0_CLAIMREG_P3_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R1_READONLY")]
pub type Cfg0ClaimregP3R1Readonly =
    crate::Reg<cfg0_claimreg_p3_r1_readonly::Cfg0ClaimregP3R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R1_READONLY"]
pub mod cfg0_claimreg_p3_r1_readonly;
#[doc = "CFG0_CLAIMREG_P3_R2_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R2_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r2_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r2_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r2_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R2_READONLY")]
pub type Cfg0ClaimregP3R2Readonly =
    crate::Reg<cfg0_claimreg_p3_r2_readonly::Cfg0ClaimregP3R2ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R2_READONLY"]
pub mod cfg0_claimreg_p3_r2_readonly;
#[doc = "CFG0_CLAIMREG_P3_R3_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R3_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r3_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r3_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r3_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R3_READONLY")]
pub type Cfg0ClaimregP3R3Readonly =
    crate::Reg<cfg0_claimreg_p3_r3_readonly::Cfg0ClaimregP3R3ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R3_READONLY"]
pub mod cfg0_claimreg_p3_r3_readonly;
#[doc = "CFG0_CLAIMREG_P3_R4_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r4_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r4_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r4_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R4_READONLY")]
pub type Cfg0ClaimregP3R4Readonly =
    crate::Reg<cfg0_claimreg_p3_r4_readonly::Cfg0ClaimregP3R4ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R4_READONLY"]
pub mod cfg0_claimreg_p3_r4_readonly;
#[doc = "CFG0_CLAIMREG_P3_R5_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r5_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r5_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r5_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R5_READONLY")]
pub type Cfg0ClaimregP3R5Readonly =
    crate::Reg<cfg0_claimreg_p3_r5_readonly::Cfg0ClaimregP3R5ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R5_READONLY"]
pub mod cfg0_claimreg_p3_r5_readonly;
#[doc = "CFG0_CLAIMREG_P3_R6_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R6_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r6_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r6_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r6_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R6_READONLY")]
pub type Cfg0ClaimregP3R6Readonly =
    crate::Reg<cfg0_claimreg_p3_r6_readonly::Cfg0ClaimregP3R6ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R6_READONLY"]
pub mod cfg0_claimreg_p3_r6_readonly;
#[doc = "CFG0_CLAIMREG_P3_R7_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R7_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r7_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r7_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r7_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R7_READONLY")]
pub type Cfg0ClaimregP3R7Readonly =
    crate::Reg<cfg0_claimreg_p3_r7_readonly::Cfg0ClaimregP3R7ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R7_READONLY"]
pub mod cfg0_claimreg_p3_r7_readonly;
#[doc = "CFG0_CLAIMREG_P3_R8_READONLY (rw) register accessor: CFG0_CLAIMREG_P3_R8_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r8_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r8_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r8_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R8_READONLY")]
pub type Cfg0ClaimregP3R8Readonly =
    crate::Reg<cfg0_claimreg_p3_r8_readonly::Cfg0ClaimregP3R8ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P3_R8_READONLY"]
pub mod cfg0_claimreg_p3_r8_readonly;
#[doc = "CFG0_FUSE_CRC_STAT_PROXY (rw) register accessor: CFG0_FUSE_CRC_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fuse_crc_stat_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fuse_crc_stat_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_fuse_crc_stat_proxy`]
module"]
#[doc(alias = "CFG0_FUSE_CRC_STAT_PROXY")]
pub type Cfg0FuseCrcStatProxy = crate::Reg<cfg0_fuse_crc_stat_proxy::Cfg0FuseCrcStatProxySpec>;
#[doc = "CFG0_FUSE_CRC_STAT_PROXY"]
pub mod cfg0_fuse_crc_stat_proxy;
#[doc = "CFG0_PBIST_EN_PROXY (rw) register accessor: CFG0_PBIST_EN_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pbist_en_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pbist_en_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_pbist_en_proxy`]
module"]
#[doc(alias = "CFG0_PBIST_EN_PROXY")]
pub type Cfg0PbistEnProxy = crate::Reg<cfg0_pbist_en_proxy::Cfg0PbistEnProxySpec>;
#[doc = "CFG0_PBIST_EN_PROXY"]
pub mod cfg0_pbist_en_proxy;
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
#[doc = "CFG0_CLAIMREG_P3_R1 (rw) register accessor: CFG0_CLAIMREG_P3_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R1")]
pub type Cfg0ClaimregP3R1 = crate::Reg<cfg0_claimreg_p3_r1::Cfg0ClaimregP3R1Spec>;
#[doc = "CFG0_CLAIMREG_P3_R1"]
pub mod cfg0_claimreg_p3_r1;
#[doc = "CFG0_CLAIMREG_P3_R2 (rw) register accessor: CFG0_CLAIMREG_P3_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r2`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R2")]
pub type Cfg0ClaimregP3R2 = crate::Reg<cfg0_claimreg_p3_r2::Cfg0ClaimregP3R2Spec>;
#[doc = "CFG0_CLAIMREG_P3_R2"]
pub mod cfg0_claimreg_p3_r2;
#[doc = "CFG0_CLAIMREG_P3_R3 (rw) register accessor: CFG0_CLAIMREG_P3_R3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r3`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R3")]
pub type Cfg0ClaimregP3R3 = crate::Reg<cfg0_claimreg_p3_r3::Cfg0ClaimregP3R3Spec>;
#[doc = "CFG0_CLAIMREG_P3_R3"]
pub mod cfg0_claimreg_p3_r3;
#[doc = "CFG0_CLAIMREG_P3_R4 (rw) register accessor: CFG0_CLAIMREG_P3_R4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r4`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R4")]
pub type Cfg0ClaimregP3R4 = crate::Reg<cfg0_claimreg_p3_r4::Cfg0ClaimregP3R4Spec>;
#[doc = "CFG0_CLAIMREG_P3_R4"]
pub mod cfg0_claimreg_p3_r4;
#[doc = "CFG0_CLAIMREG_P3_R5 (rw) register accessor: CFG0_CLAIMREG_P3_R5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r5`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R5")]
pub type Cfg0ClaimregP3R5 = crate::Reg<cfg0_claimreg_p3_r5::Cfg0ClaimregP3R5Spec>;
#[doc = "CFG0_CLAIMREG_P3_R5"]
pub mod cfg0_claimreg_p3_r5;
#[doc = "CFG0_CLAIMREG_P3_R6 (rw) register accessor: CFG0_CLAIMREG_P3_R6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r6`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R6")]
pub type Cfg0ClaimregP3R6 = crate::Reg<cfg0_claimreg_p3_r6::Cfg0ClaimregP3R6Spec>;
#[doc = "CFG0_CLAIMREG_P3_R6"]
pub mod cfg0_claimreg_p3_r6;
#[doc = "CFG0_CLAIMREG_P3_R7 (rw) register accessor: CFG0_CLAIMREG_P3_R7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r7`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R7")]
pub type Cfg0ClaimregP3R7 = crate::Reg<cfg0_claimreg_p3_r7::Cfg0ClaimregP3R7Spec>;
#[doc = "CFG0_CLAIMREG_P3_R7"]
pub mod cfg0_claimreg_p3_r7;
#[doc = "CFG0_CLAIMREG_P3_R8 (rw) register accessor: CFG0_CLAIMREG_P3_R8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p3_r8`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P3_R8")]
pub type Cfg0ClaimregP3R8 = crate::Reg<cfg0_claimreg_p3_r8::Cfg0ClaimregP3R8Spec>;
#[doc = "CFG0_CLAIMREG_P3_R8"]
pub mod cfg0_claimreg_p3_r8;
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
#[doc = "CFG0_CHNG_DDR4_FSP_REQ (rw) register accessor: CFG0_CHNG_DDR4_FSP_REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_chng_ddr4_fsp_req`]
module"]
#[doc(alias = "CFG0_CHNG_DDR4_FSP_REQ")]
pub type Cfg0ChngDdr4FspReq = crate::Reg<cfg0_chng_ddr4_fsp_req::Cfg0ChngDdr4FspReqSpec>;
#[doc = "CFG0_CHNG_DDR4_FSP_REQ"]
pub mod cfg0_chng_ddr4_fsp_req;
#[doc = "CFG0_CHNG_DDR4_FSP_ACK (rw) register accessor: CFG0_CHNG_DDR4_FSP_ACK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_chng_ddr4_fsp_ack`]
module"]
#[doc(alias = "CFG0_CHNG_DDR4_FSP_ACK")]
pub type Cfg0ChngDdr4FspAck = crate::Reg<cfg0_chng_ddr4_fsp_ack::Cfg0ChngDdr4FspAckSpec>;
#[doc = "CFG0_CHNG_DDR4_FSP_ACK"]
pub mod cfg0_chng_ddr4_fsp_ack;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_REQ (rw) register accessor: CFG0_DDR4_FSP_CLKCHNG_REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ddr4_fsp_clkchng_req`]
module"]
#[doc(alias = "CFG0_DDR4_FSP_CLKCHNG_REQ")]
pub type Cfg0Ddr4FspClkchngReq = crate::Reg<cfg0_ddr4_fsp_clkchng_req::Cfg0Ddr4FspClkchngReqSpec>;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_REQ"]
pub mod cfg0_ddr4_fsp_clkchng_req;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_ACK (rw) register accessor: CFG0_DDR4_FSP_CLKCHNG_ACK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_ack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_ack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ddr4_fsp_clkchng_ack`]
module"]
#[doc(alias = "CFG0_DDR4_FSP_CLKCHNG_ACK")]
pub type Cfg0Ddr4FspClkchngAck = crate::Reg<cfg0_ddr4_fsp_clkchng_ack::Cfg0Ddr4FspClkchngAckSpec>;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_ACK"]
pub mod cfg0_ddr4_fsp_clkchng_ack;
#[doc = "CFG0_LOCK5_KICK0 (rw) register accessor: CFG0_LOCK5_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock5_kick0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock5_kick0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock5_kick0`]
module"]
#[doc(alias = "CFG0_LOCK5_KICK0")]
pub type Cfg0Lock5Kick0 = crate::Reg<cfg0_lock5_kick0::Cfg0Lock5Kick0Spec>;
#[doc = "CFG0_LOCK5_KICK0"]
pub mod cfg0_lock5_kick0;
#[doc = "CFG0_LOCK5_KICK1 (rw) register accessor: CFG0_LOCK5_KICK1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock5_kick1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock5_kick1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock5_kick1`]
module"]
#[doc(alias = "CFG0_LOCK5_KICK1")]
pub type Cfg0Lock5Kick1 = crate::Reg<cfg0_lock5_kick1::Cfg0Lock5Kick1Spec>;
#[doc = "CFG0_LOCK5_KICK1"]
pub mod cfg0_lock5_kick1;
#[doc = "CFG0_CLAIMREG_P5_R0_READONLY (rw) register accessor: CFG0_CLAIMREG_P5_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p5_r0_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p5_r0_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p5_r0_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P5_R0_READONLY")]
pub type Cfg0ClaimregP5R0Readonly =
    crate::Reg<cfg0_claimreg_p5_r0_readonly::Cfg0ClaimregP5R0ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P5_R0_READONLY"]
pub mod cfg0_claimreg_p5_r0_readonly;
#[doc = "CFG0_CLAIMREG_P5_R1_READONLY (rw) register accessor: CFG0_CLAIMREG_P5_R1_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p5_r1_readonly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p5_r1_readonly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p5_r1_readonly`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P5_R1_READONLY")]
pub type Cfg0ClaimregP5R1Readonly =
    crate::Reg<cfg0_claimreg_p5_r1_readonly::Cfg0ClaimregP5R1ReadonlySpec>;
#[doc = "CFG0_CLAIMREG_P5_R1_READONLY"]
pub mod cfg0_claimreg_p5_r1_readonly;
#[doc = "CFG0_CHNG_DDR4_FSP_REQ_PROXY (rw) register accessor: CFG0_CHNG_DDR4_FSP_REQ_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_req_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_req_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_chng_ddr4_fsp_req_proxy`]
module"]
#[doc(alias = "CFG0_CHNG_DDR4_FSP_REQ_PROXY")]
pub type Cfg0ChngDdr4FspReqProxy =
    crate::Reg<cfg0_chng_ddr4_fsp_req_proxy::Cfg0ChngDdr4FspReqProxySpec>;
#[doc = "CFG0_CHNG_DDR4_FSP_REQ_PROXY"]
pub mod cfg0_chng_ddr4_fsp_req_proxy;
#[doc = "CFG0_CHNG_DDR4_FSP_ACK_PROXY (rw) register accessor: CFG0_CHNG_DDR4_FSP_ACK_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_ack_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_ack_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_chng_ddr4_fsp_ack_proxy`]
module"]
#[doc(alias = "CFG0_CHNG_DDR4_FSP_ACK_PROXY")]
pub type Cfg0ChngDdr4FspAckProxy =
    crate::Reg<cfg0_chng_ddr4_fsp_ack_proxy::Cfg0ChngDdr4FspAckProxySpec>;
#[doc = "CFG0_CHNG_DDR4_FSP_ACK_PROXY"]
pub mod cfg0_chng_ddr4_fsp_ack_proxy;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY (rw) register accessor: CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_req_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_req_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ddr4_fsp_clkchng_req_proxy`]
module"]
#[doc(alias = "CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY")]
pub type Cfg0Ddr4FspClkchngReqProxy =
    crate::Reg<cfg0_ddr4_fsp_clkchng_req_proxy::Cfg0Ddr4FspClkchngReqProxySpec>;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY"]
pub mod cfg0_ddr4_fsp_clkchng_req_proxy;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY (rw) register accessor: CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_ack_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_ack_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_ddr4_fsp_clkchng_ack_proxy`]
module"]
#[doc(alias = "CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY")]
pub type Cfg0Ddr4FspClkchngAckProxy =
    crate::Reg<cfg0_ddr4_fsp_clkchng_ack_proxy::Cfg0Ddr4FspClkchngAckProxySpec>;
#[doc = "CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY"]
pub mod cfg0_ddr4_fsp_clkchng_ack_proxy;
#[doc = "CFG0_LOCK5_KICK0_PROXY (rw) register accessor: CFG0_LOCK5_KICK0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock5_kick0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock5_kick0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock5_kick0_proxy`]
module"]
#[doc(alias = "CFG0_LOCK5_KICK0_PROXY")]
pub type Cfg0Lock5Kick0Proxy = crate::Reg<cfg0_lock5_kick0_proxy::Cfg0Lock5Kick0ProxySpec>;
#[doc = "CFG0_LOCK5_KICK0_PROXY"]
pub mod cfg0_lock5_kick0_proxy;
#[doc = "CFG0_LOCK5_KICK1_PROXY (rw) register accessor: CFG0_LOCK5_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock5_kick1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock5_kick1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_lock5_kick1_proxy`]
module"]
#[doc(alias = "CFG0_LOCK5_KICK1_PROXY")]
pub type Cfg0Lock5Kick1Proxy = crate::Reg<cfg0_lock5_kick1_proxy::Cfg0Lock5Kick1ProxySpec>;
#[doc = "CFG0_LOCK5_KICK1_PROXY"]
pub mod cfg0_lock5_kick1_proxy;
#[doc = "CFG0_CLAIMREG_P5_R0 (rw) register accessor: CFG0_CLAIMREG_P5_R0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p5_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p5_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p5_r0`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P5_R0")]
pub type Cfg0ClaimregP5R0 = crate::Reg<cfg0_claimreg_p5_r0::Cfg0ClaimregP5R0Spec>;
#[doc = "CFG0_CLAIMREG_P5_R0"]
pub mod cfg0_claimreg_p5_r0;
#[doc = "CFG0_CLAIMREG_P5_R1 (rw) register accessor: CFG0_CLAIMREG_P5_R1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p5_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p5_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_claimreg_p5_r1`]
module"]
#[doc(alias = "CFG0_CLAIMREG_P5_R1")]
pub type Cfg0ClaimregP5R1 = crate::Reg<cfg0_claimreg_p5_r1::Cfg0ClaimregP5R1Spec>;
#[doc = "CFG0_CLAIMREG_P5_R1"]
pub mod cfg0_claimreg_p5_r1;
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
