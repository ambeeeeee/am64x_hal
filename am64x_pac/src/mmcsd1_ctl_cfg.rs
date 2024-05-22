#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo: SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLo,
    sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi: SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHi,
    sdhc_wrap__ctl_cfg__ctlcfg_block_size: SdhcWrap_CtlCfg_CtlcfgBlockSize,
    sdhc_wrap__ctl_cfg__ctlcfg_block_count: SdhcWrap_CtlCfg_CtlcfgBlockCount,
    sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo: SdhcWrap_CtlCfg_CtlcfgArgument1Lo,
    sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi: SdhcWrap_CtlCfg_CtlcfgArgument1Hi,
    sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode: SdhcWrap_CtlCfg_CtlcfgTransferMode,
    sdhc_wrap__ctl_cfg__ctlcfg_command: SdhcWrap_CtlCfg_CtlcfgCommand,
    sdhc_wrap__ctl_cfg__ctlcfg_response: SdhcWrap_CtlCfg_CtlcfgResponse,
    _reserved9: [u8; 0x0e],
    sdhc_wrap__ctl_cfg__ctlcfg_data_port: SdhcWrap_CtlCfg_CtlcfgDataPort,
    sdhc_wrap__ctl_cfg__ctlcfg_presentstate: SdhcWrap_CtlCfg_CtlcfgPresentstate,
    sdhc_wrap__ctl_cfg__ctlcfg_host_control1: SdhcWrap_CtlCfg_CtlcfgHostControl1,
    sdhc_wrap__ctl_cfg__ctlcfg_power_control: SdhcWrap_CtlCfg_CtlcfgPowerControl,
    sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control: SdhcWrap_CtlCfg_CtlcfgBlockGapControl,
    sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control: SdhcWrap_CtlCfg_CtlcfgWakeupControl,
    sdhc_wrap__ctl_cfg__ctlcfg_clock_control: SdhcWrap_CtlCfg_CtlcfgClockControl,
    sdhc_wrap__ctl_cfg__ctlcfg_timeout_control: SdhcWrap_CtlCfg_CtlcfgTimeoutControl,
    sdhc_wrap__ctl_cfg__ctlcfg_software_reset: SdhcWrap_CtlCfg_CtlcfgSoftwareReset,
    sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts: SdhcWrap_CtlCfg_CtlcfgNormalIntrSts,
    sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts: SdhcWrap_CtlCfg_CtlcfgErrorIntrSts,
    sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena: SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEna,
    sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena: SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEna,
    sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena: SdhcWrap_CtlCfg_CtlcfgNormalIntrSigEna,
    sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena: SdhcWrap_CtlCfg_CtlcfgErrorIntrSigEna,
    sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts: SdhcWrap_CtlCfg_CtlcfgAutocmdErrSts,
    sdhc_wrap__ctl_cfg__ctlcfg_host_control2: SdhcWrap_CtlCfg_CtlcfgHostControl2,
    sdhc_wrap__ctl_cfg__ctlcfg_capabilities: SdhcWrap_CtlCfg_CtlcfgCapabilities,
    sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap: SdhcWrap_CtlCfg_CtlcfgMaxCurrentCap,
    sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts: SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrSts,
    sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts: SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntSts,
    sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status: SdhcWrap_CtlCfg_CtlcfgAdmaErrStatus,
    _reserved31: [u8; 0x03],
    sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address: SdhcWrap_CtlCfg_CtlcfgAdmaSysAddress,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value0: SdhcWrap_CtlCfg_CtlcfgPresetValue0,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value1: SdhcWrap_CtlCfg_CtlcfgPresetValue1,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value2: SdhcWrap_CtlCfg_CtlcfgPresetValue2,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value3: SdhcWrap_CtlCfg_CtlcfgPresetValue3,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value4: SdhcWrap_CtlCfg_CtlcfgPresetValue4,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value5: SdhcWrap_CtlCfg_CtlcfgPresetValue5,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value6: SdhcWrap_CtlCfg_CtlcfgPresetValue6,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value7: SdhcWrap_CtlCfg_CtlcfgPresetValue7,
    _reserved40: [u8; 0x02],
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value8: SdhcWrap_CtlCfg_CtlcfgPresetValue8,
    sdhc_wrap__ctl_cfg__ctlcfg_preset_value10: SdhcWrap_CtlCfg_CtlcfgPresetValue10,
    _reserved42: [u8; 0x02],
    sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address: SdhcWrap_CtlCfg_CtlcfgAdma3DescAddress,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size: SdhcWrap_CtlCfg_CtlcfgUhs2BlockSize,
    _reserved44: [u8; 0x02],
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count: SdhcWrap_CtlCfg_CtlcfgUhs2BlockCount,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt: SdhcWrap_CtlCfg_CtlcfgUhs2CommandPkt,
    _reserved46: [u8; 0x13],
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode: SdhcWrap_CtlCfg_CtlcfgUhs2XferMode,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command: SdhcWrap_CtlCfg_CtlcfgUhs2Command,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response: SdhcWrap_CtlCfg_CtlcfgUhs2Response,
    _reserved49: [u8; 0x13],
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select: SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelect,
    _reserved50: [u8; 0x03],
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message: SdhcWrap_CtlCfg_CtlcfgUhs2Message,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status: SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatus,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select: SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelect,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code: SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCode,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset: SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareReset,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control: SdhcWrap_CtlCfg_CtlcfgUhs2TimerControl,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts: SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSts,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena: SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsEna,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena: SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEna,
    _reserved59: [u8; 0x10],
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr: SdhcWrap_CtlCfg_CtlcfgUhs2SettingsPtr,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr: SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtr,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr: SdhcWrap_CtlCfg_CtlcfgUhs2TestPtr,
    sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr: SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtr,
    sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr: SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtr,
    _reserved64: [u8; 0x0a],
    sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control: SdhcWrap_CtlCfg_CtlcfgBootTimeoutControl,
    sdhc_wrap__ctl_cfg__ctlcfg_vendor_register: SdhcWrap_CtlCfg_CtlcfgVendorRegister,
    sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts: SdhcWrap_CtlCfg_CtlcfgSlotIntSts,
    sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver: SdhcWrap_CtlCfg_CtlcfgHostControllerVer,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings: SdhcWrap_CtlCfg_CtlcfgUhs2GenSettings,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings: SdhcWrap_CtlCfg_CtlcfgUhs2PhySettings,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings: SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettings,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap: SdhcWrap_CtlCfg_CtlcfgUhs2GenCap,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap: SdhcWrap_CtlCfg_CtlcfgUhs2PhyCap,
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap: SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCap,
    sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts: SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntSts,
    _reserved75: [u8; 0xdc],
    sdhc_wrap__ctl_cfg__ctlcfg_cq_version: SdhcWrap_CtlCfg_CtlcfgCqVersion,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities: SdhcWrap_CtlCfg_CtlcfgCqCapabilities,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_config: SdhcWrap_CtlCfg_CtlcfgCqConfig,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_control: SdhcWrap_CtlCfg_CtlcfgCqControl,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts: SdhcWrap_CtlCfg_CtlcfgCqIntrSts,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena: SdhcWrap_CtlCfg_CtlcfgCqIntrStsEna,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena: SdhcWrap_CtlCfg_CtlcfgCqIntrSigEna,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing: SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescing,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr: SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddr,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits: SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbits,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell: SdhcWrap_CtlCfg_CtlcfgCqTaskDoorBell,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif: SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotif,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status: SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatus,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks: SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasks,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear: SdhcWrap_CtlCfg_CtlcfgCqTaskClear,
    _reserved90: [u8; 0x04],
    sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1: SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2: SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response: SdhcWrap_CtlCfg_CtlcfgCqDcmdResponse,
    _reserved93: [u8; 0x04],
    sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask: SdhcWrap_CtlCfg_CtlcfgCqRespErrMask,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info: SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfo,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index: SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndex,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg: SdhcWrap_CtlCfg_CtlcfgCqCmdRespArg,
    sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id: SdhcWrap_CtlCfg_CtlcfgCqErrorTaskId,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the Lower 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLo {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo
    }
    #[doc = "0x02 - This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHi {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi
    }
    #[doc = "0x04 - This register is used to configure the number of bytes in a data block"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_block_size(&self) -> &SdhcWrap_CtlCfg_CtlcfgBlockSize {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_block_size
    }
    #[doc = "0x06 - This register is used to configure the number of data blocks"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_block_count(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgBlockCount {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_block_count
    }
    #[doc = "0x08 - This register contains Lower bits of SD Command Argument"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgArgument1Lo {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo
    }
    #[doc = "0x0a - This register contains higher bits of SD Command Argument"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgArgument1Hi {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi
    }
    #[doc = "0x0c - This register is used to control the operations of data transfers"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgTransferMode {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode
    }
    #[doc = "0x0e - This register is used to program the Command for host controller"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_command(&self) -> &SdhcWrap_CtlCfg_CtlcfgCommand {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_command
    }
    #[doc = "0x10 - This register is used to store responses from SD Cards"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_response(&self) -> &SdhcWrap_CtlCfg_CtlcfgResponse {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_response
    }
    #[doc = "0x20 - This register is used to access internal buffer"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_data_port(&self) -> &SdhcWrap_CtlCfg_CtlcfgDataPort {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_data_port
    }
    #[doc = "0x24 - The Host Driver can get status of the Host Controller from this 32-bit read-only register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_presentstate(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresentstate {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_presentstate
    }
    #[doc = "0x28 - This register is used to program DMA modes, LED Control, Data Transfer Width, High Speed Enable, Card detect test level and signal selection"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_host_control1(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgHostControl1 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_host_control1
    }
    #[doc = "0x29 - This register is used to program the SD Bus power and voltage level"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_power_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPowerControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_power_control
    }
    #[doc = "0x2a - This register is used to program the block gap request, read wait control and interrupt at block gap"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgBlockGapControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control
    }
    #[doc = "0x2b - This register is used to program the wakeup functionality"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgWakeupControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control
    }
    #[doc = "0x2c - This register is used to program the Clock frequency select, generator select, Clock enable, Internal Clock state fields This register controls SDCLK in SD Mode and RCLK in UHS-II mode."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_clock_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgClockControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_clock_control
    }
    #[doc = "0x2e - The register sets the Data Timeout counter value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_timeout_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgTimeoutControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_timeout_control
    }
    #[doc = "0x2f - This register is used to program the software reset for data, command and for all"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_software_reset(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgSoftwareReset {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_software_reset
    }
    #[doc = "0x30 - This register gives the status of all the interrupts"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgNormalIntrSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts
    }
    #[doc = "0x32 - This register gives the status of the error interrupts"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgErrorIntrSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts
    }
    #[doc = "0x34 - This register is used to enable the normal interrupt status register fields"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena
    }
    #[doc = "0x36 - This register is used to enable the Error Interrupt Status register fields"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena
    }
    #[doc = "0x38 - This register is used to enable the Normal Interrupt Signal register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgNormalIntrSigEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena
    }
    #[doc = "0x3a - This register is used to enable Error Interrupt Signal register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgErrorIntrSigEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena
    }
    #[doc = "0x3c - This register is used to indicate CMD12 response error of Auto CMD12 and CMD23 response error of Auto CMD 23"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgAutocmdErrSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts
    }
    #[doc = "0x3e - This register is used to program UHS Select Mode,UHS Select Mode,Driver Strength Select,Execute Tuning,Sampling Clock Select,Asynchronous Interrupt Enable and Preset value enable"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_host_control2(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgHostControl2 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_host_control2
    }
    #[doc = "0x40..0x48 - This register provides the HD with information specific to the HC implementation. The HC may implement these values as fixed or loaded from flash memory during power on initializa-tion."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_capabilities(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCapabilities {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_capabilities
    }
    #[doc = "0x48..0x50 - This register indicates maximum current capability for each voltage"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgMaxCurrentCap {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap
    }
    #[doc = "0x50 - This register is not physically implemented, rather it is an address where Auto CMD Error Status register can be written. Writing 1 : set each bit of the Auto CMD12 Error Status Register Writing 0 : no effect."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts
    }
    #[doc = "0x52 - This register is not physically implemented, rather it is an address where Error Interrupt Status register can be written."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts
    }
    #[doc = "0x54 - When the ADMA Error interrupt occur, this register holds the ADMA State in ADMA Error States field and ADMA System Address holds address around the error descriptor"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgAdmaErrStatus {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status
    }
    #[doc = "0x58..0x60 - This register contains the physical address used for ADMA data transfer"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgAdmaSysAddress {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address
    }
    #[doc = "0x60 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value0(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue0 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value0
    }
    #[doc = "0x62 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value1(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue1 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value1
    }
    #[doc = "0x64 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value2(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue2 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value2
    }
    #[doc = "0x66 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value3(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue3 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value3
    }
    #[doc = "0x68 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value4(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue4 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value4
    }
    #[doc = "0x6a - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value5(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue5 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value5
    }
    #[doc = "0x6c - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value6(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue6 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value6
    }
    #[doc = "0x6e - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value7(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue7 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value7
    }
    #[doc = "0x72 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value8(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue8 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value8
    }
    #[doc = "0x74 - This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_preset_value10(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgPresetValue10 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_preset_value10
    }
    #[doc = "0x78..0x80 - The start address of Integrated DMA Descriptor is set to this register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgAdma3DescAddress {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address
    }
    #[doc = "0x80 - This register is used to configure the number of bytes in a data block"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2BlockSize {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size
    }
    #[doc = "0x84 - This register is used to configure the number of data blocks"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2BlockCount {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count
    }
    #[doc = "0x88 - UHS-II Command Packet image is set to this register. The maximum length is 20 bytes. The command length varies depends on a Command Packet type. The length is specified by the UHS-II Command register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2CommandPkt {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt
    }
    #[doc = "0x9c - This register is used to control the operations of data transfers"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2XferMode {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode
    }
    #[doc = "0x9e - This register is used to program the Command for host controller"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2Command {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command
    }
    #[doc = "0xa0 - This register is used to store received UHS-II RES Packet image"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2Response {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response
    }
    #[doc = "0xb4 - This register is used to access internal buffer"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelect {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select
    }
    #[doc = "0xb8 - This register is used to access internal buffer"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2Message {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message
    }
    #[doc = "0xbc - This register shows receipt of INT MSG from which device"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatus {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status
    }
    #[doc = "0xbe - UHS-II Device Select Register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelect {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select
    }
    #[doc = "0xbf - This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCode {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code
    }
    #[doc = "0xc0 - UHS-II Software Reset Register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareReset {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset
    }
    #[doc = "0xc2 - UHS-II Timeout Control Register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2TimerControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control
    }
    #[doc = "0xc4 - This register gives the status of all UHS-II interrupts"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts
    }
    #[doc = "0xc8 - This register is used to enable the UHS-II Error Interrupt Status register fields"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena
    }
    #[doc = "0xcc - This register is used to generate UHS-II Interrupt signals"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena
    }
    #[doc = "0xe0 - This register is pointer for UHS-II settings."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2SettingsPtr {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr
    }
    #[doc = "0xe2 - This register is pointer for UHS-II Capabilities Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtr {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr
    }
    #[doc = "0xe4 - This register is pointer for UHS-II Test Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2TestPtr {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr
    }
    #[doc = "0xe6 - This register is pointer for UHS-II Shared Bus Control Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtr {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr
    }
    #[doc = "0xe8 - This register is pointer for UHS-II Vendor Specific Pointer Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtr {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr
    }
    #[doc = "0xf4 - This is used to program the boot timeout value counter"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgBootTimeoutControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control
    }
    #[doc = "0xf8 - Vendor register added for autogate sdclk, cmd11 power down timer, enhancedstrobe and eMMC hardware reset"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_vendor_register(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgVendorRegister {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_vendor_register
    }
    #[doc = "0xfc - This register is used to read the interrupt signal for each slot."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgSlotIntSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts
    }
    #[doc = "0xfe - This register is used to read the vendor version number and specification version number"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgHostControllerVer {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver
    }
    #[doc = "0x100 - Start Address of General settings is pointed by Pointer for UHS-II Setting Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2GenSettings {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings
    }
    #[doc = "0x104 - Start Address of PHY settings is pointed by Pointer for UHS-II Setting Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2PhySettings {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings
    }
    #[doc = "0x108..0x110 - Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Setting Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettings {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings
    }
    #[doc = "0x110 - Start Address of General Capabilities is pointed by Pointer for UHS-II Host Capabilities Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2GenCap {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap
    }
    #[doc = "0x114 - Start Address of PHY Capabilities is pointed by Pointer for UHS-II Host Capabilities Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2PhyCap {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap
    }
    #[doc = "0x118..0x120 - Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Capabilities Register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCap {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap
    }
    #[doc = "0x120 - This register is not physically implemented, rather it is an address where UHS-II Error Interrupt Status register can be written."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts
    }
    #[doc = "0x200 - This register provides information about the version of the eMMC CQ standard which is 285 implemented by the CQE, in BCD format. The current version is rev 5.1 The following table describes the CQBASE+00h: CQVReservedER Command Queueing Version."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_version(&self) -> &SdhcWrap_CtlCfg_CtlcfgCqVersion {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_version
    }
    #[doc = "0x204 - This register is reserved for capability indication."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqCapabilities {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities
    }
    #[doc = "0x208 - This register controls CQE behavior affecting the general operation of command queueing 290 module or operation of multiple tasks in the same time."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_config(&self) -> &SdhcWrap_CtlCfg_CtlcfgCqConfig {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_config
    }
    #[doc = "0x20c - This register controls CQE behavior affecting the general operation of command queueing 293 module or operation of multiple tasks in the same time."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_control(&self) -> &SdhcWrap_CtlCfg_CtlcfgCqControl {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_control
    }
    #[doc = "0x210 - This register indicates pending interrupts that require service. Each bit in this registers is asserted 296 in response a specific event, only if the respective bit is set in CQ ISTE register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts(&self) -> &SdhcWrap_CtlCfg_CtlcfgCqIntrSts {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts
    }
    #[doc = "0x214 - This register enables and disables the reporting of the corresponding interrupt to host soft-ware in 299 CQIS register. When a bit is set ( 1 ) and the corresponding interrupt c -ondition is active, then the 300 bit is CQIS is asserted. Interrupt sources that are disabled ( 0 ) are not indicated in the CQIS 301 register. This register is bit-index matched to CQIS register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqIntrStsEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena
    }
    #[doc = "0x218 - This register enables and disables the generation of interrupts to host software. When a bit is set 304 ( 1 ) and the corresponding bit in CQIS is set, then an interrupt is gene -rated. Interrupt sources 305 that are disabled ( 0 ) are still indicated in the CQIS register. This register is bit-index matched 306 to CQIS register."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqIntrSigEna {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena
    }
    #[doc = "0x21c - This register controls the interrupt coalescing feature."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescing {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing
    }
    #[doc = "0x220 - This register is used for configuring the lower 32 bits of the byte address of the head of the Task 312 Descriptor List in the host memory."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddr {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr
    }
    #[doc = "0x224 - This register is used for configuring the upper 32 bits of the byte address of the head of the Task 316 Descriptor List in the host memory."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbits {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits
    }
    #[doc = "0x228 - Using this register, software triggers CQE to process a new task."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqTaskDoorBell {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell
    }
    #[doc = "0x22c - This register is used by CQE to notify software about completed tasks."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotif {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif
    }
    #[doc = "0x230 - This register stores the most recent value of the device s queue status."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatus {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status
    }
    #[doc = "0x234 - This register indicates to software which tasks are queued in the device, awaiting execution."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasks {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks
    }
    #[doc = "0x238 - This register is used for removing an outstanding task in the CQE. 327. The register should be used only when CQE is in Halt state."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqTaskClear {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear
    }
    #[doc = "0x240 - The register controls the when SEND_QUEUE_STATUS commands are sent."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1
    }
    #[doc = "0x244 - This register is used for 333 configuring RCA field in SEND_QUEUE_STATUS command argu-ment."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2 {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2
    }
    #[doc = "0x248 - This register is used for passing the response of a DCMD task to software."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqDcmdResponse {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response
    }
    #[doc = "0x250 - This register controls the generation of Response Error Detection (RED) interrupt."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqRespErrMask {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask
    }
    #[doc = "0x254 - This register is updated by CQE when an error occurs on data or command related to a task activity. When such error is detected by CQE or indicated by the eMMC controller CQE stores in CQTERRI the task IDs and the command indices of the commands which were executed on the 343 command line and data lines when the error occurred.Software is expected to use this information in the error recovery procedure."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfo {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info
    }
    #[doc = "0x258 - This register stores the index of the last received command response."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndex {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index
    }
    #[doc = "0x25c - This register stores the index of the last received command response."]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqCmdRespArg {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg
    }
    #[doc = "0x260 - CQ Error Task ID Register"]
    #[inline(always)]
    pub const fn sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id(
        &self,
    ) -> &SdhcWrap_CtlCfg_CtlcfgCqErrorTaskId {
        &self.sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id
    }
}
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_lo (rw) register accessor: This register contains the Lower 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_lo")]
pub type SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLo = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec,
>;
#[doc = "This register contains the Lower 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_hi (rw) register accessor: This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_hi")]
pub type SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHi = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi::SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrHiSpec,
>;
#[doc = "This register contains the Upper 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_hi;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_block_size (rw) register accessor: This register is used to configure the number of bytes in a data block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_block_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_block_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_block_size`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_block_size")]
pub type SdhcWrap_CtlCfg_CtlcfgBlockSize =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_block_size::SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec>;
#[doc = "This register is used to configure the number of bytes in a data block"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_block_size;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_block_count (rw) register accessor: This register is used to configure the number of data blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_block_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_block_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_block_count`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_block_count")]
pub type SdhcWrap_CtlCfg_CtlcfgBlockCount =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_block_count::SdhcWrap_CtlCfg_CtlcfgBlockCountSpec>;
#[doc = "This register is used to configure the number of data blocks"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_block_count;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_argument1_lo (rw) register accessor: This register contains Lower bits of SD Command Argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_argument1_lo")]
pub type SdhcWrap_CtlCfg_CtlcfgArgument1Lo =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec>;
#[doc = "This register contains Lower bits of SD Command Argument"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_argument1_hi (rw) register accessor: This register contains higher bits of SD Command Argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_argument1_hi")]
pub type SdhcWrap_CtlCfg_CtlcfgArgument1Hi =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi::SdhcWrap_CtlCfg_CtlcfgArgument1HiSpec>;
#[doc = "This register contains higher bits of SD Command Argument"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_argument1_hi;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_transfer_mode (rw) register accessor: This register is used to control the operations of data transfers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_transfer_mode")]
pub type SdhcWrap_CtlCfg_CtlcfgTransferMode =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::SdhcWrap_CtlCfg_CtlcfgTransferModeSpec>;
#[doc = "This register is used to control the operations of data transfers"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_command (rw) register accessor: This register is used to program the Command for host controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_command`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_command")]
pub type SdhcWrap_CtlCfg_CtlcfgCommand =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_command::SdhcWrap_CtlCfg_CtlcfgCommandSpec>;
#[doc = "This register is used to program the Command for host controller"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_command;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_response (rw) register accessor: This register is used to store responses from SD Cards\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_response::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_response::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_response`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_response")]
pub type SdhcWrap_CtlCfg_CtlcfgResponse =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_response::SdhcWrap_CtlCfg_CtlcfgResponseSpec>;
#[doc = "This register is used to store responses from SD Cards"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_response;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_data_port (rw) register accessor: This register is used to access internal buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_data_port::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_data_port::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_data_port`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_data_port")]
pub type SdhcWrap_CtlCfg_CtlcfgDataPort =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_data_port::SdhcWrap_CtlCfg_CtlcfgDataPortSpec>;
#[doc = "This register is used to access internal buffer"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_data_port;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_presentstate (rw) register accessor: The Host Driver can get status of the Host Controller from this 32-bit read-only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_presentstate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_presentstate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_presentstate`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_presentstate")]
pub type SdhcWrap_CtlCfg_CtlcfgPresentstate =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_presentstate::SdhcWrap_CtlCfg_CtlcfgPresentstateSpec>;
#[doc = "The Host Driver can get status of the Host Controller from this 32-bit read-only register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_presentstate;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_host_control1 (rw) register accessor: This register is used to program DMA modes, LED Control, Data Transfer Width, High Speed Enable, Card detect test level and signal selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_host_control1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_host_control1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_host_control1`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_host_control1")]
pub type SdhcWrap_CtlCfg_CtlcfgHostControl1 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_host_control1::SdhcWrap_CtlCfg_CtlcfgHostControl1Spec>;
#[doc = "This register is used to program DMA modes, LED Control, Data Transfer Width, High Speed Enable, Card detect test level and signal selection"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_host_control1;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_power_control (rw) register accessor: This register is used to program the SD Bus power and voltage level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_power_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_power_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_power_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_power_control")]
pub type SdhcWrap_CtlCfg_CtlcfgPowerControl =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_power_control::SdhcWrap_CtlCfg_CtlcfgPowerControlSpec>;
#[doc = "This register is used to program the SD Bus power and voltage level"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_power_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_block_gap_control (rw) register accessor: This register is used to program the block gap request, read wait control and interrupt at block gap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_block_gap_control")]
pub type SdhcWrap_CtlCfg_CtlcfgBlockGapControl = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec,
>;
#[doc = "This register is used to program the block gap request, read wait control and interrupt at block gap"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_wakeup_control (rw) register accessor: This register is used to program the wakeup functionality\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_wakeup_control")]
pub type SdhcWrap_CtlCfg_CtlcfgWakeupControl =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec>;
#[doc = "This register is used to program the wakeup functionality"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_clock_control (rw) register accessor: This register is used to program the Clock frequency select, generator select, Clock enable, Internal Clock state fields This register controls SDCLK in SD Mode and RCLK in UHS-II mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_clock_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_clock_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_clock_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_clock_control")]
pub type SdhcWrap_CtlCfg_CtlcfgClockControl =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_clock_control::SdhcWrap_CtlCfg_CtlcfgClockControlSpec>;
#[doc = "This register is used to program the Clock frequency select, generator select, Clock enable, Internal Clock state fields This register controls SDCLK in SD Mode and RCLK in UHS-II mode."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_clock_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_timeout_control (rw) register accessor: The register sets the Data Timeout counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_timeout_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_timeout_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_timeout_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_timeout_control")]
pub type SdhcWrap_CtlCfg_CtlcfgTimeoutControl = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_timeout_control::SdhcWrap_CtlCfg_CtlcfgTimeoutControlSpec,
>;
#[doc = "The register sets the Data Timeout counter value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_timeout_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_software_reset (rw) register accessor: This register is used to program the software reset for data, command and for all\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_software_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_software_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_software_reset`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_software_reset")]
pub type SdhcWrap_CtlCfg_CtlcfgSoftwareReset =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_software_reset::SdhcWrap_CtlCfg_CtlcfgSoftwareResetSpec>;
#[doc = "This register is used to program the software reset for data, command and for all"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_software_reset;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts (rw) register accessor: This register gives the status of all the interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts")]
pub type SdhcWrap_CtlCfg_CtlcfgNormalIntrSts =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec>;
#[doc = "This register gives the status of all the interrupts"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts (rw) register accessor: This register gives the status of the error interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts")]
pub type SdhcWrap_CtlCfg_CtlcfgErrorIntrSts =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec>;
#[doc = "This register gives the status of the error interrupts"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts_ena (rw) register accessor: This register is used to enable the normal interrupt status register fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEna = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena::SdhcWrap_CtlCfg_CtlcfgNormalIntrStsEnaSpec,
>;
#[doc = "This register is used to enable the normal interrupt status register fields"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts_ena (rw) register accessor: This register is used to enable the Error Interrupt Status register fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEna = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena::SdhcWrap_CtlCfg_CtlcfgErrorIntrStsEnaSpec,
>;
#[doc = "This register is used to enable the Error Interrupt Status register fields"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sig_ena (rw) register accessor: This register is used to enable the Normal Interrupt Signal register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sig_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgNormalIntrSigEna = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena::SdhcWrap_CtlCfg_CtlcfgNormalIntrSigEnaSpec,
>;
#[doc = "This register is used to enable the Normal Interrupt Signal register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sig_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sig_ena (rw) register accessor: This register is used to enable Error Interrupt Signal register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sig_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgErrorIntrSigEna = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena::SdhcWrap_CtlCfg_CtlcfgErrorIntrSigEnaSpec,
>;
#[doc = "This register is used to enable Error Interrupt Signal register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sig_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_autocmd_err_sts (rw) register accessor: This register is used to indicate CMD12 response error of Auto CMD12 and CMD23 response error of Auto CMD 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_autocmd_err_sts")]
pub type SdhcWrap_CtlCfg_CtlcfgAutocmdErrSts =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts::SdhcWrap_CtlCfg_CtlcfgAutocmdErrStsSpec>;
#[doc = "This register is used to indicate CMD12 response error of Auto CMD12 and CMD23 response error of Auto CMD 23"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_autocmd_err_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_host_control2 (rw) register accessor: This register is used to program UHS Select Mode,UHS Select Mode,Driver Strength Select,Execute Tuning,Sampling Clock Select,Asynchronous Interrupt Enable and Preset value enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_host_control2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_host_control2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_host_control2`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_host_control2")]
pub type SdhcWrap_CtlCfg_CtlcfgHostControl2 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_host_control2::SdhcWrap_CtlCfg_CtlcfgHostControl2Spec>;
#[doc = "This register is used to program UHS Select Mode,UHS Select Mode,Driver Strength Select,Execute Tuning,Sampling Clock Select,Asynchronous Interrupt Enable and Preset value enable"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_host_control2;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_capabilities (rw) register accessor: This register provides the HD with information specific to the HC implementation. The HC may implement these values as fixed or loaded from flash memory during power on initializa-tion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_capabilities::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_capabilities::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_capabilities`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_capabilities")]
pub type SdhcWrap_CtlCfg_CtlcfgCapabilities =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_capabilities::SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec>;
#[doc = "This register provides the HD with information specific to the HC implementation. The HC may implement these values as fixed or loaded from flash memory during power on initializa-tion."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_capabilities;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_max_current_cap (rw) register accessor: This register indicates maximum current capability for each voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_max_current_cap")]
pub type SdhcWrap_CtlCfg_CtlcfgMaxCurrentCap =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec>;
#[doc = "This register indicates maximum current capability for each voltage"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_ACMD_Err_Sts (rw) register accessor: This register is not physically implemented, rather it is an address where Auto CMD Error Status register can be written. Writing 1 : set each bit of the Auto CMD12 Error Status Register Writing 0 : no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_ACMD_Err_Sts")]
pub type SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrSts = crate :: Reg < sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts :: SdhcWrap_CtlCfg_CtlcfgForceEvntAcmdErrStsSpec > ;
#[doc = "This register is not physically implemented, rather it is an address where Auto CMD Error Status register can be written. Writing 1 : set each bit of the Auto CMD12 Error Status Register Writing 0 : no effect."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_acmd_err_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_Err_Int_Sts (rw) register accessor: This register is not physically implemented, rather it is an address where Error Interrupt Status register can be written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_force_evnt_Err_Int_Sts")]
pub type SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntSts = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts::SdhcWrap_CtlCfg_CtlcfgForceEvntErrIntStsSpec,
>;
#[doc = "This register is not physically implemented, rather it is an address where Error Interrupt Status register can be written."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_force_evnt_err_int_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_adma_err_status (rw) register accessor: When the ADMA Error interrupt occur, this register holds the ADMA State in ADMA Error States field and ADMA System Address holds address around the error descriptor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_adma_err_status")]
pub type SdhcWrap_CtlCfg_CtlcfgAdmaErrStatus =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status::SdhcWrap_CtlCfg_CtlcfgAdmaErrStatusSpec>;
#[doc = "When the ADMA Error interrupt occur, this register holds the ADMA State in ADMA Error States field and ADMA System Address holds address around the error descriptor"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_adma_err_status;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_adma_sys_address (rw) register accessor: This register contains the physical address used for ADMA data transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_adma_sys_address")]
pub type SdhcWrap_CtlCfg_CtlcfgAdmaSysAddress = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec,
>;
#[doc = "This register contains the physical address used for ADMA data transfer"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value0 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value0`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value0")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue0 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value0::SdhcWrap_CtlCfg_CtlcfgPresetValue0Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value0;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value1 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value1`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value1")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue1 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value1::SdhcWrap_CtlCfg_CtlcfgPresetValue1Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value1;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value2 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value2`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value2")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue2 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value2::SdhcWrap_CtlCfg_CtlcfgPresetValue2Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value2;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value3 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value3`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value3")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue3 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value3::SdhcWrap_CtlCfg_CtlcfgPresetValue3Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value3;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value4 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value4`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value4")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue4 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value4::SdhcWrap_CtlCfg_CtlcfgPresetValue4Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value4;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value5 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value5`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value5")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue5 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value5;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value6 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value6`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value6")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue6 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value6::SdhcWrap_CtlCfg_CtlcfgPresetValue6Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value6;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value7 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value7`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value7")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue7 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value7::SdhcWrap_CtlCfg_CtlcfgPresetValue7Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value7;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value8 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value8`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value8")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue8 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value8::SdhcWrap_CtlCfg_CtlcfgPresetValue8Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value8;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value10 (rw) register accessor: This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_preset_value10`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_preset_value10")]
pub type SdhcWrap_CtlCfg_CtlcfgPresetValue10 =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_preset_value10::SdhcWrap_CtlCfg_CtlcfgPresetValue10Spec>;
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_preset_value10;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_adma3_desc_address (rw) register accessor: The start address of Integrated DMA Descriptor is set to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_adma3_desc_address")]
pub type SdhcWrap_CtlCfg_CtlcfgAdma3DescAddress = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec,
>;
#[doc = "The start address of Integrated DMA Descriptor is set to this register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_size (rw) register accessor: This register is used to configure the number of bytes in a data block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_size")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2BlockSize =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec>;
#[doc = "This register is used to configure the number of bytes in a data block"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_count (rw) register accessor: This register is used to configure the number of data blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_count")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2BlockCount = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec,
>;
#[doc = "This register is used to configure the number of data blocks"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command_pkt (rw) register accessor: UHS-II Command Packet image is set to this register. The maximum length is 20 bytes. The command length varies depends on a Command Packet type. The length is specified by the UHS-II Command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command_pkt")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2CommandPkt = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt::SdhcWrap_CtlCfg_CtlcfgUhs2CommandPktSpec,
>;
#[doc = "UHS-II Command Packet image is set to this register. The maximum length is 20 bytes. The command length varies depends on a Command Packet type. The length is specified by the UHS-II Command register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command_pkt;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_xfer_mode (rw) register accessor: This register is used to control the operations of data transfers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_xfer_mode")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2XferMode =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec>;
#[doc = "This register is used to control the operations of data transfers"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command (rw) register accessor: This register is used to program the Command for host controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_command")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2Command =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command::SdhcWrap_CtlCfg_CtlcfgUhs2CommandSpec>;
#[doc = "This register is used to program the Command for host controller"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_command;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_response (rw) register accessor: This register is used to store received UHS-II RES Packet image\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_response")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2Response =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response::SdhcWrap_CtlCfg_CtlcfgUhs2ResponseSpec>;
#[doc = "This register is used to store received UHS-II RES Packet image"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_response;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message_select (rw) register accessor: This register is used to access internal buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message_select")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelect = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select::SdhcWrap_CtlCfg_CtlcfgUhs2MessageSelectSpec,
>;
#[doc = "This register is used to access internal buffer"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message_select;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message (rw) register accessor: This register is used to access internal buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_message")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2Message =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message::SdhcWrap_CtlCfg_CtlcfgUhs2MessageSpec>;
#[doc = "This register is used to access internal buffer"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_message;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_intr_status (rw) register accessor: This register shows receipt of INT MSG from which device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_intr_status")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatus = crate :: Reg < sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status :: SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntrStatusSpec > ;
#[doc = "This register shows receipt of INT MSG from which device"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_intr_status;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_select (rw) register accessor: UHS-II Device Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_select")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelect = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select::SdhcWrap_CtlCfg_CtlcfgUhs2DeviceSelectSpec,
>;
#[doc = "UHS-II Device Select Register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_select;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_int_code (rw) register accessor: This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_device_int_code")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCode = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code::SdhcWrap_CtlCfg_CtlcfgUhs2DeviceIntCodeSpec,
>;
#[doc = "This register is effective when INT MSG Enable is set to 1 in the UHS-II Device Select register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_device_int_code;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_software_reset (rw) register accessor: UHS-II Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_software_reset")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareReset = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset::SdhcWrap_CtlCfg_CtlcfgUhs2SoftwareResetSpec,
>;
#[doc = "UHS-II Software Reset Register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_software_reset;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_timer_control (rw) register accessor: UHS-II Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_timer_control")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2TimerControl = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec,
>;
#[doc = "UHS-II Timeout Control Register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts (rw) register accessor: This register gives the status of all UHS-II interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSts = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts::SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsSpec,
>;
#[doc = "This register gives the status of all UHS-II interrupts"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts_ena (rw) register accessor: This register is used to enable the UHS-II Error Interrupt Status register fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sts_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsEna = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena::SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrStsEnaSpec,
>;
#[doc = "This register is used to enable the UHS-II Error Interrupt Status register fields"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sts_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sig_ena (rw) register accessor: This register is used to generate UHS-II Interrupt signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_err_intr_sig_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEna = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena::SdhcWrap_CtlCfg_CtlcfgUhs2ErrIntrSigEnaSpec,
>;
#[doc = "This register is used to generate UHS-II Interrupt signals"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_err_intr_sig_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_settings_ptr (rw) register accessor: This register is pointer for UHS-II settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_settings_ptr")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2SettingsPtr = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr::SdhcWrap_CtlCfg_CtlcfgUhs2SettingsPtrSpec,
>;
#[doc = "This register is pointer for UHS-II settings."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_settings_ptr;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_capabilities_ptr (rw) register accessor: This register is pointer for UHS-II Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_capabilities_ptr")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtr = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr::SdhcWrap_CtlCfg_CtlcfgUhs2CapabilitiesPtrSpec,
>;
#[doc = "This register is pointer for UHS-II Capabilities Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_capabilities_ptr;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_test_ptr (rw) register accessor: This register is pointer for UHS-II Test Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_test_ptr")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2TestPtr =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr::SdhcWrap_CtlCfg_CtlcfgUhs2TestPtrSpec>;
#[doc = "This register is pointer for UHS-II Test Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_test_ptr;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_shared_bus_ctrl_ptr (rw) register accessor: This register is pointer for UHS-II Shared Bus Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_shared_bus_ctrl_ptr")]
pub type SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtr = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr::SdhcWrap_CtlCfg_CtlcfgSharedBusCtrlPtrSpec,
>;
#[doc = "This register is pointer for UHS-II Shared Bus Control Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_shared_bus_ctrl_ptr;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_vendor_specfic_ptr (rw) register accessor: This register is pointer for UHS-II Vendor Specific Pointer Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_vendor_specfic_ptr")]
pub type SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtr = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec,
>;
#[doc = "This register is pointer for UHS-II Vendor Specific Pointer Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_boot_timeout_control (rw) register accessor: This is used to program the boot timeout value counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_boot_timeout_control")]
pub type SdhcWrap_CtlCfg_CtlcfgBootTimeoutControl = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control::SdhcWrap_CtlCfg_CtlcfgBootTimeoutControlSpec,
>;
#[doc = "This is used to program the boot timeout value counter"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_boot_timeout_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_vendor_register (rw) register accessor: Vendor register added for autogate sdclk, cmd11 power down timer, enhancedstrobe and eMMC hardware reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_vendor_register`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_vendor_register")]
pub type SdhcWrap_CtlCfg_CtlcfgVendorRegister = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec,
>;
#[doc = "Vendor register added for autogate sdclk, cmd11 power down timer, enhancedstrobe and eMMC hardware reset"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_vendor_register;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_slot_int_sts (rw) register accessor: This register is used to read the interrupt signal for each slot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_slot_int_sts")]
pub type SdhcWrap_CtlCfg_CtlcfgSlotIntSts =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec>;
#[doc = "This register is used to read the interrupt signal for each slot."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_host_controller_ver (rw) register accessor: This register is used to read the vendor version number and specification version number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_host_controller_ver")]
pub type SdhcWrap_CtlCfg_CtlcfgHostControllerVer = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver::SdhcWrap_CtlCfg_CtlcfgHostControllerVerSpec,
>;
#[doc = "This register is used to read the vendor version number and specification version number"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_host_controller_ver;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_settings (rw) register accessor: Start Address of General settings is pointed by Pointer for UHS-II Setting Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_settings")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2GenSettings = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings::SdhcWrap_CtlCfg_CtlcfgUhs2GenSettingsSpec,
>;
#[doc = "Start Address of General settings is pointed by Pointer for UHS-II Setting Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_settings;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_settings (rw) register accessor: Start Address of PHY settings is pointed by Pointer for UHS-II Setting Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_settings")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2PhySettings = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings::SdhcWrap_CtlCfg_CtlcfgUhs2PhySettingsSpec,
>;
#[doc = "Start Address of PHY settings is pointed by Pointer for UHS-II Setting Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_settings;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_settings (rw) register accessor: Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Setting Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_settings")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettings = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings::SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnSettingsSpec,
>;
#[doc = "Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Setting Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_settings;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_cap (rw) register accessor: Start Address of General Capabilities is pointed by Pointer for UHS-II Host Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_gen_cap")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2GenCap =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap::SdhcWrap_CtlCfg_CtlcfgUhs2GenCapSpec>;
#[doc = "Start Address of General Capabilities is pointed by Pointer for UHS-II Host Capabilities Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_gen_cap;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_cap (rw) register accessor: Start Address of PHY Capabilities is pointed by Pointer for UHS-II Host Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_phy_cap")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2PhyCap =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap::SdhcWrap_CtlCfg_CtlcfgUhs2PhyCapSpec>;
#[doc = "Start Address of PHY Capabilities is pointed by Pointer for UHS-II Host Capabilities Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_phy_cap;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_cap (rw) register accessor: Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Capabilities Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_lnk_trn_cap")]
pub type SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCap = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap::SdhcWrap_CtlCfg_CtlcfgUhs2LnkTrnCapSpec,
>;
#[doc = "Start Address of LINK/TRAN settings is pointed by Pointer for UHS-II Capabilities Register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_uhs2_lnk_trn_cap;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_force_UHSII_Err_Int_Sts (rw) register accessor: This register is not physically implemented, rather it is an address where UHS-II Error Interrupt Status register can be written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_force_UHSII_Err_Int_Sts")]
pub type SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntSts = crate :: Reg < sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts :: SdhcWrap_CtlCfg_CtlcfgForceUhsiiErrIntStsSpec > ;
#[doc = "This register is not physically implemented, rather it is an address where UHS-II Error Interrupt Status register can be written."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_force_uhsii_err_int_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_version (rw) register accessor: This register provides information about the version of the eMMC CQ standard which is 285 implemented by the CQE, in BCD format. The current version is rev 5.1 The following table describes the CQBASE+00h: CQVReservedER Command Queueing Version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_version`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_version")]
pub type SdhcWrap_CtlCfg_CtlcfgCqVersion =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_version::SdhcWrap_CtlCfg_CtlcfgCqVersionSpec>;
#[doc = "This register provides information about the version of the eMMC CQ standard which is 285 implemented by the CQE, in BCD format. The current version is rev 5.1 The following table describes the CQBASE+00h: CQVReservedER Command Queueing Version."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_version;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_capabilities (rw) register accessor: This register is reserved for capability indication.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_capabilities")]
pub type SdhcWrap_CtlCfg_CtlcfgCqCapabilities = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec,
>;
#[doc = "This register is reserved for capability indication."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_config (rw) register accessor: This register controls CQE behavior affecting the general operation of command queueing 290 module or operation of multiple tasks in the same time.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_config`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_config")]
pub type SdhcWrap_CtlCfg_CtlcfgCqConfig =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_config::SdhcWrap_CtlCfg_CtlcfgCqConfigSpec>;
#[doc = "This register controls CQE behavior affecting the general operation of command queueing 290 module or operation of multiple tasks in the same time."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_config;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_control (rw) register accessor: This register controls CQE behavior affecting the general operation of command queueing 293 module or operation of multiple tasks in the same time.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_control`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_control")]
pub type SdhcWrap_CtlCfg_CtlcfgCqControl =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_control::SdhcWrap_CtlCfg_CtlcfgCqControlSpec>;
#[doc = "This register controls CQE behavior affecting the general operation of command queueing 293 module or operation of multiple tasks in the same time."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_control;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts (rw) register accessor: This register indicates pending interrupts that require service. Each bit in this registers is asserted 296 in response a specific event, only if the respective bit is set in CQ ISTE register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts")]
pub type SdhcWrap_CtlCfg_CtlcfgCqIntrSts =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts::SdhcWrap_CtlCfg_CtlcfgCqIntrStsSpec>;
#[doc = "This register indicates pending interrupts that require service. Each bit in this registers is asserted 296 in response a specific event, only if the respective bit is set in CQ ISTE register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts_ena (rw) register accessor: This register enables and disables the reporting of the corresponding interrupt to host soft-ware in 299 CQIS register. When a bit is set ( 1 ) and the corresponding interrupt c -ondition is active, then the 300 bit is CQIS is asserted. Interrupt sources that are disabled ( 0 ) are not indicated in the CQIS 301 register. This register is bit-index matched to CQIS register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sts_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgCqIntrStsEna =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena::SdhcWrap_CtlCfg_CtlcfgCqIntrStsEnaSpec>;
#[doc = "This register enables and disables the reporting of the corresponding interrupt to host soft-ware in 299 CQIS register. When a bit is set ( 1 ) and the corresponding interrupt c -ondition is active, then the 300 bit is CQIS is asserted. Interrupt sources that are disabled ( 0 ) are not indicated in the CQIS 301 register. This register is bit-index matched to CQIS register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sts_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sig_ena (rw) register accessor: This register enables and disables the generation of interrupts to host software. When a bit is set 304 ( 1 ) and the corresponding bit in CQIS is set, then an interrupt is gene -rated. Interrupt sources 305 that are disabled ( 0 ) are still indicated in the CQIS register. This register is bit-index matched 306 to CQIS register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_sig_ena")]
pub type SdhcWrap_CtlCfg_CtlcfgCqIntrSigEna =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena::SdhcWrap_CtlCfg_CtlcfgCqIntrSigEnaSpec>;
#[doc = "This register enables and disables the generation of interrupts to host software. When a bit is set 304 ( 1 ) and the corresponding bit in CQIS is set, then an interrupt is gene -rated. Interrupt sources 305 that are disabled ( 0 ) are still indicated in the CQIS register. This register is bit-index matched 306 to CQIS register."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_sig_ena;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_coalescing (rw) register accessor: This register controls the interrupt coalescing feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_intr_coalescing")]
pub type SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescing = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing::SdhcWrap_CtlCfg_CtlcfgCqIntrCoalescingSpec,
>;
#[doc = "This register controls the interrupt coalescing feature."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_intr_coalescing;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr (rw) register accessor: This register is used for configuring the lower 32 bits of the byte address of the head of the Task 312 Descriptor List in the host memory.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr")]
pub type SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddr = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec,
>;
#[doc = "This register is used for configuring the lower 32 bits of the byte address of the head of the Task 312 Descriptor List in the host memory."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr_upbits (rw) register accessor: This register is used for configuring the upper 32 bits of the byte address of the head of the Task 316 Descriptor List in the host memory.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr_upbits")]
pub type SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbits = crate :: Reg < sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits :: SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec > ;
#[doc = "This register is used for configuring the upper 32 bits of the byte address of the head of the Task 316 Descriptor List in the host memory."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_door_bell (rw) register accessor: Using this register, software triggers CQE to process a new task.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_door_bell")]
pub type SdhcWrap_CtlCfg_CtlcfgCqTaskDoorBell = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell::SdhcWrap_CtlCfg_CtlcfgCqTaskDoorBellSpec,
>;
#[doc = "Using this register, software triggers CQE to process a new task."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_task_door_bell;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_comp_notif (rw) register accessor: This register is used by CQE to notify software about completed tasks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_comp_notif")]
pub type SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotif = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif::SdhcWrap_CtlCfg_CtlcfgCqTaskCompNotifSpec,
>;
#[doc = "This register is used by CQE to notify software about completed tasks."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_task_comp_notif;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_queue_status (rw) register accessor: This register stores the most recent value of the device s queue status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_queue_status")]
pub type SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatus = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status::SdhcWrap_CtlCfg_CtlcfgCqDevQueueStatusSpec,
>;
#[doc = "This register stores the most recent value of the device s queue status."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_queue_status;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_pending_tasks (rw) register accessor: This register indicates to software which tasks are queued in the device, awaiting execution.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_dev_pending_tasks")]
pub type SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasks = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks::SdhcWrap_CtlCfg_CtlcfgCqDevPendingTasksSpec,
>;
#[doc = "This register indicates to software which tasks are queued in the device, awaiting execution."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_dev_pending_tasks;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_clear (rw) register accessor: This register is used for removing an outstanding task in the CQE. 327. The register should be used only when CQE is in Halt state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_clear")]
pub type SdhcWrap_CtlCfg_CtlcfgCqTaskClear =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear::SdhcWrap_CtlCfg_CtlcfgCqTaskClearSpec>;
#[doc = "This register is used for removing an outstanding task in the CQE. 327. The register should be used only when CQE is in Halt state."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_task_clear;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config1 (rw) register accessor: The register controls the when SEND_QUEUE_STATUS commands are sent.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config1")]
pub type SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1 = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1::SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig1Spec,
>;
#[doc = "The register controls the when SEND_QUEUE_STATUS commands are sent."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config1;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config2 (rw) register accessor: This register is used for 333 configuring RCA field in SEND_QUEUE_STATUS command argu-ment.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config2")]
pub type SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2 = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec,
>;
#[doc = "This register is used for 333 configuring RCA field in SEND_QUEUE_STATUS command argu-ment."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_dcmd_response (rw) register accessor: This register is used for passing the response of a DCMD task to software.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_dcmd_response")]
pub type SdhcWrap_CtlCfg_CtlcfgCqDcmdResponse = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response::SdhcWrap_CtlCfg_CtlcfgCqDcmdResponseSpec,
>;
#[doc = "This register is used for passing the response of a DCMD task to software."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_dcmd_response;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_resp_err_mask (rw) register accessor: This register controls the generation of Response Error Detection (RED) interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_resp_err_mask")]
pub type SdhcWrap_CtlCfg_CtlcfgCqRespErrMask = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec,
>;
#[doc = "This register controls the generation of Response Error Detection (RED) interrupt."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_err_info (rw) register accessor: This register is updated by CQE when an error occurs on data or command related to a task activity. When such error is detected by CQE or indicated by the eMMC controller CQE stores in CQTERRI the task IDs and the command indices of the commands which were executed on the 343 command line and data lines when the error occurred.Software is expected to use this information in the error recovery procedure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_task_err_info")]
pub type SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfo = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info::SdhcWrap_CtlCfg_CtlcfgCqTaskErrInfoSpec,
>;
#[doc = "This register is updated by CQE when an error occurs on data or command related to a task activity. When such error is detected by CQE or indicated by the eMMC controller CQE stores in CQTERRI the task IDs and the command indices of the commands which were executed on the 343 command line and data lines when the error occurred.Software is expected to use this information in the error recovery procedure."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_task_err_info;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_index (rw) register accessor: This register stores the index of the last received command response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_index")]
pub type SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndex = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index::SdhcWrap_CtlCfg_CtlcfgCqCmdRespIndexSpec,
>;
#[doc = "This register stores the index of the last received command response."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_index;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_arg (rw) register accessor: This register stores the index of the last received command response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_cmd_resp_arg")]
pub type SdhcWrap_CtlCfg_CtlcfgCqCmdRespArg =
    crate::Reg<sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg::SdhcWrap_CtlCfg_CtlcfgCqCmdRespArgSpec>;
#[doc = "This register stores the index of the last received command response."]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_cmd_resp_arg;
#[doc = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_error_task_id (rw) register accessor: CQ Error Task ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id`]
module"]
#[doc(alias = "SDHC_WRAP__CTL_CFG__CTLCFG_cq_error_task_id")]
pub type SdhcWrap_CtlCfg_CtlcfgCqErrorTaskId = crate::Reg<
    sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id::SdhcWrap_CtlCfg_CtlcfgCqErrorTaskIdSpec,
>;
#[doc = "CQ Error Task ID Register"]
pub mod sdhc_wrap__ctl_cfg__ctlcfg_cq_error_task_id;
