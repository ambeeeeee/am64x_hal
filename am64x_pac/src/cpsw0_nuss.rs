#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpsw_nuss_vbusp_cpsw_nuss_idver_reg: CpswNussVbuspCpswNussIdverReg,
    cpsw_nuss_vbusp_synce_count_reg: CpswNussVbuspSynceCountReg,
    cpsw_nuss_vbusp_synce_mux_reg: CpswNussVbuspSynceMuxReg,
    cpsw_nuss_vbusp_control_reg: CpswNussVbuspControlReg,
    cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg: CpswNussVbuspSgmiiNonFiberModeReg,
    cpsw_nuss_vbusp_serdes_reset_iso_reg: CpswNussVbuspSerdesResetIsoReg,
    _reserved6: [u8; 0x04],
    cpsw_nuss_vbusp_subssystem_status_reg: CpswNussVbuspSubssystemStatusReg,
    cpsw_nuss_vbusp_subsystem_config_reg: CpswNussVbuspSubsystemConfigReg,
    _reserved8: [u8; 0x0c],
    cpsw_nuss_vbusp_rgmii1_status_reg: CpswNussVbuspRgmii1StatusReg,
    cpsw_nuss_vbusp_rgmii2_status_reg: CpswNussVbuspRgmii2StatusReg,
    _reserved10: [u8; 0xc8],
    cpsw_nuss_vbusp_sgmii_idver_reg: CpswNussVbuspSgmiiIdverReg,
    cpsw_nuss_vbusp_soft_reset_reg: CpswNussVbuspSoftResetReg,
    _reserved12: [u8; 0x08],
    cpsw_nuss_vbusp_control_reg_: CpswNussVbuspControlReg_,
    cpsw_nuss_vbusp_status_reg: CpswNussVbuspStatusReg,
    cpsw_nuss_vbusp_mr_adv_ability_reg: CpswNussVbuspMrAdvAbilityReg,
    cpsw_nuss_vbusp_mr_np_tx_reg: CpswNussVbuspMrNpTxReg,
    cpsw_nuss_vbusp_mr_lp_adv_ability_reg: CpswNussVbuspMrLpAdvAbilityReg,
    cpsw_nuss_vbusp_mr_lp_np_rx_reg: CpswNussVbuspMrLpNpRxReg,
    _reserved18: [u8; 0x18],
    cpsw_nuss_vbusp_diag_clear_reg: CpswNussVbuspDiagClearReg,
    cpsw_nuss_vbusp_diag_control_reg: CpswNussVbuspDiagControlReg,
    cpsw_nuss_vbusp_diag_status_reg: CpswNussVbuspDiagStatusReg,
    _reserved21: [u8; 0x0db4],
    cpsw_nuss_vbusp_mdio_version_reg: CpswNussVbuspMdioVersionReg,
    cpsw_nuss_vbusp_control_reg__: CpswNussVbuspControlReg_,
    cpsw_nuss_vbusp_alive_reg: CpswNussVbuspAliveReg,
    cpsw_nuss_vbusp_link_reg: CpswNussVbuspLinkReg,
    cpsw_nuss_vbusp_link_int_raw_reg: CpswNussVbuspLinkIntRawReg,
    cpsw_nuss_vbusp_link_int_masked_reg: CpswNussVbuspLinkIntMaskedReg,
    cpsw_nuss_vbusp_link_int_mask_set_reg: CpswNussVbuspLinkIntMaskSetReg,
    cpsw_nuss_vbusp_link_int_mask_clear_reg: CpswNussVbuspLinkIntMaskClearReg,
    cpsw_nuss_vbusp_user_int_raw_reg: CpswNussVbuspUserIntRawReg,
    cpsw_nuss_vbusp_user_int_masked_reg: CpswNussVbuspUserIntMaskedReg,
    cpsw_nuss_vbusp_user_int_mask_set_reg: CpswNussVbuspUserIntMaskSetReg,
    cpsw_nuss_vbusp_user_int_mask_clear_reg: CpswNussVbuspUserIntMaskClearReg,
    cpsw_nuss_vbusp_manual_if_reg: CpswNussVbuspManualIfReg,
    cpsw_nuss_vbusp_poll_reg: CpswNussVbuspPollReg,
    cpsw_nuss_vbusp_poll_en_reg: CpswNussVbuspPollEnReg,
    cpsw_nuss_vbusp_claus45_reg: CpswNussVbuspClaus45Reg,
    cpsw_nuss_vbusp_user_addr0_reg: CpswNussVbuspUserAddr0Reg,
    cpsw_nuss_vbusp_user_addr1_reg: CpswNussVbuspUserAddr1Reg,
    _reserved39: [u8; 0xb8],
    cpsw_nuss_vbusp_revision: CpswNussVbuspRevision,
    _reserved40: [u8; 0x0c],
    cpsw_nuss_vbusp_eoi_reg: CpswNussVbuspEoiReg,
    cpsw_nuss_vbusp_intr_vector_reg: CpswNussVbuspIntrVectorReg,
    _reserved42: [u8; 0xe8],
    cpsw_nuss_vbusp_enable_reg_out_pulse_0: CpswNussVbuspEnableRegOutPulse0,
    _reserved43: [u8; 0x01fc],
    cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0: CpswNussVbuspEnableClrRegOutPulse0,
    _reserved44: [u8; 0x01fc],
    cpsw_nuss_vbusp_status_reg_out_pulse_0: CpswNussVbuspStatusRegOutPulse0,
    _reserved45: [u8; 0x057c],
    cpsw_nuss_vbusp_intr_vector_reg_out_pulse: CpswNussVbuspIntrVectorRegOutPulse,
    _reserved46: [u8; 0x0001_e57c],
    cpsw_nuss_vbusp_cpsw_id_ver_reg: CpswNussVbuspCpswIdVerReg,
    cpsw_nuss_vbusp_control_reg___: CpswNussVbuspControlReg_,
    _reserved48: [u8; 0x08],
    cpsw_nuss_vbusp_em_control_reg: CpswNussVbuspEmControlReg,
    cpsw_nuss_vbusp_stat_port_en_reg: CpswNussVbuspStatPortEnReg,
    cpsw_nuss_vbusp_ptype_reg: CpswNussVbuspPtypeReg,
    cpsw_nuss_vbusp_soft_idle_reg: CpswNussVbuspSoftIdleReg,
    cpsw_nuss_vbusp_thru_rate_reg: CpswNussVbuspThruRateReg,
    cpsw_nuss_vbusp_gap_thresh_reg: CpswNussVbuspGapThreshReg,
    _reserved54: [u8; 0x04],
    cpsw_nuss_vbusp_eee_prescale_reg: CpswNussVbuspEeePrescaleReg,
    cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg: CpswNussVbuspTxGOflowThreshSetReg,
    cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg: CpswNussVbuspTxGOflowThreshClrReg,
    cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg: CpswNussVbuspTxGBufThreshSetLReg,
    cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg: CpswNussVbuspTxGBufThreshSetHReg,
    cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg: CpswNussVbuspTxGBufThreshClrLReg,
    cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg: CpswNussVbuspTxGBufThreshClrHReg,
    _reserved61: [u8; 0x08],
    cpsw_nuss_vbusp_vlan_ltype_reg: CpswNussVbuspVlanLtypeReg,
    cpsw_nuss_vbusp_est_ts_domain_reg: CpswNussVbuspEstTsDomainReg,
    cpsw_nuss_vbusp_cut_threshold_reg: CpswNussVbuspCutThresholdReg,
    cpsw_nuss_vbusp_frequency_reg: CpswNussVbuspFrequencyReg,
    cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg: CpswNussVbuspIetHoldCntLdValReg,
    _reserved66: [u8; 0x9c],
    cpsw_nuss_vbusp_tx_pri0_maxlen_reg: CpswNussVbuspTxPri0MaxlenReg,
    cpsw_nuss_vbusp_tx_pri1_maxlen_reg: CpswNussVbuspTxPri1MaxlenReg,
    cpsw_nuss_vbusp_tx_pri2_maxlen_reg: CpswNussVbuspTxPri2MaxlenReg,
    cpsw_nuss_vbusp_tx_pri3_maxlen_reg: CpswNussVbuspTxPri3MaxlenReg,
    cpsw_nuss_vbusp_tx_pri4_maxlen_reg: CpswNussVbuspTxPri4MaxlenReg,
    cpsw_nuss_vbusp_tx_pri5_maxlen_reg: CpswNussVbuspTxPri5MaxlenReg,
    cpsw_nuss_vbusp_tx_pri6_maxlen_reg: CpswNussVbuspTxPri6MaxlenReg,
    cpsw_nuss_vbusp_tx_pri7_maxlen_reg: CpswNussVbuspTxPri7MaxlenReg,
}
impl RegisterBlock {
    #[doc = "0x00 - ID Version Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_cpsw_nuss_idver_reg(&self) -> &CpswNussVbuspCpswNussIdverReg {
        &self.cpsw_nuss_vbusp_cpsw_nuss_idver_reg
    }
    #[doc = "0x04 - SyncE Count Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_synce_count_reg(&self) -> &CpswNussVbuspSynceCountReg {
        &self.cpsw_nuss_vbusp_synce_count_reg
    }
    #[doc = "0x08 - SyncE Mux Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_synce_mux_reg(&self) -> &CpswNussVbuspSynceMuxReg {
        &self.cpsw_nuss_vbusp_synce_mux_reg
    }
    #[doc = "0x0c - Control Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_control_reg(&self) -> &CpswNussVbuspControlReg {
        &self.cpsw_nuss_vbusp_control_reg
    }
    #[doc = "0x10 - SGMII NON FIBER Mode Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg(
        &self,
    ) -> &CpswNussVbuspSgmiiNonFiberModeReg {
        &self.cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg
    }
    #[doc = "0x14 - SyncE Mux Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_serdes_reset_iso_reg(&self) -> &CpswNussVbuspSerdesResetIsoReg {
        &self.cpsw_nuss_vbusp_serdes_reset_iso_reg
    }
    #[doc = "0x1c - Subsystem Status Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_subssystem_status_reg(&self) -> &CpswNussVbuspSubssystemStatusReg {
        &self.cpsw_nuss_vbusp_subssystem_status_reg
    }
    #[doc = "0x20 - Subsystem Configuration Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_subsystem_config_reg(&self) -> &CpswNussVbuspSubsystemConfigReg {
        &self.cpsw_nuss_vbusp_subsystem_config_reg
    }
    #[doc = "0x30 - RGMII1 Status Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_rgmii1_status_reg(&self) -> &CpswNussVbuspRgmii1StatusReg {
        &self.cpsw_nuss_vbusp_rgmii1_status_reg
    }
    #[doc = "0x34 - RGMII2 Status Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_rgmii2_status_reg(&self) -> &CpswNussVbuspRgmii2StatusReg {
        &self.cpsw_nuss_vbusp_rgmii2_status_reg
    }
    #[doc = "0x100 - SGMII IDVER register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_sgmii_idver_reg(&self) -> &CpswNussVbuspSgmiiIdverReg {
        &self.cpsw_nuss_vbusp_sgmii_idver_reg
    }
    #[doc = "0x104 - SGMII Soft Reset Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_soft_reset_reg(&self) -> &CpswNussVbuspSoftResetReg {
        &self.cpsw_nuss_vbusp_soft_reset_reg
    }
    #[doc = "0x110 - SGMII Control Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_control_reg_(&self) -> &CpswNussVbuspControlReg_ {
        &self.cpsw_nuss_vbusp_control_reg_
    }
    #[doc = "0x114 - SGMII Status Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_status_reg(&self) -> &CpswNussVbuspStatusReg {
        &self.cpsw_nuss_vbusp_status_reg
    }
    #[doc = "0x118 - SGMII MR Advertized Ability Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_mr_adv_ability_reg(&self) -> &CpswNussVbuspMrAdvAbilityReg {
        &self.cpsw_nuss_vbusp_mr_adv_ability_reg
    }
    #[doc = "0x11c - SGMII Next Pate Transmit Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_mr_np_tx_reg(&self) -> &CpswNussVbuspMrNpTxReg {
        &self.cpsw_nuss_vbusp_mr_np_tx_reg
    }
    #[doc = "0x120 - SGMII Link Partner Advertized Ability Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_mr_lp_adv_ability_reg(&self) -> &CpswNussVbuspMrLpAdvAbilityReg {
        &self.cpsw_nuss_vbusp_mr_lp_adv_ability_reg
    }
    #[doc = "0x124 - SGMII Link Partner Next Page Receive Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_mr_lp_np_rx_reg(&self) -> &CpswNussVbuspMrLpNpRxReg {
        &self.cpsw_nuss_vbusp_mr_lp_np_rx_reg
    }
    #[doc = "0x140 - SGMII Diagnostics Clear Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_diag_clear_reg(&self) -> &CpswNussVbuspDiagClearReg {
        &self.cpsw_nuss_vbusp_diag_clear_reg
    }
    #[doc = "0x144 - SGMII Diagnostics Control Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_diag_control_reg(&self) -> &CpswNussVbuspDiagControlReg {
        &self.cpsw_nuss_vbusp_diag_control_reg
    }
    #[doc = "0x148 - SGMII Diagnostics Status Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_diag_status_reg(&self) -> &CpswNussVbuspDiagStatusReg {
        &self.cpsw_nuss_vbusp_diag_status_reg
    }
    #[doc = "0xf00 - MDIO Version Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_mdio_version_reg(&self) -> &CpswNussVbuspMdioVersionReg {
        &self.cpsw_nuss_vbusp_mdio_version_reg
    }
    #[doc = "0xf04 - MDIO Control Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_control_reg__(&self) -> &CpswNussVbuspControlReg_ {
        &self.cpsw_nuss_vbusp_control_reg__
    }
    #[doc = "0xf08 - MDIO Alive Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_alive_reg(&self) -> &CpswNussVbuspAliveReg {
        &self.cpsw_nuss_vbusp_alive_reg
    }
    #[doc = "0xf0c - MDIO Link Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_link_reg(&self) -> &CpswNussVbuspLinkReg {
        &self.cpsw_nuss_vbusp_link_reg
    }
    #[doc = "0xf10 - MDIO Link Interrupt Raw Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_link_int_raw_reg(&self) -> &CpswNussVbuspLinkIntRawReg {
        &self.cpsw_nuss_vbusp_link_int_raw_reg
    }
    #[doc = "0xf14 - MDIO Link Interrupt Masked Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_link_int_masked_reg(&self) -> &CpswNussVbuspLinkIntMaskedReg {
        &self.cpsw_nuss_vbusp_link_int_masked_reg
    }
    #[doc = "0xf18 - MDIO Link Interrupt Mask Set Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_link_int_mask_set_reg(&self) -> &CpswNussVbuspLinkIntMaskSetReg {
        &self.cpsw_nuss_vbusp_link_int_mask_set_reg
    }
    #[doc = "0xf1c - MDIO Link Interrupt Mask Clear Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_link_int_mask_clear_reg(
        &self,
    ) -> &CpswNussVbuspLinkIntMaskClearReg {
        &self.cpsw_nuss_vbusp_link_int_mask_clear_reg
    }
    #[doc = "0xf20 - MDIO User Interrupt Raw Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_user_int_raw_reg(&self) -> &CpswNussVbuspUserIntRawReg {
        &self.cpsw_nuss_vbusp_user_int_raw_reg
    }
    #[doc = "0xf24 - MDIO User Interrupt Masked Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_user_int_masked_reg(&self) -> &CpswNussVbuspUserIntMaskedReg {
        &self.cpsw_nuss_vbusp_user_int_masked_reg
    }
    #[doc = "0xf28 - MDIO User Interrupt Mask Set Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_user_int_mask_set_reg(&self) -> &CpswNussVbuspUserIntMaskSetReg {
        &self.cpsw_nuss_vbusp_user_int_mask_set_reg
    }
    #[doc = "0xf2c - MDIO User Interrupt Mask Clear Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_user_int_mask_clear_reg(
        &self,
    ) -> &CpswNussVbuspUserIntMaskClearReg {
        &self.cpsw_nuss_vbusp_user_int_mask_clear_reg
    }
    #[doc = "0xf30 - MDIO Manual Interface Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_manual_if_reg(&self) -> &CpswNussVbuspManualIfReg {
        &self.cpsw_nuss_vbusp_manual_if_reg
    }
    #[doc = "0xf34 - MDIO Poll Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_poll_reg(&self) -> &CpswNussVbuspPollReg {
        &self.cpsw_nuss_vbusp_poll_reg
    }
    #[doc = "0xf38 - MDIO Poll Enable Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_poll_en_reg(&self) -> &CpswNussVbuspPollEnReg {
        &self.cpsw_nuss_vbusp_poll_en_reg
    }
    #[doc = "0xf3c - MDIO Clause45 Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_claus45_reg(&self) -> &CpswNussVbuspClaus45Reg {
        &self.cpsw_nuss_vbusp_claus45_reg
    }
    #[doc = "0xf40 - MDIO Address 0 Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_user_addr0_reg(&self) -> &CpswNussVbuspUserAddr0Reg {
        &self.cpsw_nuss_vbusp_user_addr0_reg
    }
    #[doc = "0xf44 - MDIO Address 1 Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_user_addr1_reg(&self) -> &CpswNussVbuspUserAddr1Reg {
        &self.cpsw_nuss_vbusp_user_addr1_reg
    }
    #[doc = "0x1000 - Revision Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_revision(&self) -> &CpswNussVbuspRevision {
        &self.cpsw_nuss_vbusp_revision
    }
    #[doc = "0x1010 - End of Interrupt Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_eoi_reg(&self) -> &CpswNussVbuspEoiReg {
        &self.cpsw_nuss_vbusp_eoi_reg
    }
    #[doc = "0x1014 - Interrupt Vector Register"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_intr_vector_reg(&self) -> &CpswNussVbuspIntrVectorReg {
        &self.cpsw_nuss_vbusp_intr_vector_reg
    }
    #[doc = "0x1100 - Enable Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_enable_reg_out_pulse_0(&self) -> &CpswNussVbuspEnableRegOutPulse0 {
        &self.cpsw_nuss_vbusp_enable_reg_out_pulse_0
    }
    #[doc = "0x1300 - Enable Clear Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0(
        &self,
    ) -> &CpswNussVbuspEnableClrRegOutPulse0 {
        &self.cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0
    }
    #[doc = "0x1500 - Status Register 0"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_status_reg_out_pulse_0(&self) -> &CpswNussVbuspStatusRegOutPulse0 {
        &self.cpsw_nuss_vbusp_status_reg_out_pulse_0
    }
    #[doc = "0x1a80 - Interrupt Vector for out_pulse"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_intr_vector_reg_out_pulse(
        &self,
    ) -> &CpswNussVbuspIntrVectorRegOutPulse {
        &self.cpsw_nuss_vbusp_intr_vector_reg_out_pulse
    }
    #[doc = "0x20000 - CPSW ID Version"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_cpsw_id_ver_reg(&self) -> &CpswNussVbuspCpswIdVerReg {
        &self.cpsw_nuss_vbusp_cpsw_id_ver_reg
    }
    #[doc = "0x20004 - CPSW Switch Control"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_control_reg___(&self) -> &CpswNussVbuspControlReg_ {
        &self.cpsw_nuss_vbusp_control_reg___
    }
    #[doc = "0x20010 - CPSW Emulation Control"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_em_control_reg(&self) -> &CpswNussVbuspEmControlReg {
        &self.cpsw_nuss_vbusp_em_control_reg
    }
    #[doc = "0x20014 - CPSW Statistics Port Enable"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_stat_port_en_reg(&self) -> &CpswNussVbuspStatPortEnReg {
        &self.cpsw_nuss_vbusp_stat_port_en_reg
    }
    #[doc = "0x20018 - CPSW Transmit Priority Type"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_ptype_reg(&self) -> &CpswNussVbuspPtypeReg {
        &self.cpsw_nuss_vbusp_ptype_reg
    }
    #[doc = "0x2001c - CPSW Software Idle"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_soft_idle_reg(&self) -> &CpswNussVbuspSoftIdleReg {
        &self.cpsw_nuss_vbusp_soft_idle_reg
    }
    #[doc = "0x20020 - CPSW Thru Rate"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_thru_rate_reg(&self) -> &CpswNussVbuspThruRateReg {
        &self.cpsw_nuss_vbusp_thru_rate_reg
    }
    #[doc = "0x20024 - CPSW Transmit FIFO Short Gap Threshold"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_gap_thresh_reg(&self) -> &CpswNussVbuspGapThreshReg {
        &self.cpsw_nuss_vbusp_gap_thresh_reg
    }
    #[doc = "0x2002c - CPSW Energy Efficient Ethernet Prescale Value"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_eee_prescale_reg(&self) -> &CpswNussVbuspEeePrescaleReg {
        &self.cpsw_nuss_vbusp_eee_prescale_reg
    }
    #[doc = "0x20030 - CPSW PFC Tx Global Out Flow Threshold Set"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg(
        &self,
    ) -> &CpswNussVbuspTxGOflowThreshSetReg {
        &self.cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg
    }
    #[doc = "0x20034 - CPSW PFC Tx Global Out Flow Threshold Clear"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg(
        &self,
    ) -> &CpswNussVbuspTxGOflowThreshClrReg {
        &self.cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg
    }
    #[doc = "0x20038 - CPSW PFC Global Tx Buffer Threshold Set Low"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg(
        &self,
    ) -> &CpswNussVbuspTxGBufThreshSetLReg {
        &self.cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg
    }
    #[doc = "0x2003c - CPSW PFC Global Tx Buffer Threshold Set High"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg(
        &self,
    ) -> &CpswNussVbuspTxGBufThreshSetHReg {
        &self.cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg
    }
    #[doc = "0x20040 - CPSW PFC Global Tx Buffer Threshold Clear Low"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg(
        &self,
    ) -> &CpswNussVbuspTxGBufThreshClrLReg {
        &self.cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg
    }
    #[doc = "0x20044 - CPSW PFC Global Tx Buffer Threshold Clear High"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg(
        &self,
    ) -> &CpswNussVbuspTxGBufThreshClrHReg {
        &self.cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg
    }
    #[doc = "0x20050 - VLAN Length/type"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_vlan_ltype_reg(&self) -> &CpswNussVbuspVlanLtypeReg {
        &self.cpsw_nuss_vbusp_vlan_ltype_reg
    }
    #[doc = "0x20054 - Enhanced Scheduled Traffic Host Event Domain"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_est_ts_domain_reg(&self) -> &CpswNussVbuspEstTsDomainReg {
        &self.cpsw_nuss_vbusp_est_ts_domain_reg
    }
    #[doc = "0x20058 - Cut-thru Threshold"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_cut_threshold_reg(&self) -> &CpswNussVbuspCutThresholdReg {
        &self.cpsw_nuss_vbusp_cut_threshold_reg
    }
    #[doc = "0x2005c - CPSW CPPI_CLK Frequency in Mhz"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_frequency_reg(&self) -> &CpswNussVbuspFrequencyReg {
        &self.cpsw_nuss_vbusp_frequency_reg
    }
    #[doc = "0x20060 - IET Hold Count Load Value"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg(
        &self,
    ) -> &CpswNussVbuspIetHoldCntLdValReg {
        &self.cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg
    }
    #[doc = "0x20100 - Transmit Priority 0 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri0_maxlen_reg(&self) -> &CpswNussVbuspTxPri0MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri0_maxlen_reg
    }
    #[doc = "0x20104 - Transmit Priority 1 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri1_maxlen_reg(&self) -> &CpswNussVbuspTxPri1MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri1_maxlen_reg
    }
    #[doc = "0x20108 - Transmit Priority 2 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri2_maxlen_reg(&self) -> &CpswNussVbuspTxPri2MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri2_maxlen_reg
    }
    #[doc = "0x2010c - Transmit Priority 3 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri3_maxlen_reg(&self) -> &CpswNussVbuspTxPri3MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri3_maxlen_reg
    }
    #[doc = "0x20110 - Transmit Priority 4 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri4_maxlen_reg(&self) -> &CpswNussVbuspTxPri4MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri4_maxlen_reg
    }
    #[doc = "0x20114 - Transmit Priority 5 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri5_maxlen_reg(&self) -> &CpswNussVbuspTxPri5MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri5_maxlen_reg
    }
    #[doc = "0x20118 - Transmit Priority 6 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri6_maxlen_reg(&self) -> &CpswNussVbuspTxPri6MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri6_maxlen_reg
    }
    #[doc = "0x2011c - Transmit Priority 7 Maximum Length"]
    #[inline(always)]
    pub const fn cpsw_nuss_vbusp_tx_pri7_maxlen_reg(&self) -> &CpswNussVbuspTxPri7MaxlenReg {
        &self.cpsw_nuss_vbusp_tx_pri7_maxlen_reg
    }
}
#[doc = "CPSW_NUSS_VBUSP_CPSW_NUSS_IDVER_REG (rw) register accessor: ID Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_cpsw_nuss_idver_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_cpsw_nuss_idver_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_cpsw_nuss_idver_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CPSW_NUSS_IDVER_REG")]
pub type CpswNussVbuspCpswNussIdverReg =
    crate::Reg<cpsw_nuss_vbusp_cpsw_nuss_idver_reg::CpswNussVbuspCpswNussIdverRegSpec>;
