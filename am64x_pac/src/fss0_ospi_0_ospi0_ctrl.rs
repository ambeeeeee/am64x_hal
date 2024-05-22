#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramPartitionCfgReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndAhbAddrTriggerReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRemapAddrReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsTxThreshReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRxThreshReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsNoOfPollsBefExpReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskReg,
    _reserved18: [u8; 0x08],
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsLowerWrProtReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsUpperWrProtReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlReg,
    _reserved21: [u8; 0x04],
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferCtrlReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferWatermarkReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferStartReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferWatermarkReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferStartReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferNumBytesReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeReg,
    _reserved30: [u8; 0x08],
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrReg,
    _reserved33: [u8; 0x08],
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataUpperReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataLowerReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataUpperReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperReg,
    _reserved42: [u8; 0x1c],
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerReg,
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperReg,
    _reserved44: [u8; 0x14],
    ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg:
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdReg,
}
impl RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg
    }
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg
    }
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg
    }
    #[doc = "0x0c - Octal-SPI Device Delay Register: This register is used to introduce relative delays into the generation of the master output signals. All timings are defined in cycles of the SPI REFERENCE CLOCK/ext_clk, defined in this table as SPI master ref clock."]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg
    }
    #[doc = "0x10 - Read Data Capture Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg
    }
    #[doc = "0x14 - Device Size Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg
    }
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramPartitionCfgReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg
    }
    #[doc = "0x1c - Indirect AHB Address Trigger Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndAhbAddrTriggerReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg
    }
    #[doc = "0x20 - DMA Peripheral Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg
    }
    #[doc = "0x24 - Remap Address Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRemapAddrReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg
    }
    #[doc = "0x28 - Mode Bit Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg
    }
    #[doc = "0x2c - SRAM Fill Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg
    }
    #[doc = "0x30 - TX Threshold Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsTxThreshReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg
    }
    #[doc = "0x34 - RX Threshold Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRxThreshReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg
    }
    #[doc = "0x38 - Write Completion Control Register: This register defines how the controller will poll the device following a write transfer"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg
    }
    #[doc = "0x3c - Polling Expiration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsNoOfPollsBefExpReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg
    }
    #[doc = "0x40 - Interrupt Status Register: The status fields in this register are set when the described event occurs and the interrupt is enabled in the mask register. When any of these bit fields are set, the interrupt output is asserted high. The fields are each cleared by writing a 1 to the field. Note that bit fields 6 thru 10 are only valid when legacy SPI mode is active."]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg
    }
    #[doc = "0x44 - Interrupt Mask: 0 : the interrupt for the corresponding interrupt status register bit is disabled. 1 : the interrupt for the corresponding interrupt status register bit is enabled."]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg
    }
    #[doc = "0x50 - Lower Write Protection Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsLowerWrProtReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg
    }
    #[doc = "0x54 - Upper Write Protection Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsUpperWrProtReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg
    }
    #[doc = "0x58 - Write Protection Control Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlReg {
        &self
            .ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg
    }
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferCtrlReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg
    }
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferWatermarkReg
    {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg
    }
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferStartReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg
    }
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesReg
    {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg
    }
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg
    }
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferWatermarkReg
    {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg
    }
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferStartReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg
    }
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferNumBytesReg
    {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg
    }
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeReg
    {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg
    }
    #[doc = "0x8c - Flash Command Control Memory Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg
    }
    #[doc = "0x90 - Flash Command Control Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg
    }
    #[doc = "0x94 - Flash Command Address Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg
    }
    #[doc = "0xa0 - Flash Command Read Data Register (Lower)"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg
    }
    #[doc = "0xa4 - Flash Command Read Data Register (Upper)"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataUpperReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg
    }
    #[doc = "0xa8 - Flash Command Write Data Register (Lower)"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataLowerReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg
    }
    #[doc = "0xac - Flash Command Write Data Register (Upper)"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataUpperReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg
    }
    #[doc = "0xb0 - Polling Flash Status Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg
    }
    #[doc = "0xb4 - PHY Configuration Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg
    }
    #[doc = "0xb8 - PHY DLL Master Control Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg
    }
    #[doc = "0xbc - DLL Observable Register Lower"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg
    }
    #[doc = "0xc0 - DLL Observable Register Upper"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg
    }
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg
    }
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperReg {
        & self . ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg
    }
    #[doc = "0xfc - Module ID Register"]
    #[inline(always)]
    pub const fn ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg(
        &self,
    ) -> &Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdReg {
        &self.ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg
    }
}
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_config_reg (rw) register accessor: Octal-SPI Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg`]
module"]
#[doc(alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_config_reg")]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec > ;
#[doc = "Octal-SPI Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_rd_config_reg (rw) register accessor: Device Read Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_rd_config_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec > ;
#[doc = "Device Read Instruction Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_wr_config_reg (rw) register accessor: Device Write Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_wr_config_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec > ;
#[doc = "Device Write Instruction Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_delay_reg (rw) register accessor: Octal-SPI Device Delay Register: This register is used to introduce relative delays into the generation of the master output signals. All timings are defined in cycles of the SPI REFERENCE CLOCK/ext_clk, defined in this table as SPI master ref clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_delay_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec > ;
#[doc = "Octal-SPI Device Delay Register: This register is used to introduce relative delays into the generation of the master output signals. All timings are defined in cycles of the SPI REFERENCE CLOCK/ext_clk, defined in this table as SPI master ref clock."]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rd_data_capture_reg (rw) register accessor: Read Data Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rd_data_capture_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec > ;
#[doc = "Read Data Capture Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_size_config_reg (rw) register accessor: Device Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_size_config_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec > ;
#[doc = "Device Size Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_partition_cfg_reg (rw) register accessor: SRAM Partition Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_partition_cfg_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramPartitionCfgReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramPartitionCfgRegSpec > ;
#[doc = "SRAM Partition Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_partition_cfg_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_ind_AHB_addr_trigger_reg (rw) register accessor: Indirect AHB Address Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_ind_AHB_addr_trigger_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndAhbAddrTriggerReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndAhbAddrTriggerRegSpec > ;
#[doc = "Indirect AHB Address Trigger Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_ind_ahb_addr_trigger_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dma_periph_config_reg (rw) register accessor: DMA Peripheral Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dma_periph_config_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec > ;
#[doc = "DMA Peripheral Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_remap_addr_reg (rw) register accessor: Remap Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_remap_addr_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRemapAddrReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRemapAddrRegSpec > ;
#[doc = "Remap Address Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_remap_addr_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_mode_bit_config_reg (rw) register accessor: Mode Bit Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_mode_bit_config_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec > ;
#[doc = "Mode Bit Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_fill_reg (rw) register accessor: SRAM Fill Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_fill_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec > ;
#[doc = "SRAM Fill Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_tx_thresh_reg (rw) register accessor: TX Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_tx_thresh_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsTxThreshReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsTxThreshRegSpec > ;
#[doc = "TX Threshold Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_tx_thresh_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rx_thresh_reg (rw) register accessor: RX Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rx_thresh_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRxThreshReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRxThreshRegSpec > ;
#[doc = "RX Threshold Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rx_thresh_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_write_completion_ctrl_reg (rw) register accessor: Write Completion Control Register: This register defines how the controller will poll the device following a write transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_write_completion_ctrl_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec > ;
#[doc = "Write Completion Control Register: This register defines how the controller will poll the device following a write transfer"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_no_of_polls_bef_exp_reg (rw) register accessor: Polling Expiration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_no_of_polls_bef_exp_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsNoOfPollsBefExpReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsNoOfPollsBefExpRegSpec > ;
#[doc = "Polling Expiration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_no_of_polls_bef_exp_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_status_reg (rw) register accessor: Interrupt Status Register: The status fields in this register are set when the described event occurs and the interrupt is enabled in the mask register. When any of these bit fields are set, the interrupt output is asserted high. The fields are each cleared by writing a 1 to the field. Note that bit fields 6 thru 10 are only valid when legacy SPI mode is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_status_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqStatusRegSpec > ;
#[doc = "Interrupt Status Register: The status fields in this register are set when the described event occurs and the interrupt is enabled in the mask register. When any of these bit fields are set, the interrupt output is asserted high. The fields are each cleared by writing a 1 to the field. Note that bit fields 6 thru 10 are only valid when legacy SPI mode is active."]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_status_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_mask_reg (rw) register accessor: Interrupt Mask: 0 : the interrupt for the corresponding interrupt status register bit is disabled. 1 : the interrupt for the corresponding interrupt status register bit is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_irq_mask_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIrqMaskRegSpec > ;
#[doc = "Interrupt Mask: 0 : the interrupt for the corresponding interrupt status register bit is disabled. 1 : the interrupt for the corresponding interrupt status register bit is enabled."]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_irq_mask_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_lower_wr_prot_reg (rw) register accessor: Lower Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_lower_wr_prot_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsLowerWrProtReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsLowerWrProtRegSpec > ;
#[doc = "Lower Write Protection Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_lower_wr_prot_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_upper_wr_prot_reg (rw) register accessor: Upper Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_upper_wr_prot_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsUpperWrProtReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsUpperWrProtRegSpec > ;
#[doc = "Upper Write Protection Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_upper_wr_prot_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_wr_prot_ctrl_reg (rw) register accessor: Write Protection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_wr_prot_ctrl_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec > ;
#[doc = "Write Protection Control Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_ctrl_reg (rw) register accessor: Indirect Read Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_ctrl_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferCtrlReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferCtrlRegSpec > ;
#[doc = "Indirect Read Transfer Control Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_ctrl_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_watermark_reg (rw) register accessor: Indirect Read Transfer Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_watermark_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferWatermarkReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferWatermarkRegSpec > ;
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_watermark_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_start_reg (rw) register accessor: Indirect Read Transfer Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_start_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferStartReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferStartRegSpec > ;
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_start_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_num_bytes_reg (rw) register accessor: Indirect Read Transfer Number Bytes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_num_bytes_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec > ;
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_ctrl_reg (rw) register accessor: Indirect Write Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_ctrl_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec > ;
#[doc = "Indirect Write Transfer Control Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_watermark_reg (rw) register accessor: Indirect Write Transfer Watermark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_watermark_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferWatermarkReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferWatermarkRegSpec > ;
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_watermark_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_start_reg (rw) register accessor: Indirect Write Transfer Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_start_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferStartReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferStartRegSpec > ;
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_start_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_num_bytes_reg (rw) register accessor: Indirect Write Transfer Number Bytes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_num_bytes_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferNumBytesReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferNumBytesRegSpec > ;
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_num_bytes_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_trigger_addr_range_reg (rw) register accessor: Indirect Trigger Address Range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_trigger_addr_range_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec > ;
#[doc = "Indirect Trigger Address Range Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_command_ctrl_mem_reg (rw) register accessor: Flash Command Control Memory Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_command_ctrl_mem_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec > ;
#[doc = "Flash Command Control Memory Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_ctrl_reg (rw) register accessor: Flash Command Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_ctrl_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec > ;
#[doc = "Flash Command Control Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_addr_reg (rw) register accessor: Flash Command Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_addr_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec > ;
#[doc = "Flash Command Address Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_lower_reg (rw) register accessor: Flash Command Read Data Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_lower_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec > ;
#[doc = "Flash Command Read Data Register (Lower)"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_upper_reg (rw) register accessor: Flash Command Read Data Register (Upper)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_upper_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataUpperReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataUpperRegSpec > ;
#[doc = "Flash Command Read Data Register (Upper)"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_upper_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_wr_data_lower_reg (rw) register accessor: Flash Command Write Data Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_wr_data_lower_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataLowerReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataLowerRegSpec > ;
#[doc = "Flash Command Write Data Register (Lower)"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_lower_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_wr_data_upper_reg (rw) register accessor: Flash Command Write Data Register (Upper)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_wr_data_upper_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataUpperReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashWrDataUpperRegSpec > ;
#[doc = "Flash Command Write Data Register (Upper)"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_wr_data_upper_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_polling_flash_status_reg (rw) register accessor: Polling Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_polling_flash_status_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec > ;
#[doc = "Polling Flash Status Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_configuration_reg (rw) register accessor: PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_configuration_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec > ;
#[doc = "PHY Configuration Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_master_control_reg (rw) register accessor: PHY DLL Master Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_master_control_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec > ;
#[doc = "PHY DLL Master Control Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_lower_reg (rw) register accessor: DLL Observable Register Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_lower_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec > ;
#[doc = "DLL Observable Register Lower"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_upper_reg (rw) register accessor: DLL Observable Register Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_upper_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec > ;
#[doc = "DLL Observable Register Upper"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_lower_reg (rw) register accessor: Opcode Extension Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_lower_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec > ;
#[doc = "Opcode Extension Register (Lower)"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_upper_reg (rw) register accessor: Opcode Extension Register (Upper)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_upper_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec > ;
#[doc = "Opcode Extension Register (Upper)"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg;
#[doc = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_module_id_reg (rw) register accessor: Module ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg`]
module"]
#[doc(
    alias = "OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_module_id_reg"
)]
pub type Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdReg = crate :: Reg < ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg :: Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec > ;
#[doc = "Module ID Register"]
pub mod ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg;
