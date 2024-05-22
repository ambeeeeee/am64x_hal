#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs__ss_cfg__sscfg_ss_id_rev_reg: Regs_SsCfg_SscfgSsIdRevReg,
    regs__ss_cfg__sscfg_ss_ctl_reg: Regs_SsCfg_SscfgSsCtlReg,
    _reserved2: [u8; 0x18],
    regs__ss_cfg__sscfg_v2a_ctl_reg: Regs_SsCfg_SscfgV2aCtlReg,
    regs__ss_cfg__sscfg_v2a_r1_mat_reg: Regs_SsCfg_SscfgV2aR1MatReg,
    regs__ss_cfg__sscfg_v2a_r2_mat_reg: Regs_SsCfg_SscfgV2aR2MatReg,
    regs__ss_cfg__sscfg_v2a_r3_mat_reg: Regs_SsCfg_SscfgV2aR3MatReg,
    regs__ss_cfg__sscfg_v2a_def_pri_map_reg: Regs_SsCfg_SscfgV2aDefPriMapReg,
    regs__ss_cfg__sscfg_v2a_r1_pri_map_reg: Regs_SsCfg_SscfgV2aR1PriMapReg,
    regs__ss_cfg__sscfg_v2a_r2_pri_map_reg: Regs_SsCfg_SscfgV2aR2PriMapReg,
    regs__ss_cfg__sscfg_v2a_r3_pri_map_reg: Regs_SsCfg_SscfgV2aR3PriMapReg,
    _reserved10: [u8; 0x30],
    regs__ss_cfg__sscfg_v2a_aerr_log1_reg: Regs_SsCfg_SscfgV2aAerrLog1Reg,
    regs__ss_cfg__sscfg_v2a_aerr_log2_reg: Regs_SsCfg_SscfgV2aAerrLog2Reg,
    _reserved12: [u8; 0x24],
    regs__ss_cfg__sscfg_v2a_bus_to: Regs_SsCfg_SscfgV2aBusTo,
    regs__ss_cfg__sscfg_v2a_int_raw_reg: Regs_SsCfg_SscfgV2aIntRawReg,
    regs__ss_cfg__sscfg_v2a_int_stat_reg: Regs_SsCfg_SscfgV2aIntStatReg,
    regs__ss_cfg__sscfg_v2a_int_set_reg: Regs_SsCfg_SscfgV2aIntSetReg,
    regs__ss_cfg__sscfg_v2a_int_clr_reg: Regs_SsCfg_SscfgV2aIntClrReg,
    regs__ss_cfg__sscfg_v2a_eoi_reg: Regs_SsCfg_SscfgV2aEoiReg,
    _reserved18: [u8; 0x4c],
    regs__ss_cfg__sscfg_perf_cnt_sel_reg: Regs_SsCfg_SscfgPerfCntSelReg,
    regs__ss_cfg__sscfg_perf_cnt1_reg: Regs_SsCfg_SscfgPerfCnt1Reg,
    regs__ss_cfg__sscfg_perf_cnt2_reg: Regs_SsCfg_SscfgPerfCnt2Reg,
    regs__ss_cfg__sscfg_perf_cnt3_reg: Regs_SsCfg_SscfgPerfCnt3Reg,
    regs__ss_cfg__sscfg_perf_cnt4_reg: Regs_SsCfg_SscfgPerfCnt4Reg,
    _reserved23: [u8; 0x0c],
    regs__ss_cfg__sscfg_ecc_ctrl_reg: Regs_SsCfg_SscfgEccCtrlReg,
    regs__ss_cfg__sscfg_ecc_rid_indx_reg: Regs_SsCfg_SscfgEccRidIndxReg,
    regs__ss_cfg__sscfg_ecc_rid_val_reg: Regs_SsCfg_SscfgEccRidValReg,
    _reserved26: [u8; 0x04],
    regs__ss_cfg__sscfg_ecc_r0_str_addr_reg: Regs_SsCfg_SscfgEccR0StrAddrReg,
    regs__ss_cfg__sscfg_ecc_r0_end_addr_reg: Regs_SsCfg_SscfgEccR0EndAddrReg,
    regs__ss_cfg__sscfg_ecc_r1_str_addr_reg: Regs_SsCfg_SscfgEccR1StrAddrReg,
    regs__ss_cfg__sscfg_ecc_r1_end_addr_reg: Regs_SsCfg_SscfgEccR1EndAddrReg,
    regs__ss_cfg__sscfg_ecc_r2_str_addr_reg: Regs_SsCfg_SscfgEccR2StrAddrReg,
    regs__ss_cfg__sscfg_ecc_r2_end_addr_reg: Regs_SsCfg_SscfgEccR2EndAddrReg,
    _reserved32: [u8; 0x08],
    regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg: Regs_SsCfg_SscfgEcc1bErrCntReg,
    regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg: Regs_SsCfg_SscfgEcc1bErrThrshReg,
    regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg: Regs_SsCfg_SscfgEcc1bErrAdrLogReg,
    regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg: Regs_SsCfg_SscfgEcc1bErrMskLogReg,
    regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg: Regs_SsCfg_SscfgEcc2bErrAdrLogReg,
    regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg: Regs_SsCfg_SscfgEcc2bErrMskLogReg,
    _reserved38: [u8; 0x1c],
    regs__ss_cfg__sscfg_phy_test_ctrl1_reg: Regs_SsCfg_SscfgPhyTestCtrl1Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl2_reg: Regs_SsCfg_SscfgPhyTestCtrl2Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl3_reg: Regs_SsCfg_SscfgPhyTestCtrl3Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl4_reg: Regs_SsCfg_SscfgPhyTestCtrl4Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl5_reg: Regs_SsCfg_SscfgPhyTestCtrl5Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl6_reg: Regs_SsCfg_SscfgPhyTestCtrl6Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl7_reg: Regs_SsCfg_SscfgPhyTestCtrl7Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl8_reg: Regs_SsCfg_SscfgPhyTestCtrl8Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl9_reg: Regs_SsCfg_SscfgPhyTestCtrl9Reg,
    regs__ss_cfg__sscfg_phy_test_ctrl10_reg: Regs_SsCfg_SscfgPhyTestCtrl10Reg,
    _reserved48: [u8; 0x14],
    regs__ss_cfg__sscfg_phy_test_stat1_reg: Regs_SsCfg_SscfgPhyTestStat1Reg,
    regs__ss_cfg__sscfg_phy_test_stat2_reg: Regs_SsCfg_SscfgPhyTestStat2Reg,
}
impl RegisterBlock {
    #[doc = "0x00 - The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ss_id_rev_reg(&self) -> &Regs_SsCfg_SscfgSsIdRevReg {
        &self.regs__ss_cfg__sscfg_ss_id_rev_reg
    }
    #[doc = "0x04 - The Subsystem Control Register contains fields for control functions required for submodules in the subsystem."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ss_ctl_reg(&self) -> &Regs_SsCfg_SscfgSsCtlReg {
        &self.regs__ss_cfg__sscfg_ss_ctl_reg
    }
    #[doc = "0x20 - The VBUSM2AXI Control register contains control functions required for the VBUSM2AXI submodule."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_ctl_reg(&self) -> &Regs_SsCfg_SscfgV2aCtlReg {
        &self.regs__ss_cfg__sscfg_v2a_ctl_reg
    }
    #[doc = "0x24 - The Range 1 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 1 Match Register uses the associated Range 1 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_r1_mat_reg(&self) -> &Regs_SsCfg_SscfgV2aR1MatReg {
        &self.regs__ss_cfg__sscfg_v2a_r1_mat_reg
    }
    #[doc = "0x28 - The Range 2 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 2 Match Register uses the associated Range 2 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_r2_mat_reg(&self) -> &Regs_SsCfg_SscfgV2aR2MatReg {
        &self.regs__ss_cfg__sscfg_v2a_r2_mat_reg
    }
    #[doc = "0x2c - The Range 3 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 3 Match Register uses the associated Range 3 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_r3_mat_reg(&self) -> &Regs_SsCfg_SscfgV2aR3MatReg {
        &self.regs__ss_cfg__sscfg_v2a_r3_mat_reg
    }
    #[doc = "0x30 - The Default Priority Mapping Register is the default map for the inbound VBUSM.C priority to AXI priority."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_def_pri_map_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgV2aDefPriMapReg {
        &self.regs__ss_cfg__sscfg_v2a_def_pri_map_reg
    }
    #[doc = "0x34 - The Range 1 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 1 occurs. This allows the priority level to be changed from the Default Priority Mapping value."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_r1_pri_map_reg(&self) -> &Regs_SsCfg_SscfgV2aR1PriMapReg {
        &self.regs__ss_cfg__sscfg_v2a_r1_pri_map_reg
    }
    #[doc = "0x38 - The Range 2 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 2 occurs. This allows the priority level to be changed from the Default Priority Mapping value."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_r2_pri_map_reg(&self) -> &Regs_SsCfg_SscfgV2aR2PriMapReg {
        &self.regs__ss_cfg__sscfg_v2a_r2_pri_map_reg
    }
    #[doc = "0x3c - The Range 3 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 3 occurs. This allows the priority level to be changed from the Default Priority Mapping value."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_r3_pri_map_reg(&self) -> &Regs_SsCfg_SscfgV2aR3PriMapReg {
        &self.regs__ss_cfg__sscfg_v2a_r3_pri_map_reg
    }
    #[doc = "0x70 - The Address Error Log 1 register displays the RouteID and lsb of the address for the first VBUSM.C command that was outside the programmed addressing range. Writing a 0x1 will clear all fields. Writing any other value has no effect. The Address Error Log 2 register will also be cleared upon writing this register."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_aerr_log1_reg(&self) -> &Regs_SsCfg_SscfgV2aAerrLog1Reg {
        &self.regs__ss_cfg__sscfg_v2a_aerr_log1_reg
    }
    #[doc = "0x74 - The Address Error Log 2 registers displays the msb of the address for the first VBUSM.C command that was outside the programmed addressing range. This register will be cleared upon writing the Address Error Log 1 register."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_aerr_log2_reg(&self) -> &Regs_SsCfg_SscfgV2aAerrLog2Reg {
        &self.regs__ss_cfg__sscfg_v2a_aerr_log2_reg
    }
    #[doc = "0x9c - REGS__SS_CFG__SSCFG_V2A_BUS_TO"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_bus_to(&self) -> &Regs_SsCfg_SscfgV2aBusTo {
        &self.regs__ss_cfg__sscfg_v2a_bus_to
    }
    #[doc = "0xa0 - REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_int_raw_reg(&self) -> &Regs_SsCfg_SscfgV2aIntRawReg {
        &self.regs__ss_cfg__sscfg_v2a_int_raw_reg
    }
    #[doc = "0xa4 - REGS__SS_CFG__SSCFG_V2A_INT_STAT_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_int_stat_reg(&self) -> &Regs_SsCfg_SscfgV2aIntStatReg {
        &self.regs__ss_cfg__sscfg_v2a_int_stat_reg
    }
    #[doc = "0xa8 - REGS__SS_CFG__SSCFG_V2A_INT_SET_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_int_set_reg(&self) -> &Regs_SsCfg_SscfgV2aIntSetReg {
        &self.regs__ss_cfg__sscfg_v2a_int_set_reg
    }
    #[doc = "0xac - REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_int_clr_reg(&self) -> &Regs_SsCfg_SscfgV2aIntClrReg {
        &self.regs__ss_cfg__sscfg_v2a_int_clr_reg
    }
    #[doc = "0xb0 - REGS__SS_CFG__SSCFG_V2A_EOI_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_v2a_eoi_reg(&self) -> &Regs_SsCfg_SscfgV2aEoiReg {
        &self.regs__ss_cfg__sscfg_v2a_eoi_reg
    }
    #[doc = "0x100 - The Performance Counter Select register is used to select the statistic type to be counted in the corresponding Performance Counter register."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_perf_cnt_sel_reg(&self) -> &Regs_SsCfg_SscfgPerfCntSelReg {
        &self.regs__ss_cfg__sscfg_perf_cnt_sel_reg
    }
    #[doc = "0x104 - REGS__SS_CFG__SSCFG_PERF_CNT1_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_perf_cnt1_reg(&self) -> &Regs_SsCfg_SscfgPerfCnt1Reg {
        &self.regs__ss_cfg__sscfg_perf_cnt1_reg
    }
    #[doc = "0x108 - REGS__SS_CFG__SSCFG_PERF_CNT2_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_perf_cnt2_reg(&self) -> &Regs_SsCfg_SscfgPerfCnt2Reg {
        &self.regs__ss_cfg__sscfg_perf_cnt2_reg
    }
    #[doc = "0x10c - REGS__SS_CFG__SSCFG_PERF_CNT3_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_perf_cnt3_reg(&self) -> &Regs_SsCfg_SscfgPerfCnt3Reg {
        &self.regs__ss_cfg__sscfg_perf_cnt3_reg
    }
    #[doc = "0x110 - REGS__SS_CFG__SSCFG_PERF_CNT4_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_perf_cnt4_reg(&self) -> &Regs_SsCfg_SscfgPerfCnt4Reg {
        &self.regs__ss_cfg__sscfg_perf_cnt4_reg
    }
    #[doc = "0x120 - REGS__SS_CFG__SSCFG_ECC_CTRL_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_ctrl_reg(&self) -> &Regs_SsCfg_SscfgEccCtrlReg {
        &self.regs__ss_cfg__sscfg_ecc_ctrl_reg
    }
    #[doc = "0x124 - REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_rid_indx_reg(&self) -> &Regs_SsCfg_SscfgEccRidIndxReg {
        &self.regs__ss_cfg__sscfg_ecc_rid_indx_reg
    }
    #[doc = "0x128 - REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_rid_val_reg(&self) -> &Regs_SsCfg_SscfgEccRidValReg {
        &self.regs__ss_cfg__sscfg_ecc_rid_val_reg
    }
    #[doc = "0x130 - REGS__SS_CFG__SSCFG_ECC_R0_STR_ADDR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_r0_str_addr_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEccR0StrAddrReg {
        &self.regs__ss_cfg__sscfg_ecc_r0_str_addr_reg
    }
    #[doc = "0x134 - REGS__SS_CFG__SSCFG_ECC_R0_END_ADDR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_r0_end_addr_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEccR0EndAddrReg {
        &self.regs__ss_cfg__sscfg_ecc_r0_end_addr_reg
    }
    #[doc = "0x138 - REGS__SS_CFG__SSCFG_ECC_R1_STR_ADDR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_r1_str_addr_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEccR1StrAddrReg {
        &self.regs__ss_cfg__sscfg_ecc_r1_str_addr_reg
    }
    #[doc = "0x13c - REGS__SS_CFG__SSCFG_ECC_R1_END_ADDR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_r1_end_addr_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEccR1EndAddrReg {
        &self.regs__ss_cfg__sscfg_ecc_r1_end_addr_reg
    }
    #[doc = "0x140 - REGS__SS_CFG__SSCFG_ECC_R2_STR_ADDR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_r2_str_addr_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEccR2StrAddrReg {
        &self.regs__ss_cfg__sscfg_ecc_r2_str_addr_reg
    }
    #[doc = "0x144 - REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_r2_end_addr_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEccR2EndAddrReg {
        &self.regs__ss_cfg__sscfg_ecc_r2_end_addr_reg
    }
    #[doc = "0x150 - REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg(&self) -> &Regs_SsCfg_SscfgEcc1bErrCntReg {
        &self.regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg
    }
    #[doc = "0x154 - REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEcc1bErrThrshReg {
        &self.regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg
    }
    #[doc = "0x158 - REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEcc1bErrAdrLogReg {
        &self.regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg
    }
    #[doc = "0x15c - REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEcc1bErrMskLogReg {
        &self.regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg
    }
    #[doc = "0x160 - REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEcc2bErrAdrLogReg {
        &self.regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg
    }
    #[doc = "0x164 - REGS__SS_CFG__SSCFG_ECC_2B_ERR_MSK_LOG_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgEcc2bErrMskLogReg {
        &self.regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg
    }
    #[doc = "0x184 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl1_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl1Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl1_reg
    }
    #[doc = "0x188 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl2_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl2Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl2_reg
    }
    #[doc = "0x18c - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl3_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl3Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl3_reg
    }
    #[doc = "0x190 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl4_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl4Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl4_reg
    }
    #[doc = "0x194 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl5_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl5Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl5_reg
    }
    #[doc = "0x198 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl6_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl6Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl6_reg
    }
    #[doc = "0x19c - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl7_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl7Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl7_reg
    }
    #[doc = "0x1a0 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl8_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl8Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl8_reg
    }
    #[doc = "0x1a4 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl9_reg(&self) -> &Regs_SsCfg_SscfgPhyTestCtrl9Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl9_reg
    }
    #[doc = "0x1a8 - REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_ctrl10_reg(
        &self,
    ) -> &Regs_SsCfg_SscfgPhyTestCtrl10Reg {
        &self.regs__ss_cfg__sscfg_phy_test_ctrl10_reg
    }
    #[doc = "0x1c0 - REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_stat1_reg(&self) -> &Regs_SsCfg_SscfgPhyTestStat1Reg {
        &self.regs__ss_cfg__sscfg_phy_test_stat1_reg
    }
    #[doc = "0x1c4 - REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_test_stat2_reg(&self) -> &Regs_SsCfg_SscfgPhyTestStat2Reg {
        &self.regs__ss_cfg__sscfg_phy_test_stat2_reg
    }
}
#[doc = "REGS__SS_CFG__SSCFG_SS_ID_REV_REG (rw) register accessor: The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ss_id_rev_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ss_id_rev_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ss_id_rev_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_SS_ID_REV_REG")]
pub type Regs_SsCfg_SscfgSsIdRevReg =
    crate::Reg<regs__ss_cfg__sscfg_ss_id_rev_reg::Regs_SsCfg_SscfgSsIdRevRegSpec>;