#[doc = "ID Version Register"]
pub mod cpsw_nuss_vbusp_cpsw_nuss_idver_reg;
#[doc = "CPSW_NUSS_VBUSP_SYNCE_COUNT_REG (rw) register accessor: SyncE Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_synce_count_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_synce_count_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_synce_count_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SYNCE_COUNT_REG")]
pub type CpswNussVbuspSynceCountReg =
    crate::Reg<cpsw_nuss_vbusp_synce_count_reg::CpswNussVbuspSynceCountRegSpec>;
#[doc = "SyncE Count Register"]
pub mod cpsw_nuss_vbusp_synce_count_reg;
#[doc = "CPSW_NUSS_VBUSP_SYNCE_MUX_REG (rw) register accessor: SyncE Mux Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_synce_mux_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_synce_mux_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_synce_mux_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SYNCE_MUX_REG")]
pub type CpswNussVbuspSynceMuxReg =
    crate::Reg<cpsw_nuss_vbusp_synce_mux_reg::CpswNussVbuspSynceMuxRegSpec>;
#[doc = "SyncE Mux Register"]
pub mod cpsw_nuss_vbusp_synce_mux_reg;
#[doc = "CPSW_NUSS_VBUSP_CONTROL_REG (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_control_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CONTROL_REG")]
pub type CpswNussVbuspControlReg =
    crate::Reg<cpsw_nuss_vbusp_control_reg::CpswNussVbuspControlRegSpec>;
