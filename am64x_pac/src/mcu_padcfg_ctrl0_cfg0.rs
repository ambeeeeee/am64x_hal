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
    _reserved13: [u8; 0x0fcc],
    cfg0_pid_proxy: Cfg0PidProxy,
    _reserved14: [u8; 0x04],
    cfg0_mmr_cfg1_proxy: Cfg0MmrCfg1Proxy,
    _reserved15: [u8; 0x0ffc],
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
    _reserved26: [u8; 0x0fcc],
    cfg0_padconfig0: Cfg0Padconfig0,
    cfg0_padconfig1: Cfg0Padconfig1,
    cfg0_padconfig2: Cfg0Padconfig2,
    cfg0_padconfig3: Cfg0Padconfig3,
    cfg0_padconfig4: Cfg0Padconfig4,
    cfg0_padconfig5: Cfg0Padconfig5,
    cfg0_padconfig6: Cfg0Padconfig6,
    cfg0_padconfig7: Cfg0Padconfig7,
    cfg0_padconfig8: Cfg0Padconfig8,
    cfg0_padconfig9: Cfg0Padconfig9,
    cfg0_padconfig10: Cfg0Padconfig10,
    cfg0_padconfig11: Cfg0Padconfig11,
    cfg0_padconfig12: Cfg0Padconfig12,
    cfg0_padconfig13: Cfg0Padconfig13,
    cfg0_padconfig14: Cfg0Padconfig14,
    cfg0_padconfig15: Cfg0Padconfig15,
    cfg0_padconfig16: Cfg0Padconfig16,
    cfg0_padconfig17: Cfg0Padconfig17,
    cfg0_padconfig18: Cfg0Padconfig18,
    cfg0_padconfig19: Cfg0Padconfig19,
    cfg0_padconfig20: Cfg0Padconfig20,
    cfg0_padconfig21: Cfg0Padconfig21,
    cfg0_padconfig22: Cfg0Padconfig22,
    cfg0_padconfig23: Cfg0Padconfig23,
    cfg0_padconfig24: Cfg0Padconfig24,
    cfg0_padconfig25: Cfg0Padconfig25,
    cfg0_padconfig26: Cfg0Padconfig26,
    cfg0_padconfig27: Cfg0Padconfig27,
    cfg0_padconfig28: Cfg0Padconfig28,
    cfg0_padconfig29: Cfg0Padconfig29,
    cfg0_padconfig30: Cfg0Padconfig30,
    cfg0_padconfig31: Cfg0Padconfig31,
    cfg0_padconfig32: Cfg0Padconfig32,
    _reserved59: [u8; 0x0f84],
    cfg0_lock1_kick0: Cfg0Lock1Kick0,
    cfg0_lock1_kick1: Cfg0Lock1Kick1,
    _reserved61: [u8; 0xf0],
    cfg0_claimreg_p1_r0_readonly: Cfg0ClaimregP1R0Readonly,
    cfg0_claimreg_p1_r1_readonly: Cfg0ClaimregP1R1Readonly,
    _reserved63: [u8; 0x0ef8],
    cfg0_padconfig0_proxy: Cfg0Padconfig0Proxy,
    cfg0_padconfig1_proxy: Cfg0Padconfig1Proxy,
    cfg0_padconfig2_proxy: Cfg0Padconfig2Proxy,
    cfg0_padconfig3_proxy: Cfg0Padconfig3Proxy,
    cfg0_padconfig4_proxy: Cfg0Padconfig4Proxy,
    cfg0_padconfig5_proxy: Cfg0Padconfig5Proxy,
    cfg0_padconfig6_proxy: Cfg0Padconfig6Proxy,
    cfg0_padconfig7_proxy: Cfg0Padconfig7Proxy,
    cfg0_padconfig8_proxy: Cfg0Padconfig8Proxy,
    cfg0_padconfig9_proxy: Cfg0Padconfig9Proxy,
    cfg0_padconfig10_proxy: Cfg0Padconfig10Proxy,
    cfg0_padconfig11_proxy: Cfg0Padconfig11Proxy,
    cfg0_padconfig12_proxy: Cfg0Padconfig12Proxy,
    cfg0_padconfig13_proxy: Cfg0Padconfig13Proxy,
    cfg0_padconfig14_proxy: Cfg0Padconfig14Proxy,
    cfg0_padconfig15_proxy: Cfg0Padconfig15Proxy,
    cfg0_padconfig16_proxy: Cfg0Padconfig16Proxy,
    cfg0_padconfig17_proxy: Cfg0Padconfig17Proxy,
    cfg0_padconfig18_proxy: Cfg0Padconfig18Proxy,
    cfg0_padconfig19_proxy: Cfg0Padconfig19Proxy,
    cfg0_padconfig20_proxy: Cfg0Padconfig20Proxy,
    cfg0_padconfig21_proxy: Cfg0Padconfig21Proxy,
    cfg0_padconfig22_proxy: Cfg0Padconfig22Proxy,
    cfg0_padconfig23_proxy: Cfg0Padconfig23Proxy,
    cfg0_padconfig24_proxy: Cfg0Padconfig24Proxy,
    cfg0_padconfig25_proxy: Cfg0Padconfig25Proxy,
    cfg0_padconfig26_proxy: Cfg0Padconfig26Proxy,
    cfg0_padconfig27_proxy: Cfg0Padconfig27Proxy,
    cfg0_padconfig28_proxy: Cfg0Padconfig28Proxy,
    cfg0_padconfig29_proxy: Cfg0Padconfig29Proxy,
    cfg0_padconfig30_proxy: Cfg0Padconfig30Proxy,
    cfg0_padconfig31_proxy: Cfg0Padconfig31Proxy,
    cfg0_padconfig32_proxy: Cfg0Padconfig32Proxy,
    _reserved96: [u8; 0x0f84],
    cfg0_lock1_kick0_proxy: Cfg0Lock1Kick0Proxy,
    cfg0_lock1_kick1_proxy: Cfg0Lock1Kick1Proxy,
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
    #[doc = "0x4000 - CFG0_PADCONFIG0"]
    #[inline(always)]
    pub const fn cfg0_padconfig0(&self) -> &Cfg0Padconfig0 {
        &self.cfg0_padconfig0
    }
    #[doc = "0x4004 - CFG0_PADCONFIG1"]
    #[inline(always)]
    pub const fn cfg0_padconfig1(&self) -> &Cfg0Padconfig1 {
        &self.cfg0_padconfig1
    }
    #[doc = "0x4008 - CFG0_PADCONFIG2"]
    #[inline(always)]
    pub const fn cfg0_padconfig2(&self) -> &Cfg0Padconfig2 {
        &self.cfg0_padconfig2
    }
    #[doc = "0x400c - CFG0_PADCONFIG3"]
    #[inline(always)]
    pub const fn cfg0_padconfig3(&self) -> &Cfg0Padconfig3 {
        &self.cfg0_padconfig3
    }
    #[doc = "0x4010 - CFG0_PADCONFIG4"]
    #[inline(always)]
    pub const fn cfg0_padconfig4(&self) -> &Cfg0Padconfig4 {
        &self.cfg0_padconfig4
    }
    #[doc = "0x4014 - CFG0_PADCONFIG5"]
    #[inline(always)]
    pub const fn cfg0_padconfig5(&self) -> &Cfg0Padconfig5 {
        &self.cfg0_padconfig5
    }
    #[doc = "0x4018 - CFG0_PADCONFIG6"]
    #[inline(always)]
    pub const fn cfg0_padconfig6(&self) -> &Cfg0Padconfig6 {
        &self.cfg0_padconfig6
    }
    #[doc = "0x401c - CFG0_PADCONFIG7"]
    #[inline(always)]
    pub const fn cfg0_padconfig7(&self) -> &Cfg0Padconfig7 {
        &self.cfg0_padconfig7
    }
    #[doc = "0x4020 - CFG0_PADCONFIG8"]
    #[inline(always)]
    pub const fn cfg0_padconfig8(&self) -> &Cfg0Padconfig8 {
        &self.cfg0_padconfig8
    }
    #[doc = "0x4024 - CFG0_PADCONFIG9"]
    #[inline(always)]
    pub const fn cfg0_padconfig9(&self) -> &Cfg0Padconfig9 {
        &self.cfg0_padconfig9
    }
    #[doc = "0x4028 - CFG0_PADCONFIG10"]
    #[inline(always)]
    pub const fn cfg0_padconfig10(&self) -> &Cfg0Padconfig10 {
        &self.cfg0_padconfig10
    }
    #[doc = "0x402c - CFG0_PADCONFIG11"]
    #[inline(always)]
    pub const fn cfg0_padconfig11(&self) -> &Cfg0Padconfig11 {
        &self.cfg0_padconfig11
    }
    #[doc = "0x4030 - CFG0_PADCONFIG12"]
    #[inline(always)]
    pub const fn cfg0_padconfig12(&self) -> &Cfg0Padconfig12 {
        &self.cfg0_padconfig12
    }
    #[doc = "0x4034 - CFG0_PADCONFIG13"]
    #[inline(always)]
    pub const fn cfg0_padconfig13(&self) -> &Cfg0Padconfig13 {
        &self.cfg0_padconfig13
    }
    #[doc = "0x4038 - CFG0_PADCONFIG14"]
    #[inline(always)]
    pub const fn cfg0_padconfig14(&self) -> &Cfg0Padconfig14 {
        &self.cfg0_padconfig14
    }
    #[doc = "0x403c - CFG0_PADCONFIG15"]
    #[inline(always)]
    pub const fn cfg0_padconfig15(&self) -> &Cfg0Padconfig15 {
        &self.cfg0_padconfig15
    }
    #[doc = "0x4040 - CFG0_PADCONFIG16"]
    #[inline(always)]
    pub const fn cfg0_padconfig16(&self) -> &Cfg0Padconfig16 {
        &self.cfg0_padconfig16
    }
    #[doc = "0x4044 - CFG0_PADCONFIG17"]
    #[inline(always)]
    pub const fn cfg0_padconfig17(&self) -> &Cfg0Padconfig17 {
        &self.cfg0_padconfig17
    }
    #[doc = "0x4048 - CFG0_PADCONFIG18"]
    #[inline(always)]
    pub const fn cfg0_padconfig18(&self) -> &Cfg0Padconfig18 {
        &self.cfg0_padconfig18
    }
    #[doc = "0x404c - CFG0_PADCONFIG19"]
    #[inline(always)]
    pub const fn cfg0_padconfig19(&self) -> &Cfg0Padconfig19 {
        &self.cfg0_padconfig19
    }
    #[doc = "0x4050 - CFG0_PADCONFIG20"]
    #[inline(always)]
    pub const fn cfg0_padconfig20(&self) -> &Cfg0Padconfig20 {
        &self.cfg0_padconfig20
    }
    #[doc = "0x4054 - CFG0_PADCONFIG21"]
    #[inline(always)]
    pub const fn cfg0_padconfig21(&self) -> &Cfg0Padconfig21 {
        &self.cfg0_padconfig21
    }
    #[doc = "0x4058 - CFG0_PADCONFIG22"]
    #[inline(always)]
    pub const fn cfg0_padconfig22(&self) -> &Cfg0Padconfig22 {
        &self.cfg0_padconfig22
    }
    #[doc = "0x405c - CFG0_PADCONFIG23"]
    #[inline(always)]
    pub const fn cfg0_padconfig23(&self) -> &Cfg0Padconfig23 {
        &self.cfg0_padconfig23
    }
    #[doc = "0x4060 - CFG0_PADCONFIG24"]
    #[inline(always)]
    pub const fn cfg0_padconfig24(&self) -> &Cfg0Padconfig24 {
        &self.cfg0_padconfig24
    }
    #[doc = "0x4064 - CFG0_PADCONFIG25"]
    #[inline(always)]
    pub const fn cfg0_padconfig25(&self) -> &Cfg0Padconfig25 {
        &self.cfg0_padconfig25
    }
    #[doc = "0x4068 - CFG0_PADCONFIG26"]
    #[inline(always)]
    pub const fn cfg0_padconfig26(&self) -> &Cfg0Padconfig26 {
        &self.cfg0_padconfig26
    }
    #[doc = "0x406c - CFG0_PADCONFIG27"]
    #[inline(always)]
    pub const fn cfg0_padconfig27(&self) -> &Cfg0Padconfig27 {
        &self.cfg0_padconfig27
    }
    #[doc = "0x4070 - CFG0_PADCONFIG28"]
    #[inline(always)]
    pub const fn cfg0_padconfig28(&self) -> &Cfg0Padconfig28 {
        &self.cfg0_padconfig28
    }
    #[doc = "0x4074 - CFG0_PADCONFIG29"]
    #[inline(always)]
    pub const fn cfg0_padconfig29(&self) -> &Cfg0Padconfig29 {
        &self.cfg0_padconfig29
    }
    #[doc = "0x4078 - CFG0_PADCONFIG30"]
    #[inline(always)]
    pub const fn cfg0_padconfig30(&self) -> &Cfg0Padconfig30 {
        &self.cfg0_padconfig30
    }
    #[doc = "0x407c - CFG0_PADCONFIG31"]
    #[inline(always)]
    pub const fn cfg0_padconfig31(&self) -> &Cfg0Padconfig31 {
        &self.cfg0_padconfig31
    }
    #[doc = "0x4080 - CFG0_PADCONFIG32"]
    #[inline(always)]
    pub const fn cfg0_padconfig32(&self) -> &Cfg0Padconfig32 {
        &self.cfg0_padconfig32
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
    #[doc = "0x6000 - CFG0_PADCONFIG0_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig0_proxy(&self) -> &Cfg0Padconfig0Proxy {
        &self.cfg0_padconfig0_proxy
    }
    #[doc = "0x6004 - CFG0_PADCONFIG1_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig1_proxy(&self) -> &Cfg0Padconfig1Proxy {
        &self.cfg0_padconfig1_proxy
    }
    #[doc = "0x6008 - CFG0_PADCONFIG2_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig2_proxy(&self) -> &Cfg0Padconfig2Proxy {
        &self.cfg0_padconfig2_proxy
    }
    #[doc = "0x600c - CFG0_PADCONFIG3_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig3_proxy(&self) -> &Cfg0Padconfig3Proxy {
        &self.cfg0_padconfig3_proxy
    }
    #[doc = "0x6010 - CFG0_PADCONFIG4_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig4_proxy(&self) -> &Cfg0Padconfig4Proxy {
        &self.cfg0_padconfig4_proxy
    }
    #[doc = "0x6014 - CFG0_PADCONFIG5_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig5_proxy(&self) -> &Cfg0Padconfig5Proxy {
        &self.cfg0_padconfig5_proxy
    }
    #[doc = "0x6018 - CFG0_PADCONFIG6_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig6_proxy(&self) -> &Cfg0Padconfig6Proxy {
        &self.cfg0_padconfig6_proxy
    }
    #[doc = "0x601c - CFG0_PADCONFIG7_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig7_proxy(&self) -> &Cfg0Padconfig7Proxy {
        &self.cfg0_padconfig7_proxy
    }
    #[doc = "0x6020 - CFG0_PADCONFIG8_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig8_proxy(&self) -> &Cfg0Padconfig8Proxy {
        &self.cfg0_padconfig8_proxy
    }
    #[doc = "0x6024 - CFG0_PADCONFIG9_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig9_proxy(&self) -> &Cfg0Padconfig9Proxy {
        &self.cfg0_padconfig9_proxy
    }
    #[doc = "0x6028 - CFG0_PADCONFIG10_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig10_proxy(&self) -> &Cfg0Padconfig10Proxy {
        &self.cfg0_padconfig10_proxy
    }
    #[doc = "0x602c - CFG0_PADCONFIG11_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig11_proxy(&self) -> &Cfg0Padconfig11Proxy {
        &self.cfg0_padconfig11_proxy
    }
    #[doc = "0x6030 - CFG0_PADCONFIG12_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig12_proxy(&self) -> &Cfg0Padconfig12Proxy {
        &self.cfg0_padconfig12_proxy
    }
    #[doc = "0x6034 - CFG0_PADCONFIG13_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig13_proxy(&self) -> &Cfg0Padconfig13Proxy {
        &self.cfg0_padconfig13_proxy
    }
    #[doc = "0x6038 - CFG0_PADCONFIG14_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig14_proxy(&self) -> &Cfg0Padconfig14Proxy {
        &self.cfg0_padconfig14_proxy
    }
    #[doc = "0x603c - CFG0_PADCONFIG15_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig15_proxy(&self) -> &Cfg0Padconfig15Proxy {
        &self.cfg0_padconfig15_proxy
    }
    #[doc = "0x6040 - CFG0_PADCONFIG16_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig16_proxy(&self) -> &Cfg0Padconfig16Proxy {
        &self.cfg0_padconfig16_proxy
    }
    #[doc = "0x6044 - CFG0_PADCONFIG17_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig17_proxy(&self) -> &Cfg0Padconfig17Proxy {
        &self.cfg0_padconfig17_proxy
    }
    #[doc = "0x6048 - CFG0_PADCONFIG18_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig18_proxy(&self) -> &Cfg0Padconfig18Proxy {
        &self.cfg0_padconfig18_proxy
    }
    #[doc = "0x604c - CFG0_PADCONFIG19_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig19_proxy(&self) -> &Cfg0Padconfig19Proxy {
        &self.cfg0_padconfig19_proxy
    }
    #[doc = "0x6050 - CFG0_PADCONFIG20_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig20_proxy(&self) -> &Cfg0Padconfig20Proxy {
        &self.cfg0_padconfig20_proxy
    }
    #[doc = "0x6054 - CFG0_PADCONFIG21_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig21_proxy(&self) -> &Cfg0Padconfig21Proxy {
        &self.cfg0_padconfig21_proxy
    }
    #[doc = "0x6058 - CFG0_PADCONFIG22_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig22_proxy(&self) -> &Cfg0Padconfig22Proxy {
        &self.cfg0_padconfig22_proxy
    }
    #[doc = "0x605c - CFG0_PADCONFIG23_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig23_proxy(&self) -> &Cfg0Padconfig23Proxy {
        &self.cfg0_padconfig23_proxy
    }
    #[doc = "0x6060 - CFG0_PADCONFIG24_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig24_proxy(&self) -> &Cfg0Padconfig24Proxy {
        &self.cfg0_padconfig24_proxy
    }
    #[doc = "0x6064 - CFG0_PADCONFIG25_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig25_proxy(&self) -> &Cfg0Padconfig25Proxy {
        &self.cfg0_padconfig25_proxy
    }
    #[doc = "0x6068 - CFG0_PADCONFIG26_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig26_proxy(&self) -> &Cfg0Padconfig26Proxy {
        &self.cfg0_padconfig26_proxy
    }
    #[doc = "0x606c - CFG0_PADCONFIG27_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig27_proxy(&self) -> &Cfg0Padconfig27Proxy {
        &self.cfg0_padconfig27_proxy
    }
    #[doc = "0x6070 - CFG0_PADCONFIG28_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig28_proxy(&self) -> &Cfg0Padconfig28Proxy {
        &self.cfg0_padconfig28_proxy
    }
    #[doc = "0x6074 - CFG0_PADCONFIG29_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig29_proxy(&self) -> &Cfg0Padconfig29Proxy {
        &self.cfg0_padconfig29_proxy
    }
    #[doc = "0x6078 - CFG0_PADCONFIG30_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig30_proxy(&self) -> &Cfg0Padconfig30Proxy {
        &self.cfg0_padconfig30_proxy
    }
    #[doc = "0x607c - CFG0_PADCONFIG31_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig31_proxy(&self) -> &Cfg0Padconfig31Proxy {
        &self.cfg0_padconfig31_proxy
    }
    #[doc = "0x6080 - CFG0_PADCONFIG32_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig32_proxy(&self) -> &Cfg0Padconfig32Proxy {
        &self.cfg0_padconfig32_proxy
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
#[doc = "CFG0_PADCONFIG0 (rw) register accessor: CFG0_PADCONFIG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig0`]
module"]
#[doc(alias = "CFG0_PADCONFIG0")]
pub type Cfg0Padconfig0 = crate::Reg<cfg0_padconfig0::Cfg0Padconfig0Spec>;
#[doc = "CFG0_PADCONFIG0"]
pub mod cfg0_padconfig0;
#[doc = "CFG0_PADCONFIG1 (rw) register accessor: CFG0_PADCONFIG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig1`]
module"]
#[doc(alias = "CFG0_PADCONFIG1")]
pub type Cfg0Padconfig1 = crate::Reg<cfg0_padconfig1::Cfg0Padconfig1Spec>;
#[doc = "CFG0_PADCONFIG1"]
pub mod cfg0_padconfig1;
#[doc = "CFG0_PADCONFIG2 (rw) register accessor: CFG0_PADCONFIG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig2`]
module"]
#[doc(alias = "CFG0_PADCONFIG2")]
pub type Cfg0Padconfig2 = crate::Reg<cfg0_padconfig2::Cfg0Padconfig2Spec>;
#[doc = "CFG0_PADCONFIG2"]
pub mod cfg0_padconfig2;
#[doc = "CFG0_PADCONFIG3 (rw) register accessor: CFG0_PADCONFIG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig3`]
module"]
#[doc(alias = "CFG0_PADCONFIG3")]
pub type Cfg0Padconfig3 = crate::Reg<cfg0_padconfig3::Cfg0Padconfig3Spec>;
#[doc = "CFG0_PADCONFIG3"]
pub mod cfg0_padconfig3;
#[doc = "CFG0_PADCONFIG4 (rw) register accessor: CFG0_PADCONFIG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig4`]
module"]
#[doc(alias = "CFG0_PADCONFIG4")]
pub type Cfg0Padconfig4 = crate::Reg<cfg0_padconfig4::Cfg0Padconfig4Spec>;
#[doc = "CFG0_PADCONFIG4"]
pub mod cfg0_padconfig4;
#[doc = "CFG0_PADCONFIG5 (rw) register accessor: CFG0_PADCONFIG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig5`]
module"]
#[doc(alias = "CFG0_PADCONFIG5")]
pub type Cfg0Padconfig5 = crate::Reg<cfg0_padconfig5::Cfg0Padconfig5Spec>;
#[doc = "CFG0_PADCONFIG5"]
pub mod cfg0_padconfig5;
#[doc = "CFG0_PADCONFIG6 (rw) register accessor: CFG0_PADCONFIG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig6`]
module"]
#[doc(alias = "CFG0_PADCONFIG6")]
pub type Cfg0Padconfig6 = crate::Reg<cfg0_padconfig6::Cfg0Padconfig6Spec>;
#[doc = "CFG0_PADCONFIG6"]
pub mod cfg0_padconfig6;
#[doc = "CFG0_PADCONFIG7 (rw) register accessor: CFG0_PADCONFIG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig7`]
module"]
#[doc(alias = "CFG0_PADCONFIG7")]
pub type Cfg0Padconfig7 = crate::Reg<cfg0_padconfig7::Cfg0Padconfig7Spec>;
#[doc = "CFG0_PADCONFIG7"]
pub mod cfg0_padconfig7;
#[doc = "CFG0_PADCONFIG8 (rw) register accessor: CFG0_PADCONFIG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig8`]
module"]
#[doc(alias = "CFG0_PADCONFIG8")]
pub type Cfg0Padconfig8 = crate::Reg<cfg0_padconfig8::Cfg0Padconfig8Spec>;
#[doc = "CFG0_PADCONFIG8"]
pub mod cfg0_padconfig8;
#[doc = "CFG0_PADCONFIG9 (rw) register accessor: CFG0_PADCONFIG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig9`]
module"]
#[doc(alias = "CFG0_PADCONFIG9")]
pub type Cfg0Padconfig9 = crate::Reg<cfg0_padconfig9::Cfg0Padconfig9Spec>;
#[doc = "CFG0_PADCONFIG9"]
pub mod cfg0_padconfig9;
#[doc = "CFG0_PADCONFIG10 (rw) register accessor: CFG0_PADCONFIG10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig10`]
module"]
#[doc(alias = "CFG0_PADCONFIG10")]
pub type Cfg0Padconfig10 = crate::Reg<cfg0_padconfig10::Cfg0Padconfig10Spec>;
#[doc = "CFG0_PADCONFIG10"]
pub mod cfg0_padconfig10;
#[doc = "CFG0_PADCONFIG11 (rw) register accessor: CFG0_PADCONFIG11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig11`]
module"]
#[doc(alias = "CFG0_PADCONFIG11")]
pub type Cfg0Padconfig11 = crate::Reg<cfg0_padconfig11::Cfg0Padconfig11Spec>;
#[doc = "CFG0_PADCONFIG11"]
pub mod cfg0_padconfig11;
#[doc = "CFG0_PADCONFIG12 (rw) register accessor: CFG0_PADCONFIG12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig12`]
module"]
#[doc(alias = "CFG0_PADCONFIG12")]
pub type Cfg0Padconfig12 = crate::Reg<cfg0_padconfig12::Cfg0Padconfig12Spec>;
#[doc = "CFG0_PADCONFIG12"]
pub mod cfg0_padconfig12;
#[doc = "CFG0_PADCONFIG13 (rw) register accessor: CFG0_PADCONFIG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig13`]
module"]
#[doc(alias = "CFG0_PADCONFIG13")]
pub type Cfg0Padconfig13 = crate::Reg<cfg0_padconfig13::Cfg0Padconfig13Spec>;
#[doc = "CFG0_PADCONFIG13"]
pub mod cfg0_padconfig13;
#[doc = "CFG0_PADCONFIG14 (rw) register accessor: CFG0_PADCONFIG14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig14`]
module"]
#[doc(alias = "CFG0_PADCONFIG14")]
pub type Cfg0Padconfig14 = crate::Reg<cfg0_padconfig14::Cfg0Padconfig14Spec>;
#[doc = "CFG0_PADCONFIG14"]
pub mod cfg0_padconfig14;
#[doc = "CFG0_PADCONFIG15 (rw) register accessor: CFG0_PADCONFIG15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig15`]
module"]
#[doc(alias = "CFG0_PADCONFIG15")]
pub type Cfg0Padconfig15 = crate::Reg<cfg0_padconfig15::Cfg0Padconfig15Spec>;
#[doc = "CFG0_PADCONFIG15"]
pub mod cfg0_padconfig15;
#[doc = "CFG0_PADCONFIG16 (rw) register accessor: CFG0_PADCONFIG16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig16`]
module"]
#[doc(alias = "CFG0_PADCONFIG16")]
pub type Cfg0Padconfig16 = crate::Reg<cfg0_padconfig16::Cfg0Padconfig16Spec>;
#[doc = "CFG0_PADCONFIG16"]
pub mod cfg0_padconfig16;
#[doc = "CFG0_PADCONFIG17 (rw) register accessor: CFG0_PADCONFIG17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig17`]
module"]
#[doc(alias = "CFG0_PADCONFIG17")]
pub type Cfg0Padconfig17 = crate::Reg<cfg0_padconfig17::Cfg0Padconfig17Spec>;
#[doc = "CFG0_PADCONFIG17"]
pub mod cfg0_padconfig17;
#[doc = "CFG0_PADCONFIG18 (rw) register accessor: CFG0_PADCONFIG18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig18`]
module"]
#[doc(alias = "CFG0_PADCONFIG18")]
pub type Cfg0Padconfig18 = crate::Reg<cfg0_padconfig18::Cfg0Padconfig18Spec>;
#[doc = "CFG0_PADCONFIG18"]
pub mod cfg0_padconfig18;
#[doc = "CFG0_PADCONFIG19 (rw) register accessor: CFG0_PADCONFIG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig19`]
module"]
#[doc(alias = "CFG0_PADCONFIG19")]
pub type Cfg0Padconfig19 = crate::Reg<cfg0_padconfig19::Cfg0Padconfig19Spec>;
#[doc = "CFG0_PADCONFIG19"]
pub mod cfg0_padconfig19;
#[doc = "CFG0_PADCONFIG20 (rw) register accessor: CFG0_PADCONFIG20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig20`]
module"]
#[doc(alias = "CFG0_PADCONFIG20")]
pub type Cfg0Padconfig20 = crate::Reg<cfg0_padconfig20::Cfg0Padconfig20Spec>;
#[doc = "CFG0_PADCONFIG20"]
pub mod cfg0_padconfig20;
#[doc = "CFG0_PADCONFIG21 (rw) register accessor: CFG0_PADCONFIG21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig21`]
module"]
#[doc(alias = "CFG0_PADCONFIG21")]
pub type Cfg0Padconfig21 = crate::Reg<cfg0_padconfig21::Cfg0Padconfig21Spec>;
#[doc = "CFG0_PADCONFIG21"]
pub mod cfg0_padconfig21;
#[doc = "CFG0_PADCONFIG22 (rw) register accessor: CFG0_PADCONFIG22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig22`]
module"]
#[doc(alias = "CFG0_PADCONFIG22")]
pub type Cfg0Padconfig22 = crate::Reg<cfg0_padconfig22::Cfg0Padconfig22Spec>;
#[doc = "CFG0_PADCONFIG22"]
pub mod cfg0_padconfig22;
#[doc = "CFG0_PADCONFIG23 (rw) register accessor: CFG0_PADCONFIG23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig23`]
module"]
#[doc(alias = "CFG0_PADCONFIG23")]
pub type Cfg0Padconfig23 = crate::Reg<cfg0_padconfig23::Cfg0Padconfig23Spec>;
#[doc = "CFG0_PADCONFIG23"]
pub mod cfg0_padconfig23;
#[doc = "CFG0_PADCONFIG24 (rw) register accessor: CFG0_PADCONFIG24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig24`]
module"]
#[doc(alias = "CFG0_PADCONFIG24")]
pub type Cfg0Padconfig24 = crate::Reg<cfg0_padconfig24::Cfg0Padconfig24Spec>;
#[doc = "CFG0_PADCONFIG24"]
pub mod cfg0_padconfig24;
#[doc = "CFG0_PADCONFIG25 (rw) register accessor: CFG0_PADCONFIG25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig25`]
module"]
#[doc(alias = "CFG0_PADCONFIG25")]
pub type Cfg0Padconfig25 = crate::Reg<cfg0_padconfig25::Cfg0Padconfig25Spec>;
#[doc = "CFG0_PADCONFIG25"]
pub mod cfg0_padconfig25;
#[doc = "CFG0_PADCONFIG26 (rw) register accessor: CFG0_PADCONFIG26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig26`]
module"]
#[doc(alias = "CFG0_PADCONFIG26")]
pub type Cfg0Padconfig26 = crate::Reg<cfg0_padconfig26::Cfg0Padconfig26Spec>;
#[doc = "CFG0_PADCONFIG26"]
pub mod cfg0_padconfig26;
#[doc = "CFG0_PADCONFIG27 (rw) register accessor: CFG0_PADCONFIG27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig27`]
module"]
#[doc(alias = "CFG0_PADCONFIG27")]
pub type Cfg0Padconfig27 = crate::Reg<cfg0_padconfig27::Cfg0Padconfig27Spec>;
#[doc = "CFG0_PADCONFIG27"]
pub mod cfg0_padconfig27;
#[doc = "CFG0_PADCONFIG28 (rw) register accessor: CFG0_PADCONFIG28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig28`]
module"]
#[doc(alias = "CFG0_PADCONFIG28")]
pub type Cfg0Padconfig28 = crate::Reg<cfg0_padconfig28::Cfg0Padconfig28Spec>;
#[doc = "CFG0_PADCONFIG28"]
pub mod cfg0_padconfig28;
#[doc = "CFG0_PADCONFIG29 (rw) register accessor: CFG0_PADCONFIG29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig29`]
module"]
#[doc(alias = "CFG0_PADCONFIG29")]
pub type Cfg0Padconfig29 = crate::Reg<cfg0_padconfig29::Cfg0Padconfig29Spec>;
#[doc = "CFG0_PADCONFIG29"]
pub mod cfg0_padconfig29;
#[doc = "CFG0_PADCONFIG30 (rw) register accessor: CFG0_PADCONFIG30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig30`]
module"]
#[doc(alias = "CFG0_PADCONFIG30")]
pub type Cfg0Padconfig30 = crate::Reg<cfg0_padconfig30::Cfg0Padconfig30Spec>;
#[doc = "CFG0_PADCONFIG30"]
pub mod cfg0_padconfig30;
#[doc = "CFG0_PADCONFIG31 (rw) register accessor: CFG0_PADCONFIG31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig31`]
module"]
#[doc(alias = "CFG0_PADCONFIG31")]
pub type Cfg0Padconfig31 = crate::Reg<cfg0_padconfig31::Cfg0Padconfig31Spec>;
#[doc = "CFG0_PADCONFIG31"]
pub mod cfg0_padconfig31;
#[doc = "CFG0_PADCONFIG32 (rw) register accessor: CFG0_PADCONFIG32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig32`]
module"]
#[doc(alias = "CFG0_PADCONFIG32")]
pub type Cfg0Padconfig32 = crate::Reg<cfg0_padconfig32::Cfg0Padconfig32Spec>;
#[doc = "CFG0_PADCONFIG32"]
pub mod cfg0_padconfig32;
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
#[doc = "CFG0_PADCONFIG0_PROXY (rw) register accessor: CFG0_PADCONFIG0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig0_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig0_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig0_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG0_PROXY")]
pub type Cfg0Padconfig0Proxy = crate::Reg<cfg0_padconfig0_proxy::Cfg0Padconfig0ProxySpec>;
#[doc = "CFG0_PADCONFIG0_PROXY"]
pub mod cfg0_padconfig0_proxy;
#[doc = "CFG0_PADCONFIG1_PROXY (rw) register accessor: CFG0_PADCONFIG1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig1_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig1_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig1_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG1_PROXY")]
pub type Cfg0Padconfig1Proxy = crate::Reg<cfg0_padconfig1_proxy::Cfg0Padconfig1ProxySpec>;
#[doc = "CFG0_PADCONFIG1_PROXY"]
pub mod cfg0_padconfig1_proxy;
#[doc = "CFG0_PADCONFIG2_PROXY (rw) register accessor: CFG0_PADCONFIG2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig2_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig2_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig2_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG2_PROXY")]
pub type Cfg0Padconfig2Proxy = crate::Reg<cfg0_padconfig2_proxy::Cfg0Padconfig2ProxySpec>;
#[doc = "CFG0_PADCONFIG2_PROXY"]
pub mod cfg0_padconfig2_proxy;
#[doc = "CFG0_PADCONFIG3_PROXY (rw) register accessor: CFG0_PADCONFIG3_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig3_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig3_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig3_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG3_PROXY")]
pub type Cfg0Padconfig3Proxy = crate::Reg<cfg0_padconfig3_proxy::Cfg0Padconfig3ProxySpec>;
#[doc = "CFG0_PADCONFIG3_PROXY"]
pub mod cfg0_padconfig3_proxy;
#[doc = "CFG0_PADCONFIG4_PROXY (rw) register accessor: CFG0_PADCONFIG4_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig4_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig4_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig4_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG4_PROXY")]
pub type Cfg0Padconfig4Proxy = crate::Reg<cfg0_padconfig4_proxy::Cfg0Padconfig4ProxySpec>;
#[doc = "CFG0_PADCONFIG4_PROXY"]
pub mod cfg0_padconfig4_proxy;
#[doc = "CFG0_PADCONFIG5_PROXY (rw) register accessor: CFG0_PADCONFIG5_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig5_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig5_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig5_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG5_PROXY")]
pub type Cfg0Padconfig5Proxy = crate::Reg<cfg0_padconfig5_proxy::Cfg0Padconfig5ProxySpec>;
#[doc = "CFG0_PADCONFIG5_PROXY"]
pub mod cfg0_padconfig5_proxy;
#[doc = "CFG0_PADCONFIG6_PROXY (rw) register accessor: CFG0_PADCONFIG6_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig6_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig6_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig6_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG6_PROXY")]
pub type Cfg0Padconfig6Proxy = crate::Reg<cfg0_padconfig6_proxy::Cfg0Padconfig6ProxySpec>;
#[doc = "CFG0_PADCONFIG6_PROXY"]
pub mod cfg0_padconfig6_proxy;
#[doc = "CFG0_PADCONFIG7_PROXY (rw) register accessor: CFG0_PADCONFIG7_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig7_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig7_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig7_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG7_PROXY")]
pub type Cfg0Padconfig7Proxy = crate::Reg<cfg0_padconfig7_proxy::Cfg0Padconfig7ProxySpec>;
#[doc = "CFG0_PADCONFIG7_PROXY"]
pub mod cfg0_padconfig7_proxy;
#[doc = "CFG0_PADCONFIG8_PROXY (rw) register accessor: CFG0_PADCONFIG8_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig8_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig8_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig8_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG8_PROXY")]
pub type Cfg0Padconfig8Proxy = crate::Reg<cfg0_padconfig8_proxy::Cfg0Padconfig8ProxySpec>;
#[doc = "CFG0_PADCONFIG8_PROXY"]
pub mod cfg0_padconfig8_proxy;
#[doc = "CFG0_PADCONFIG9_PROXY (rw) register accessor: CFG0_PADCONFIG9_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig9_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig9_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig9_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG9_PROXY")]
pub type Cfg0Padconfig9Proxy = crate::Reg<cfg0_padconfig9_proxy::Cfg0Padconfig9ProxySpec>;
#[doc = "CFG0_PADCONFIG9_PROXY"]
pub mod cfg0_padconfig9_proxy;
#[doc = "CFG0_PADCONFIG10_PROXY (rw) register accessor: CFG0_PADCONFIG10_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig10_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig10_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig10_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG10_PROXY")]
pub type Cfg0Padconfig10Proxy = crate::Reg<cfg0_padconfig10_proxy::Cfg0Padconfig10ProxySpec>;
#[doc = "CFG0_PADCONFIG10_PROXY"]
pub mod cfg0_padconfig10_proxy;
#[doc = "CFG0_PADCONFIG11_PROXY (rw) register accessor: CFG0_PADCONFIG11_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig11_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig11_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig11_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG11_PROXY")]
pub type Cfg0Padconfig11Proxy = crate::Reg<cfg0_padconfig11_proxy::Cfg0Padconfig11ProxySpec>;
#[doc = "CFG0_PADCONFIG11_PROXY"]
pub mod cfg0_padconfig11_proxy;
#[doc = "CFG0_PADCONFIG12_PROXY (rw) register accessor: CFG0_PADCONFIG12_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig12_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig12_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig12_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG12_PROXY")]
pub type Cfg0Padconfig12Proxy = crate::Reg<cfg0_padconfig12_proxy::Cfg0Padconfig12ProxySpec>;
#[doc = "CFG0_PADCONFIG12_PROXY"]
pub mod cfg0_padconfig12_proxy;
#[doc = "CFG0_PADCONFIG13_PROXY (rw) register accessor: CFG0_PADCONFIG13_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig13_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig13_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig13_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG13_PROXY")]
pub type Cfg0Padconfig13Proxy = crate::Reg<cfg0_padconfig13_proxy::Cfg0Padconfig13ProxySpec>;
#[doc = "CFG0_PADCONFIG13_PROXY"]
pub mod cfg0_padconfig13_proxy;
#[doc = "CFG0_PADCONFIG14_PROXY (rw) register accessor: CFG0_PADCONFIG14_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig14_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig14_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig14_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG14_PROXY")]
pub type Cfg0Padconfig14Proxy = crate::Reg<cfg0_padconfig14_proxy::Cfg0Padconfig14ProxySpec>;
#[doc = "CFG0_PADCONFIG14_PROXY"]
pub mod cfg0_padconfig14_proxy;
#[doc = "CFG0_PADCONFIG15_PROXY (rw) register accessor: CFG0_PADCONFIG15_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig15_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig15_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig15_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG15_PROXY")]
pub type Cfg0Padconfig15Proxy = crate::Reg<cfg0_padconfig15_proxy::Cfg0Padconfig15ProxySpec>;
#[doc = "CFG0_PADCONFIG15_PROXY"]
pub mod cfg0_padconfig15_proxy;
#[doc = "CFG0_PADCONFIG16_PROXY (rw) register accessor: CFG0_PADCONFIG16_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig16_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig16_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig16_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG16_PROXY")]
pub type Cfg0Padconfig16Proxy = crate::Reg<cfg0_padconfig16_proxy::Cfg0Padconfig16ProxySpec>;
#[doc = "CFG0_PADCONFIG16_PROXY"]
pub mod cfg0_padconfig16_proxy;
#[doc = "CFG0_PADCONFIG17_PROXY (rw) register accessor: CFG0_PADCONFIG17_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig17_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig17_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig17_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG17_PROXY")]
pub type Cfg0Padconfig17Proxy = crate::Reg<cfg0_padconfig17_proxy::Cfg0Padconfig17ProxySpec>;
#[doc = "CFG0_PADCONFIG17_PROXY"]
pub mod cfg0_padconfig17_proxy;
#[doc = "CFG0_PADCONFIG18_PROXY (rw) register accessor: CFG0_PADCONFIG18_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig18_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig18_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig18_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG18_PROXY")]
pub type Cfg0Padconfig18Proxy = crate::Reg<cfg0_padconfig18_proxy::Cfg0Padconfig18ProxySpec>;
#[doc = "CFG0_PADCONFIG18_PROXY"]
pub mod cfg0_padconfig18_proxy;
#[doc = "CFG0_PADCONFIG19_PROXY (rw) register accessor: CFG0_PADCONFIG19_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig19_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig19_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig19_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG19_PROXY")]
pub type Cfg0Padconfig19Proxy = crate::Reg<cfg0_padconfig19_proxy::Cfg0Padconfig19ProxySpec>;
#[doc = "CFG0_PADCONFIG19_PROXY"]
pub mod cfg0_padconfig19_proxy;
#[doc = "CFG0_PADCONFIG20_PROXY (rw) register accessor: CFG0_PADCONFIG20_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig20_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig20_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig20_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG20_PROXY")]
pub type Cfg0Padconfig20Proxy = crate::Reg<cfg0_padconfig20_proxy::Cfg0Padconfig20ProxySpec>;
#[doc = "CFG0_PADCONFIG20_PROXY"]
pub mod cfg0_padconfig20_proxy;
#[doc = "CFG0_PADCONFIG21_PROXY (rw) register accessor: CFG0_PADCONFIG21_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig21_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig21_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig21_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG21_PROXY")]
pub type Cfg0Padconfig21Proxy = crate::Reg<cfg0_padconfig21_proxy::Cfg0Padconfig21ProxySpec>;
#[doc = "CFG0_PADCONFIG21_PROXY"]
pub mod cfg0_padconfig21_proxy;
#[doc = "CFG0_PADCONFIG22_PROXY (rw) register accessor: CFG0_PADCONFIG22_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig22_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig22_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig22_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG22_PROXY")]
pub type Cfg0Padconfig22Proxy = crate::Reg<cfg0_padconfig22_proxy::Cfg0Padconfig22ProxySpec>;
#[doc = "CFG0_PADCONFIG22_PROXY"]
pub mod cfg0_padconfig22_proxy;
#[doc = "CFG0_PADCONFIG23_PROXY (rw) register accessor: CFG0_PADCONFIG23_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig23_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig23_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig23_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG23_PROXY")]
pub type Cfg0Padconfig23Proxy = crate::Reg<cfg0_padconfig23_proxy::Cfg0Padconfig23ProxySpec>;
#[doc = "CFG0_PADCONFIG23_PROXY"]
pub mod cfg0_padconfig23_proxy;
#[doc = "CFG0_PADCONFIG24_PROXY (rw) register accessor: CFG0_PADCONFIG24_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig24_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig24_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig24_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG24_PROXY")]
pub type Cfg0Padconfig24Proxy = crate::Reg<cfg0_padconfig24_proxy::Cfg0Padconfig24ProxySpec>;
#[doc = "CFG0_PADCONFIG24_PROXY"]
pub mod cfg0_padconfig24_proxy;
#[doc = "CFG0_PADCONFIG25_PROXY (rw) register accessor: CFG0_PADCONFIG25_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig25_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig25_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig25_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG25_PROXY")]
pub type Cfg0Padconfig25Proxy = crate::Reg<cfg0_padconfig25_proxy::Cfg0Padconfig25ProxySpec>;
#[doc = "CFG0_PADCONFIG25_PROXY"]
pub mod cfg0_padconfig25_proxy;
#[doc = "CFG0_PADCONFIG26_PROXY (rw) register accessor: CFG0_PADCONFIG26_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig26_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig26_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig26_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG26_PROXY")]
pub type Cfg0Padconfig26Proxy = crate::Reg<cfg0_padconfig26_proxy::Cfg0Padconfig26ProxySpec>;
#[doc = "CFG0_PADCONFIG26_PROXY"]
pub mod cfg0_padconfig26_proxy;
#[doc = "CFG0_PADCONFIG27_PROXY (rw) register accessor: CFG0_PADCONFIG27_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig27_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig27_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig27_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG27_PROXY")]
pub type Cfg0Padconfig27Proxy = crate::Reg<cfg0_padconfig27_proxy::Cfg0Padconfig27ProxySpec>;
#[doc = "CFG0_PADCONFIG27_PROXY"]
pub mod cfg0_padconfig27_proxy;
#[doc = "CFG0_PADCONFIG28_PROXY (rw) register accessor: CFG0_PADCONFIG28_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig28_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig28_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig28_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG28_PROXY")]
pub type Cfg0Padconfig28Proxy = crate::Reg<cfg0_padconfig28_proxy::Cfg0Padconfig28ProxySpec>;
#[doc = "CFG0_PADCONFIG28_PROXY"]
pub mod cfg0_padconfig28_proxy;
#[doc = "CFG0_PADCONFIG29_PROXY (rw) register accessor: CFG0_PADCONFIG29_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig29_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig29_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig29_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG29_PROXY")]
pub type Cfg0Padconfig29Proxy = crate::Reg<cfg0_padconfig29_proxy::Cfg0Padconfig29ProxySpec>;
#[doc = "CFG0_PADCONFIG29_PROXY"]
pub mod cfg0_padconfig29_proxy;
#[doc = "CFG0_PADCONFIG30_PROXY (rw) register accessor: CFG0_PADCONFIG30_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig30_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig30_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig30_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG30_PROXY")]
pub type Cfg0Padconfig30Proxy = crate::Reg<cfg0_padconfig30_proxy::Cfg0Padconfig30ProxySpec>;
#[doc = "CFG0_PADCONFIG30_PROXY"]
pub mod cfg0_padconfig30_proxy;
#[doc = "CFG0_PADCONFIG31_PROXY (rw) register accessor: CFG0_PADCONFIG31_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig31_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig31_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig31_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG31_PROXY")]
pub type Cfg0Padconfig31Proxy = crate::Reg<cfg0_padconfig31_proxy::Cfg0Padconfig31ProxySpec>;
#[doc = "CFG0_PADCONFIG31_PROXY"]
pub mod cfg0_padconfig31_proxy;
#[doc = "CFG0_PADCONFIG32_PROXY (rw) register accessor: CFG0_PADCONFIG32_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig32_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig32_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig32_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG32_PROXY")]
pub type Cfg0Padconfig32Proxy = crate::Reg<cfg0_padconfig32_proxy::Cfg0Padconfig32ProxySpec>;
#[doc = "CFG0_PADCONFIG32_PROXY"]
pub mod cfg0_padconfig32_proxy;
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