#[doc = "The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem."]
pub mod regs__ss_cfg__sscfg_ss_id_rev_reg;
#[doc = "REGS__SS_CFG__SSCFG_SS_CTL_REG (rw) register accessor: The Subsystem Control Register contains fields for control functions required for submodules in the subsystem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ss_ctl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ss_ctl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ss_ctl_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_SS_CTL_REG")]
pub type Regs_SsCfg_SscfgSsCtlReg =
    crate::Reg<regs__ss_cfg__sscfg_ss_ctl_reg::Regs_SsCfg_SscfgSsCtlRegSpec>;
#[doc = "The Subsystem Control Register contains fields for control functions required for submodules in the subsystem."]
pub mod regs__ss_cfg__sscfg_ss_ctl_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_CTL_REG (rw) register accessor: The VBUSM2AXI Control register contains control functions required for the VBUSM2AXI submodule.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_ctl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_ctl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_ctl_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_CTL_REG")]
pub type Regs_SsCfg_SscfgV2aCtlReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_ctl_reg::Regs_SsCfg_SscfgV2aCtlRegSpec>;
#[doc = "The VBUSM2AXI Control register contains control functions required for the VBUSM2AXI submodule."]
pub mod regs__ss_cfg__sscfg_v2a_ctl_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_R1_MAT_REG (rw) register accessor: The Range 1 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 1 Match Register uses the associated Range 1 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r1_mat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r1_mat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_r1_mat_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_R1_MAT_REG")]
pub type Regs_SsCfg_SscfgV2aR1MatReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_r1_mat_reg::Regs_SsCfg_SscfgV2aR1MatRegSpec>;
#[doc = "The Range 1 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 1 Match Register uses the associated Range 1 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches."]
pub mod regs__ss_cfg__sscfg_v2a_r1_mat_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_R2_MAT_REG (rw) register accessor: The Range 2 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 2 Match Register uses the associated Range 2 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r2_mat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r2_mat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_r2_mat_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_R2_MAT_REG")]
pub type Regs_SsCfg_SscfgV2aR2MatReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_r2_mat_reg::Regs_SsCfg_SscfgV2aR2MatRegSpec>;
#[doc = "The Range 2 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 2 Match Register uses the associated Range 2 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches."]
pub mod regs__ss_cfg__sscfg_v2a_r2_mat_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_R3_MAT_REG (rw) register accessor: The Range 3 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 3 Match Register uses the associated Range 3 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r3_mat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r3_mat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_r3_mat_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_R3_MAT_REG")]
pub type Regs_SsCfg_SscfgV2aR3MatReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_r3_mat_reg::Regs_SsCfg_SscfgV2aR3MatRegSpec>;
#[doc = "The Range 3 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 3 Match Register uses the associated Range 3 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches."]
pub mod regs__ss_cfg__sscfg_v2a_r3_mat_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_DEF_PRI_MAP_REG (rw) register accessor: The Default Priority Mapping Register is the default map for the inbound VBUSM.C priority to AXI priority.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_def_pri_map_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_def_pri_map_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_def_pri_map_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_DEF_PRI_MAP_REG")]
pub type Regs_SsCfg_SscfgV2aDefPriMapReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_def_pri_map_reg::Regs_SsCfg_SscfgV2aDefPriMapRegSpec>;
#[doc = "The Default Priority Mapping Register is the default map for the inbound VBUSM.C priority to AXI priority."]
pub mod regs__ss_cfg__sscfg_v2a_def_pri_map_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_R1_PRI_MAP_REG (rw) register accessor: The Range 1 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 1 occurs. This allows the priority level to be changed from the Default Priority Mapping value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r1_pri_map_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r1_pri_map_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_r1_pri_map_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_R1_PRI_MAP_REG")]
pub type Regs_SsCfg_SscfgV2aR1PriMapReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_r1_pri_map_reg::Regs_SsCfg_SscfgV2aR1PriMapRegSpec>;
#[doc = "The Range 1 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 1 occurs. This allows the priority level to be changed from the Default Priority Mapping value."]
pub mod regs__ss_cfg__sscfg_v2a_r1_pri_map_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_R2_PRI_MAP_REG (rw) register accessor: The Range 2 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 2 occurs. This allows the priority level to be changed from the Default Priority Mapping value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_r2_pri_map_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_R2_PRI_MAP_REG")]
pub type Regs_SsCfg_SscfgV2aR2PriMapReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::Regs_SsCfg_SscfgV2aR2PriMapRegSpec>;
#[doc = "The Range 2 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 2 occurs. This allows the priority level to be changed from the Default Priority Mapping value."]
pub mod regs__ss_cfg__sscfg_v2a_r2_pri_map_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_R3_PRI_MAP_REG (rw) register accessor: The Range 3 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 3 occurs. This allows the priority level to be changed from the Default Priority Mapping value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r3_pri_map_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r3_pri_map_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_r3_pri_map_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_R3_PRI_MAP_REG")]
pub type Regs_SsCfg_SscfgV2aR3PriMapReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_r3_pri_map_reg::Regs_SsCfg_SscfgV2aR3PriMapRegSpec>;
#[doc = "The Range 3 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 3 occurs. This allows the priority level to be changed from the Default Priority Mapping value."]
pub mod regs__ss_cfg__sscfg_v2a_r3_pri_map_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_AERR_LOG1_REG (rw) register accessor: The Address Error Log 1 register displays the RouteID and lsb of the address for the first VBUSM.C command that was outside the programmed addressing range. Writing a 0x1 will clear all fields. Writing any other value has no effect. The Address Error Log 2 register will also be cleared upon writing this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_aerr_log1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_aerr_log1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_aerr_log1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_AERR_LOG1_REG")]
pub type Regs_SsCfg_SscfgV2aAerrLog1Reg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_aerr_log1_reg::Regs_SsCfg_SscfgV2aAerrLog1RegSpec>;
#[doc = "The Address Error Log 1 register displays the RouteID and lsb of the address for the first VBUSM.C command that was outside the programmed addressing range. Writing a 0x1 will clear all fields. Writing any other value has no effect. The Address Error Log 2 register will also be cleared upon writing this register."]
pub mod regs__ss_cfg__sscfg_v2a_aerr_log1_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_AERR_LOG2_REG (rw) register accessor: The Address Error Log 2 registers displays the msb of the address for the first VBUSM.C command that was outside the programmed addressing range. This register will be cleared upon writing the Address Error Log 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_aerr_log2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_aerr_log2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_aerr_log2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_AERR_LOG2_REG")]
pub type Regs_SsCfg_SscfgV2aAerrLog2Reg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_aerr_log2_reg::Regs_SsCfg_SscfgV2aAerrLog2RegSpec>;
#[doc = "The Address Error Log 2 registers displays the msb of the address for the first VBUSM.C command that was outside the programmed addressing range. This register will be cleared upon writing the Address Error Log 1 register."]
pub mod regs__ss_cfg__sscfg_v2a_aerr_log2_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_BUS_TO (rw) register accessor: REGS__SS_CFG__SSCFG_V2A_BUS_TO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_bus_to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_bus_to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_bus_to`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_BUS_TO")]
pub type Regs_SsCfg_SscfgV2aBusTo =
    crate::Reg<regs__ss_cfg__sscfg_v2a_bus_to::Regs_SsCfg_SscfgV2aBusToSpec>;
#[doc = "REGS__SS_CFG__SSCFG_V2A_BUS_TO"]
pub mod regs__ss_cfg__sscfg_v2a_bus_to;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG (rw) register accessor: REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_int_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_int_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_int_raw_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG")]
pub type Regs_SsCfg_SscfgV2aIntRawReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_int_raw_reg::Regs_SsCfg_SscfgV2aIntRawRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_RAW_REG"]
pub mod regs__ss_cfg__sscfg_v2a_int_raw_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_STAT_REG (rw) register accessor: REGS__SS_CFG__SSCFG_V2A_INT_STAT_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_int_stat_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_int_stat_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_int_stat_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_INT_STAT_REG")]
pub type Regs_SsCfg_SscfgV2aIntStatReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_int_stat_reg::Regs_SsCfg_SscfgV2aIntStatRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_STAT_REG"]
pub mod regs__ss_cfg__sscfg_v2a_int_stat_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_SET_REG (rw) register accessor: REGS__SS_CFG__SSCFG_V2A_INT_SET_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_int_set_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_int_set_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_int_set_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_INT_SET_REG")]
pub type Regs_SsCfg_SscfgV2aIntSetReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_int_set_reg::Regs_SsCfg_SscfgV2aIntSetRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_SET_REG"]
pub mod regs__ss_cfg__sscfg_v2a_int_set_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_int_clr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_int_clr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_int_clr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG")]
pub type Regs_SsCfg_SscfgV2aIntClrReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_int_clr_reg::Regs_SsCfg_SscfgV2aIntClrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_V2A_INT_CLR_REG"]
pub mod regs__ss_cfg__sscfg_v2a_int_clr_reg;
#[doc = "REGS__SS_CFG__SSCFG_V2A_EOI_REG (rw) register accessor: REGS__SS_CFG__SSCFG_V2A_EOI_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_v2a_eoi_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_V2A_EOI_REG")]
pub type Regs_SsCfg_SscfgV2aEoiReg =
    crate::Reg<regs__ss_cfg__sscfg_v2a_eoi_reg::Regs_SsCfg_SscfgV2aEoiRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_V2A_EOI_REG"]
pub mod regs__ss_cfg__sscfg_v2a_eoi_reg;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT_SEL_REG (rw) register accessor: The Performance Counter Select register is used to select the statistic type to be counted in the corresponding Performance Counter register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_perf_cnt_sel_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_perf_cnt_sel_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_perf_cnt_sel_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PERF_CNT_SEL_REG")]
pub type Regs_SsCfg_SscfgPerfCntSelReg =
    crate::Reg<regs__ss_cfg__sscfg_perf_cnt_sel_reg::Regs_SsCfg_SscfgPerfCntSelRegSpec>;
#[doc = "The Performance Counter Select register is used to select the statistic type to be counted in the corresponding Performance Counter register."]
pub mod regs__ss_cfg__sscfg_perf_cnt_sel_reg;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT1_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PERF_CNT1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_perf_cnt1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_perf_cnt1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_perf_cnt1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PERF_CNT1_REG")]
pub type Regs_SsCfg_SscfgPerfCnt1Reg =
    crate::Reg<regs__ss_cfg__sscfg_perf_cnt1_reg::Regs_SsCfg_SscfgPerfCnt1RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT1_REG"]
pub mod regs__ss_cfg__sscfg_perf_cnt1_reg;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT2_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PERF_CNT2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_perf_cnt2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_perf_cnt2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_perf_cnt2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PERF_CNT2_REG")]
pub type Regs_SsCfg_SscfgPerfCnt2Reg =
    crate::Reg<regs__ss_cfg__sscfg_perf_cnt2_reg::Regs_SsCfg_SscfgPerfCnt2RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT2_REG"]
pub mod regs__ss_cfg__sscfg_perf_cnt2_reg;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT3_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PERF_CNT3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_perf_cnt3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_perf_cnt3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_perf_cnt3_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PERF_CNT3_REG")]
pub type Regs_SsCfg_SscfgPerfCnt3Reg =
    crate::Reg<regs__ss_cfg__sscfg_perf_cnt3_reg::Regs_SsCfg_SscfgPerfCnt3RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT3_REG"]
pub mod regs__ss_cfg__sscfg_perf_cnt3_reg;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT4_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PERF_CNT4_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_perf_cnt4_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_perf_cnt4_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_perf_cnt4_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PERF_CNT4_REG")]
pub type Regs_SsCfg_SscfgPerfCnt4Reg =
    crate::Reg<regs__ss_cfg__sscfg_perf_cnt4_reg::Regs_SsCfg_SscfgPerfCnt4RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PERF_CNT4_REG"]
pub mod regs__ss_cfg__sscfg_perf_cnt4_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_CTRL_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_CTRL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_ctrl_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_CTRL_REG")]
pub type Regs_SsCfg_SscfgEccCtrlReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_ctrl_reg::Regs_SsCfg_SscfgEccCtrlRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_CTRL_REG"]
pub mod regs__ss_cfg__sscfg_ecc_ctrl_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_rid_indx_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_rid_indx_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_rid_indx_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG")]
pub type Regs_SsCfg_SscfgEccRidIndxReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_rid_indx_reg::Regs_SsCfg_SscfgEccRidIndxRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_RID_INDX_REG"]
pub mod regs__ss_cfg__sscfg_ecc_rid_indx_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_rid_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_rid_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_rid_val_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG")]
pub type Regs_SsCfg_SscfgEccRidValReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_rid_val_reg::Regs_SsCfg_SscfgEccRidValRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_RID_VAL_REG"]
pub mod regs__ss_cfg__sscfg_ecc_rid_val_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R0_STR_ADDR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_R0_STR_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r0_str_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r0_str_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_r0_str_addr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_R0_STR_ADDR_REG")]
pub type Regs_SsCfg_SscfgEccR0StrAddrReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_r0_str_addr_reg::Regs_SsCfg_SscfgEccR0StrAddrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R0_STR_ADDR_REG"]
pub mod regs__ss_cfg__sscfg_ecc_r0_str_addr_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R0_END_ADDR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_R0_END_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r0_end_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r0_end_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_r0_end_addr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_R0_END_ADDR_REG")]
pub type Regs_SsCfg_SscfgEccR0EndAddrReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_r0_end_addr_reg::Regs_SsCfg_SscfgEccR0EndAddrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R0_END_ADDR_REG"]
pub mod regs__ss_cfg__sscfg_ecc_r0_end_addr_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R1_STR_ADDR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_R1_STR_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r1_str_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r1_str_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_r1_str_addr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_R1_STR_ADDR_REG")]
pub type Regs_SsCfg_SscfgEccR1StrAddrReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_r1_str_addr_reg::Regs_SsCfg_SscfgEccR1StrAddrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R1_STR_ADDR_REG"]
pub mod regs__ss_cfg__sscfg_ecc_r1_str_addr_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R1_END_ADDR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_R1_END_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r1_end_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r1_end_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_r1_end_addr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_R1_END_ADDR_REG")]
pub type Regs_SsCfg_SscfgEccR1EndAddrReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_r1_end_addr_reg::Regs_SsCfg_SscfgEccR1EndAddrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R1_END_ADDR_REG"]
pub mod regs__ss_cfg__sscfg_ecc_r1_end_addr_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R2_STR_ADDR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_R2_STR_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r2_str_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r2_str_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_r2_str_addr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_R2_STR_ADDR_REG")]
pub type Regs_SsCfg_SscfgEccR2StrAddrReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_r2_str_addr_reg::Regs_SsCfg_SscfgEccR2StrAddrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R2_STR_ADDR_REG"]
pub mod regs__ss_cfg__sscfg_ecc_r2_str_addr_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_r2_end_addr_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG")]
pub type Regs_SsCfg_SscfgEccR2EndAddrReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_r2_end_addr_reg::Regs_SsCfg_SscfgEccR2EndAddrRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_R2_END_ADDR_REG"]
pub mod regs__ss_cfg__sscfg_ecc_r2_end_addr_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG")]
pub type Regs_SsCfg_SscfgEcc1bErrCntReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::Regs_SsCfg_SscfgEcc1bErrCntRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG"]
pub mod regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG")]
pub type Regs_SsCfg_SscfgEcc1bErrThrshReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::Regs_SsCfg_SscfgEcc1bErrThrshRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG"]
pub mod regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG")]
pub type Regs_SsCfg_SscfgEcc1bErrAdrLogReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG"]
pub mod regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG")]
pub type Regs_SsCfg_SscfgEcc1bErrMskLogReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG"]
pub mod regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG")]
pub type Regs_SsCfg_SscfgEcc2bErrAdrLogReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG"]
pub mod regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg;
#[doc = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_MSK_LOG_REG (rw) register accessor: REGS__SS_CFG__SSCFG_ECC_2B_ERR_MSK_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_MSK_LOG_REG")]
pub type Regs_SsCfg_SscfgEcc2bErrMskLogReg =
    crate::Reg<regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg::Regs_SsCfg_SscfgEcc2bErrMskLogRegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_MSK_LOG_REG"]
pub mod regs__ss_cfg__sscfg_ecc_2b_err_msk_log_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl1Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl1_reg::Regs_SsCfg_SscfgPhyTestCtrl1RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl1_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl2Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl2_reg::Regs_SsCfg_SscfgPhyTestCtrl2RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl2_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl3_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl3Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl3_reg::Regs_SsCfg_SscfgPhyTestCtrl3RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl3_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl4_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl4_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl4_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl4Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl4_reg::Regs_SsCfg_SscfgPhyTestCtrl4RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl4_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl5_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl5_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl5_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl5Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl5_reg::Regs_SsCfg_SscfgPhyTestCtrl5RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl5_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl6_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl6_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl6_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl6Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl6_reg::Regs_SsCfg_SscfgPhyTestCtrl6RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl6_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl7_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl7_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl7_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl7Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl7_reg::Regs_SsCfg_SscfgPhyTestCtrl7RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl7_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl8_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl8_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl8_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl8Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl8_reg::Regs_SsCfg_SscfgPhyTestCtrl8RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl8_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl9_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl9_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl9_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl9Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl9_reg::Regs_SsCfg_SscfgPhyTestCtrl9RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl9_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl10_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl10_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_ctrl10_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG")]
pub type Regs_SsCfg_SscfgPhyTestCtrl10Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_ctrl10_reg::Regs_SsCfg_SscfgPhyTestCtrl10RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_ctrl10_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_stat1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_stat1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_stat1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG")]
pub type Regs_SsCfg_SscfgPhyTestStat1Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_stat1_reg::Regs_SsCfg_SscfgPhyTestStat1RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_stat1_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG (rw) register accessor: REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_stat2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_stat2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_test_stat2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG")]
pub type Regs_SsCfg_SscfgPhyTestStat2Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_test_stat2_reg::Regs_SsCfg_SscfgPhyTestStat2RegSpec>;
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG"]
pub mod regs__ss_cfg__sscfg_phy_test_stat2_reg;