#[doc = "Control Register"]
pub mod cpsw_nuss_vbusp_control_reg;
#[doc = "CPSW_NUSS_VBUSP_SGMII_NON_FIBER_MODE_REG (rw) register accessor: SGMII NON FIBER Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SGMII_NON_FIBER_MODE_REG")]
pub type CpswNussVbuspSgmiiNonFiberModeReg =
    crate::Reg<cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::CpswNussVbuspSgmiiNonFiberModeRegSpec>;
#[doc = "SGMII NON FIBER Mode Register"]
pub mod cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg;
#[doc = "CPSW_NUSS_VBUSP_SERDES_RESET_ISO_REG (rw) register accessor: SyncE Mux Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_serdes_reset_iso_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_serdes_reset_iso_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_serdes_reset_iso_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SERDES_RESET_ISO_REG")]
pub type CpswNussVbuspSerdesResetIsoReg =
    crate::Reg<cpsw_nuss_vbusp_serdes_reset_iso_reg::CpswNussVbuspSerdesResetIsoRegSpec>;
#[doc = "SyncE Mux Register"]
pub mod cpsw_nuss_vbusp_serdes_reset_iso_reg;
#[doc = "CPSW_NUSS_VBUSP_SUBSSYSTEM_STATUS_REG (rw) register accessor: Subsystem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_subssystem_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_subssystem_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_subssystem_status_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SUBSSYSTEM_STATUS_REG")]
pub type CpswNussVbuspSubssystemStatusReg =
    crate::Reg<cpsw_nuss_vbusp_subssystem_status_reg::CpswNussVbuspSubssystemStatusRegSpec>;
#[doc = "Subsystem Status Register"]
pub mod cpsw_nuss_vbusp_subssystem_status_reg;
#[doc = "CPSW_NUSS_VBUSP_SUBSYSTEM_CONFIG_REG (rw) register accessor: Subsystem Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_subsystem_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_subsystem_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_subsystem_config_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SUBSYSTEM_CONFIG_REG")]
pub type CpswNussVbuspSubsystemConfigReg =
    crate::Reg<cpsw_nuss_vbusp_subsystem_config_reg::CpswNussVbuspSubsystemConfigRegSpec>;
#[doc = "Subsystem Configuration Register"]
pub mod cpsw_nuss_vbusp_subsystem_config_reg;
#[doc = "CPSW_NUSS_VBUSP_RGMII1_STATUS_REG (rw) register accessor: RGMII1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_rgmii1_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_rgmii1_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_rgmii1_status_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_RGMII1_STATUS_REG")]
pub type CpswNussVbuspRgmii1StatusReg =
    crate::Reg<cpsw_nuss_vbusp_rgmii1_status_reg::CpswNussVbuspRgmii1StatusRegSpec>;
#[doc = "RGMII1 Status Register"]
pub mod cpsw_nuss_vbusp_rgmii1_status_reg;
#[doc = "CPSW_NUSS_VBUSP_RGMII2_STATUS_REG (rw) register accessor: RGMII2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_rgmii2_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_rgmii2_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_rgmii2_status_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_RGMII2_STATUS_REG")]
pub type CpswNussVbuspRgmii2StatusReg =
    crate::Reg<cpsw_nuss_vbusp_rgmii2_status_reg::CpswNussVbuspRgmii2StatusRegSpec>;
#[doc = "RGMII2 Status Register"]
pub mod cpsw_nuss_vbusp_rgmii2_status_reg;
#[doc = "CPSW_NUSS_VBUSP_SGMII_IDVER_REG (rw) register accessor: SGMII IDVER register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_sgmii_idver_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_sgmii_idver_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_sgmii_idver_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SGMII_IDVER_REG")]
pub type CpswNussVbuspSgmiiIdverReg =
    crate::Reg<cpsw_nuss_vbusp_sgmii_idver_reg::CpswNussVbuspSgmiiIdverRegSpec>;
#[doc = "SGMII IDVER register"]
pub mod cpsw_nuss_vbusp_sgmii_idver_reg;
#[doc = "CPSW_NUSS_VBUSP_SOFT_RESET_REG (rw) register accessor: SGMII Soft Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_soft_reset_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_soft_reset_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_soft_reset_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SOFT_RESET_REG")]
pub type CpswNussVbuspSoftResetReg =
    crate::Reg<cpsw_nuss_vbusp_soft_reset_reg::CpswNussVbuspSoftResetRegSpec>;
#[doc = "SGMII Soft Reset Register"]
pub mod cpsw_nuss_vbusp_soft_reset_reg;
#[doc = "CPSW_NUSS_VBUSP_CONTROL_REG_ (rw) register accessor: SGMII Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_control_reg_`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CONTROL_REG_")]
pub type CpswNussVbuspControlReg_ =
    crate::Reg<cpsw_nuss_vbusp_control_reg_::CpswNussVbuspControlReg_Spec>;
#[doc = "SGMII Control Register"]
pub mod cpsw_nuss_vbusp_control_reg_;
#[doc = "CPSW_NUSS_VBUSP_STATUS_REG (rw) register accessor: SGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_status_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_STATUS_REG")]
pub type CpswNussVbuspStatusReg =
    crate::Reg<cpsw_nuss_vbusp_status_reg::CpswNussVbuspStatusRegSpec>;
#[doc = "SGMII Status Register"]
pub mod cpsw_nuss_vbusp_status_reg;
#[doc = "CPSW_NUSS_VBUSP_MR_ADV_ABILITY_REG (rw) register accessor: SGMII MR Advertized Ability Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_adv_ability_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_adv_ability_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_mr_adv_ability_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_MR_ADV_ABILITY_REG")]
pub type CpswNussVbuspMrAdvAbilityReg =
    crate::Reg<cpsw_nuss_vbusp_mr_adv_ability_reg::CpswNussVbuspMrAdvAbilityRegSpec>;
#[doc = "SGMII MR Advertized Ability Register"]
pub mod cpsw_nuss_vbusp_mr_adv_ability_reg;
#[doc = "CPSW_NUSS_VBUSP_MR_NP_TX_REG (rw) register accessor: SGMII Next Pate Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_np_tx_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_np_tx_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_mr_np_tx_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_MR_NP_TX_REG")]
pub type CpswNussVbuspMrNpTxReg =
    crate::Reg<cpsw_nuss_vbusp_mr_np_tx_reg::CpswNussVbuspMrNpTxRegSpec>;
#[doc = "SGMII Next Pate Transmit Register"]
pub mod cpsw_nuss_vbusp_mr_np_tx_reg;
#[doc = "CPSW_NUSS_VBUSP_MR_LP_ADV_ABILITY_REG (rw) register accessor: SGMII Link Partner Advertized Ability Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_lp_adv_ability_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_lp_adv_ability_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_mr_lp_adv_ability_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_MR_LP_ADV_ABILITY_REG")]
pub type CpswNussVbuspMrLpAdvAbilityReg =
    crate::Reg<cpsw_nuss_vbusp_mr_lp_adv_ability_reg::CpswNussVbuspMrLpAdvAbilityRegSpec>;
#[doc = "SGMII Link Partner Advertized Ability Register"]
pub mod cpsw_nuss_vbusp_mr_lp_adv_ability_reg;
#[doc = "CPSW_NUSS_VBUSP_MR_LP_NP_RX_REG (rw) register accessor: SGMII Link Partner Next Page Receive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_lp_np_rx_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_lp_np_rx_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_mr_lp_np_rx_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_MR_LP_NP_RX_REG")]
pub type CpswNussVbuspMrLpNpRxReg =
    crate::Reg<cpsw_nuss_vbusp_mr_lp_np_rx_reg::CpswNussVbuspMrLpNpRxRegSpec>;
#[doc = "SGMII Link Partner Next Page Receive Register"]
pub mod cpsw_nuss_vbusp_mr_lp_np_rx_reg;
#[doc = "CPSW_NUSS_VBUSP_DIAG_CLEAR_REG (rw) register accessor: SGMII Diagnostics Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_diag_clear_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_diag_clear_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_diag_clear_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_DIAG_CLEAR_REG")]
pub type CpswNussVbuspDiagClearReg =
    crate::Reg<cpsw_nuss_vbusp_diag_clear_reg::CpswNussVbuspDiagClearRegSpec>;
#[doc = "SGMII Diagnostics Clear Register"]
pub mod cpsw_nuss_vbusp_diag_clear_reg;
#[doc = "CPSW_NUSS_VBUSP_DIAG_CONTROL_REG (rw) register accessor: SGMII Diagnostics Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_diag_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_diag_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_diag_control_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_DIAG_CONTROL_REG")]
pub type CpswNussVbuspDiagControlReg =
    crate::Reg<cpsw_nuss_vbusp_diag_control_reg::CpswNussVbuspDiagControlRegSpec>;
#[doc = "SGMII Diagnostics Control Register"]
pub mod cpsw_nuss_vbusp_diag_control_reg;
#[doc = "CPSW_NUSS_VBUSP_DIAG_STATUS_REG (rw) register accessor: SGMII Diagnostics Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_diag_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_diag_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_diag_status_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_DIAG_STATUS_REG")]
pub type CpswNussVbuspDiagStatusReg =
    crate::Reg<cpsw_nuss_vbusp_diag_status_reg::CpswNussVbuspDiagStatusRegSpec>;
#[doc = "SGMII Diagnostics Status Register"]
pub mod cpsw_nuss_vbusp_diag_status_reg;
#[doc = "CPSW_NUSS_VBUSP_MDIO_VERSION_REG (rw) register accessor: MDIO Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mdio_version_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mdio_version_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_mdio_version_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_MDIO_VERSION_REG")]
pub type CpswNussVbuspMdioVersionReg =
    crate::Reg<cpsw_nuss_vbusp_mdio_version_reg::CpswNussVbuspMdioVersionRegSpec>;
#[doc = "MDIO Version Register"]
pub mod cpsw_nuss_vbusp_mdio_version_reg;
#[doc = "CPSW_NUSS_VBUSP_CONTROL_REG__ (rw) register accessor: MDIO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg__::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg__::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_control_reg__`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CONTROL_REG__")]
pub type CpswNussVbuspControlReg__ =
    crate::Reg<cpsw_nuss_vbusp_control_reg__::CpswNussVbuspControlReg_Spec>;
#[doc = "MDIO Control Register"]
pub mod cpsw_nuss_vbusp_control_reg__;
#[doc = "CPSW_NUSS_VBUSP_ALIVE_REG (rw) register accessor: MDIO Alive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_alive_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_alive_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_alive_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_ALIVE_REG")]
pub type CpswNussVbuspAliveReg = crate::Reg<cpsw_nuss_vbusp_alive_reg::CpswNussVbuspAliveRegSpec>;
#[doc = "MDIO Alive Register"]
pub mod cpsw_nuss_vbusp_alive_reg;
#[doc = "CPSW_NUSS_VBUSP_LINK_REG (rw) register accessor: MDIO Link Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_link_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_LINK_REG")]
pub type CpswNussVbuspLinkReg = crate::Reg<cpsw_nuss_vbusp_link_reg::CpswNussVbuspLinkRegSpec>;
#[doc = "MDIO Link Register"]
pub mod cpsw_nuss_vbusp_link_reg;
#[doc = "CPSW_NUSS_VBUSP_LINK_INT_RAW_REG (rw) register accessor: MDIO Link Interrupt Raw Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_link_int_raw_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_LINK_INT_RAW_REG")]
pub type CpswNussVbuspLinkIntRawReg =
    crate::Reg<cpsw_nuss_vbusp_link_int_raw_reg::CpswNussVbuspLinkIntRawRegSpec>;
#[doc = "MDIO Link Interrupt Raw Register"]
pub mod cpsw_nuss_vbusp_link_int_raw_reg;
#[doc = "CPSW_NUSS_VBUSP_LINK_INT_MASKED_REG (rw) register accessor: MDIO Link Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_masked_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_masked_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_link_int_masked_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_LINK_INT_MASKED_REG")]
pub type CpswNussVbuspLinkIntMaskedReg =
    crate::Reg<cpsw_nuss_vbusp_link_int_masked_reg::CpswNussVbuspLinkIntMaskedRegSpec>;
#[doc = "MDIO Link Interrupt Masked Register"]
pub mod cpsw_nuss_vbusp_link_int_masked_reg;
#[doc = "CPSW_NUSS_VBUSP_LINK_INT_MASK_SET_REG (rw) register accessor: MDIO Link Interrupt Mask Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_mask_set_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_mask_set_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_link_int_mask_set_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_LINK_INT_MASK_SET_REG")]
pub type CpswNussVbuspLinkIntMaskSetReg =
    crate::Reg<cpsw_nuss_vbusp_link_int_mask_set_reg::CpswNussVbuspLinkIntMaskSetRegSpec>;
#[doc = "MDIO Link Interrupt Mask Set Register"]
pub mod cpsw_nuss_vbusp_link_int_mask_set_reg;
#[doc = "CPSW_NUSS_VBUSP_LINK_INT_MASK_CLEAR_REG (rw) register accessor: MDIO Link Interrupt Mask Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_mask_clear_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_mask_clear_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_link_int_mask_clear_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_LINK_INT_MASK_CLEAR_REG")]
pub type CpswNussVbuspLinkIntMaskClearReg =
    crate::Reg<cpsw_nuss_vbusp_link_int_mask_clear_reg::CpswNussVbuspLinkIntMaskClearRegSpec>;
#[doc = "MDIO Link Interrupt Mask Clear Register"]
pub mod cpsw_nuss_vbusp_link_int_mask_clear_reg;
#[doc = "CPSW_NUSS_VBUSP_USER_INT_RAW_REG (rw) register accessor: MDIO User Interrupt Raw Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_raw_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_raw_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_user_int_raw_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_USER_INT_RAW_REG")]
pub type CpswNussVbuspUserIntRawReg =
    crate::Reg<cpsw_nuss_vbusp_user_int_raw_reg::CpswNussVbuspUserIntRawRegSpec>;
#[doc = "MDIO User Interrupt Raw Register"]
pub mod cpsw_nuss_vbusp_user_int_raw_reg;
#[doc = "CPSW_NUSS_VBUSP_USER_INT_MASKED_REG (rw) register accessor: MDIO User Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_masked_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_masked_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_user_int_masked_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_USER_INT_MASKED_REG")]
pub type CpswNussVbuspUserIntMaskedReg =
    crate::Reg<cpsw_nuss_vbusp_user_int_masked_reg::CpswNussVbuspUserIntMaskedRegSpec>;
#[doc = "MDIO User Interrupt Masked Register"]
pub mod cpsw_nuss_vbusp_user_int_masked_reg;
#[doc = "CPSW_NUSS_VBUSP_USER_INT_MASK_SET_REG (rw) register accessor: MDIO User Interrupt Mask Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_mask_set_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_mask_set_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_user_int_mask_set_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_USER_INT_MASK_SET_REG")]
pub type CpswNussVbuspUserIntMaskSetReg =
    crate::Reg<cpsw_nuss_vbusp_user_int_mask_set_reg::CpswNussVbuspUserIntMaskSetRegSpec>;
#[doc = "MDIO User Interrupt Mask Set Register"]
pub mod cpsw_nuss_vbusp_user_int_mask_set_reg;
#[doc = "CPSW_NUSS_VBUSP_USER_INT_MASK_CLEAR_REG (rw) register accessor: MDIO User Interrupt Mask Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_mask_clear_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_mask_clear_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_user_int_mask_clear_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_USER_INT_MASK_CLEAR_REG")]
pub type CpswNussVbuspUserIntMaskClearReg =
    crate::Reg<cpsw_nuss_vbusp_user_int_mask_clear_reg::CpswNussVbuspUserIntMaskClearRegSpec>;
#[doc = "MDIO User Interrupt Mask Clear Register"]
pub mod cpsw_nuss_vbusp_user_int_mask_clear_reg;
#[doc = "CPSW_NUSS_VBUSP_MANUAL_IF_REG (rw) register accessor: MDIO Manual Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_manual_if_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_manual_if_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_manual_if_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_MANUAL_IF_REG")]
pub type CpswNussVbuspManualIfReg =
    crate::Reg<cpsw_nuss_vbusp_manual_if_reg::CpswNussVbuspManualIfRegSpec>;
#[doc = "MDIO Manual Interface Register"]
pub mod cpsw_nuss_vbusp_manual_if_reg;
#[doc = "CPSW_NUSS_VBUSP_POLL_REG (rw) register accessor: MDIO Poll Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_poll_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_poll_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_poll_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_POLL_REG")]
pub type CpswNussVbuspPollReg = crate::Reg<cpsw_nuss_vbusp_poll_reg::CpswNussVbuspPollRegSpec>;
#[doc = "MDIO Poll Register"]
pub mod cpsw_nuss_vbusp_poll_reg;
#[doc = "CPSW_NUSS_VBUSP_POLL_EN_REG (rw) register accessor: MDIO Poll Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_poll_en_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_poll_en_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_poll_en_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_POLL_EN_REG")]
pub type CpswNussVbuspPollEnReg =
    crate::Reg<cpsw_nuss_vbusp_poll_en_reg::CpswNussVbuspPollEnRegSpec>;
#[doc = "MDIO Poll Enable Register"]
pub mod cpsw_nuss_vbusp_poll_en_reg;
#[doc = "CPSW_NUSS_VBUSP_CLAUS45_REG (rw) register accessor: MDIO Clause45 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_claus45_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_claus45_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_claus45_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CLAUS45_REG")]
pub type CpswNussVbuspClaus45Reg =
    crate::Reg<cpsw_nuss_vbusp_claus45_reg::CpswNussVbuspClaus45RegSpec>;
#[doc = "MDIO Clause45 Register"]
pub mod cpsw_nuss_vbusp_claus45_reg;
#[doc = "CPSW_NUSS_VBUSP_USER_ADDR0_REG (rw) register accessor: MDIO Address 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_addr0_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_addr0_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_user_addr0_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_USER_ADDR0_REG")]
pub type CpswNussVbuspUserAddr0Reg =
    crate::Reg<cpsw_nuss_vbusp_user_addr0_reg::CpswNussVbuspUserAddr0RegSpec>;
#[doc = "MDIO Address 0 Register"]
pub mod cpsw_nuss_vbusp_user_addr0_reg;
#[doc = "CPSW_NUSS_VBUSP_USER_ADDR1_REG (rw) register accessor: MDIO Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_addr1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_addr1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_user_addr1_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_USER_ADDR1_REG")]
pub type CpswNussVbuspUserAddr1Reg =
    crate::Reg<cpsw_nuss_vbusp_user_addr1_reg::CpswNussVbuspUserAddr1RegSpec>;
#[doc = "MDIO Address 1 Register"]
pub mod cpsw_nuss_vbusp_user_addr1_reg;
#[doc = "CPSW_NUSS_VBUSP_REVISION (rw) register accessor: Revision Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_revision`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_REVISION")]
pub type CpswNussVbuspRevision = crate::Reg<cpsw_nuss_vbusp_revision::CpswNussVbuspRevisionSpec>;
#[doc = "Revision Register"]
pub mod cpsw_nuss_vbusp_revision;
#[doc = "CPSW_NUSS_VBUSP_eoi_reg (rw) register accessor: End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_eoi_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_eoi_reg")]
pub type CpswNussVbuspEoiReg = crate::Reg<cpsw_nuss_vbusp_eoi_reg::CpswNussVbuspEoiRegSpec>;
#[doc = "End of Interrupt Register"]
pub mod cpsw_nuss_vbusp_eoi_reg;
#[doc = "CPSW_NUSS_VBUSP_intr_vector_reg (rw) register accessor: Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_intr_vector_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_intr_vector_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_intr_vector_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_intr_vector_reg")]
pub type CpswNussVbuspIntrVectorReg =
    crate::Reg<cpsw_nuss_vbusp_intr_vector_reg::CpswNussVbuspIntrVectorRegSpec>;
#[doc = "Interrupt Vector Register"]
pub mod cpsw_nuss_vbusp_intr_vector_reg;
#[doc = "CPSW_NUSS_VBUSP_enable_reg_out_pulse_0 (rw) register accessor: Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_enable_reg_out_pulse_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_enable_reg_out_pulse_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_enable_reg_out_pulse_0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_enable_reg_out_pulse_0")]
pub type CpswNussVbuspEnableRegOutPulse0 =
    crate::Reg<cpsw_nuss_vbusp_enable_reg_out_pulse_0::CpswNussVbuspEnableRegOutPulse0Spec>;
#[doc = "Enable Register 0"]
pub mod cpsw_nuss_vbusp_enable_reg_out_pulse_0;
#[doc = "CPSW_NUSS_VBUSP_enable_clr_reg_out_pulse_0 (rw) register accessor: Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_enable_clr_reg_out_pulse_0")]
pub type CpswNussVbuspEnableClrRegOutPulse0 =
    crate::Reg<cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0::CpswNussVbuspEnableClrRegOutPulse0Spec>;
#[doc = "Enable Clear Register 0"]
pub mod cpsw_nuss_vbusp_enable_clr_reg_out_pulse_0;
#[doc = "CPSW_NUSS_VBUSP_status_reg_out_pulse_0 (rw) register accessor: Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_status_reg_out_pulse_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_status_reg_out_pulse_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_status_reg_out_pulse_0`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_status_reg_out_pulse_0")]
pub type CpswNussVbuspStatusRegOutPulse0 =
    crate::Reg<cpsw_nuss_vbusp_status_reg_out_pulse_0::CpswNussVbuspStatusRegOutPulse0Spec>;
#[doc = "Status Register 0"]
pub mod cpsw_nuss_vbusp_status_reg_out_pulse_0;
#[doc = "CPSW_NUSS_VBUSP_intr_vector_reg_out_pulse (rw) register accessor: Interrupt Vector for out_pulse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_intr_vector_reg_out_pulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_intr_vector_reg_out_pulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_intr_vector_reg_out_pulse`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_intr_vector_reg_out_pulse")]
pub type CpswNussVbuspIntrVectorRegOutPulse =
    crate::Reg<cpsw_nuss_vbusp_intr_vector_reg_out_pulse::CpswNussVbuspIntrVectorRegOutPulseSpec>;
#[doc = "Interrupt Vector for out_pulse"]
pub mod cpsw_nuss_vbusp_intr_vector_reg_out_pulse;
#[doc = "CPSW_NUSS_VBUSP_CPSW_ID_VER_REG (rw) register accessor: CPSW ID Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_cpsw_id_ver_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_cpsw_id_ver_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_cpsw_id_ver_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CPSW_ID_VER_REG")]
pub type CpswNussVbuspCpswIdVerReg =
    crate::Reg<cpsw_nuss_vbusp_cpsw_id_ver_reg::CpswNussVbuspCpswIdVerRegSpec>;
#[doc = "CPSW ID Version"]
pub mod cpsw_nuss_vbusp_cpsw_id_ver_reg;
#[doc = "CPSW_NUSS_VBUSP_CONTROL_REG___ (rw) register accessor: CPSW Switch Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg___::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg___::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_control_reg___`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CONTROL_REG___")]
pub type CpswNussVbuspControlReg___ =
    crate::Reg<cpsw_nuss_vbusp_control_reg___::CpswNussVbuspControlReg_Spec>;
#[doc = "CPSW Switch Control"]
pub mod cpsw_nuss_vbusp_control_reg___;
#[doc = "CPSW_NUSS_VBUSP_EM_CONTROL_REG (rw) register accessor: CPSW Emulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_em_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_em_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_em_control_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_EM_CONTROL_REG")]
pub type CpswNussVbuspEmControlReg =
    crate::Reg<cpsw_nuss_vbusp_em_control_reg::CpswNussVbuspEmControlRegSpec>;
#[doc = "CPSW Emulation Control"]
pub mod cpsw_nuss_vbusp_em_control_reg;
#[doc = "CPSW_NUSS_VBUSP_STAT_PORT_EN_REG (rw) register accessor: CPSW Statistics Port Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_stat_port_en_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_stat_port_en_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_stat_port_en_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_STAT_PORT_EN_REG")]
pub type CpswNussVbuspStatPortEnReg =
    crate::Reg<cpsw_nuss_vbusp_stat_port_en_reg::CpswNussVbuspStatPortEnRegSpec>;
#[doc = "CPSW Statistics Port Enable"]
pub mod cpsw_nuss_vbusp_stat_port_en_reg;
#[doc = "CPSW_NUSS_VBUSP_PTYPE_REG (rw) register accessor: CPSW Transmit Priority Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_ptype_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_ptype_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_ptype_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_PTYPE_REG")]
pub type CpswNussVbuspPtypeReg = crate::Reg<cpsw_nuss_vbusp_ptype_reg::CpswNussVbuspPtypeRegSpec>;
#[doc = "CPSW Transmit Priority Type"]
pub mod cpsw_nuss_vbusp_ptype_reg;
#[doc = "CPSW_NUSS_VBUSP_SOFT_IDLE_REG (rw) register accessor: CPSW Software Idle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_soft_idle_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_soft_idle_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_soft_idle_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_SOFT_IDLE_REG")]
pub type CpswNussVbuspSoftIdleReg =
    crate::Reg<cpsw_nuss_vbusp_soft_idle_reg::CpswNussVbuspSoftIdleRegSpec>;
#[doc = "CPSW Software Idle"]
pub mod cpsw_nuss_vbusp_soft_idle_reg;
#[doc = "CPSW_NUSS_VBUSP_THRU_RATE_REG (rw) register accessor: CPSW Thru Rate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_thru_rate_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_thru_rate_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_thru_rate_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_THRU_RATE_REG")]
pub type CpswNussVbuspThruRateReg =
    crate::Reg<cpsw_nuss_vbusp_thru_rate_reg::CpswNussVbuspThruRateRegSpec>;
#[doc = "CPSW Thru Rate"]
pub mod cpsw_nuss_vbusp_thru_rate_reg;
#[doc = "CPSW_NUSS_VBUSP_GAP_THRESH_REG (rw) register accessor: CPSW Transmit FIFO Short Gap Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_gap_thresh_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_gap_thresh_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_gap_thresh_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_GAP_THRESH_REG")]
pub type CpswNussVbuspGapThreshReg =
    crate::Reg<cpsw_nuss_vbusp_gap_thresh_reg::CpswNussVbuspGapThreshRegSpec>;
#[doc = "CPSW Transmit FIFO Short Gap Threshold"]
pub mod cpsw_nuss_vbusp_gap_thresh_reg;
#[doc = "CPSW_NUSS_VBUSP_EEE_PRESCALE_REG (rw) register accessor: CPSW Energy Efficient Ethernet Prescale Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_eee_prescale_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_eee_prescale_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_eee_prescale_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_EEE_PRESCALE_REG")]
pub type CpswNussVbuspEeePrescaleReg =
    crate::Reg<cpsw_nuss_vbusp_eee_prescale_reg::CpswNussVbuspEeePrescaleRegSpec>;
#[doc = "CPSW Energy Efficient Ethernet Prescale Value"]
pub mod cpsw_nuss_vbusp_eee_prescale_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_SET_REG (rw) register accessor: CPSW PFC Tx Global Out Flow Threshold Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_SET_REG")]
pub type CpswNussVbuspTxGOflowThreshSetReg =
    crate::Reg<cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::CpswNussVbuspTxGOflowThreshSetRegSpec>;
#[doc = "CPSW PFC Tx Global Out Flow Threshold Set"]
pub mod cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_CLR_REG (rw) register accessor: CPSW PFC Tx Global Out Flow Threshold Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_CLR_REG")]
pub type CpswNussVbuspTxGOflowThreshClrReg =
    crate::Reg<cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg::CpswNussVbuspTxGOflowThreshClrRegSpec>;
#[doc = "CPSW PFC Tx Global Out Flow Threshold Clear"]
pub mod cpsw_nuss_vbusp_tx_g_oflow_thresh_clr_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_L_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Set Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_L_REG")]
pub type CpswNussVbuspTxGBufThreshSetLReg =
    crate::Reg<cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg::CpswNussVbuspTxGBufThreshSetLRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Set Low"]
pub mod cpsw_nuss_vbusp_tx_g_buf_thresh_set_l_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_H_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Set High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_H_REG")]
pub type CpswNussVbuspTxGBufThreshSetHReg =
    crate::Reg<cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::CpswNussVbuspTxGBufThreshSetHRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Set High"]
pub mod cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_L_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Clear Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_L_REG")]
pub type CpswNussVbuspTxGBufThreshClrLReg =
    crate::Reg<cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::CpswNussVbuspTxGBufThreshClrLRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Clear Low"]
pub mod cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_H_REG (rw) register accessor: CPSW PFC Global Tx Buffer Threshold Clear High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_H_REG")]
pub type CpswNussVbuspTxGBufThreshClrHReg =
    crate::Reg<cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg::CpswNussVbuspTxGBufThreshClrHRegSpec>;
#[doc = "CPSW PFC Global Tx Buffer Threshold Clear High"]
pub mod cpsw_nuss_vbusp_tx_g_buf_thresh_clr_h_reg;
#[doc = "CPSW_NUSS_VBUSP_VLAN_LTYPE_REG (rw) register accessor: VLAN Length/type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_vlan_ltype_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_vlan_ltype_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_vlan_ltype_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_VLAN_LTYPE_REG")]
pub type CpswNussVbuspVlanLtypeReg =
    crate::Reg<cpsw_nuss_vbusp_vlan_ltype_reg::CpswNussVbuspVlanLtypeRegSpec>;
#[doc = "VLAN Length/type"]
pub mod cpsw_nuss_vbusp_vlan_ltype_reg;
#[doc = "CPSW_NUSS_VBUSP_EST_TS_DOMAIN_REG (rw) register accessor: Enhanced Scheduled Traffic Host Event Domain\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_est_ts_domain_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_est_ts_domain_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_est_ts_domain_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_EST_TS_DOMAIN_REG")]
pub type CpswNussVbuspEstTsDomainReg =
    crate::Reg<cpsw_nuss_vbusp_est_ts_domain_reg::CpswNussVbuspEstTsDomainRegSpec>;
#[doc = "Enhanced Scheduled Traffic Host Event Domain"]
pub mod cpsw_nuss_vbusp_est_ts_domain_reg;
#[doc = "CPSW_NUSS_VBUSP_CUT_THRESHOLD_REG (rw) register accessor: Cut-thru Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_cut_threshold_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_cut_threshold_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_cut_threshold_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_CUT_THRESHOLD_REG")]
pub type CpswNussVbuspCutThresholdReg =
    crate::Reg<cpsw_nuss_vbusp_cut_threshold_reg::CpswNussVbuspCutThresholdRegSpec>;
#[doc = "Cut-thru Threshold"]
pub mod cpsw_nuss_vbusp_cut_threshold_reg;
#[doc = "CPSW_NUSS_VBUSP_FREQUENCY_REG (rw) register accessor: CPSW CPPI_CLK Frequency in Mhz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_frequency_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_frequency_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_frequency_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_FREQUENCY_REG")]
pub type CpswNussVbuspFrequencyReg =
    crate::Reg<cpsw_nuss_vbusp_frequency_reg::CpswNussVbuspFrequencyRegSpec>;
#[doc = "CPSW CPPI_CLK Frequency in Mhz"]
pub mod cpsw_nuss_vbusp_frequency_reg;
#[doc = "CPSW_NUSS_VBUSP_IET_HOLD_CNT_LD_VAL_REG (rw) register accessor: IET Hold Count Load Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_IET_HOLD_CNT_LD_VAL_REG")]
pub type CpswNussVbuspIetHoldCntLdValReg =
    crate::Reg<cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg::CpswNussVbuspIetHoldCntLdValRegSpec>;
#[doc = "IET Hold Count Load Value"]
pub mod cpsw_nuss_vbusp_iet_hold_cnt_ld_val_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI0_MAXLEN_REG (rw) register accessor: Transmit Priority 0 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri0_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri0_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri0_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI0_MAXLEN_REG")]
pub type CpswNussVbuspTxPri0MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri0_maxlen_reg::CpswNussVbuspTxPri0MaxlenRegSpec>;
#[doc = "Transmit Priority 0 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri0_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI1_MAXLEN_REG (rw) register accessor: Transmit Priority 1 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri1_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri1_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri1_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI1_MAXLEN_REG")]
pub type CpswNussVbuspTxPri1MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri1_maxlen_reg::CpswNussVbuspTxPri1MaxlenRegSpec>;
#[doc = "Transmit Priority 1 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri1_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI2_MAXLEN_REG (rw) register accessor: Transmit Priority 2 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri2_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri2_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri2_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI2_MAXLEN_REG")]
pub type CpswNussVbuspTxPri2MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri2_maxlen_reg::CpswNussVbuspTxPri2MaxlenRegSpec>;
#[doc = "Transmit Priority 2 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri2_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI3_MAXLEN_REG (rw) register accessor: Transmit Priority 3 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri3_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri3_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri3_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI3_MAXLEN_REG")]
pub type CpswNussVbuspTxPri3MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri3_maxlen_reg::CpswNussVbuspTxPri3MaxlenRegSpec>;
#[doc = "Transmit Priority 3 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri3_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI4_MAXLEN_REG (rw) register accessor: Transmit Priority 4 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri4_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri4_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri4_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI4_MAXLEN_REG")]
pub type CpswNussVbuspTxPri4MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri4_maxlen_reg::CpswNussVbuspTxPri4MaxlenRegSpec>;
#[doc = "Transmit Priority 4 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri4_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI5_MAXLEN_REG (rw) register accessor: Transmit Priority 5 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri5_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri5_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri5_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI5_MAXLEN_REG")]
pub type CpswNussVbuspTxPri5MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri5_maxlen_reg::CpswNussVbuspTxPri5MaxlenRegSpec>;
#[doc = "Transmit Priority 5 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri5_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI6_MAXLEN_REG (rw) register accessor: Transmit Priority 6 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri6_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri6_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri6_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI6_MAXLEN_REG")]
pub type CpswNussVbuspTxPri6MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri6_maxlen_reg::CpswNussVbuspTxPri6MaxlenRegSpec>;
#[doc = "Transmit Priority 6 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri6_maxlen_reg;
#[doc = "CPSW_NUSS_VBUSP_TX_PRI7_MAXLEN_REG (rw) register accessor: Transmit Priority 7 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri7_maxlen_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri7_maxlen_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpsw_nuss_vbusp_tx_pri7_maxlen_reg`]
module"]
#[doc(alias = "CPSW_NUSS_VBUSP_TX_PRI7_MAXLEN_REG")]
pub type CpswNussVbuspTxPri7MaxlenReg =
    crate::Reg<cpsw_nuss_vbusp_tx_pri7_maxlen_reg::CpswNussVbuspTxPri7MaxlenRegSpec>;
#[doc = "Transmit Priority 7 Maximum Length"]
pub mod cpsw_nuss_vbusp_tx_pri7_maxlen_reg;
