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
    cfg0_padconfig33: Cfg0Padconfig33,
    cfg0_padconfig34: Cfg0Padconfig34,
    cfg0_padconfig35: Cfg0Padconfig35,
    cfg0_padconfig36: Cfg0Padconfig36,
    cfg0_padconfig37: Cfg0Padconfig37,
    cfg0_padconfig38: Cfg0Padconfig38,
    cfg0_padconfig39: Cfg0Padconfig39,
    cfg0_padconfig40: Cfg0Padconfig40,
    cfg0_padconfig41: Cfg0Padconfig41,
    cfg0_padconfig42: Cfg0Padconfig42,
    cfg0_padconfig43: Cfg0Padconfig43,
    cfg0_padconfig44: Cfg0Padconfig44,
    cfg0_padconfig45: Cfg0Padconfig45,
    cfg0_padconfig46: Cfg0Padconfig46,
    cfg0_padconfig47: Cfg0Padconfig47,
    cfg0_padconfig48: Cfg0Padconfig48,
    cfg0_padconfig49: Cfg0Padconfig49,
    cfg0_padconfig50: Cfg0Padconfig50,
    cfg0_padconfig51: Cfg0Padconfig51,
    cfg0_padconfig52: Cfg0Padconfig52,
    cfg0_padconfig53: Cfg0Padconfig53,
    cfg0_padconfig54: Cfg0Padconfig54,
    cfg0_padconfig55: Cfg0Padconfig55,
    cfg0_padconfig56: Cfg0Padconfig56,
    cfg0_padconfig57: Cfg0Padconfig57,
    cfg0_padconfig58: Cfg0Padconfig58,
    cfg0_padconfig59: Cfg0Padconfig59,
    cfg0_padconfig60: Cfg0Padconfig60,
    cfg0_padconfig61: Cfg0Padconfig61,
    cfg0_padconfig62: Cfg0Padconfig62,
    cfg0_padconfig63: Cfg0Padconfig63,
    cfg0_padconfig64: Cfg0Padconfig64,
    cfg0_padconfig65: Cfg0Padconfig65,
    cfg0_padconfig66: Cfg0Padconfig66,
    cfg0_padconfig67: Cfg0Padconfig67,
    cfg0_padconfig68: Cfg0Padconfig68,
    cfg0_padconfig69: Cfg0Padconfig69,
    cfg0_padconfig70: Cfg0Padconfig70,
    cfg0_padconfig71: Cfg0Padconfig71,
    cfg0_padconfig72: Cfg0Padconfig72,
    cfg0_padconfig73: Cfg0Padconfig73,
    cfg0_padconfig74: Cfg0Padconfig74,
    cfg0_padconfig75: Cfg0Padconfig75,
    cfg0_padconfig76: Cfg0Padconfig76,
    cfg0_padconfig77: Cfg0Padconfig77,
    cfg0_padconfig78: Cfg0Padconfig78,
    cfg0_padconfig79: Cfg0Padconfig79,
    cfg0_padconfig80: Cfg0Padconfig80,
    cfg0_padconfig81: Cfg0Padconfig81,
    cfg0_padconfig82: Cfg0Padconfig82,
    cfg0_padconfig83: Cfg0Padconfig83,
    cfg0_padconfig84: Cfg0Padconfig84,
    cfg0_padconfig85: Cfg0Padconfig85,
    cfg0_padconfig86: Cfg0Padconfig86,
    cfg0_padconfig87: Cfg0Padconfig87,
    cfg0_padconfig88: Cfg0Padconfig88,
    cfg0_padconfig89: Cfg0Padconfig89,
    cfg0_padconfig90: Cfg0Padconfig90,
    cfg0_padconfig91: Cfg0Padconfig91,
    cfg0_padconfig92: Cfg0Padconfig92,
    cfg0_padconfig93: Cfg0Padconfig93,
    cfg0_padconfig94: Cfg0Padconfig94,
    cfg0_padconfig95: Cfg0Padconfig95,
    cfg0_padconfig96: Cfg0Padconfig96,
    cfg0_padconfig97: Cfg0Padconfig97,
    cfg0_padconfig98: Cfg0Padconfig98,
    cfg0_padconfig99: Cfg0Padconfig99,
    cfg0_padconfig100: Cfg0Padconfig100,
    cfg0_padconfig101: Cfg0Padconfig101,
    cfg0_padconfig102: Cfg0Padconfig102,
    cfg0_padconfig103: Cfg0Padconfig103,
    cfg0_padconfig104: Cfg0Padconfig104,
    cfg0_padconfig105: Cfg0Padconfig105,
    cfg0_padconfig106: Cfg0Padconfig106,
    cfg0_padconfig107: Cfg0Padconfig107,
    cfg0_padconfig108: Cfg0Padconfig108,
    cfg0_padconfig109: Cfg0Padconfig109,
    cfg0_padconfig110: Cfg0Padconfig110,
    cfg0_padconfig111: Cfg0Padconfig111,
    cfg0_padconfig112: Cfg0Padconfig112,
    cfg0_padconfig113: Cfg0Padconfig113,
    cfg0_padconfig114: Cfg0Padconfig114,
    cfg0_padconfig115: Cfg0Padconfig115,
    cfg0_padconfig116: Cfg0Padconfig116,
    cfg0_padconfig117: Cfg0Padconfig117,
    cfg0_padconfig118: Cfg0Padconfig118,
    cfg0_padconfig119: Cfg0Padconfig119,
    cfg0_padconfig120: Cfg0Padconfig120,
    cfg0_padconfig121: Cfg0Padconfig121,
    cfg0_padconfig122: Cfg0Padconfig122,
    cfg0_padconfig123: Cfg0Padconfig123,
    cfg0_padconfig124: Cfg0Padconfig124,
    cfg0_padconfig125: Cfg0Padconfig125,
    cfg0_padconfig126: Cfg0Padconfig126,
    cfg0_padconfig127: Cfg0Padconfig127,
    cfg0_padconfig128: Cfg0Padconfig128,
    cfg0_padconfig129: Cfg0Padconfig129,
    cfg0_padconfig130: Cfg0Padconfig130,
    cfg0_padconfig131: Cfg0Padconfig131,
    cfg0_padconfig132: Cfg0Padconfig132,
    cfg0_padconfig133: Cfg0Padconfig133,
    cfg0_padconfig134: Cfg0Padconfig134,
    cfg0_padconfig135: Cfg0Padconfig135,
    cfg0_padconfig136: Cfg0Padconfig136,
    cfg0_padconfig137: Cfg0Padconfig137,
    cfg0_padconfig138: Cfg0Padconfig138,
    cfg0_padconfig139: Cfg0Padconfig139,
    cfg0_padconfig140: Cfg0Padconfig140,
    cfg0_padconfig141: Cfg0Padconfig141,
    cfg0_padconfig142: Cfg0Padconfig142,
    cfg0_padconfig143: Cfg0Padconfig143,
    cfg0_padconfig144: Cfg0Padconfig144,
    cfg0_padconfig145: Cfg0Padconfig145,
    cfg0_padconfig146: Cfg0Padconfig146,
    cfg0_padconfig147: Cfg0Padconfig147,
    cfg0_padconfig148: Cfg0Padconfig148,
    cfg0_padconfig149: Cfg0Padconfig149,
    cfg0_padconfig150: Cfg0Padconfig150,
    cfg0_padconfig151: Cfg0Padconfig151,
    cfg0_padconfig152: Cfg0Padconfig152,
    cfg0_padconfig153: Cfg0Padconfig153,
    cfg0_padconfig154: Cfg0Padconfig154,
    cfg0_padconfig155: Cfg0Padconfig155,
    cfg0_padconfig156: Cfg0Padconfig156,
    cfg0_padconfig157: Cfg0Padconfig157,
    cfg0_padconfig158: Cfg0Padconfig158,
    cfg0_padconfig159: Cfg0Padconfig159,
    cfg0_padconfig160: Cfg0Padconfig160,
    cfg0_padconfig161: Cfg0Padconfig161,
    cfg0_padconfig162: Cfg0Padconfig162,
    cfg0_padconfig163: Cfg0Padconfig163,
    cfg0_padconfig164: Cfg0Padconfig164,
    cfg0_padconfig165: Cfg0Padconfig165,
    cfg0_padconfig166: Cfg0Padconfig166,
    cfg0_padconfig167: Cfg0Padconfig167,
    cfg0_padconfig168: Cfg0Padconfig168,
    cfg0_padconfig169: Cfg0Padconfig169,
    cfg0_padconfig170: Cfg0Padconfig170,
    cfg0_padconfig171: Cfg0Padconfig171,
    cfg0_padconfig172: Cfg0Padconfig172,
    cfg0_padconfig173: Cfg0Padconfig173,
    cfg0_padconfig174: Cfg0Padconfig174,
    cfg0_padconfig175: Cfg0Padconfig175,
    cfg0_padconfig176: Cfg0Padconfig176,
    cfg0_padconfig177: Cfg0Padconfig177,
    cfg0_padconfig178: Cfg0Padconfig178,
    cfg0_padconfig179: Cfg0Padconfig179,
    _reserved206: [u8; 0x0d38],
    cfg0_lock1_kick0: Cfg0Lock1Kick0,
    cfg0_lock1_kick1: Cfg0Lock1Kick1,
    _reserved208: [u8; 0x0ff0],
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
    cfg0_padconfig33_proxy: Cfg0Padconfig33Proxy,
    cfg0_padconfig34_proxy: Cfg0Padconfig34Proxy,
    cfg0_padconfig35_proxy: Cfg0Padconfig35Proxy,
    cfg0_padconfig36_proxy: Cfg0Padconfig36Proxy,
    cfg0_padconfig37_proxy: Cfg0Padconfig37Proxy,
    cfg0_padconfig38_proxy: Cfg0Padconfig38Proxy,
    cfg0_padconfig39_proxy: Cfg0Padconfig39Proxy,
    cfg0_padconfig40_proxy: Cfg0Padconfig40Proxy,
    cfg0_padconfig41_proxy: Cfg0Padconfig41Proxy,
    cfg0_padconfig42_proxy: Cfg0Padconfig42Proxy,
    cfg0_padconfig43_proxy: Cfg0Padconfig43Proxy,
    cfg0_padconfig44_proxy: Cfg0Padconfig44Proxy,
    cfg0_padconfig45_proxy: Cfg0Padconfig45Proxy,
    cfg0_padconfig46_proxy: Cfg0Padconfig46Proxy,
    cfg0_padconfig47_proxy: Cfg0Padconfig47Proxy,
    cfg0_padconfig48_proxy: Cfg0Padconfig48Proxy,
    cfg0_padconfig49_proxy: Cfg0Padconfig49Proxy,
    cfg0_padconfig50_proxy: Cfg0Padconfig50Proxy,
    cfg0_padconfig51_proxy: Cfg0Padconfig51Proxy,
    cfg0_padconfig52_proxy: Cfg0Padconfig52Proxy,
    cfg0_padconfig53_proxy: Cfg0Padconfig53Proxy,
    cfg0_padconfig54_proxy: Cfg0Padconfig54Proxy,
    cfg0_padconfig55_proxy: Cfg0Padconfig55Proxy,
    cfg0_padconfig56_proxy: Cfg0Padconfig56Proxy,
    cfg0_padconfig57_proxy: Cfg0Padconfig57Proxy,
    cfg0_padconfig58_proxy: Cfg0Padconfig58Proxy,
    cfg0_padconfig59_proxy: Cfg0Padconfig59Proxy,
    cfg0_padconfig60_proxy: Cfg0Padconfig60Proxy,
    cfg0_padconfig61_proxy: Cfg0Padconfig61Proxy,
    cfg0_padconfig62_proxy: Cfg0Padconfig62Proxy,
    cfg0_padconfig63_proxy: Cfg0Padconfig63Proxy,
    cfg0_padconfig64_proxy: Cfg0Padconfig64Proxy,
    cfg0_padconfig65_proxy: Cfg0Padconfig65Proxy,
    cfg0_padconfig66_proxy: Cfg0Padconfig66Proxy,
    cfg0_padconfig67_proxy: Cfg0Padconfig67Proxy,
    cfg0_padconfig68_proxy: Cfg0Padconfig68Proxy,
    cfg0_padconfig69_proxy: Cfg0Padconfig69Proxy,
    cfg0_padconfig70_proxy: Cfg0Padconfig70Proxy,
    cfg0_padconfig71_proxy: Cfg0Padconfig71Proxy,
    cfg0_padconfig72_proxy: Cfg0Padconfig72Proxy,
    cfg0_padconfig73_proxy: Cfg0Padconfig73Proxy,
    cfg0_padconfig74_proxy: Cfg0Padconfig74Proxy,
    cfg0_padconfig75_proxy: Cfg0Padconfig75Proxy,
    cfg0_padconfig76_proxy: Cfg0Padconfig76Proxy,
    cfg0_padconfig77_proxy: Cfg0Padconfig77Proxy,
    cfg0_padconfig78_proxy: Cfg0Padconfig78Proxy,
    cfg0_padconfig79_proxy: Cfg0Padconfig79Proxy,
    cfg0_padconfig80_proxy: Cfg0Padconfig80Proxy,
    cfg0_padconfig81_proxy: Cfg0Padconfig81Proxy,
    cfg0_padconfig82_proxy: Cfg0Padconfig82Proxy,
    cfg0_padconfig83_proxy: Cfg0Padconfig83Proxy,
    cfg0_padconfig84_proxy: Cfg0Padconfig84Proxy,
    cfg0_padconfig85_proxy: Cfg0Padconfig85Proxy,
    cfg0_padconfig86_proxy: Cfg0Padconfig86Proxy,
    cfg0_padconfig87_proxy: Cfg0Padconfig87Proxy,
    cfg0_padconfig88_proxy: Cfg0Padconfig88Proxy,
    cfg0_padconfig89_proxy: Cfg0Padconfig89Proxy,
    cfg0_padconfig90_proxy: Cfg0Padconfig90Proxy,
    cfg0_padconfig91_proxy: Cfg0Padconfig91Proxy,
    cfg0_padconfig92_proxy: Cfg0Padconfig92Proxy,
    cfg0_padconfig93_proxy: Cfg0Padconfig93Proxy,
    cfg0_padconfig94_proxy: Cfg0Padconfig94Proxy,
    cfg0_padconfig95_proxy: Cfg0Padconfig95Proxy,
    cfg0_padconfig96_proxy: Cfg0Padconfig96Proxy,
    cfg0_padconfig97_proxy: Cfg0Padconfig97Proxy,
    cfg0_padconfig98_proxy: Cfg0Padconfig98Proxy,
    cfg0_padconfig99_proxy: Cfg0Padconfig99Proxy,
    cfg0_padconfig100_proxy: Cfg0Padconfig100Proxy,
    cfg0_padconfig101_proxy: Cfg0Padconfig101Proxy,
    cfg0_padconfig102_proxy: Cfg0Padconfig102Proxy,
    cfg0_padconfig103_proxy: Cfg0Padconfig103Proxy,
    cfg0_padconfig104_proxy: Cfg0Padconfig104Proxy,
    cfg0_padconfig105_proxy: Cfg0Padconfig105Proxy,
    cfg0_padconfig106_proxy: Cfg0Padconfig106Proxy,
    cfg0_padconfig107_proxy: Cfg0Padconfig107Proxy,
    cfg0_padconfig108_proxy: Cfg0Padconfig108Proxy,
    cfg0_padconfig109_proxy: Cfg0Padconfig109Proxy,
    cfg0_padconfig110_proxy: Cfg0Padconfig110Proxy,
    cfg0_padconfig111_proxy: Cfg0Padconfig111Proxy,
    cfg0_padconfig112_proxy: Cfg0Padconfig112Proxy,
    cfg0_padconfig113_proxy: Cfg0Padconfig113Proxy,
    cfg0_padconfig114_proxy: Cfg0Padconfig114Proxy,
    cfg0_padconfig115_proxy: Cfg0Padconfig115Proxy,
    cfg0_padconfig116_proxy: Cfg0Padconfig116Proxy,
    cfg0_padconfig117_proxy: Cfg0Padconfig117Proxy,
    cfg0_padconfig118_proxy: Cfg0Padconfig118Proxy,
    cfg0_padconfig119_proxy: Cfg0Padconfig119Proxy,
    cfg0_padconfig120_proxy: Cfg0Padconfig120Proxy,
    cfg0_padconfig121_proxy: Cfg0Padconfig121Proxy,
    cfg0_padconfig122_proxy: Cfg0Padconfig122Proxy,
    cfg0_padconfig123_proxy: Cfg0Padconfig123Proxy,
    cfg0_padconfig124_proxy: Cfg0Padconfig124Proxy,
    cfg0_padconfig125_proxy: Cfg0Padconfig125Proxy,
    cfg0_padconfig126_proxy: Cfg0Padconfig126Proxy,
    cfg0_padconfig127_proxy: Cfg0Padconfig127Proxy,
    cfg0_padconfig128_proxy: Cfg0Padconfig128Proxy,
    cfg0_padconfig129_proxy: Cfg0Padconfig129Proxy,
    cfg0_padconfig130_proxy: Cfg0Padconfig130Proxy,
    cfg0_padconfig131_proxy: Cfg0Padconfig131Proxy,
    cfg0_padconfig132_proxy: Cfg0Padconfig132Proxy,
    cfg0_padconfig133_proxy: Cfg0Padconfig133Proxy,
    cfg0_padconfig134_proxy: Cfg0Padconfig134Proxy,
    cfg0_padconfig135_proxy: Cfg0Padconfig135Proxy,
    cfg0_padconfig136_proxy: Cfg0Padconfig136Proxy,
    cfg0_padconfig137_proxy: Cfg0Padconfig137Proxy,
    cfg0_padconfig138_proxy: Cfg0Padconfig138Proxy,
    cfg0_padconfig139_proxy: Cfg0Padconfig139Proxy,
    cfg0_padconfig140_proxy: Cfg0Padconfig140Proxy,
    cfg0_padconfig141_proxy: Cfg0Padconfig141Proxy,
    cfg0_padconfig142_proxy: Cfg0Padconfig142Proxy,
    cfg0_padconfig143_proxy: Cfg0Padconfig143Proxy,
    cfg0_padconfig144_proxy: Cfg0Padconfig144Proxy,
    cfg0_padconfig145_proxy: Cfg0Padconfig145Proxy,
    cfg0_padconfig146_proxy: Cfg0Padconfig146Proxy,
    cfg0_padconfig147_proxy: Cfg0Padconfig147Proxy,
    cfg0_padconfig148_proxy: Cfg0Padconfig148Proxy,
    cfg0_padconfig149_proxy: Cfg0Padconfig149Proxy,
    cfg0_padconfig150_proxy: Cfg0Padconfig150Proxy,
    cfg0_padconfig151_proxy: Cfg0Padconfig151Proxy,
    cfg0_padconfig152_proxy: Cfg0Padconfig152Proxy,
    cfg0_padconfig153_proxy: Cfg0Padconfig153Proxy,
    cfg0_padconfig154_proxy: Cfg0Padconfig154Proxy,
    cfg0_padconfig155_proxy: Cfg0Padconfig155Proxy,
    cfg0_padconfig156_proxy: Cfg0Padconfig156Proxy,
    cfg0_padconfig157_proxy: Cfg0Padconfig157Proxy,
    cfg0_padconfig158_proxy: Cfg0Padconfig158Proxy,
    cfg0_padconfig159_proxy: Cfg0Padconfig159Proxy,
    cfg0_padconfig160_proxy: Cfg0Padconfig160Proxy,
    cfg0_padconfig161_proxy: Cfg0Padconfig161Proxy,
    cfg0_padconfig162_proxy: Cfg0Padconfig162Proxy,
    cfg0_padconfig163_proxy: Cfg0Padconfig163Proxy,
    cfg0_padconfig164_proxy: Cfg0Padconfig164Proxy,
    cfg0_padconfig165_proxy: Cfg0Padconfig165Proxy,
    cfg0_padconfig166_proxy: Cfg0Padconfig166Proxy,
    cfg0_padconfig167_proxy: Cfg0Padconfig167Proxy,
    cfg0_padconfig168_proxy: Cfg0Padconfig168Proxy,
    cfg0_padconfig169_proxy: Cfg0Padconfig169Proxy,
    cfg0_padconfig170_proxy: Cfg0Padconfig170Proxy,
    cfg0_padconfig171_proxy: Cfg0Padconfig171Proxy,
    cfg0_padconfig172_proxy: Cfg0Padconfig172Proxy,
    cfg0_padconfig173_proxy: Cfg0Padconfig173Proxy,
    cfg0_padconfig174_proxy: Cfg0Padconfig174Proxy,
    cfg0_padconfig175_proxy: Cfg0Padconfig175Proxy,
    cfg0_padconfig176_proxy: Cfg0Padconfig176Proxy,
    cfg0_padconfig177_proxy: Cfg0Padconfig177Proxy,
    cfg0_padconfig178_proxy: Cfg0Padconfig178Proxy,
    cfg0_padconfig179_proxy: Cfg0Padconfig179Proxy,
    _reserved388: [u8; 0x0d38],
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
    #[doc = "0x4084 - CFG0_PADCONFIG33"]
    #[inline(always)]
    pub const fn cfg0_padconfig33(&self) -> &Cfg0Padconfig33 {
        &self.cfg0_padconfig33
    }
    #[doc = "0x4088 - CFG0_PADCONFIG34"]
    #[inline(always)]
    pub const fn cfg0_padconfig34(&self) -> &Cfg0Padconfig34 {
        &self.cfg0_padconfig34
    }
    #[doc = "0x408c - CFG0_PADCONFIG35"]
    #[inline(always)]
    pub const fn cfg0_padconfig35(&self) -> &Cfg0Padconfig35 {
        &self.cfg0_padconfig35
    }
    #[doc = "0x4090 - CFG0_PADCONFIG36"]
    #[inline(always)]
    pub const fn cfg0_padconfig36(&self) -> &Cfg0Padconfig36 {
        &self.cfg0_padconfig36
    }
    #[doc = "0x4094 - CFG0_PADCONFIG37"]
    #[inline(always)]
    pub const fn cfg0_padconfig37(&self) -> &Cfg0Padconfig37 {
        &self.cfg0_padconfig37
    }
    #[doc = "0x4098 - CFG0_PADCONFIG38"]
    #[inline(always)]
    pub const fn cfg0_padconfig38(&self) -> &Cfg0Padconfig38 {
        &self.cfg0_padconfig38
    }
    #[doc = "0x409c - CFG0_PADCONFIG39"]
    #[inline(always)]
    pub const fn cfg0_padconfig39(&self) -> &Cfg0Padconfig39 {
        &self.cfg0_padconfig39
    }
    #[doc = "0x40a0 - CFG0_PADCONFIG40"]
    #[inline(always)]
    pub const fn cfg0_padconfig40(&self) -> &Cfg0Padconfig40 {
        &self.cfg0_padconfig40
    }
    #[doc = "0x40a4 - CFG0_PADCONFIG41"]
    #[inline(always)]
    pub const fn cfg0_padconfig41(&self) -> &Cfg0Padconfig41 {
        &self.cfg0_padconfig41
    }
    #[doc = "0x40a8 - CFG0_PADCONFIG42"]
    #[inline(always)]
    pub const fn cfg0_padconfig42(&self) -> &Cfg0Padconfig42 {
        &self.cfg0_padconfig42
    }
    #[doc = "0x40ac - CFG0_PADCONFIG43"]
    #[inline(always)]
    pub const fn cfg0_padconfig43(&self) -> &Cfg0Padconfig43 {
        &self.cfg0_padconfig43
    }
    #[doc = "0x40b0 - CFG0_PADCONFIG44"]
    #[inline(always)]
    pub const fn cfg0_padconfig44(&self) -> &Cfg0Padconfig44 {
        &self.cfg0_padconfig44
    }
    #[doc = "0x40b4 - CFG0_PADCONFIG45"]
    #[inline(always)]
    pub const fn cfg0_padconfig45(&self) -> &Cfg0Padconfig45 {
        &self.cfg0_padconfig45
    }
    #[doc = "0x40b8 - CFG0_PADCONFIG46"]
    #[inline(always)]
    pub const fn cfg0_padconfig46(&self) -> &Cfg0Padconfig46 {
        &self.cfg0_padconfig46
    }
    #[doc = "0x40bc - CFG0_PADCONFIG47"]
    #[inline(always)]
    pub const fn cfg0_padconfig47(&self) -> &Cfg0Padconfig47 {
        &self.cfg0_padconfig47
    }
    #[doc = "0x40c0 - CFG0_PADCONFIG48"]
    #[inline(always)]
    pub const fn cfg0_padconfig48(&self) -> &Cfg0Padconfig48 {
        &self.cfg0_padconfig48
    }
    #[doc = "0x40c4 - CFG0_PADCONFIG49"]
    #[inline(always)]
    pub const fn cfg0_padconfig49(&self) -> &Cfg0Padconfig49 {
        &self.cfg0_padconfig49
    }
    #[doc = "0x40c8 - CFG0_PADCONFIG50"]
    #[inline(always)]
    pub const fn cfg0_padconfig50(&self) -> &Cfg0Padconfig50 {
        &self.cfg0_padconfig50
    }
    #[doc = "0x40cc - CFG0_PADCONFIG51"]
    #[inline(always)]
    pub const fn cfg0_padconfig51(&self) -> &Cfg0Padconfig51 {
        &self.cfg0_padconfig51
    }
    #[doc = "0x40d0 - CFG0_PADCONFIG52"]
    #[inline(always)]
    pub const fn cfg0_padconfig52(&self) -> &Cfg0Padconfig52 {
        &self.cfg0_padconfig52
    }
    #[doc = "0x40d4 - CFG0_PADCONFIG53"]
    #[inline(always)]
    pub const fn cfg0_padconfig53(&self) -> &Cfg0Padconfig53 {
        &self.cfg0_padconfig53
    }
    #[doc = "0x40d8 - CFG0_PADCONFIG54"]
    #[inline(always)]
    pub const fn cfg0_padconfig54(&self) -> &Cfg0Padconfig54 {
        &self.cfg0_padconfig54
    }
    #[doc = "0x40dc - CFG0_PADCONFIG55"]
    #[inline(always)]
    pub const fn cfg0_padconfig55(&self) -> &Cfg0Padconfig55 {
        &self.cfg0_padconfig55
    }
    #[doc = "0x40e0 - CFG0_PADCONFIG56"]
    #[inline(always)]
    pub const fn cfg0_padconfig56(&self) -> &Cfg0Padconfig56 {
        &self.cfg0_padconfig56
    }
    #[doc = "0x40e4 - CFG0_PADCONFIG57"]
    #[inline(always)]
    pub const fn cfg0_padconfig57(&self) -> &Cfg0Padconfig57 {
        &self.cfg0_padconfig57
    }
    #[doc = "0x40e8 - CFG0_PADCONFIG58"]
    #[inline(always)]
    pub const fn cfg0_padconfig58(&self) -> &Cfg0Padconfig58 {
        &self.cfg0_padconfig58
    }
    #[doc = "0x40ec - CFG0_PADCONFIG59"]
    #[inline(always)]
    pub const fn cfg0_padconfig59(&self) -> &Cfg0Padconfig59 {
        &self.cfg0_padconfig59
    }
    #[doc = "0x40f0 - CFG0_PADCONFIG60"]
    #[inline(always)]
    pub const fn cfg0_padconfig60(&self) -> &Cfg0Padconfig60 {
        &self.cfg0_padconfig60
    }
    #[doc = "0x40f4 - CFG0_PADCONFIG61"]
    #[inline(always)]
    pub const fn cfg0_padconfig61(&self) -> &Cfg0Padconfig61 {
        &self.cfg0_padconfig61
    }
    #[doc = "0x40f8 - CFG0_PADCONFIG62"]
    #[inline(always)]
    pub const fn cfg0_padconfig62(&self) -> &Cfg0Padconfig62 {
        &self.cfg0_padconfig62
    }
    #[doc = "0x40fc - CFG0_PADCONFIG63"]
    #[inline(always)]
    pub const fn cfg0_padconfig63(&self) -> &Cfg0Padconfig63 {
        &self.cfg0_padconfig63
    }
    #[doc = "0x4100 - CFG0_PADCONFIG64"]
    #[inline(always)]
    pub const fn cfg0_padconfig64(&self) -> &Cfg0Padconfig64 {
        &self.cfg0_padconfig64
    }
    #[doc = "0x4104 - CFG0_PADCONFIG65"]
    #[inline(always)]
    pub const fn cfg0_padconfig65(&self) -> &Cfg0Padconfig65 {
        &self.cfg0_padconfig65
    }
    #[doc = "0x4108 - CFG0_PADCONFIG66"]
    #[inline(always)]
    pub const fn cfg0_padconfig66(&self) -> &Cfg0Padconfig66 {
        &self.cfg0_padconfig66
    }
    #[doc = "0x410c - CFG0_PADCONFIG67"]
    #[inline(always)]
    pub const fn cfg0_padconfig67(&self) -> &Cfg0Padconfig67 {
        &self.cfg0_padconfig67
    }
    #[doc = "0x4110 - CFG0_PADCONFIG68"]
    #[inline(always)]
    pub const fn cfg0_padconfig68(&self) -> &Cfg0Padconfig68 {
        &self.cfg0_padconfig68
    }
    #[doc = "0x4114 - CFG0_PADCONFIG69"]
    #[inline(always)]
    pub const fn cfg0_padconfig69(&self) -> &Cfg0Padconfig69 {
        &self.cfg0_padconfig69
    }
    #[doc = "0x4118 - CFG0_PADCONFIG70"]
    #[inline(always)]
    pub const fn cfg0_padconfig70(&self) -> &Cfg0Padconfig70 {
        &self.cfg0_padconfig70
    }
    #[doc = "0x411c - CFG0_PADCONFIG71"]
    #[inline(always)]
    pub const fn cfg0_padconfig71(&self) -> &Cfg0Padconfig71 {
        &self.cfg0_padconfig71
    }
    #[doc = "0x4120 - CFG0_PADCONFIG72"]
    #[inline(always)]
    pub const fn cfg0_padconfig72(&self) -> &Cfg0Padconfig72 {
        &self.cfg0_padconfig72
    }
    #[doc = "0x4124 - CFG0_PADCONFIG73"]
    #[inline(always)]
    pub const fn cfg0_padconfig73(&self) -> &Cfg0Padconfig73 {
        &self.cfg0_padconfig73
    }
    #[doc = "0x4128 - CFG0_PADCONFIG74"]
    #[inline(always)]
    pub const fn cfg0_padconfig74(&self) -> &Cfg0Padconfig74 {
        &self.cfg0_padconfig74
    }
    #[doc = "0x412c - CFG0_PADCONFIG75"]
    #[inline(always)]
    pub const fn cfg0_padconfig75(&self) -> &Cfg0Padconfig75 {
        &self.cfg0_padconfig75
    }
    #[doc = "0x4130 - CFG0_PADCONFIG76"]
    #[inline(always)]
    pub const fn cfg0_padconfig76(&self) -> &Cfg0Padconfig76 {
        &self.cfg0_padconfig76
    }
    #[doc = "0x4134 - CFG0_PADCONFIG77"]
    #[inline(always)]
    pub const fn cfg0_padconfig77(&self) -> &Cfg0Padconfig77 {
        &self.cfg0_padconfig77
    }
    #[doc = "0x4138 - CFG0_PADCONFIG78"]
    #[inline(always)]
    pub const fn cfg0_padconfig78(&self) -> &Cfg0Padconfig78 {
        &self.cfg0_padconfig78
    }
    #[doc = "0x413c - CFG0_PADCONFIG79"]
    #[inline(always)]
    pub const fn cfg0_padconfig79(&self) -> &Cfg0Padconfig79 {
        &self.cfg0_padconfig79
    }
    #[doc = "0x4140 - CFG0_PADCONFIG80"]
    #[inline(always)]
    pub const fn cfg0_padconfig80(&self) -> &Cfg0Padconfig80 {
        &self.cfg0_padconfig80
    }
    #[doc = "0x4144 - CFG0_PADCONFIG81"]
    #[inline(always)]
    pub const fn cfg0_padconfig81(&self) -> &Cfg0Padconfig81 {
        &self.cfg0_padconfig81
    }
    #[doc = "0x4148 - CFG0_PADCONFIG82"]
    #[inline(always)]
    pub const fn cfg0_padconfig82(&self) -> &Cfg0Padconfig82 {
        &self.cfg0_padconfig82
    }
    #[doc = "0x414c - CFG0_PADCONFIG83"]
    #[inline(always)]
    pub const fn cfg0_padconfig83(&self) -> &Cfg0Padconfig83 {
        &self.cfg0_padconfig83
    }
    #[doc = "0x4150 - CFG0_PADCONFIG84"]
    #[inline(always)]
    pub const fn cfg0_padconfig84(&self) -> &Cfg0Padconfig84 {
        &self.cfg0_padconfig84
    }
    #[doc = "0x4154 - CFG0_PADCONFIG85"]
    #[inline(always)]
    pub const fn cfg0_padconfig85(&self) -> &Cfg0Padconfig85 {
        &self.cfg0_padconfig85
    }
    #[doc = "0x4158 - CFG0_PADCONFIG86"]
    #[inline(always)]
    pub const fn cfg0_padconfig86(&self) -> &Cfg0Padconfig86 {
        &self.cfg0_padconfig86
    }
    #[doc = "0x415c - CFG0_PADCONFIG87"]
    #[inline(always)]
    pub const fn cfg0_padconfig87(&self) -> &Cfg0Padconfig87 {
        &self.cfg0_padconfig87
    }
    #[doc = "0x4160 - CFG0_PADCONFIG88"]
    #[inline(always)]
    pub const fn cfg0_padconfig88(&self) -> &Cfg0Padconfig88 {
        &self.cfg0_padconfig88
    }
    #[doc = "0x4164 - CFG0_PADCONFIG89"]
    #[inline(always)]
    pub const fn cfg0_padconfig89(&self) -> &Cfg0Padconfig89 {
        &self.cfg0_padconfig89
    }
    #[doc = "0x4168 - CFG0_PADCONFIG90"]
    #[inline(always)]
    pub const fn cfg0_padconfig90(&self) -> &Cfg0Padconfig90 {
        &self.cfg0_padconfig90
    }
    #[doc = "0x416c - CFG0_PADCONFIG91"]
    #[inline(always)]
    pub const fn cfg0_padconfig91(&self) -> &Cfg0Padconfig91 {
        &self.cfg0_padconfig91
    }
    #[doc = "0x4170 - CFG0_PADCONFIG92"]
    #[inline(always)]
    pub const fn cfg0_padconfig92(&self) -> &Cfg0Padconfig92 {
        &self.cfg0_padconfig92
    }
    #[doc = "0x4174 - CFG0_PADCONFIG93"]
    #[inline(always)]
    pub const fn cfg0_padconfig93(&self) -> &Cfg0Padconfig93 {
        &self.cfg0_padconfig93
    }
    #[doc = "0x4178 - CFG0_PADCONFIG94"]
    #[inline(always)]
    pub const fn cfg0_padconfig94(&self) -> &Cfg0Padconfig94 {
        &self.cfg0_padconfig94
    }
    #[doc = "0x417c - CFG0_PADCONFIG95"]
    #[inline(always)]
    pub const fn cfg0_padconfig95(&self) -> &Cfg0Padconfig95 {
        &self.cfg0_padconfig95
    }
    #[doc = "0x4180 - CFG0_PADCONFIG96"]
    #[inline(always)]
    pub const fn cfg0_padconfig96(&self) -> &Cfg0Padconfig96 {
        &self.cfg0_padconfig96
    }
    #[doc = "0x4184 - CFG0_PADCONFIG97"]
    #[inline(always)]
    pub const fn cfg0_padconfig97(&self) -> &Cfg0Padconfig97 {
        &self.cfg0_padconfig97
    }
    #[doc = "0x4188 - CFG0_PADCONFIG98"]
    #[inline(always)]
    pub const fn cfg0_padconfig98(&self) -> &Cfg0Padconfig98 {
        &self.cfg0_padconfig98
    }
    #[doc = "0x418c - CFG0_PADCONFIG99"]
    #[inline(always)]
    pub const fn cfg0_padconfig99(&self) -> &Cfg0Padconfig99 {
        &self.cfg0_padconfig99
    }
    #[doc = "0x4190 - CFG0_PADCONFIG100"]
    #[inline(always)]
    pub const fn cfg0_padconfig100(&self) -> &Cfg0Padconfig100 {
        &self.cfg0_padconfig100
    }
    #[doc = "0x4194 - CFG0_PADCONFIG101"]
    #[inline(always)]
    pub const fn cfg0_padconfig101(&self) -> &Cfg0Padconfig101 {
        &self.cfg0_padconfig101
    }
    #[doc = "0x4198 - CFG0_PADCONFIG102"]
    #[inline(always)]
    pub const fn cfg0_padconfig102(&self) -> &Cfg0Padconfig102 {
        &self.cfg0_padconfig102
    }
    #[doc = "0x419c - CFG0_PADCONFIG103"]
    #[inline(always)]
    pub const fn cfg0_padconfig103(&self) -> &Cfg0Padconfig103 {
        &self.cfg0_padconfig103
    }
    #[doc = "0x41a0 - CFG0_PADCONFIG104"]
    #[inline(always)]
    pub const fn cfg0_padconfig104(&self) -> &Cfg0Padconfig104 {
        &self.cfg0_padconfig104
    }
    #[doc = "0x41a4 - CFG0_PADCONFIG105"]
    #[inline(always)]
    pub const fn cfg0_padconfig105(&self) -> &Cfg0Padconfig105 {
        &self.cfg0_padconfig105
    }
    #[doc = "0x41a8 - CFG0_PADCONFIG106"]
    #[inline(always)]
    pub const fn cfg0_padconfig106(&self) -> &Cfg0Padconfig106 {
        &self.cfg0_padconfig106
    }
    #[doc = "0x41ac - CFG0_PADCONFIG107"]
    #[inline(always)]
    pub const fn cfg0_padconfig107(&self) -> &Cfg0Padconfig107 {
        &self.cfg0_padconfig107
    }
    #[doc = "0x41b0 - CFG0_PADCONFIG108"]
    #[inline(always)]
    pub const fn cfg0_padconfig108(&self) -> &Cfg0Padconfig108 {
        &self.cfg0_padconfig108
    }
    #[doc = "0x41b4 - CFG0_PADCONFIG109"]
    #[inline(always)]
    pub const fn cfg0_padconfig109(&self) -> &Cfg0Padconfig109 {
        &self.cfg0_padconfig109
    }
    #[doc = "0x41b8 - CFG0_PADCONFIG110"]
    #[inline(always)]
    pub const fn cfg0_padconfig110(&self) -> &Cfg0Padconfig110 {
        &self.cfg0_padconfig110
    }
    #[doc = "0x41bc - CFG0_PADCONFIG111"]
    #[inline(always)]
    pub const fn cfg0_padconfig111(&self) -> &Cfg0Padconfig111 {
        &self.cfg0_padconfig111
    }
    #[doc = "0x41c0 - CFG0_PADCONFIG112"]
    #[inline(always)]
    pub const fn cfg0_padconfig112(&self) -> &Cfg0Padconfig112 {
        &self.cfg0_padconfig112
    }
    #[doc = "0x41c4 - CFG0_PADCONFIG113"]
    #[inline(always)]
    pub const fn cfg0_padconfig113(&self) -> &Cfg0Padconfig113 {
        &self.cfg0_padconfig113
    }
    #[doc = "0x41c8 - CFG0_PADCONFIG114"]
    #[inline(always)]
    pub const fn cfg0_padconfig114(&self) -> &Cfg0Padconfig114 {
        &self.cfg0_padconfig114
    }
    #[doc = "0x41cc - CFG0_PADCONFIG115"]
    #[inline(always)]
    pub const fn cfg0_padconfig115(&self) -> &Cfg0Padconfig115 {
        &self.cfg0_padconfig115
    }
    #[doc = "0x41d0 - CFG0_PADCONFIG116"]
    #[inline(always)]
    pub const fn cfg0_padconfig116(&self) -> &Cfg0Padconfig116 {
        &self.cfg0_padconfig116
    }
    #[doc = "0x41d4 - CFG0_PADCONFIG117"]
    #[inline(always)]
    pub const fn cfg0_padconfig117(&self) -> &Cfg0Padconfig117 {
        &self.cfg0_padconfig117
    }
    #[doc = "0x41d8 - CFG0_PADCONFIG118"]
    #[inline(always)]
    pub const fn cfg0_padconfig118(&self) -> &Cfg0Padconfig118 {
        &self.cfg0_padconfig118
    }
    #[doc = "0x41dc - CFG0_PADCONFIG119"]
    #[inline(always)]
    pub const fn cfg0_padconfig119(&self) -> &Cfg0Padconfig119 {
        &self.cfg0_padconfig119
    }
    #[doc = "0x41e0 - CFG0_PADCONFIG120"]
    #[inline(always)]
    pub const fn cfg0_padconfig120(&self) -> &Cfg0Padconfig120 {
        &self.cfg0_padconfig120
    }
    #[doc = "0x41e4 - CFG0_PADCONFIG121"]
    #[inline(always)]
    pub const fn cfg0_padconfig121(&self) -> &Cfg0Padconfig121 {
        &self.cfg0_padconfig121
    }
    #[doc = "0x41e8 - CFG0_PADCONFIG122"]
    #[inline(always)]
    pub const fn cfg0_padconfig122(&self) -> &Cfg0Padconfig122 {
        &self.cfg0_padconfig122
    }
    #[doc = "0x41ec - CFG0_PADCONFIG123"]
    #[inline(always)]
    pub const fn cfg0_padconfig123(&self) -> &Cfg0Padconfig123 {
        &self.cfg0_padconfig123
    }
    #[doc = "0x41f0 - CFG0_PADCONFIG124"]
    #[inline(always)]
    pub const fn cfg0_padconfig124(&self) -> &Cfg0Padconfig124 {
        &self.cfg0_padconfig124
    }
    #[doc = "0x41f4 - CFG0_PADCONFIG125"]
    #[inline(always)]
    pub const fn cfg0_padconfig125(&self) -> &Cfg0Padconfig125 {
        &self.cfg0_padconfig125
    }
    #[doc = "0x41f8 - CFG0_PADCONFIG126"]
    #[inline(always)]
    pub const fn cfg0_padconfig126(&self) -> &Cfg0Padconfig126 {
        &self.cfg0_padconfig126
    }
    #[doc = "0x41fc - CFG0_PADCONFIG127"]
    #[inline(always)]
    pub const fn cfg0_padconfig127(&self) -> &Cfg0Padconfig127 {
        &self.cfg0_padconfig127
    }
    #[doc = "0x4200 - CFG0_PADCONFIG128"]
    #[inline(always)]
    pub const fn cfg0_padconfig128(&self) -> &Cfg0Padconfig128 {
        &self.cfg0_padconfig128
    }
    #[doc = "0x4204 - CFG0_PADCONFIG129"]
    #[inline(always)]
    pub const fn cfg0_padconfig129(&self) -> &Cfg0Padconfig129 {
        &self.cfg0_padconfig129
    }
    #[doc = "0x4208 - CFG0_PADCONFIG130"]
    #[inline(always)]
    pub const fn cfg0_padconfig130(&self) -> &Cfg0Padconfig130 {
        &self.cfg0_padconfig130
    }
    #[doc = "0x420c - CFG0_PADCONFIG131"]
    #[inline(always)]
    pub const fn cfg0_padconfig131(&self) -> &Cfg0Padconfig131 {
        &self.cfg0_padconfig131
    }
    #[doc = "0x4210 - CFG0_PADCONFIG132"]
    #[inline(always)]
    pub const fn cfg0_padconfig132(&self) -> &Cfg0Padconfig132 {
        &self.cfg0_padconfig132
    }
    #[doc = "0x4214 - CFG0_PADCONFIG133"]
    #[inline(always)]
    pub const fn cfg0_padconfig133(&self) -> &Cfg0Padconfig133 {
        &self.cfg0_padconfig133
    }
    #[doc = "0x4218 - CFG0_PADCONFIG134"]
    #[inline(always)]
    pub const fn cfg0_padconfig134(&self) -> &Cfg0Padconfig134 {
        &self.cfg0_padconfig134
    }
    #[doc = "0x421c - CFG0_PADCONFIG135"]
    #[inline(always)]
    pub const fn cfg0_padconfig135(&self) -> &Cfg0Padconfig135 {
        &self.cfg0_padconfig135
    }
    #[doc = "0x4220 - CFG0_PADCONFIG136"]
    #[inline(always)]
    pub const fn cfg0_padconfig136(&self) -> &Cfg0Padconfig136 {
        &self.cfg0_padconfig136
    }
    #[doc = "0x4224 - CFG0_PADCONFIG137"]
    #[inline(always)]
    pub const fn cfg0_padconfig137(&self) -> &Cfg0Padconfig137 {
        &self.cfg0_padconfig137
    }
    #[doc = "0x4228 - CFG0_PADCONFIG138"]
    #[inline(always)]
    pub const fn cfg0_padconfig138(&self) -> &Cfg0Padconfig138 {
        &self.cfg0_padconfig138
    }
    #[doc = "0x422c - CFG0_PADCONFIG139"]
    #[inline(always)]
    pub const fn cfg0_padconfig139(&self) -> &Cfg0Padconfig139 {
        &self.cfg0_padconfig139
    }
    #[doc = "0x4230 - CFG0_PADCONFIG140"]
    #[inline(always)]
    pub const fn cfg0_padconfig140(&self) -> &Cfg0Padconfig140 {
        &self.cfg0_padconfig140
    }
    #[doc = "0x4234 - CFG0_PADCONFIG141"]
    #[inline(always)]
    pub const fn cfg0_padconfig141(&self) -> &Cfg0Padconfig141 {
        &self.cfg0_padconfig141
    }
    #[doc = "0x4238 - CFG0_PADCONFIG142"]
    #[inline(always)]
    pub const fn cfg0_padconfig142(&self) -> &Cfg0Padconfig142 {
        &self.cfg0_padconfig142
    }
    #[doc = "0x423c - CFG0_PADCONFIG143"]
    #[inline(always)]
    pub const fn cfg0_padconfig143(&self) -> &Cfg0Padconfig143 {
        &self.cfg0_padconfig143
    }
    #[doc = "0x4240 - CFG0_PADCONFIG144"]
    #[inline(always)]
    pub const fn cfg0_padconfig144(&self) -> &Cfg0Padconfig144 {
        &self.cfg0_padconfig144
    }
    #[doc = "0x4244 - CFG0_PADCONFIG145"]
    #[inline(always)]
    pub const fn cfg0_padconfig145(&self) -> &Cfg0Padconfig145 {
        &self.cfg0_padconfig145
    }
    #[doc = "0x4248 - CFG0_PADCONFIG146"]
    #[inline(always)]
    pub const fn cfg0_padconfig146(&self) -> &Cfg0Padconfig146 {
        &self.cfg0_padconfig146
    }
    #[doc = "0x424c - CFG0_PADCONFIG147"]
    #[inline(always)]
    pub const fn cfg0_padconfig147(&self) -> &Cfg0Padconfig147 {
        &self.cfg0_padconfig147
    }
    #[doc = "0x4250 - CFG0_PADCONFIG148"]
    #[inline(always)]
    pub const fn cfg0_padconfig148(&self) -> &Cfg0Padconfig148 {
        &self.cfg0_padconfig148
    }
    #[doc = "0x4254 - CFG0_PADCONFIG149"]
    #[inline(always)]
    pub const fn cfg0_padconfig149(&self) -> &Cfg0Padconfig149 {
        &self.cfg0_padconfig149
    }
    #[doc = "0x4258 - CFG0_PADCONFIG150"]
    #[inline(always)]
    pub const fn cfg0_padconfig150(&self) -> &Cfg0Padconfig150 {
        &self.cfg0_padconfig150
    }
    #[doc = "0x425c - CFG0_PADCONFIG151"]
    #[inline(always)]
    pub const fn cfg0_padconfig151(&self) -> &Cfg0Padconfig151 {
        &self.cfg0_padconfig151
    }
    #[doc = "0x4260 - CFG0_PADCONFIG152"]
    #[inline(always)]
    pub const fn cfg0_padconfig152(&self) -> &Cfg0Padconfig152 {
        &self.cfg0_padconfig152
    }
    #[doc = "0x4264 - CFG0_PADCONFIG153"]
    #[inline(always)]
    pub const fn cfg0_padconfig153(&self) -> &Cfg0Padconfig153 {
        &self.cfg0_padconfig153
    }
    #[doc = "0x4268 - CFG0_PADCONFIG154"]
    #[inline(always)]
    pub const fn cfg0_padconfig154(&self) -> &Cfg0Padconfig154 {
        &self.cfg0_padconfig154
    }
    #[doc = "0x426c - CFG0_PADCONFIG155"]
    #[inline(always)]
    pub const fn cfg0_padconfig155(&self) -> &Cfg0Padconfig155 {
        &self.cfg0_padconfig155
    }
    #[doc = "0x4270 - CFG0_PADCONFIG156"]
    #[inline(always)]
    pub const fn cfg0_padconfig156(&self) -> &Cfg0Padconfig156 {
        &self.cfg0_padconfig156
    }
    #[doc = "0x4274 - CFG0_PADCONFIG157"]
    #[inline(always)]
    pub const fn cfg0_padconfig157(&self) -> &Cfg0Padconfig157 {
        &self.cfg0_padconfig157
    }
    #[doc = "0x4278 - CFG0_PADCONFIG158"]
    #[inline(always)]
    pub const fn cfg0_padconfig158(&self) -> &Cfg0Padconfig158 {
        &self.cfg0_padconfig158
    }
    #[doc = "0x427c - CFG0_PADCONFIG159"]
    #[inline(always)]
    pub const fn cfg0_padconfig159(&self) -> &Cfg0Padconfig159 {
        &self.cfg0_padconfig159
    }
    #[doc = "0x4280 - CFG0_PADCONFIG160"]
    #[inline(always)]
    pub const fn cfg0_padconfig160(&self) -> &Cfg0Padconfig160 {
        &self.cfg0_padconfig160
    }
    #[doc = "0x4284 - CFG0_PADCONFIG161"]
    #[inline(always)]
    pub const fn cfg0_padconfig161(&self) -> &Cfg0Padconfig161 {
        &self.cfg0_padconfig161
    }
    #[doc = "0x4288 - CFG0_PADCONFIG162"]
    #[inline(always)]
    pub const fn cfg0_padconfig162(&self) -> &Cfg0Padconfig162 {
        &self.cfg0_padconfig162
    }
    #[doc = "0x428c - CFG0_PADCONFIG163"]
    #[inline(always)]
    pub const fn cfg0_padconfig163(&self) -> &Cfg0Padconfig163 {
        &self.cfg0_padconfig163
    }
    #[doc = "0x4290 - CFG0_PADCONFIG164"]
    #[inline(always)]
    pub const fn cfg0_padconfig164(&self) -> &Cfg0Padconfig164 {
        &self.cfg0_padconfig164
    }
    #[doc = "0x4294 - CFG0_PADCONFIG165"]
    #[inline(always)]
    pub const fn cfg0_padconfig165(&self) -> &Cfg0Padconfig165 {
        &self.cfg0_padconfig165
    }
    #[doc = "0x4298 - CFG0_PADCONFIG166"]
    #[inline(always)]
    pub const fn cfg0_padconfig166(&self) -> &Cfg0Padconfig166 {
        &self.cfg0_padconfig166
    }
    #[doc = "0x429c - CFG0_PADCONFIG167"]
    #[inline(always)]
    pub const fn cfg0_padconfig167(&self) -> &Cfg0Padconfig167 {
        &self.cfg0_padconfig167
    }
    #[doc = "0x42a0 - CFG0_PADCONFIG168"]
    #[inline(always)]
    pub const fn cfg0_padconfig168(&self) -> &Cfg0Padconfig168 {
        &self.cfg0_padconfig168
    }
    #[doc = "0x42a4 - CFG0_PADCONFIG169"]
    #[inline(always)]
    pub const fn cfg0_padconfig169(&self) -> &Cfg0Padconfig169 {
        &self.cfg0_padconfig169
    }
    #[doc = "0x42a8 - CFG0_PADCONFIG170"]
    #[inline(always)]
    pub const fn cfg0_padconfig170(&self) -> &Cfg0Padconfig170 {
        &self.cfg0_padconfig170
    }
    #[doc = "0x42ac - CFG0_PADCONFIG171"]
    #[inline(always)]
    pub const fn cfg0_padconfig171(&self) -> &Cfg0Padconfig171 {
        &self.cfg0_padconfig171
    }
    #[doc = "0x42b0 - CFG0_PADCONFIG172"]
    #[inline(always)]
    pub const fn cfg0_padconfig172(&self) -> &Cfg0Padconfig172 {
        &self.cfg0_padconfig172
    }
    #[doc = "0x42b4 - CFG0_PADCONFIG173"]
    #[inline(always)]
    pub const fn cfg0_padconfig173(&self) -> &Cfg0Padconfig173 {
        &self.cfg0_padconfig173
    }
    #[doc = "0x42b8 - CFG0_PADCONFIG174"]
    #[inline(always)]
    pub const fn cfg0_padconfig174(&self) -> &Cfg0Padconfig174 {
        &self.cfg0_padconfig174
    }
    #[doc = "0x42bc - CFG0_PADCONFIG175"]
    #[inline(always)]
    pub const fn cfg0_padconfig175(&self) -> &Cfg0Padconfig175 {
        &self.cfg0_padconfig175
    }
    #[doc = "0x42c0 - CFG0_PADCONFIG176"]
    #[inline(always)]
    pub const fn cfg0_padconfig176(&self) -> &Cfg0Padconfig176 {
        &self.cfg0_padconfig176
    }
    #[doc = "0x42c4 - CFG0_PADCONFIG177"]
    #[inline(always)]
    pub const fn cfg0_padconfig177(&self) -> &Cfg0Padconfig177 {
        &self.cfg0_padconfig177
    }
    #[doc = "0x42c8 - CFG0_PADCONFIG178"]
    #[inline(always)]
    pub const fn cfg0_padconfig178(&self) -> &Cfg0Padconfig178 {
        &self.cfg0_padconfig178
    }
    #[doc = "0x42cc - CFG0_PADCONFIG179"]
    #[inline(always)]
    pub const fn cfg0_padconfig179(&self) -> &Cfg0Padconfig179 {
        &self.cfg0_padconfig179
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
    #[doc = "0x6084 - CFG0_PADCONFIG33_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig33_proxy(&self) -> &Cfg0Padconfig33Proxy {
        &self.cfg0_padconfig33_proxy
    }
    #[doc = "0x6088 - CFG0_PADCONFIG34_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig34_proxy(&self) -> &Cfg0Padconfig34Proxy {
        &self.cfg0_padconfig34_proxy
    }
    #[doc = "0x608c - CFG0_PADCONFIG35_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig35_proxy(&self) -> &Cfg0Padconfig35Proxy {
        &self.cfg0_padconfig35_proxy
    }
    #[doc = "0x6090 - CFG0_PADCONFIG36_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig36_proxy(&self) -> &Cfg0Padconfig36Proxy {
        &self.cfg0_padconfig36_proxy
    }
    #[doc = "0x6094 - CFG0_PADCONFIG37_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig37_proxy(&self) -> &Cfg0Padconfig37Proxy {
        &self.cfg0_padconfig37_proxy
    }
    #[doc = "0x6098 - CFG0_PADCONFIG38_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig38_proxy(&self) -> &Cfg0Padconfig38Proxy {
        &self.cfg0_padconfig38_proxy
    }
    #[doc = "0x609c - CFG0_PADCONFIG39_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig39_proxy(&self) -> &Cfg0Padconfig39Proxy {
        &self.cfg0_padconfig39_proxy
    }
    #[doc = "0x60a0 - CFG0_PADCONFIG40_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig40_proxy(&self) -> &Cfg0Padconfig40Proxy {
        &self.cfg0_padconfig40_proxy
    }
    #[doc = "0x60a4 - CFG0_PADCONFIG41_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig41_proxy(&self) -> &Cfg0Padconfig41Proxy {
        &self.cfg0_padconfig41_proxy
    }
    #[doc = "0x60a8 - CFG0_PADCONFIG42_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig42_proxy(&self) -> &Cfg0Padconfig42Proxy {
        &self.cfg0_padconfig42_proxy
    }
    #[doc = "0x60ac - CFG0_PADCONFIG43_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig43_proxy(&self) -> &Cfg0Padconfig43Proxy {
        &self.cfg0_padconfig43_proxy
    }
    #[doc = "0x60b0 - CFG0_PADCONFIG44_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig44_proxy(&self) -> &Cfg0Padconfig44Proxy {
        &self.cfg0_padconfig44_proxy
    }
    #[doc = "0x60b4 - CFG0_PADCONFIG45_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig45_proxy(&self) -> &Cfg0Padconfig45Proxy {
        &self.cfg0_padconfig45_proxy
    }
    #[doc = "0x60b8 - CFG0_PADCONFIG46_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig46_proxy(&self) -> &Cfg0Padconfig46Proxy {
        &self.cfg0_padconfig46_proxy
    }
    #[doc = "0x60bc - CFG0_PADCONFIG47_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig47_proxy(&self) -> &Cfg0Padconfig47Proxy {
        &self.cfg0_padconfig47_proxy
    }
    #[doc = "0x60c0 - CFG0_PADCONFIG48_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig48_proxy(&self) -> &Cfg0Padconfig48Proxy {
        &self.cfg0_padconfig48_proxy
    }
    #[doc = "0x60c4 - CFG0_PADCONFIG49_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig49_proxy(&self) -> &Cfg0Padconfig49Proxy {
        &self.cfg0_padconfig49_proxy
    }
    #[doc = "0x60c8 - CFG0_PADCONFIG50_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig50_proxy(&self) -> &Cfg0Padconfig50Proxy {
        &self.cfg0_padconfig50_proxy
    }
    #[doc = "0x60cc - CFG0_PADCONFIG51_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig51_proxy(&self) -> &Cfg0Padconfig51Proxy {
        &self.cfg0_padconfig51_proxy
    }
    #[doc = "0x60d0 - CFG0_PADCONFIG52_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig52_proxy(&self) -> &Cfg0Padconfig52Proxy {
        &self.cfg0_padconfig52_proxy
    }
    #[doc = "0x60d4 - CFG0_PADCONFIG53_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig53_proxy(&self) -> &Cfg0Padconfig53Proxy {
        &self.cfg0_padconfig53_proxy
    }
    #[doc = "0x60d8 - CFG0_PADCONFIG54_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig54_proxy(&self) -> &Cfg0Padconfig54Proxy {
        &self.cfg0_padconfig54_proxy
    }
    #[doc = "0x60dc - CFG0_PADCONFIG55_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig55_proxy(&self) -> &Cfg0Padconfig55Proxy {
        &self.cfg0_padconfig55_proxy
    }
    #[doc = "0x60e0 - CFG0_PADCONFIG56_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig56_proxy(&self) -> &Cfg0Padconfig56Proxy {
        &self.cfg0_padconfig56_proxy
    }
    #[doc = "0x60e4 - CFG0_PADCONFIG57_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig57_proxy(&self) -> &Cfg0Padconfig57Proxy {
        &self.cfg0_padconfig57_proxy
    }
    #[doc = "0x60e8 - CFG0_PADCONFIG58_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig58_proxy(&self) -> &Cfg0Padconfig58Proxy {
        &self.cfg0_padconfig58_proxy
    }
    #[doc = "0x60ec - CFG0_PADCONFIG59_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig59_proxy(&self) -> &Cfg0Padconfig59Proxy {
        &self.cfg0_padconfig59_proxy
    }
    #[doc = "0x60f0 - CFG0_PADCONFIG60_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig60_proxy(&self) -> &Cfg0Padconfig60Proxy {
        &self.cfg0_padconfig60_proxy
    }
    #[doc = "0x60f4 - CFG0_PADCONFIG61_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig61_proxy(&self) -> &Cfg0Padconfig61Proxy {
        &self.cfg0_padconfig61_proxy
    }
    #[doc = "0x60f8 - CFG0_PADCONFIG62_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig62_proxy(&self) -> &Cfg0Padconfig62Proxy {
        &self.cfg0_padconfig62_proxy
    }
    #[doc = "0x60fc - CFG0_PADCONFIG63_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig63_proxy(&self) -> &Cfg0Padconfig63Proxy {
        &self.cfg0_padconfig63_proxy
    }
    #[doc = "0x6100 - CFG0_PADCONFIG64_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig64_proxy(&self) -> &Cfg0Padconfig64Proxy {
        &self.cfg0_padconfig64_proxy
    }
    #[doc = "0x6104 - CFG0_PADCONFIG65_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig65_proxy(&self) -> &Cfg0Padconfig65Proxy {
        &self.cfg0_padconfig65_proxy
    }
    #[doc = "0x6108 - CFG0_PADCONFIG66_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig66_proxy(&self) -> &Cfg0Padconfig66Proxy {
        &self.cfg0_padconfig66_proxy
    }
    #[doc = "0x610c - CFG0_PADCONFIG67_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig67_proxy(&self) -> &Cfg0Padconfig67Proxy {
        &self.cfg0_padconfig67_proxy
    }
    #[doc = "0x6110 - CFG0_PADCONFIG68_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig68_proxy(&self) -> &Cfg0Padconfig68Proxy {
        &self.cfg0_padconfig68_proxy
    }
    #[doc = "0x6114 - CFG0_PADCONFIG69_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig69_proxy(&self) -> &Cfg0Padconfig69Proxy {
        &self.cfg0_padconfig69_proxy
    }
    #[doc = "0x6118 - CFG0_PADCONFIG70_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig70_proxy(&self) -> &Cfg0Padconfig70Proxy {
        &self.cfg0_padconfig70_proxy
    }
    #[doc = "0x611c - CFG0_PADCONFIG71_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig71_proxy(&self) -> &Cfg0Padconfig71Proxy {
        &self.cfg0_padconfig71_proxy
    }
    #[doc = "0x6120 - CFG0_PADCONFIG72_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig72_proxy(&self) -> &Cfg0Padconfig72Proxy {
        &self.cfg0_padconfig72_proxy
    }
    #[doc = "0x6124 - CFG0_PADCONFIG73_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig73_proxy(&self) -> &Cfg0Padconfig73Proxy {
        &self.cfg0_padconfig73_proxy
    }
    #[doc = "0x6128 - CFG0_PADCONFIG74_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig74_proxy(&self) -> &Cfg0Padconfig74Proxy {
        &self.cfg0_padconfig74_proxy
    }
    #[doc = "0x612c - CFG0_PADCONFIG75_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig75_proxy(&self) -> &Cfg0Padconfig75Proxy {
        &self.cfg0_padconfig75_proxy
    }
    #[doc = "0x6130 - CFG0_PADCONFIG76_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig76_proxy(&self) -> &Cfg0Padconfig76Proxy {
        &self.cfg0_padconfig76_proxy
    }
    #[doc = "0x6134 - CFG0_PADCONFIG77_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig77_proxy(&self) -> &Cfg0Padconfig77Proxy {
        &self.cfg0_padconfig77_proxy
    }
    #[doc = "0x6138 - CFG0_PADCONFIG78_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig78_proxy(&self) -> &Cfg0Padconfig78Proxy {
        &self.cfg0_padconfig78_proxy
    }
    #[doc = "0x613c - CFG0_PADCONFIG79_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig79_proxy(&self) -> &Cfg0Padconfig79Proxy {
        &self.cfg0_padconfig79_proxy
    }
    #[doc = "0x6140 - CFG0_PADCONFIG80_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig80_proxy(&self) -> &Cfg0Padconfig80Proxy {
        &self.cfg0_padconfig80_proxy
    }
    #[doc = "0x6144 - CFG0_PADCONFIG81_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig81_proxy(&self) -> &Cfg0Padconfig81Proxy {
        &self.cfg0_padconfig81_proxy
    }
    #[doc = "0x6148 - CFG0_PADCONFIG82_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig82_proxy(&self) -> &Cfg0Padconfig82Proxy {
        &self.cfg0_padconfig82_proxy
    }
    #[doc = "0x614c - CFG0_PADCONFIG83_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig83_proxy(&self) -> &Cfg0Padconfig83Proxy {
        &self.cfg0_padconfig83_proxy
    }
    #[doc = "0x6150 - CFG0_PADCONFIG84_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig84_proxy(&self) -> &Cfg0Padconfig84Proxy {
        &self.cfg0_padconfig84_proxy
    }
    #[doc = "0x6154 - CFG0_PADCONFIG85_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig85_proxy(&self) -> &Cfg0Padconfig85Proxy {
        &self.cfg0_padconfig85_proxy
    }
    #[doc = "0x6158 - CFG0_PADCONFIG86_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig86_proxy(&self) -> &Cfg0Padconfig86Proxy {
        &self.cfg0_padconfig86_proxy
    }
    #[doc = "0x615c - CFG0_PADCONFIG87_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig87_proxy(&self) -> &Cfg0Padconfig87Proxy {
        &self.cfg0_padconfig87_proxy
    }
    #[doc = "0x6160 - CFG0_PADCONFIG88_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig88_proxy(&self) -> &Cfg0Padconfig88Proxy {
        &self.cfg0_padconfig88_proxy
    }
    #[doc = "0x6164 - CFG0_PADCONFIG89_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig89_proxy(&self) -> &Cfg0Padconfig89Proxy {
        &self.cfg0_padconfig89_proxy
    }
    #[doc = "0x6168 - CFG0_PADCONFIG90_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig90_proxy(&self) -> &Cfg0Padconfig90Proxy {
        &self.cfg0_padconfig90_proxy
    }
    #[doc = "0x616c - CFG0_PADCONFIG91_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig91_proxy(&self) -> &Cfg0Padconfig91Proxy {
        &self.cfg0_padconfig91_proxy
    }
    #[doc = "0x6170 - CFG0_PADCONFIG92_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig92_proxy(&self) -> &Cfg0Padconfig92Proxy {
        &self.cfg0_padconfig92_proxy
    }
    #[doc = "0x6174 - CFG0_PADCONFIG93_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig93_proxy(&self) -> &Cfg0Padconfig93Proxy {
        &self.cfg0_padconfig93_proxy
    }
    #[doc = "0x6178 - CFG0_PADCONFIG94_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig94_proxy(&self) -> &Cfg0Padconfig94Proxy {
        &self.cfg0_padconfig94_proxy
    }
    #[doc = "0x617c - CFG0_PADCONFIG95_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig95_proxy(&self) -> &Cfg0Padconfig95Proxy {
        &self.cfg0_padconfig95_proxy
    }
    #[doc = "0x6180 - CFG0_PADCONFIG96_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig96_proxy(&self) -> &Cfg0Padconfig96Proxy {
        &self.cfg0_padconfig96_proxy
    }
    #[doc = "0x6184 - CFG0_PADCONFIG97_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig97_proxy(&self) -> &Cfg0Padconfig97Proxy {
        &self.cfg0_padconfig97_proxy
    }
    #[doc = "0x6188 - CFG0_PADCONFIG98_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig98_proxy(&self) -> &Cfg0Padconfig98Proxy {
        &self.cfg0_padconfig98_proxy
    }
    #[doc = "0x618c - CFG0_PADCONFIG99_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig99_proxy(&self) -> &Cfg0Padconfig99Proxy {
        &self.cfg0_padconfig99_proxy
    }
    #[doc = "0x6190 - CFG0_PADCONFIG100_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig100_proxy(&self) -> &Cfg0Padconfig100Proxy {
        &self.cfg0_padconfig100_proxy
    }
    #[doc = "0x6194 - CFG0_PADCONFIG101_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig101_proxy(&self) -> &Cfg0Padconfig101Proxy {
        &self.cfg0_padconfig101_proxy
    }
    #[doc = "0x6198 - CFG0_PADCONFIG102_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig102_proxy(&self) -> &Cfg0Padconfig102Proxy {
        &self.cfg0_padconfig102_proxy
    }
    #[doc = "0x619c - CFG0_PADCONFIG103_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig103_proxy(&self) -> &Cfg0Padconfig103Proxy {
        &self.cfg0_padconfig103_proxy
    }
    #[doc = "0x61a0 - CFG0_PADCONFIG104_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig104_proxy(&self) -> &Cfg0Padconfig104Proxy {
        &self.cfg0_padconfig104_proxy
    }
    #[doc = "0x61a4 - CFG0_PADCONFIG105_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig105_proxy(&self) -> &Cfg0Padconfig105Proxy {
        &self.cfg0_padconfig105_proxy
    }
    #[doc = "0x61a8 - CFG0_PADCONFIG106_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig106_proxy(&self) -> &Cfg0Padconfig106Proxy {
        &self.cfg0_padconfig106_proxy
    }
    #[doc = "0x61ac - CFG0_PADCONFIG107_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig107_proxy(&self) -> &Cfg0Padconfig107Proxy {
        &self.cfg0_padconfig107_proxy
    }
    #[doc = "0x61b0 - CFG0_PADCONFIG108_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig108_proxy(&self) -> &Cfg0Padconfig108Proxy {
        &self.cfg0_padconfig108_proxy
    }
    #[doc = "0x61b4 - CFG0_PADCONFIG109_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig109_proxy(&self) -> &Cfg0Padconfig109Proxy {
        &self.cfg0_padconfig109_proxy
    }
    #[doc = "0x61b8 - CFG0_PADCONFIG110_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig110_proxy(&self) -> &Cfg0Padconfig110Proxy {
        &self.cfg0_padconfig110_proxy
    }
    #[doc = "0x61bc - CFG0_PADCONFIG111_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig111_proxy(&self) -> &Cfg0Padconfig111Proxy {
        &self.cfg0_padconfig111_proxy
    }
    #[doc = "0x61c0 - CFG0_PADCONFIG112_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig112_proxy(&self) -> &Cfg0Padconfig112Proxy {
        &self.cfg0_padconfig112_proxy
    }
    #[doc = "0x61c4 - CFG0_PADCONFIG113_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig113_proxy(&self) -> &Cfg0Padconfig113Proxy {
        &self.cfg0_padconfig113_proxy
    }
    #[doc = "0x61c8 - CFG0_PADCONFIG114_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig114_proxy(&self) -> &Cfg0Padconfig114Proxy {
        &self.cfg0_padconfig114_proxy
    }
    #[doc = "0x61cc - CFG0_PADCONFIG115_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig115_proxy(&self) -> &Cfg0Padconfig115Proxy {
        &self.cfg0_padconfig115_proxy
    }
    #[doc = "0x61d0 - CFG0_PADCONFIG116_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig116_proxy(&self) -> &Cfg0Padconfig116Proxy {
        &self.cfg0_padconfig116_proxy
    }
    #[doc = "0x61d4 - CFG0_PADCONFIG117_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig117_proxy(&self) -> &Cfg0Padconfig117Proxy {
        &self.cfg0_padconfig117_proxy
    }
    #[doc = "0x61d8 - CFG0_PADCONFIG118_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig118_proxy(&self) -> &Cfg0Padconfig118Proxy {
        &self.cfg0_padconfig118_proxy
    }
    #[doc = "0x61dc - CFG0_PADCONFIG119_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig119_proxy(&self) -> &Cfg0Padconfig119Proxy {
        &self.cfg0_padconfig119_proxy
    }
    #[doc = "0x61e0 - CFG0_PADCONFIG120_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig120_proxy(&self) -> &Cfg0Padconfig120Proxy {
        &self.cfg0_padconfig120_proxy
    }
    #[doc = "0x61e4 - CFG0_PADCONFIG121_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig121_proxy(&self) -> &Cfg0Padconfig121Proxy {
        &self.cfg0_padconfig121_proxy
    }
    #[doc = "0x61e8 - CFG0_PADCONFIG122_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig122_proxy(&self) -> &Cfg0Padconfig122Proxy {
        &self.cfg0_padconfig122_proxy
    }
    #[doc = "0x61ec - CFG0_PADCONFIG123_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig123_proxy(&self) -> &Cfg0Padconfig123Proxy {
        &self.cfg0_padconfig123_proxy
    }
    #[doc = "0x61f0 - CFG0_PADCONFIG124_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig124_proxy(&self) -> &Cfg0Padconfig124Proxy {
        &self.cfg0_padconfig124_proxy
    }
    #[doc = "0x61f4 - CFG0_PADCONFIG125_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig125_proxy(&self) -> &Cfg0Padconfig125Proxy {
        &self.cfg0_padconfig125_proxy
    }
    #[doc = "0x61f8 - CFG0_PADCONFIG126_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig126_proxy(&self) -> &Cfg0Padconfig126Proxy {
        &self.cfg0_padconfig126_proxy
    }
    #[doc = "0x61fc - CFG0_PADCONFIG127_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig127_proxy(&self) -> &Cfg0Padconfig127Proxy {
        &self.cfg0_padconfig127_proxy
    }
    #[doc = "0x6200 - CFG0_PADCONFIG128_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig128_proxy(&self) -> &Cfg0Padconfig128Proxy {
        &self.cfg0_padconfig128_proxy
    }
    #[doc = "0x6204 - CFG0_PADCONFIG129_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig129_proxy(&self) -> &Cfg0Padconfig129Proxy {
        &self.cfg0_padconfig129_proxy
    }
    #[doc = "0x6208 - CFG0_PADCONFIG130_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig130_proxy(&self) -> &Cfg0Padconfig130Proxy {
        &self.cfg0_padconfig130_proxy
    }
    #[doc = "0x620c - CFG0_PADCONFIG131_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig131_proxy(&self) -> &Cfg0Padconfig131Proxy {
        &self.cfg0_padconfig131_proxy
    }
    #[doc = "0x6210 - CFG0_PADCONFIG132_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig132_proxy(&self) -> &Cfg0Padconfig132Proxy {
        &self.cfg0_padconfig132_proxy
    }
    #[doc = "0x6214 - CFG0_PADCONFIG133_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig133_proxy(&self) -> &Cfg0Padconfig133Proxy {
        &self.cfg0_padconfig133_proxy
    }
    #[doc = "0x6218 - CFG0_PADCONFIG134_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig134_proxy(&self) -> &Cfg0Padconfig134Proxy {
        &self.cfg0_padconfig134_proxy
    }
    #[doc = "0x621c - CFG0_PADCONFIG135_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig135_proxy(&self) -> &Cfg0Padconfig135Proxy {
        &self.cfg0_padconfig135_proxy
    }
    #[doc = "0x6220 - CFG0_PADCONFIG136_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig136_proxy(&self) -> &Cfg0Padconfig136Proxy {
        &self.cfg0_padconfig136_proxy
    }
    #[doc = "0x6224 - CFG0_PADCONFIG137_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig137_proxy(&self) -> &Cfg0Padconfig137Proxy {
        &self.cfg0_padconfig137_proxy
    }
    #[doc = "0x6228 - CFG0_PADCONFIG138_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig138_proxy(&self) -> &Cfg0Padconfig138Proxy {
        &self.cfg0_padconfig138_proxy
    }
    #[doc = "0x622c - CFG0_PADCONFIG139_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig139_proxy(&self) -> &Cfg0Padconfig139Proxy {
        &self.cfg0_padconfig139_proxy
    }
    #[doc = "0x6230 - CFG0_PADCONFIG140_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig140_proxy(&self) -> &Cfg0Padconfig140Proxy {
        &self.cfg0_padconfig140_proxy
    }
    #[doc = "0x6234 - CFG0_PADCONFIG141_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig141_proxy(&self) -> &Cfg0Padconfig141Proxy {
        &self.cfg0_padconfig141_proxy
    }
    #[doc = "0x6238 - CFG0_PADCONFIG142_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig142_proxy(&self) -> &Cfg0Padconfig142Proxy {
        &self.cfg0_padconfig142_proxy
    }
    #[doc = "0x623c - CFG0_PADCONFIG143_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig143_proxy(&self) -> &Cfg0Padconfig143Proxy {
        &self.cfg0_padconfig143_proxy
    }
    #[doc = "0x6240 - CFG0_PADCONFIG144_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig144_proxy(&self) -> &Cfg0Padconfig144Proxy {
        &self.cfg0_padconfig144_proxy
    }
    #[doc = "0x6244 - CFG0_PADCONFIG145_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig145_proxy(&self) -> &Cfg0Padconfig145Proxy {
        &self.cfg0_padconfig145_proxy
    }
    #[doc = "0x6248 - CFG0_PADCONFIG146_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig146_proxy(&self) -> &Cfg0Padconfig146Proxy {
        &self.cfg0_padconfig146_proxy
    }
    #[doc = "0x624c - CFG0_PADCONFIG147_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig147_proxy(&self) -> &Cfg0Padconfig147Proxy {
        &self.cfg0_padconfig147_proxy
    }
    #[doc = "0x6250 - CFG0_PADCONFIG148_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig148_proxy(&self) -> &Cfg0Padconfig148Proxy {
        &self.cfg0_padconfig148_proxy
    }
    #[doc = "0x6254 - CFG0_PADCONFIG149_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig149_proxy(&self) -> &Cfg0Padconfig149Proxy {
        &self.cfg0_padconfig149_proxy
    }
    #[doc = "0x6258 - CFG0_PADCONFIG150_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig150_proxy(&self) -> &Cfg0Padconfig150Proxy {
        &self.cfg0_padconfig150_proxy
    }
    #[doc = "0x625c - CFG0_PADCONFIG151_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig151_proxy(&self) -> &Cfg0Padconfig151Proxy {
        &self.cfg0_padconfig151_proxy
    }
    #[doc = "0x6260 - CFG0_PADCONFIG152_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig152_proxy(&self) -> &Cfg0Padconfig152Proxy {
        &self.cfg0_padconfig152_proxy
    }
    #[doc = "0x6264 - CFG0_PADCONFIG153_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig153_proxy(&self) -> &Cfg0Padconfig153Proxy {
        &self.cfg0_padconfig153_proxy
    }
    #[doc = "0x6268 - CFG0_PADCONFIG154_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig154_proxy(&self) -> &Cfg0Padconfig154Proxy {
        &self.cfg0_padconfig154_proxy
    }
    #[doc = "0x626c - CFG0_PADCONFIG155_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig155_proxy(&self) -> &Cfg0Padconfig155Proxy {
        &self.cfg0_padconfig155_proxy
    }
    #[doc = "0x6270 - CFG0_PADCONFIG156_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig156_proxy(&self) -> &Cfg0Padconfig156Proxy {
        &self.cfg0_padconfig156_proxy
    }
    #[doc = "0x6274 - CFG0_PADCONFIG157_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig157_proxy(&self) -> &Cfg0Padconfig157Proxy {
        &self.cfg0_padconfig157_proxy
    }
    #[doc = "0x6278 - CFG0_PADCONFIG158_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig158_proxy(&self) -> &Cfg0Padconfig158Proxy {
        &self.cfg0_padconfig158_proxy
    }
    #[doc = "0x627c - CFG0_PADCONFIG159_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig159_proxy(&self) -> &Cfg0Padconfig159Proxy {
        &self.cfg0_padconfig159_proxy
    }
    #[doc = "0x6280 - CFG0_PADCONFIG160_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig160_proxy(&self) -> &Cfg0Padconfig160Proxy {
        &self.cfg0_padconfig160_proxy
    }
    #[doc = "0x6284 - CFG0_PADCONFIG161_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig161_proxy(&self) -> &Cfg0Padconfig161Proxy {
        &self.cfg0_padconfig161_proxy
    }
    #[doc = "0x6288 - CFG0_PADCONFIG162_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig162_proxy(&self) -> &Cfg0Padconfig162Proxy {
        &self.cfg0_padconfig162_proxy
    }
    #[doc = "0x628c - CFG0_PADCONFIG163_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig163_proxy(&self) -> &Cfg0Padconfig163Proxy {
        &self.cfg0_padconfig163_proxy
    }
    #[doc = "0x6290 - CFG0_PADCONFIG164_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig164_proxy(&self) -> &Cfg0Padconfig164Proxy {
        &self.cfg0_padconfig164_proxy
    }
    #[doc = "0x6294 - CFG0_PADCONFIG165_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig165_proxy(&self) -> &Cfg0Padconfig165Proxy {
        &self.cfg0_padconfig165_proxy
    }
    #[doc = "0x6298 - CFG0_PADCONFIG166_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig166_proxy(&self) -> &Cfg0Padconfig166Proxy {
        &self.cfg0_padconfig166_proxy
    }
    #[doc = "0x629c - CFG0_PADCONFIG167_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig167_proxy(&self) -> &Cfg0Padconfig167Proxy {
        &self.cfg0_padconfig167_proxy
    }
    #[doc = "0x62a0 - CFG0_PADCONFIG168_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig168_proxy(&self) -> &Cfg0Padconfig168Proxy {
        &self.cfg0_padconfig168_proxy
    }
    #[doc = "0x62a4 - CFG0_PADCONFIG169_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig169_proxy(&self) -> &Cfg0Padconfig169Proxy {
        &self.cfg0_padconfig169_proxy
    }
    #[doc = "0x62a8 - CFG0_PADCONFIG170_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig170_proxy(&self) -> &Cfg0Padconfig170Proxy {
        &self.cfg0_padconfig170_proxy
    }
    #[doc = "0x62ac - CFG0_PADCONFIG171_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig171_proxy(&self) -> &Cfg0Padconfig171Proxy {
        &self.cfg0_padconfig171_proxy
    }
    #[doc = "0x62b0 - CFG0_PADCONFIG172_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig172_proxy(&self) -> &Cfg0Padconfig172Proxy {
        &self.cfg0_padconfig172_proxy
    }
    #[doc = "0x62b4 - CFG0_PADCONFIG173_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig173_proxy(&self) -> &Cfg0Padconfig173Proxy {
        &self.cfg0_padconfig173_proxy
    }
    #[doc = "0x62b8 - CFG0_PADCONFIG174_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig174_proxy(&self) -> &Cfg0Padconfig174Proxy {
        &self.cfg0_padconfig174_proxy
    }
    #[doc = "0x62bc - CFG0_PADCONFIG175_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig175_proxy(&self) -> &Cfg0Padconfig175Proxy {
        &self.cfg0_padconfig175_proxy
    }
    #[doc = "0x62c0 - CFG0_PADCONFIG176_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig176_proxy(&self) -> &Cfg0Padconfig176Proxy {
        &self.cfg0_padconfig176_proxy
    }
    #[doc = "0x62c4 - CFG0_PADCONFIG177_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig177_proxy(&self) -> &Cfg0Padconfig177Proxy {
        &self.cfg0_padconfig177_proxy
    }
    #[doc = "0x62c8 - CFG0_PADCONFIG178_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig178_proxy(&self) -> &Cfg0Padconfig178Proxy {
        &self.cfg0_padconfig178_proxy
    }
    #[doc = "0x62cc - CFG0_PADCONFIG179_PROXY"]
    #[inline(always)]
    pub const fn cfg0_padconfig179_proxy(&self) -> &Cfg0Padconfig179Proxy {
        &self.cfg0_padconfig179_proxy
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
#[doc = "CFG0_PADCONFIG33 (rw) register accessor: CFG0_PADCONFIG33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig33`]
module"]
#[doc(alias = "CFG0_PADCONFIG33")]
pub type Cfg0Padconfig33 = crate::Reg<cfg0_padconfig33::Cfg0Padconfig33Spec>;
#[doc = "CFG0_PADCONFIG33"]
pub mod cfg0_padconfig33;
#[doc = "CFG0_PADCONFIG34 (rw) register accessor: CFG0_PADCONFIG34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig34`]
module"]
#[doc(alias = "CFG0_PADCONFIG34")]
pub type Cfg0Padconfig34 = crate::Reg<cfg0_padconfig34::Cfg0Padconfig34Spec>;
#[doc = "CFG0_PADCONFIG34"]
pub mod cfg0_padconfig34;
#[doc = "CFG0_PADCONFIG35 (rw) register accessor: CFG0_PADCONFIG35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig35`]
module"]
#[doc(alias = "CFG0_PADCONFIG35")]
pub type Cfg0Padconfig35 = crate::Reg<cfg0_padconfig35::Cfg0Padconfig35Spec>;
#[doc = "CFG0_PADCONFIG35"]
pub mod cfg0_padconfig35;
#[doc = "CFG0_PADCONFIG36 (rw) register accessor: CFG0_PADCONFIG36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig36`]
module"]
#[doc(alias = "CFG0_PADCONFIG36")]
pub type Cfg0Padconfig36 = crate::Reg<cfg0_padconfig36::Cfg0Padconfig36Spec>;
#[doc = "CFG0_PADCONFIG36"]
pub mod cfg0_padconfig36;
#[doc = "CFG0_PADCONFIG37 (rw) register accessor: CFG0_PADCONFIG37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig37`]
module"]
#[doc(alias = "CFG0_PADCONFIG37")]
pub type Cfg0Padconfig37 = crate::Reg<cfg0_padconfig37::Cfg0Padconfig37Spec>;
#[doc = "CFG0_PADCONFIG37"]
pub mod cfg0_padconfig37;
#[doc = "CFG0_PADCONFIG38 (rw) register accessor: CFG0_PADCONFIG38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig38`]
module"]
#[doc(alias = "CFG0_PADCONFIG38")]
pub type Cfg0Padconfig38 = crate::Reg<cfg0_padconfig38::Cfg0Padconfig38Spec>;
#[doc = "CFG0_PADCONFIG38"]
pub mod cfg0_padconfig38;
#[doc = "CFG0_PADCONFIG39 (rw) register accessor: CFG0_PADCONFIG39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig39`]
module"]
#[doc(alias = "CFG0_PADCONFIG39")]
pub type Cfg0Padconfig39 = crate::Reg<cfg0_padconfig39::Cfg0Padconfig39Spec>;
#[doc = "CFG0_PADCONFIG39"]
pub mod cfg0_padconfig39;
#[doc = "CFG0_PADCONFIG40 (rw) register accessor: CFG0_PADCONFIG40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig40`]
module"]
#[doc(alias = "CFG0_PADCONFIG40")]
pub type Cfg0Padconfig40 = crate::Reg<cfg0_padconfig40::Cfg0Padconfig40Spec>;
#[doc = "CFG0_PADCONFIG40"]
pub mod cfg0_padconfig40;
#[doc = "CFG0_PADCONFIG41 (rw) register accessor: CFG0_PADCONFIG41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig41`]
module"]
#[doc(alias = "CFG0_PADCONFIG41")]
pub type Cfg0Padconfig41 = crate::Reg<cfg0_padconfig41::Cfg0Padconfig41Spec>;
#[doc = "CFG0_PADCONFIG41"]
pub mod cfg0_padconfig41;
#[doc = "CFG0_PADCONFIG42 (rw) register accessor: CFG0_PADCONFIG42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig42`]
module"]
#[doc(alias = "CFG0_PADCONFIG42")]
pub type Cfg0Padconfig42 = crate::Reg<cfg0_padconfig42::Cfg0Padconfig42Spec>;
#[doc = "CFG0_PADCONFIG42"]
pub mod cfg0_padconfig42;
#[doc = "CFG0_PADCONFIG43 (rw) register accessor: CFG0_PADCONFIG43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig43`]
module"]
#[doc(alias = "CFG0_PADCONFIG43")]
pub type Cfg0Padconfig43 = crate::Reg<cfg0_padconfig43::Cfg0Padconfig43Spec>;
#[doc = "CFG0_PADCONFIG43"]
pub mod cfg0_padconfig43;
#[doc = "CFG0_PADCONFIG44 (rw) register accessor: CFG0_PADCONFIG44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig44`]
module"]
#[doc(alias = "CFG0_PADCONFIG44")]
pub type Cfg0Padconfig44 = crate::Reg<cfg0_padconfig44::Cfg0Padconfig44Spec>;
#[doc = "CFG0_PADCONFIG44"]
pub mod cfg0_padconfig44;
#[doc = "CFG0_PADCONFIG45 (rw) register accessor: CFG0_PADCONFIG45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig45`]
module"]
#[doc(alias = "CFG0_PADCONFIG45")]
pub type Cfg0Padconfig45 = crate::Reg<cfg0_padconfig45::Cfg0Padconfig45Spec>;
#[doc = "CFG0_PADCONFIG45"]
pub mod cfg0_padconfig45;
#[doc = "CFG0_PADCONFIG46 (rw) register accessor: CFG0_PADCONFIG46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig46`]
module"]
#[doc(alias = "CFG0_PADCONFIG46")]
pub type Cfg0Padconfig46 = crate::Reg<cfg0_padconfig46::Cfg0Padconfig46Spec>;
#[doc = "CFG0_PADCONFIG46"]
pub mod cfg0_padconfig46;
#[doc = "CFG0_PADCONFIG47 (rw) register accessor: CFG0_PADCONFIG47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig47`]
module"]
#[doc(alias = "CFG0_PADCONFIG47")]
pub type Cfg0Padconfig47 = crate::Reg<cfg0_padconfig47::Cfg0Padconfig47Spec>;
#[doc = "CFG0_PADCONFIG47"]
pub mod cfg0_padconfig47;
#[doc = "CFG0_PADCONFIG48 (rw) register accessor: CFG0_PADCONFIG48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig48`]
module"]
#[doc(alias = "CFG0_PADCONFIG48")]
pub type Cfg0Padconfig48 = crate::Reg<cfg0_padconfig48::Cfg0Padconfig48Spec>;
#[doc = "CFG0_PADCONFIG48"]
pub mod cfg0_padconfig48;
#[doc = "CFG0_PADCONFIG49 (rw) register accessor: CFG0_PADCONFIG49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig49`]
module"]
#[doc(alias = "CFG0_PADCONFIG49")]
pub type Cfg0Padconfig49 = crate::Reg<cfg0_padconfig49::Cfg0Padconfig49Spec>;
#[doc = "CFG0_PADCONFIG49"]
pub mod cfg0_padconfig49;
#[doc = "CFG0_PADCONFIG50 (rw) register accessor: CFG0_PADCONFIG50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig50`]
module"]
#[doc(alias = "CFG0_PADCONFIG50")]
pub type Cfg0Padconfig50 = crate::Reg<cfg0_padconfig50::Cfg0Padconfig50Spec>;
#[doc = "CFG0_PADCONFIG50"]
pub mod cfg0_padconfig50;
#[doc = "CFG0_PADCONFIG51 (rw) register accessor: CFG0_PADCONFIG51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig51`]
module"]
#[doc(alias = "CFG0_PADCONFIG51")]
pub type Cfg0Padconfig51 = crate::Reg<cfg0_padconfig51::Cfg0Padconfig51Spec>;
#[doc = "CFG0_PADCONFIG51"]
pub mod cfg0_padconfig51;
#[doc = "CFG0_PADCONFIG52 (rw) register accessor: CFG0_PADCONFIG52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig52`]
module"]
#[doc(alias = "CFG0_PADCONFIG52")]
pub type Cfg0Padconfig52 = crate::Reg<cfg0_padconfig52::Cfg0Padconfig52Spec>;
#[doc = "CFG0_PADCONFIG52"]
pub mod cfg0_padconfig52;
#[doc = "CFG0_PADCONFIG53 (rw) register accessor: CFG0_PADCONFIG53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig53`]
module"]
#[doc(alias = "CFG0_PADCONFIG53")]
pub type Cfg0Padconfig53 = crate::Reg<cfg0_padconfig53::Cfg0Padconfig53Spec>;
#[doc = "CFG0_PADCONFIG53"]
pub mod cfg0_padconfig53;
#[doc = "CFG0_PADCONFIG54 (rw) register accessor: CFG0_PADCONFIG54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig54`]
module"]
#[doc(alias = "CFG0_PADCONFIG54")]
pub type Cfg0Padconfig54 = crate::Reg<cfg0_padconfig54::Cfg0Padconfig54Spec>;
#[doc = "CFG0_PADCONFIG54"]
pub mod cfg0_padconfig54;
#[doc = "CFG0_PADCONFIG55 (rw) register accessor: CFG0_PADCONFIG55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig55`]
module"]
#[doc(alias = "CFG0_PADCONFIG55")]
pub type Cfg0Padconfig55 = crate::Reg<cfg0_padconfig55::Cfg0Padconfig55Spec>;
#[doc = "CFG0_PADCONFIG55"]
pub mod cfg0_padconfig55;
#[doc = "CFG0_PADCONFIG56 (rw) register accessor: CFG0_PADCONFIG56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig56`]
module"]
#[doc(alias = "CFG0_PADCONFIG56")]
pub type Cfg0Padconfig56 = crate::Reg<cfg0_padconfig56::Cfg0Padconfig56Spec>;
#[doc = "CFG0_PADCONFIG56"]
pub mod cfg0_padconfig56;
#[doc = "CFG0_PADCONFIG57 (rw) register accessor: CFG0_PADCONFIG57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig57`]
module"]
#[doc(alias = "CFG0_PADCONFIG57")]
pub type Cfg0Padconfig57 = crate::Reg<cfg0_padconfig57::Cfg0Padconfig57Spec>;
#[doc = "CFG0_PADCONFIG57"]
pub mod cfg0_padconfig57;
#[doc = "CFG0_PADCONFIG58 (rw) register accessor: CFG0_PADCONFIG58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig58`]
module"]
#[doc(alias = "CFG0_PADCONFIG58")]
pub type Cfg0Padconfig58 = crate::Reg<cfg0_padconfig58::Cfg0Padconfig58Spec>;
#[doc = "CFG0_PADCONFIG58"]
pub mod cfg0_padconfig58;
#[doc = "CFG0_PADCONFIG59 (rw) register accessor: CFG0_PADCONFIG59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig59`]
module"]
#[doc(alias = "CFG0_PADCONFIG59")]
pub type Cfg0Padconfig59 = crate::Reg<cfg0_padconfig59::Cfg0Padconfig59Spec>;
#[doc = "CFG0_PADCONFIG59"]
pub mod cfg0_padconfig59;
#[doc = "CFG0_PADCONFIG60 (rw) register accessor: CFG0_PADCONFIG60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig60`]
module"]
#[doc(alias = "CFG0_PADCONFIG60")]
pub type Cfg0Padconfig60 = crate::Reg<cfg0_padconfig60::Cfg0Padconfig60Spec>;
#[doc = "CFG0_PADCONFIG60"]
pub mod cfg0_padconfig60;
#[doc = "CFG0_PADCONFIG61 (rw) register accessor: CFG0_PADCONFIG61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig61`]
module"]
#[doc(alias = "CFG0_PADCONFIG61")]
pub type Cfg0Padconfig61 = crate::Reg<cfg0_padconfig61::Cfg0Padconfig61Spec>;
#[doc = "CFG0_PADCONFIG61"]
pub mod cfg0_padconfig61;
#[doc = "CFG0_PADCONFIG62 (rw) register accessor: CFG0_PADCONFIG62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig62`]
module"]
#[doc(alias = "CFG0_PADCONFIG62")]
pub type Cfg0Padconfig62 = crate::Reg<cfg0_padconfig62::Cfg0Padconfig62Spec>;
#[doc = "CFG0_PADCONFIG62"]
pub mod cfg0_padconfig62;
#[doc = "CFG0_PADCONFIG63 (rw) register accessor: CFG0_PADCONFIG63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig63`]
module"]
#[doc(alias = "CFG0_PADCONFIG63")]
pub type Cfg0Padconfig63 = crate::Reg<cfg0_padconfig63::Cfg0Padconfig63Spec>;
#[doc = "CFG0_PADCONFIG63"]
pub mod cfg0_padconfig63;
#[doc = "CFG0_PADCONFIG64 (rw) register accessor: CFG0_PADCONFIG64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig64`]
module"]
#[doc(alias = "CFG0_PADCONFIG64")]
pub type Cfg0Padconfig64 = crate::Reg<cfg0_padconfig64::Cfg0Padconfig64Spec>;
#[doc = "CFG0_PADCONFIG64"]
pub mod cfg0_padconfig64;
#[doc = "CFG0_PADCONFIG65 (rw) register accessor: CFG0_PADCONFIG65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig65`]
module"]
#[doc(alias = "CFG0_PADCONFIG65")]
pub type Cfg0Padconfig65 = crate::Reg<cfg0_padconfig65::Cfg0Padconfig65Spec>;
#[doc = "CFG0_PADCONFIG65"]
pub mod cfg0_padconfig65;
#[doc = "CFG0_PADCONFIG66 (rw) register accessor: CFG0_PADCONFIG66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig66`]
module"]
#[doc(alias = "CFG0_PADCONFIG66")]
pub type Cfg0Padconfig66 = crate::Reg<cfg0_padconfig66::Cfg0Padconfig66Spec>;
#[doc = "CFG0_PADCONFIG66"]
pub mod cfg0_padconfig66;
#[doc = "CFG0_PADCONFIG67 (rw) register accessor: CFG0_PADCONFIG67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig67`]
module"]
#[doc(alias = "CFG0_PADCONFIG67")]
pub type Cfg0Padconfig67 = crate::Reg<cfg0_padconfig67::Cfg0Padconfig67Spec>;
#[doc = "CFG0_PADCONFIG67"]
pub mod cfg0_padconfig67;
#[doc = "CFG0_PADCONFIG68 (rw) register accessor: CFG0_PADCONFIG68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig68`]
module"]
#[doc(alias = "CFG0_PADCONFIG68")]
pub type Cfg0Padconfig68 = crate::Reg<cfg0_padconfig68::Cfg0Padconfig68Spec>;
#[doc = "CFG0_PADCONFIG68"]
pub mod cfg0_padconfig68;
#[doc = "CFG0_PADCONFIG69 (rw) register accessor: CFG0_PADCONFIG69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig69`]
module"]
#[doc(alias = "CFG0_PADCONFIG69")]
pub type Cfg0Padconfig69 = crate::Reg<cfg0_padconfig69::Cfg0Padconfig69Spec>;
#[doc = "CFG0_PADCONFIG69"]
pub mod cfg0_padconfig69;
#[doc = "CFG0_PADCONFIG70 (rw) register accessor: CFG0_PADCONFIG70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig70`]
module"]
#[doc(alias = "CFG0_PADCONFIG70")]
pub type Cfg0Padconfig70 = crate::Reg<cfg0_padconfig70::Cfg0Padconfig70Spec>;
#[doc = "CFG0_PADCONFIG70"]
pub mod cfg0_padconfig70;
#[doc = "CFG0_PADCONFIG71 (rw) register accessor: CFG0_PADCONFIG71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig71`]
module"]
#[doc(alias = "CFG0_PADCONFIG71")]
pub type Cfg0Padconfig71 = crate::Reg<cfg0_padconfig71::Cfg0Padconfig71Spec>;
#[doc = "CFG0_PADCONFIG71"]
pub mod cfg0_padconfig71;
#[doc = "CFG0_PADCONFIG72 (rw) register accessor: CFG0_PADCONFIG72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig72`]
module"]
#[doc(alias = "CFG0_PADCONFIG72")]
pub type Cfg0Padconfig72 = crate::Reg<cfg0_padconfig72::Cfg0Padconfig72Spec>;
#[doc = "CFG0_PADCONFIG72"]
pub mod cfg0_padconfig72;
#[doc = "CFG0_PADCONFIG73 (rw) register accessor: CFG0_PADCONFIG73\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig73::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig73`]
module"]
#[doc(alias = "CFG0_PADCONFIG73")]
pub type Cfg0Padconfig73 = crate::Reg<cfg0_padconfig73::Cfg0Padconfig73Spec>;
#[doc = "CFG0_PADCONFIG73"]
pub mod cfg0_padconfig73;
#[doc = "CFG0_PADCONFIG74 (rw) register accessor: CFG0_PADCONFIG74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig74`]
module"]
#[doc(alias = "CFG0_PADCONFIG74")]
pub type Cfg0Padconfig74 = crate::Reg<cfg0_padconfig74::Cfg0Padconfig74Spec>;
#[doc = "CFG0_PADCONFIG74"]
pub mod cfg0_padconfig74;
#[doc = "CFG0_PADCONFIG75 (rw) register accessor: CFG0_PADCONFIG75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig75`]
module"]
#[doc(alias = "CFG0_PADCONFIG75")]
pub type Cfg0Padconfig75 = crate::Reg<cfg0_padconfig75::Cfg0Padconfig75Spec>;
#[doc = "CFG0_PADCONFIG75"]
pub mod cfg0_padconfig75;
#[doc = "CFG0_PADCONFIG76 (rw) register accessor: CFG0_PADCONFIG76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig76`]
module"]
#[doc(alias = "CFG0_PADCONFIG76")]
pub type Cfg0Padconfig76 = crate::Reg<cfg0_padconfig76::Cfg0Padconfig76Spec>;
#[doc = "CFG0_PADCONFIG76"]
pub mod cfg0_padconfig76;
#[doc = "CFG0_PADCONFIG77 (rw) register accessor: CFG0_PADCONFIG77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig77`]
module"]
#[doc(alias = "CFG0_PADCONFIG77")]
pub type Cfg0Padconfig77 = crate::Reg<cfg0_padconfig77::Cfg0Padconfig77Spec>;
#[doc = "CFG0_PADCONFIG77"]
pub mod cfg0_padconfig77;
#[doc = "CFG0_PADCONFIG78 (rw) register accessor: CFG0_PADCONFIG78\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig78`]
module"]
#[doc(alias = "CFG0_PADCONFIG78")]
pub type Cfg0Padconfig78 = crate::Reg<cfg0_padconfig78::Cfg0Padconfig78Spec>;
#[doc = "CFG0_PADCONFIG78"]
pub mod cfg0_padconfig78;
#[doc = "CFG0_PADCONFIG79 (rw) register accessor: CFG0_PADCONFIG79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig79`]
module"]
#[doc(alias = "CFG0_PADCONFIG79")]
pub type Cfg0Padconfig79 = crate::Reg<cfg0_padconfig79::Cfg0Padconfig79Spec>;
#[doc = "CFG0_PADCONFIG79"]
pub mod cfg0_padconfig79;
#[doc = "CFG0_PADCONFIG80 (rw) register accessor: CFG0_PADCONFIG80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig80`]
module"]
#[doc(alias = "CFG0_PADCONFIG80")]
pub type Cfg0Padconfig80 = crate::Reg<cfg0_padconfig80::Cfg0Padconfig80Spec>;
#[doc = "CFG0_PADCONFIG80"]
pub mod cfg0_padconfig80;
#[doc = "CFG0_PADCONFIG81 (rw) register accessor: CFG0_PADCONFIG81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig81`]
module"]
#[doc(alias = "CFG0_PADCONFIG81")]
pub type Cfg0Padconfig81 = crate::Reg<cfg0_padconfig81::Cfg0Padconfig81Spec>;
#[doc = "CFG0_PADCONFIG81"]
pub mod cfg0_padconfig81;
#[doc = "CFG0_PADCONFIG82 (rw) register accessor: CFG0_PADCONFIG82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig82`]
module"]
#[doc(alias = "CFG0_PADCONFIG82")]
pub type Cfg0Padconfig82 = crate::Reg<cfg0_padconfig82::Cfg0Padconfig82Spec>;
#[doc = "CFG0_PADCONFIG82"]
pub mod cfg0_padconfig82;
#[doc = "CFG0_PADCONFIG83 (rw) register accessor: CFG0_PADCONFIG83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig83`]
module"]
#[doc(alias = "CFG0_PADCONFIG83")]
pub type Cfg0Padconfig83 = crate::Reg<cfg0_padconfig83::Cfg0Padconfig83Spec>;
#[doc = "CFG0_PADCONFIG83"]
pub mod cfg0_padconfig83;
#[doc = "CFG0_PADCONFIG84 (rw) register accessor: CFG0_PADCONFIG84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig84`]
module"]
#[doc(alias = "CFG0_PADCONFIG84")]
pub type Cfg0Padconfig84 = crate::Reg<cfg0_padconfig84::Cfg0Padconfig84Spec>;
#[doc = "CFG0_PADCONFIG84"]
pub mod cfg0_padconfig84;
#[doc = "CFG0_PADCONFIG85 (rw) register accessor: CFG0_PADCONFIG85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig85`]
module"]
#[doc(alias = "CFG0_PADCONFIG85")]
pub type Cfg0Padconfig85 = crate::Reg<cfg0_padconfig85::Cfg0Padconfig85Spec>;
#[doc = "CFG0_PADCONFIG85"]
pub mod cfg0_padconfig85;
#[doc = "CFG0_PADCONFIG86 (rw) register accessor: CFG0_PADCONFIG86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig86`]
module"]
#[doc(alias = "CFG0_PADCONFIG86")]
pub type Cfg0Padconfig86 = crate::Reg<cfg0_padconfig86::Cfg0Padconfig86Spec>;
#[doc = "CFG0_PADCONFIG86"]
pub mod cfg0_padconfig86;
#[doc = "CFG0_PADCONFIG87 (rw) register accessor: CFG0_PADCONFIG87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig87`]
module"]
#[doc(alias = "CFG0_PADCONFIG87")]
pub type Cfg0Padconfig87 = crate::Reg<cfg0_padconfig87::Cfg0Padconfig87Spec>;
#[doc = "CFG0_PADCONFIG87"]
pub mod cfg0_padconfig87;
#[doc = "CFG0_PADCONFIG88 (rw) register accessor: CFG0_PADCONFIG88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig88`]
module"]
#[doc(alias = "CFG0_PADCONFIG88")]
pub type Cfg0Padconfig88 = crate::Reg<cfg0_padconfig88::Cfg0Padconfig88Spec>;
#[doc = "CFG0_PADCONFIG88"]
pub mod cfg0_padconfig88;
#[doc = "CFG0_PADCONFIG89 (rw) register accessor: CFG0_PADCONFIG89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig89`]
module"]
#[doc(alias = "CFG0_PADCONFIG89")]
pub type Cfg0Padconfig89 = crate::Reg<cfg0_padconfig89::Cfg0Padconfig89Spec>;
#[doc = "CFG0_PADCONFIG89"]
pub mod cfg0_padconfig89;
#[doc = "CFG0_PADCONFIG90 (rw) register accessor: CFG0_PADCONFIG90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig90`]
module"]
#[doc(alias = "CFG0_PADCONFIG90")]
pub type Cfg0Padconfig90 = crate::Reg<cfg0_padconfig90::Cfg0Padconfig90Spec>;
#[doc = "CFG0_PADCONFIG90"]
pub mod cfg0_padconfig90;
#[doc = "CFG0_PADCONFIG91 (rw) register accessor: CFG0_PADCONFIG91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig91`]
module"]
#[doc(alias = "CFG0_PADCONFIG91")]
pub type Cfg0Padconfig91 = crate::Reg<cfg0_padconfig91::Cfg0Padconfig91Spec>;
#[doc = "CFG0_PADCONFIG91"]
pub mod cfg0_padconfig91;
#[doc = "CFG0_PADCONFIG92 (rw) register accessor: CFG0_PADCONFIG92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig92`]
module"]
#[doc(alias = "CFG0_PADCONFIG92")]
pub type Cfg0Padconfig92 = crate::Reg<cfg0_padconfig92::Cfg0Padconfig92Spec>;
#[doc = "CFG0_PADCONFIG92"]
pub mod cfg0_padconfig92;
#[doc = "CFG0_PADCONFIG93 (rw) register accessor: CFG0_PADCONFIG93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig93`]
module"]
#[doc(alias = "CFG0_PADCONFIG93")]
pub type Cfg0Padconfig93 = crate::Reg<cfg0_padconfig93::Cfg0Padconfig93Spec>;
#[doc = "CFG0_PADCONFIG93"]
pub mod cfg0_padconfig93;
#[doc = "CFG0_PADCONFIG94 (rw) register accessor: CFG0_PADCONFIG94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig94`]
module"]
#[doc(alias = "CFG0_PADCONFIG94")]
pub type Cfg0Padconfig94 = crate::Reg<cfg0_padconfig94::Cfg0Padconfig94Spec>;
#[doc = "CFG0_PADCONFIG94"]
pub mod cfg0_padconfig94;
#[doc = "CFG0_PADCONFIG95 (rw) register accessor: CFG0_PADCONFIG95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig95`]
module"]
#[doc(alias = "CFG0_PADCONFIG95")]
pub type Cfg0Padconfig95 = crate::Reg<cfg0_padconfig95::Cfg0Padconfig95Spec>;
#[doc = "CFG0_PADCONFIG95"]
pub mod cfg0_padconfig95;
#[doc = "CFG0_PADCONFIG96 (rw) register accessor: CFG0_PADCONFIG96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig96`]
module"]
#[doc(alias = "CFG0_PADCONFIG96")]
pub type Cfg0Padconfig96 = crate::Reg<cfg0_padconfig96::Cfg0Padconfig96Spec>;
#[doc = "CFG0_PADCONFIG96"]
pub mod cfg0_padconfig96;
#[doc = "CFG0_PADCONFIG97 (rw) register accessor: CFG0_PADCONFIG97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig97`]
module"]
#[doc(alias = "CFG0_PADCONFIG97")]
pub type Cfg0Padconfig97 = crate::Reg<cfg0_padconfig97::Cfg0Padconfig97Spec>;
#[doc = "CFG0_PADCONFIG97"]
pub mod cfg0_padconfig97;
#[doc = "CFG0_PADCONFIG98 (rw) register accessor: CFG0_PADCONFIG98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig98`]
module"]
#[doc(alias = "CFG0_PADCONFIG98")]
pub type Cfg0Padconfig98 = crate::Reg<cfg0_padconfig98::Cfg0Padconfig98Spec>;
#[doc = "CFG0_PADCONFIG98"]
pub mod cfg0_padconfig98;
#[doc = "CFG0_PADCONFIG99 (rw) register accessor: CFG0_PADCONFIG99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig99`]
module"]
#[doc(alias = "CFG0_PADCONFIG99")]
pub type Cfg0Padconfig99 = crate::Reg<cfg0_padconfig99::Cfg0Padconfig99Spec>;
#[doc = "CFG0_PADCONFIG99"]
pub mod cfg0_padconfig99;
#[doc = "CFG0_PADCONFIG100 (rw) register accessor: CFG0_PADCONFIG100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig100`]
module"]
#[doc(alias = "CFG0_PADCONFIG100")]
pub type Cfg0Padconfig100 = crate::Reg<cfg0_padconfig100::Cfg0Padconfig100Spec>;
#[doc = "CFG0_PADCONFIG100"]
pub mod cfg0_padconfig100;
#[doc = "CFG0_PADCONFIG101 (rw) register accessor: CFG0_PADCONFIG101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig101`]
module"]
#[doc(alias = "CFG0_PADCONFIG101")]
pub type Cfg0Padconfig101 = crate::Reg<cfg0_padconfig101::Cfg0Padconfig101Spec>;
#[doc = "CFG0_PADCONFIG101"]
pub mod cfg0_padconfig101;
#[doc = "CFG0_PADCONFIG102 (rw) register accessor: CFG0_PADCONFIG102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig102`]
module"]
#[doc(alias = "CFG0_PADCONFIG102")]
pub type Cfg0Padconfig102 = crate::Reg<cfg0_padconfig102::Cfg0Padconfig102Spec>;
#[doc = "CFG0_PADCONFIG102"]
pub mod cfg0_padconfig102;
#[doc = "CFG0_PADCONFIG103 (rw) register accessor: CFG0_PADCONFIG103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig103`]
module"]
#[doc(alias = "CFG0_PADCONFIG103")]
pub type Cfg0Padconfig103 = crate::Reg<cfg0_padconfig103::Cfg0Padconfig103Spec>;
#[doc = "CFG0_PADCONFIG103"]
pub mod cfg0_padconfig103;
#[doc = "CFG0_PADCONFIG104 (rw) register accessor: CFG0_PADCONFIG104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig104`]
module"]
#[doc(alias = "CFG0_PADCONFIG104")]
pub type Cfg0Padconfig104 = crate::Reg<cfg0_padconfig104::Cfg0Padconfig104Spec>;
#[doc = "CFG0_PADCONFIG104"]
pub mod cfg0_padconfig104;
#[doc = "CFG0_PADCONFIG105 (rw) register accessor: CFG0_PADCONFIG105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig105`]
module"]
#[doc(alias = "CFG0_PADCONFIG105")]
pub type Cfg0Padconfig105 = crate::Reg<cfg0_padconfig105::Cfg0Padconfig105Spec>;
#[doc = "CFG0_PADCONFIG105"]
pub mod cfg0_padconfig105;
#[doc = "CFG0_PADCONFIG106 (rw) register accessor: CFG0_PADCONFIG106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig106`]
module"]
#[doc(alias = "CFG0_PADCONFIG106")]
pub type Cfg0Padconfig106 = crate::Reg<cfg0_padconfig106::Cfg0Padconfig106Spec>;
#[doc = "CFG0_PADCONFIG106"]
pub mod cfg0_padconfig106;
#[doc = "CFG0_PADCONFIG107 (rw) register accessor: CFG0_PADCONFIG107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig107`]
module"]
#[doc(alias = "CFG0_PADCONFIG107")]
pub type Cfg0Padconfig107 = crate::Reg<cfg0_padconfig107::Cfg0Padconfig107Spec>;
#[doc = "CFG0_PADCONFIG107"]
pub mod cfg0_padconfig107;
#[doc = "CFG0_PADCONFIG108 (rw) register accessor: CFG0_PADCONFIG108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig108::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig108`]
module"]
#[doc(alias = "CFG0_PADCONFIG108")]
pub type Cfg0Padconfig108 = crate::Reg<cfg0_padconfig108::Cfg0Padconfig108Spec>;
#[doc = "CFG0_PADCONFIG108"]
pub mod cfg0_padconfig108;
#[doc = "CFG0_PADCONFIG109 (rw) register accessor: CFG0_PADCONFIG109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig109`]
module"]
#[doc(alias = "CFG0_PADCONFIG109")]
pub type Cfg0Padconfig109 = crate::Reg<cfg0_padconfig109::Cfg0Padconfig109Spec>;
#[doc = "CFG0_PADCONFIG109"]
pub mod cfg0_padconfig109;
#[doc = "CFG0_PADCONFIG110 (rw) register accessor: CFG0_PADCONFIG110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig110::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig110`]
module"]
#[doc(alias = "CFG0_PADCONFIG110")]
pub type Cfg0Padconfig110 = crate::Reg<cfg0_padconfig110::Cfg0Padconfig110Spec>;
#[doc = "CFG0_PADCONFIG110"]
pub mod cfg0_padconfig110;
#[doc = "CFG0_PADCONFIG111 (rw) register accessor: CFG0_PADCONFIG111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig111`]
module"]
#[doc(alias = "CFG0_PADCONFIG111")]
pub type Cfg0Padconfig111 = crate::Reg<cfg0_padconfig111::Cfg0Padconfig111Spec>;
#[doc = "CFG0_PADCONFIG111"]
pub mod cfg0_padconfig111;
#[doc = "CFG0_PADCONFIG112 (rw) register accessor: CFG0_PADCONFIG112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig112::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig112`]
module"]
#[doc(alias = "CFG0_PADCONFIG112")]
pub type Cfg0Padconfig112 = crate::Reg<cfg0_padconfig112::Cfg0Padconfig112Spec>;
#[doc = "CFG0_PADCONFIG112"]
pub mod cfg0_padconfig112;
#[doc = "CFG0_PADCONFIG113 (rw) register accessor: CFG0_PADCONFIG113\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig113::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig113`]
module"]
#[doc(alias = "CFG0_PADCONFIG113")]
pub type Cfg0Padconfig113 = crate::Reg<cfg0_padconfig113::Cfg0Padconfig113Spec>;
#[doc = "CFG0_PADCONFIG113"]
pub mod cfg0_padconfig113;
#[doc = "CFG0_PADCONFIG114 (rw) register accessor: CFG0_PADCONFIG114\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig114::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig114`]
module"]
#[doc(alias = "CFG0_PADCONFIG114")]
pub type Cfg0Padconfig114 = crate::Reg<cfg0_padconfig114::Cfg0Padconfig114Spec>;
#[doc = "CFG0_PADCONFIG114"]
pub mod cfg0_padconfig114;
#[doc = "CFG0_PADCONFIG115 (rw) register accessor: CFG0_PADCONFIG115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig115::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig115`]
module"]
#[doc(alias = "CFG0_PADCONFIG115")]
pub type Cfg0Padconfig115 = crate::Reg<cfg0_padconfig115::Cfg0Padconfig115Spec>;
#[doc = "CFG0_PADCONFIG115"]
pub mod cfg0_padconfig115;
#[doc = "CFG0_PADCONFIG116 (rw) register accessor: CFG0_PADCONFIG116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig116::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig116`]
module"]
#[doc(alias = "CFG0_PADCONFIG116")]
pub type Cfg0Padconfig116 = crate::Reg<cfg0_padconfig116::Cfg0Padconfig116Spec>;
#[doc = "CFG0_PADCONFIG116"]
pub mod cfg0_padconfig116;
#[doc = "CFG0_PADCONFIG117 (rw) register accessor: CFG0_PADCONFIG117\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig117::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig117`]
module"]
#[doc(alias = "CFG0_PADCONFIG117")]
pub type Cfg0Padconfig117 = crate::Reg<cfg0_padconfig117::Cfg0Padconfig117Spec>;
#[doc = "CFG0_PADCONFIG117"]
pub mod cfg0_padconfig117;
#[doc = "CFG0_PADCONFIG118 (rw) register accessor: CFG0_PADCONFIG118\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig118::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig118`]
module"]
#[doc(alias = "CFG0_PADCONFIG118")]
pub type Cfg0Padconfig118 = crate::Reg<cfg0_padconfig118::Cfg0Padconfig118Spec>;
#[doc = "CFG0_PADCONFIG118"]
pub mod cfg0_padconfig118;
#[doc = "CFG0_PADCONFIG119 (rw) register accessor: CFG0_PADCONFIG119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig119::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig119::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig119`]
module"]
#[doc(alias = "CFG0_PADCONFIG119")]
pub type Cfg0Padconfig119 = crate::Reg<cfg0_padconfig119::Cfg0Padconfig119Spec>;
#[doc = "CFG0_PADCONFIG119"]
pub mod cfg0_padconfig119;
#[doc = "CFG0_PADCONFIG120 (rw) register accessor: CFG0_PADCONFIG120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig120::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig120`]
module"]
#[doc(alias = "CFG0_PADCONFIG120")]
pub type Cfg0Padconfig120 = crate::Reg<cfg0_padconfig120::Cfg0Padconfig120Spec>;
#[doc = "CFG0_PADCONFIG120"]
pub mod cfg0_padconfig120;
#[doc = "CFG0_PADCONFIG121 (rw) register accessor: CFG0_PADCONFIG121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig121::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig121`]
module"]
#[doc(alias = "CFG0_PADCONFIG121")]
pub type Cfg0Padconfig121 = crate::Reg<cfg0_padconfig121::Cfg0Padconfig121Spec>;
#[doc = "CFG0_PADCONFIG121"]
pub mod cfg0_padconfig121;
#[doc = "CFG0_PADCONFIG122 (rw) register accessor: CFG0_PADCONFIG122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig122::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig122`]
module"]
#[doc(alias = "CFG0_PADCONFIG122")]
pub type Cfg0Padconfig122 = crate::Reg<cfg0_padconfig122::Cfg0Padconfig122Spec>;
#[doc = "CFG0_PADCONFIG122"]
pub mod cfg0_padconfig122;
#[doc = "CFG0_PADCONFIG123 (rw) register accessor: CFG0_PADCONFIG123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig123`]
module"]
#[doc(alias = "CFG0_PADCONFIG123")]
pub type Cfg0Padconfig123 = crate::Reg<cfg0_padconfig123::Cfg0Padconfig123Spec>;
#[doc = "CFG0_PADCONFIG123"]
pub mod cfg0_padconfig123;
#[doc = "CFG0_PADCONFIG124 (rw) register accessor: CFG0_PADCONFIG124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig124::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig124`]
module"]
#[doc(alias = "CFG0_PADCONFIG124")]
pub type Cfg0Padconfig124 = crate::Reg<cfg0_padconfig124::Cfg0Padconfig124Spec>;
#[doc = "CFG0_PADCONFIG124"]
pub mod cfg0_padconfig124;
#[doc = "CFG0_PADCONFIG125 (rw) register accessor: CFG0_PADCONFIG125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig125`]
module"]
#[doc(alias = "CFG0_PADCONFIG125")]
pub type Cfg0Padconfig125 = crate::Reg<cfg0_padconfig125::Cfg0Padconfig125Spec>;
#[doc = "CFG0_PADCONFIG125"]
pub mod cfg0_padconfig125;
#[doc = "CFG0_PADCONFIG126 (rw) register accessor: CFG0_PADCONFIG126\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig126::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig126`]
module"]
#[doc(alias = "CFG0_PADCONFIG126")]
pub type Cfg0Padconfig126 = crate::Reg<cfg0_padconfig126::Cfg0Padconfig126Spec>;
#[doc = "CFG0_PADCONFIG126"]
pub mod cfg0_padconfig126;
#[doc = "CFG0_PADCONFIG127 (rw) register accessor: CFG0_PADCONFIG127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig127::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig127`]
module"]
#[doc(alias = "CFG0_PADCONFIG127")]
pub type Cfg0Padconfig127 = crate::Reg<cfg0_padconfig127::Cfg0Padconfig127Spec>;
#[doc = "CFG0_PADCONFIG127"]
pub mod cfg0_padconfig127;
#[doc = "CFG0_PADCONFIG128 (rw) register accessor: CFG0_PADCONFIG128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig128`]
module"]
#[doc(alias = "CFG0_PADCONFIG128")]
pub type Cfg0Padconfig128 = crate::Reg<cfg0_padconfig128::Cfg0Padconfig128Spec>;
#[doc = "CFG0_PADCONFIG128"]
pub mod cfg0_padconfig128;
#[doc = "CFG0_PADCONFIG129 (rw) register accessor: CFG0_PADCONFIG129\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig129`]
module"]
#[doc(alias = "CFG0_PADCONFIG129")]
pub type Cfg0Padconfig129 = crate::Reg<cfg0_padconfig129::Cfg0Padconfig129Spec>;
#[doc = "CFG0_PADCONFIG129"]
pub mod cfg0_padconfig129;
#[doc = "CFG0_PADCONFIG130 (rw) register accessor: CFG0_PADCONFIG130\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig130`]
module"]
#[doc(alias = "CFG0_PADCONFIG130")]
pub type Cfg0Padconfig130 = crate::Reg<cfg0_padconfig130::Cfg0Padconfig130Spec>;
#[doc = "CFG0_PADCONFIG130"]
pub mod cfg0_padconfig130;
#[doc = "CFG0_PADCONFIG131 (rw) register accessor: CFG0_PADCONFIG131\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig131`]
module"]
#[doc(alias = "CFG0_PADCONFIG131")]
pub type Cfg0Padconfig131 = crate::Reg<cfg0_padconfig131::Cfg0Padconfig131Spec>;
#[doc = "CFG0_PADCONFIG131"]
pub mod cfg0_padconfig131;
#[doc = "CFG0_PADCONFIG132 (rw) register accessor: CFG0_PADCONFIG132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig132`]
module"]
#[doc(alias = "CFG0_PADCONFIG132")]
pub type Cfg0Padconfig132 = crate::Reg<cfg0_padconfig132::Cfg0Padconfig132Spec>;
#[doc = "CFG0_PADCONFIG132"]
pub mod cfg0_padconfig132;
#[doc = "CFG0_PADCONFIG133 (rw) register accessor: CFG0_PADCONFIG133\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig133`]
module"]
#[doc(alias = "CFG0_PADCONFIG133")]
pub type Cfg0Padconfig133 = crate::Reg<cfg0_padconfig133::Cfg0Padconfig133Spec>;
#[doc = "CFG0_PADCONFIG133"]
pub mod cfg0_padconfig133;
#[doc = "CFG0_PADCONFIG134 (rw) register accessor: CFG0_PADCONFIG134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig134`]
module"]
#[doc(alias = "CFG0_PADCONFIG134")]
pub type Cfg0Padconfig134 = crate::Reg<cfg0_padconfig134::Cfg0Padconfig134Spec>;
#[doc = "CFG0_PADCONFIG134"]
pub mod cfg0_padconfig134;
#[doc = "CFG0_PADCONFIG135 (rw) register accessor: CFG0_PADCONFIG135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig135`]
module"]
#[doc(alias = "CFG0_PADCONFIG135")]
pub type Cfg0Padconfig135 = crate::Reg<cfg0_padconfig135::Cfg0Padconfig135Spec>;
#[doc = "CFG0_PADCONFIG135"]
pub mod cfg0_padconfig135;
#[doc = "CFG0_PADCONFIG136 (rw) register accessor: CFG0_PADCONFIG136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig136`]
module"]
#[doc(alias = "CFG0_PADCONFIG136")]
pub type Cfg0Padconfig136 = crate::Reg<cfg0_padconfig136::Cfg0Padconfig136Spec>;
#[doc = "CFG0_PADCONFIG136"]
pub mod cfg0_padconfig136;
#[doc = "CFG0_PADCONFIG137 (rw) register accessor: CFG0_PADCONFIG137\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig137`]
module"]
#[doc(alias = "CFG0_PADCONFIG137")]
pub type Cfg0Padconfig137 = crate::Reg<cfg0_padconfig137::Cfg0Padconfig137Spec>;
#[doc = "CFG0_PADCONFIG137"]
pub mod cfg0_padconfig137;
#[doc = "CFG0_PADCONFIG138 (rw) register accessor: CFG0_PADCONFIG138\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig138`]
module"]
#[doc(alias = "CFG0_PADCONFIG138")]
pub type Cfg0Padconfig138 = crate::Reg<cfg0_padconfig138::Cfg0Padconfig138Spec>;
#[doc = "CFG0_PADCONFIG138"]
pub mod cfg0_padconfig138;
#[doc = "CFG0_PADCONFIG139 (rw) register accessor: CFG0_PADCONFIG139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig139`]
module"]
#[doc(alias = "CFG0_PADCONFIG139")]
pub type Cfg0Padconfig139 = crate::Reg<cfg0_padconfig139::Cfg0Padconfig139Spec>;
#[doc = "CFG0_PADCONFIG139"]
pub mod cfg0_padconfig139;
#[doc = "CFG0_PADCONFIG140 (rw) register accessor: CFG0_PADCONFIG140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig140`]
module"]
#[doc(alias = "CFG0_PADCONFIG140")]
pub type Cfg0Padconfig140 = crate::Reg<cfg0_padconfig140::Cfg0Padconfig140Spec>;
#[doc = "CFG0_PADCONFIG140"]
pub mod cfg0_padconfig140;
#[doc = "CFG0_PADCONFIG141 (rw) register accessor: CFG0_PADCONFIG141\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig141::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig141::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig141`]
module"]
#[doc(alias = "CFG0_PADCONFIG141")]
pub type Cfg0Padconfig141 = crate::Reg<cfg0_padconfig141::Cfg0Padconfig141Spec>;
#[doc = "CFG0_PADCONFIG141"]
pub mod cfg0_padconfig141;
#[doc = "CFG0_PADCONFIG142 (rw) register accessor: CFG0_PADCONFIG142\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig142::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig142::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig142`]
module"]
#[doc(alias = "CFG0_PADCONFIG142")]
pub type Cfg0Padconfig142 = crate::Reg<cfg0_padconfig142::Cfg0Padconfig142Spec>;
#[doc = "CFG0_PADCONFIG142"]
pub mod cfg0_padconfig142;
#[doc = "CFG0_PADCONFIG143 (rw) register accessor: CFG0_PADCONFIG143\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig143::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig143::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig143`]
module"]
#[doc(alias = "CFG0_PADCONFIG143")]
pub type Cfg0Padconfig143 = crate::Reg<cfg0_padconfig143::Cfg0Padconfig143Spec>;
#[doc = "CFG0_PADCONFIG143"]
pub mod cfg0_padconfig143;
#[doc = "CFG0_PADCONFIG144 (rw) register accessor: CFG0_PADCONFIG144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig144::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig144`]
module"]
#[doc(alias = "CFG0_PADCONFIG144")]
pub type Cfg0Padconfig144 = crate::Reg<cfg0_padconfig144::Cfg0Padconfig144Spec>;
#[doc = "CFG0_PADCONFIG144"]
pub mod cfg0_padconfig144;
#[doc = "CFG0_PADCONFIG145 (rw) register accessor: CFG0_PADCONFIG145\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig145::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig145::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig145`]
module"]
#[doc(alias = "CFG0_PADCONFIG145")]
pub type Cfg0Padconfig145 = crate::Reg<cfg0_padconfig145::Cfg0Padconfig145Spec>;
#[doc = "CFG0_PADCONFIG145"]
pub mod cfg0_padconfig145;
#[doc = "CFG0_PADCONFIG146 (rw) register accessor: CFG0_PADCONFIG146\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig146::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig146::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig146`]
module"]
#[doc(alias = "CFG0_PADCONFIG146")]
pub type Cfg0Padconfig146 = crate::Reg<cfg0_padconfig146::Cfg0Padconfig146Spec>;
#[doc = "CFG0_PADCONFIG146"]
pub mod cfg0_padconfig146;
#[doc = "CFG0_PADCONFIG147 (rw) register accessor: CFG0_PADCONFIG147\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig147::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig147::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig147`]
module"]
#[doc(alias = "CFG0_PADCONFIG147")]
pub type Cfg0Padconfig147 = crate::Reg<cfg0_padconfig147::Cfg0Padconfig147Spec>;
#[doc = "CFG0_PADCONFIG147"]
pub mod cfg0_padconfig147;
#[doc = "CFG0_PADCONFIG148 (rw) register accessor: CFG0_PADCONFIG148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig148::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig148`]
module"]
#[doc(alias = "CFG0_PADCONFIG148")]
pub type Cfg0Padconfig148 = crate::Reg<cfg0_padconfig148::Cfg0Padconfig148Spec>;
#[doc = "CFG0_PADCONFIG148"]
pub mod cfg0_padconfig148;
#[doc = "CFG0_PADCONFIG149 (rw) register accessor: CFG0_PADCONFIG149\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig149::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig149::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig149`]
module"]
#[doc(alias = "CFG0_PADCONFIG149")]
pub type Cfg0Padconfig149 = crate::Reg<cfg0_padconfig149::Cfg0Padconfig149Spec>;
#[doc = "CFG0_PADCONFIG149"]
pub mod cfg0_padconfig149;
#[doc = "CFG0_PADCONFIG150 (rw) register accessor: CFG0_PADCONFIG150\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig150::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig150::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig150`]
module"]
#[doc(alias = "CFG0_PADCONFIG150")]
pub type Cfg0Padconfig150 = crate::Reg<cfg0_padconfig150::Cfg0Padconfig150Spec>;
#[doc = "CFG0_PADCONFIG150"]
pub mod cfg0_padconfig150;
#[doc = "CFG0_PADCONFIG151 (rw) register accessor: CFG0_PADCONFIG151\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig151::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig151::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig151`]
module"]
#[doc(alias = "CFG0_PADCONFIG151")]
pub type Cfg0Padconfig151 = crate::Reg<cfg0_padconfig151::Cfg0Padconfig151Spec>;
#[doc = "CFG0_PADCONFIG151"]
pub mod cfg0_padconfig151;
#[doc = "CFG0_PADCONFIG152 (rw) register accessor: CFG0_PADCONFIG152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig152::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig152`]
module"]
#[doc(alias = "CFG0_PADCONFIG152")]
pub type Cfg0Padconfig152 = crate::Reg<cfg0_padconfig152::Cfg0Padconfig152Spec>;
#[doc = "CFG0_PADCONFIG152"]
pub mod cfg0_padconfig152;
#[doc = "CFG0_PADCONFIG153 (rw) register accessor: CFG0_PADCONFIG153\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig153::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig153::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig153`]
module"]
#[doc(alias = "CFG0_PADCONFIG153")]
pub type Cfg0Padconfig153 = crate::Reg<cfg0_padconfig153::Cfg0Padconfig153Spec>;
#[doc = "CFG0_PADCONFIG153"]
pub mod cfg0_padconfig153;
#[doc = "CFG0_PADCONFIG154 (rw) register accessor: CFG0_PADCONFIG154\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig154::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig154::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig154`]
module"]
#[doc(alias = "CFG0_PADCONFIG154")]
pub type Cfg0Padconfig154 = crate::Reg<cfg0_padconfig154::Cfg0Padconfig154Spec>;
#[doc = "CFG0_PADCONFIG154"]
pub mod cfg0_padconfig154;
#[doc = "CFG0_PADCONFIG155 (rw) register accessor: CFG0_PADCONFIG155\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig155`]
module"]
#[doc(alias = "CFG0_PADCONFIG155")]
pub type Cfg0Padconfig155 = crate::Reg<cfg0_padconfig155::Cfg0Padconfig155Spec>;
#[doc = "CFG0_PADCONFIG155"]
pub mod cfg0_padconfig155;
#[doc = "CFG0_PADCONFIG156 (rw) register accessor: CFG0_PADCONFIG156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig156`]
module"]
#[doc(alias = "CFG0_PADCONFIG156")]
pub type Cfg0Padconfig156 = crate::Reg<cfg0_padconfig156::Cfg0Padconfig156Spec>;
#[doc = "CFG0_PADCONFIG156"]
pub mod cfg0_padconfig156;
#[doc = "CFG0_PADCONFIG157 (rw) register accessor: CFG0_PADCONFIG157\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig157`]
module"]
#[doc(alias = "CFG0_PADCONFIG157")]
pub type Cfg0Padconfig157 = crate::Reg<cfg0_padconfig157::Cfg0Padconfig157Spec>;
#[doc = "CFG0_PADCONFIG157"]
pub mod cfg0_padconfig157;
#[doc = "CFG0_PADCONFIG158 (rw) register accessor: CFG0_PADCONFIG158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig158`]
module"]
#[doc(alias = "CFG0_PADCONFIG158")]
pub type Cfg0Padconfig158 = crate::Reg<cfg0_padconfig158::Cfg0Padconfig158Spec>;
#[doc = "CFG0_PADCONFIG158"]
pub mod cfg0_padconfig158;
#[doc = "CFG0_PADCONFIG159 (rw) register accessor: CFG0_PADCONFIG159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig159::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig159`]
module"]
#[doc(alias = "CFG0_PADCONFIG159")]
pub type Cfg0Padconfig159 = crate::Reg<cfg0_padconfig159::Cfg0Padconfig159Spec>;
#[doc = "CFG0_PADCONFIG159"]
pub mod cfg0_padconfig159;
#[doc = "CFG0_PADCONFIG160 (rw) register accessor: CFG0_PADCONFIG160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig160::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig160`]
module"]
#[doc(alias = "CFG0_PADCONFIG160")]
pub type Cfg0Padconfig160 = crate::Reg<cfg0_padconfig160::Cfg0Padconfig160Spec>;
#[doc = "CFG0_PADCONFIG160"]
pub mod cfg0_padconfig160;
#[doc = "CFG0_PADCONFIG161 (rw) register accessor: CFG0_PADCONFIG161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig161::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig161::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig161`]
module"]
#[doc(alias = "CFG0_PADCONFIG161")]
pub type Cfg0Padconfig161 = crate::Reg<cfg0_padconfig161::Cfg0Padconfig161Spec>;
#[doc = "CFG0_PADCONFIG161"]
pub mod cfg0_padconfig161;
#[doc = "CFG0_PADCONFIG162 (rw) register accessor: CFG0_PADCONFIG162\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig162::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig162::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig162`]
module"]
#[doc(alias = "CFG0_PADCONFIG162")]
pub type Cfg0Padconfig162 = crate::Reg<cfg0_padconfig162::Cfg0Padconfig162Spec>;
#[doc = "CFG0_PADCONFIG162"]
pub mod cfg0_padconfig162;
#[doc = "CFG0_PADCONFIG163 (rw) register accessor: CFG0_PADCONFIG163\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig163::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig163::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig163`]
module"]
#[doc(alias = "CFG0_PADCONFIG163")]
pub type Cfg0Padconfig163 = crate::Reg<cfg0_padconfig163::Cfg0Padconfig163Spec>;
#[doc = "CFG0_PADCONFIG163"]
pub mod cfg0_padconfig163;
#[doc = "CFG0_PADCONFIG164 (rw) register accessor: CFG0_PADCONFIG164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig164::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig164::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig164`]
module"]
#[doc(alias = "CFG0_PADCONFIG164")]
pub type Cfg0Padconfig164 = crate::Reg<cfg0_padconfig164::Cfg0Padconfig164Spec>;
#[doc = "CFG0_PADCONFIG164"]
pub mod cfg0_padconfig164;
#[doc = "CFG0_PADCONFIG165 (rw) register accessor: CFG0_PADCONFIG165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig165::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig165::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig165`]
module"]
#[doc(alias = "CFG0_PADCONFIG165")]
pub type Cfg0Padconfig165 = crate::Reg<cfg0_padconfig165::Cfg0Padconfig165Spec>;
#[doc = "CFG0_PADCONFIG165"]
pub mod cfg0_padconfig165;
#[doc = "CFG0_PADCONFIG166 (rw) register accessor: CFG0_PADCONFIG166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig166::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig166::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig166`]
module"]
#[doc(alias = "CFG0_PADCONFIG166")]
pub type Cfg0Padconfig166 = crate::Reg<cfg0_padconfig166::Cfg0Padconfig166Spec>;
#[doc = "CFG0_PADCONFIG166"]
pub mod cfg0_padconfig166;
#[doc = "CFG0_PADCONFIG167 (rw) register accessor: CFG0_PADCONFIG167\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig167::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig167::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig167`]
module"]
#[doc(alias = "CFG0_PADCONFIG167")]
pub type Cfg0Padconfig167 = crate::Reg<cfg0_padconfig167::Cfg0Padconfig167Spec>;
#[doc = "CFG0_PADCONFIG167"]
pub mod cfg0_padconfig167;
#[doc = "CFG0_PADCONFIG168 (rw) register accessor: CFG0_PADCONFIG168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig168::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig168::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig168`]
module"]
#[doc(alias = "CFG0_PADCONFIG168")]
pub type Cfg0Padconfig168 = crate::Reg<cfg0_padconfig168::Cfg0Padconfig168Spec>;
#[doc = "CFG0_PADCONFIG168"]
pub mod cfg0_padconfig168;
#[doc = "CFG0_PADCONFIG169 (rw) register accessor: CFG0_PADCONFIG169\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig169::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig169::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig169`]
module"]
#[doc(alias = "CFG0_PADCONFIG169")]
pub type Cfg0Padconfig169 = crate::Reg<cfg0_padconfig169::Cfg0Padconfig169Spec>;
#[doc = "CFG0_PADCONFIG169"]
pub mod cfg0_padconfig169;
#[doc = "CFG0_PADCONFIG170 (rw) register accessor: CFG0_PADCONFIG170\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig170::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig170::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig170`]
module"]
#[doc(alias = "CFG0_PADCONFIG170")]
pub type Cfg0Padconfig170 = crate::Reg<cfg0_padconfig170::Cfg0Padconfig170Spec>;
#[doc = "CFG0_PADCONFIG170"]
pub mod cfg0_padconfig170;
#[doc = "CFG0_PADCONFIG171 (rw) register accessor: CFG0_PADCONFIG171\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig171::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig171::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig171`]
module"]
#[doc(alias = "CFG0_PADCONFIG171")]
pub type Cfg0Padconfig171 = crate::Reg<cfg0_padconfig171::Cfg0Padconfig171Spec>;
#[doc = "CFG0_PADCONFIG171"]
pub mod cfg0_padconfig171;
#[doc = "CFG0_PADCONFIG172 (rw) register accessor: CFG0_PADCONFIG172\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig172::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig172::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig172`]
module"]
#[doc(alias = "CFG0_PADCONFIG172")]
pub type Cfg0Padconfig172 = crate::Reg<cfg0_padconfig172::Cfg0Padconfig172Spec>;
#[doc = "CFG0_PADCONFIG172"]
pub mod cfg0_padconfig172;
#[doc = "CFG0_PADCONFIG173 (rw) register accessor: CFG0_PADCONFIG173\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig173::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig173::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig173`]
module"]
#[doc(alias = "CFG0_PADCONFIG173")]
pub type Cfg0Padconfig173 = crate::Reg<cfg0_padconfig173::Cfg0Padconfig173Spec>;
#[doc = "CFG0_PADCONFIG173"]
pub mod cfg0_padconfig173;
#[doc = "CFG0_PADCONFIG174 (rw) register accessor: CFG0_PADCONFIG174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig174::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig174::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig174`]
module"]
#[doc(alias = "CFG0_PADCONFIG174")]
pub type Cfg0Padconfig174 = crate::Reg<cfg0_padconfig174::Cfg0Padconfig174Spec>;
#[doc = "CFG0_PADCONFIG174"]
pub mod cfg0_padconfig174;
#[doc = "CFG0_PADCONFIG175 (rw) register accessor: CFG0_PADCONFIG175\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig175::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig175::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig175`]
module"]
#[doc(alias = "CFG0_PADCONFIG175")]
pub type Cfg0Padconfig175 = crate::Reg<cfg0_padconfig175::Cfg0Padconfig175Spec>;
#[doc = "CFG0_PADCONFIG175"]
pub mod cfg0_padconfig175;
#[doc = "CFG0_PADCONFIG176 (rw) register accessor: CFG0_PADCONFIG176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig176::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig176::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig176`]
module"]
#[doc(alias = "CFG0_PADCONFIG176")]
pub type Cfg0Padconfig176 = crate::Reg<cfg0_padconfig176::Cfg0Padconfig176Spec>;
#[doc = "CFG0_PADCONFIG176"]
pub mod cfg0_padconfig176;
#[doc = "CFG0_PADCONFIG177 (rw) register accessor: CFG0_PADCONFIG177\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig177::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig177::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig177`]
module"]
#[doc(alias = "CFG0_PADCONFIG177")]
pub type Cfg0Padconfig177 = crate::Reg<cfg0_padconfig177::Cfg0Padconfig177Spec>;
#[doc = "CFG0_PADCONFIG177"]
pub mod cfg0_padconfig177;
#[doc = "CFG0_PADCONFIG178 (rw) register accessor: CFG0_PADCONFIG178\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig178::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig178::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig178`]
module"]
#[doc(alias = "CFG0_PADCONFIG178")]
pub type Cfg0Padconfig178 = crate::Reg<cfg0_padconfig178::Cfg0Padconfig178Spec>;
#[doc = "CFG0_PADCONFIG178"]
pub mod cfg0_padconfig178;
#[doc = "CFG0_PADCONFIG179 (rw) register accessor: CFG0_PADCONFIG179\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig179::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig179::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig179`]
module"]
#[doc(alias = "CFG0_PADCONFIG179")]
pub type Cfg0Padconfig179 = crate::Reg<cfg0_padconfig179::Cfg0Padconfig179Spec>;
#[doc = "CFG0_PADCONFIG179"]
pub mod cfg0_padconfig179;
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
#[doc = "CFG0_PADCONFIG33_PROXY (rw) register accessor: CFG0_PADCONFIG33_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig33_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig33_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig33_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG33_PROXY")]
pub type Cfg0Padconfig33Proxy = crate::Reg<cfg0_padconfig33_proxy::Cfg0Padconfig33ProxySpec>;
#[doc = "CFG0_PADCONFIG33_PROXY"]
pub mod cfg0_padconfig33_proxy;
#[doc = "CFG0_PADCONFIG34_PROXY (rw) register accessor: CFG0_PADCONFIG34_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig34_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig34_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig34_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG34_PROXY")]
pub type Cfg0Padconfig34Proxy = crate::Reg<cfg0_padconfig34_proxy::Cfg0Padconfig34ProxySpec>;
#[doc = "CFG0_PADCONFIG34_PROXY"]
pub mod cfg0_padconfig34_proxy;
#[doc = "CFG0_PADCONFIG35_PROXY (rw) register accessor: CFG0_PADCONFIG35_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig35_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig35_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig35_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG35_PROXY")]
pub type Cfg0Padconfig35Proxy = crate::Reg<cfg0_padconfig35_proxy::Cfg0Padconfig35ProxySpec>;
#[doc = "CFG0_PADCONFIG35_PROXY"]
pub mod cfg0_padconfig35_proxy;
#[doc = "CFG0_PADCONFIG36_PROXY (rw) register accessor: CFG0_PADCONFIG36_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig36_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig36_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig36_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG36_PROXY")]
pub type Cfg0Padconfig36Proxy = crate::Reg<cfg0_padconfig36_proxy::Cfg0Padconfig36ProxySpec>;
#[doc = "CFG0_PADCONFIG36_PROXY"]
pub mod cfg0_padconfig36_proxy;
#[doc = "CFG0_PADCONFIG37_PROXY (rw) register accessor: CFG0_PADCONFIG37_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig37_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig37_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig37_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG37_PROXY")]
pub type Cfg0Padconfig37Proxy = crate::Reg<cfg0_padconfig37_proxy::Cfg0Padconfig37ProxySpec>;
#[doc = "CFG0_PADCONFIG37_PROXY"]
pub mod cfg0_padconfig37_proxy;
#[doc = "CFG0_PADCONFIG38_PROXY (rw) register accessor: CFG0_PADCONFIG38_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig38_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig38_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig38_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG38_PROXY")]
pub type Cfg0Padconfig38Proxy = crate::Reg<cfg0_padconfig38_proxy::Cfg0Padconfig38ProxySpec>;
#[doc = "CFG0_PADCONFIG38_PROXY"]
pub mod cfg0_padconfig38_proxy;
#[doc = "CFG0_PADCONFIG39_PROXY (rw) register accessor: CFG0_PADCONFIG39_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig39_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig39_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig39_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG39_PROXY")]
pub type Cfg0Padconfig39Proxy = crate::Reg<cfg0_padconfig39_proxy::Cfg0Padconfig39ProxySpec>;
#[doc = "CFG0_PADCONFIG39_PROXY"]
pub mod cfg0_padconfig39_proxy;
#[doc = "CFG0_PADCONFIG40_PROXY (rw) register accessor: CFG0_PADCONFIG40_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig40_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig40_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig40_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG40_PROXY")]
pub type Cfg0Padconfig40Proxy = crate::Reg<cfg0_padconfig40_proxy::Cfg0Padconfig40ProxySpec>;
#[doc = "CFG0_PADCONFIG40_PROXY"]
pub mod cfg0_padconfig40_proxy;
#[doc = "CFG0_PADCONFIG41_PROXY (rw) register accessor: CFG0_PADCONFIG41_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig41_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig41_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig41_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG41_PROXY")]
pub type Cfg0Padconfig41Proxy = crate::Reg<cfg0_padconfig41_proxy::Cfg0Padconfig41ProxySpec>;
#[doc = "CFG0_PADCONFIG41_PROXY"]
pub mod cfg0_padconfig41_proxy;
#[doc = "CFG0_PADCONFIG42_PROXY (rw) register accessor: CFG0_PADCONFIG42_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig42_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig42_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig42_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG42_PROXY")]
pub type Cfg0Padconfig42Proxy = crate::Reg<cfg0_padconfig42_proxy::Cfg0Padconfig42ProxySpec>;
#[doc = "CFG0_PADCONFIG42_PROXY"]
pub mod cfg0_padconfig42_proxy;
#[doc = "CFG0_PADCONFIG43_PROXY (rw) register accessor: CFG0_PADCONFIG43_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig43_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig43_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig43_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG43_PROXY")]
pub type Cfg0Padconfig43Proxy = crate::Reg<cfg0_padconfig43_proxy::Cfg0Padconfig43ProxySpec>;
#[doc = "CFG0_PADCONFIG43_PROXY"]
pub mod cfg0_padconfig43_proxy;
#[doc = "CFG0_PADCONFIG44_PROXY (rw) register accessor: CFG0_PADCONFIG44_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig44_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig44_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig44_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG44_PROXY")]
pub type Cfg0Padconfig44Proxy = crate::Reg<cfg0_padconfig44_proxy::Cfg0Padconfig44ProxySpec>;
#[doc = "CFG0_PADCONFIG44_PROXY"]
pub mod cfg0_padconfig44_proxy;
#[doc = "CFG0_PADCONFIG45_PROXY (rw) register accessor: CFG0_PADCONFIG45_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig45_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig45_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig45_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG45_PROXY")]
pub type Cfg0Padconfig45Proxy = crate::Reg<cfg0_padconfig45_proxy::Cfg0Padconfig45ProxySpec>;
#[doc = "CFG0_PADCONFIG45_PROXY"]
pub mod cfg0_padconfig45_proxy;
#[doc = "CFG0_PADCONFIG46_PROXY (rw) register accessor: CFG0_PADCONFIG46_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig46_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig46_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig46_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG46_PROXY")]
pub type Cfg0Padconfig46Proxy = crate::Reg<cfg0_padconfig46_proxy::Cfg0Padconfig46ProxySpec>;
#[doc = "CFG0_PADCONFIG46_PROXY"]
pub mod cfg0_padconfig46_proxy;
#[doc = "CFG0_PADCONFIG47_PROXY (rw) register accessor: CFG0_PADCONFIG47_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig47_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig47_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig47_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG47_PROXY")]
pub type Cfg0Padconfig47Proxy = crate::Reg<cfg0_padconfig47_proxy::Cfg0Padconfig47ProxySpec>;
#[doc = "CFG0_PADCONFIG47_PROXY"]
pub mod cfg0_padconfig47_proxy;
#[doc = "CFG0_PADCONFIG48_PROXY (rw) register accessor: CFG0_PADCONFIG48_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig48_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig48_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig48_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG48_PROXY")]
pub type Cfg0Padconfig48Proxy = crate::Reg<cfg0_padconfig48_proxy::Cfg0Padconfig48ProxySpec>;
#[doc = "CFG0_PADCONFIG48_PROXY"]
pub mod cfg0_padconfig48_proxy;
#[doc = "CFG0_PADCONFIG49_PROXY (rw) register accessor: CFG0_PADCONFIG49_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig49_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig49_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig49_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG49_PROXY")]
pub type Cfg0Padconfig49Proxy = crate::Reg<cfg0_padconfig49_proxy::Cfg0Padconfig49ProxySpec>;
#[doc = "CFG0_PADCONFIG49_PROXY"]
pub mod cfg0_padconfig49_proxy;
#[doc = "CFG0_PADCONFIG50_PROXY (rw) register accessor: CFG0_PADCONFIG50_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig50_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig50_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig50_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG50_PROXY")]
pub type Cfg0Padconfig50Proxy = crate::Reg<cfg0_padconfig50_proxy::Cfg0Padconfig50ProxySpec>;
#[doc = "CFG0_PADCONFIG50_PROXY"]
pub mod cfg0_padconfig50_proxy;
#[doc = "CFG0_PADCONFIG51_PROXY (rw) register accessor: CFG0_PADCONFIG51_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig51_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig51_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig51_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG51_PROXY")]
pub type Cfg0Padconfig51Proxy = crate::Reg<cfg0_padconfig51_proxy::Cfg0Padconfig51ProxySpec>;
#[doc = "CFG0_PADCONFIG51_PROXY"]
pub mod cfg0_padconfig51_proxy;
#[doc = "CFG0_PADCONFIG52_PROXY (rw) register accessor: CFG0_PADCONFIG52_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig52_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig52_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig52_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG52_PROXY")]
pub type Cfg0Padconfig52Proxy = crate::Reg<cfg0_padconfig52_proxy::Cfg0Padconfig52ProxySpec>;
#[doc = "CFG0_PADCONFIG52_PROXY"]
pub mod cfg0_padconfig52_proxy;
#[doc = "CFG0_PADCONFIG53_PROXY (rw) register accessor: CFG0_PADCONFIG53_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig53_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig53_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig53_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG53_PROXY")]
pub type Cfg0Padconfig53Proxy = crate::Reg<cfg0_padconfig53_proxy::Cfg0Padconfig53ProxySpec>;
#[doc = "CFG0_PADCONFIG53_PROXY"]
pub mod cfg0_padconfig53_proxy;
#[doc = "CFG0_PADCONFIG54_PROXY (rw) register accessor: CFG0_PADCONFIG54_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig54_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig54_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig54_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG54_PROXY")]
pub type Cfg0Padconfig54Proxy = crate::Reg<cfg0_padconfig54_proxy::Cfg0Padconfig54ProxySpec>;
#[doc = "CFG0_PADCONFIG54_PROXY"]
pub mod cfg0_padconfig54_proxy;
#[doc = "CFG0_PADCONFIG55_PROXY (rw) register accessor: CFG0_PADCONFIG55_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig55_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig55_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig55_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG55_PROXY")]
pub type Cfg0Padconfig55Proxy = crate::Reg<cfg0_padconfig55_proxy::Cfg0Padconfig55ProxySpec>;
#[doc = "CFG0_PADCONFIG55_PROXY"]
pub mod cfg0_padconfig55_proxy;
#[doc = "CFG0_PADCONFIG56_PROXY (rw) register accessor: CFG0_PADCONFIG56_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig56_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig56_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig56_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG56_PROXY")]
pub type Cfg0Padconfig56Proxy = crate::Reg<cfg0_padconfig56_proxy::Cfg0Padconfig56ProxySpec>;
#[doc = "CFG0_PADCONFIG56_PROXY"]
pub mod cfg0_padconfig56_proxy;
#[doc = "CFG0_PADCONFIG57_PROXY (rw) register accessor: CFG0_PADCONFIG57_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig57_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig57_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig57_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG57_PROXY")]
pub type Cfg0Padconfig57Proxy = crate::Reg<cfg0_padconfig57_proxy::Cfg0Padconfig57ProxySpec>;
#[doc = "CFG0_PADCONFIG57_PROXY"]
pub mod cfg0_padconfig57_proxy;
#[doc = "CFG0_PADCONFIG58_PROXY (rw) register accessor: CFG0_PADCONFIG58_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig58_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig58_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig58_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG58_PROXY")]
pub type Cfg0Padconfig58Proxy = crate::Reg<cfg0_padconfig58_proxy::Cfg0Padconfig58ProxySpec>;
#[doc = "CFG0_PADCONFIG58_PROXY"]
pub mod cfg0_padconfig58_proxy;
#[doc = "CFG0_PADCONFIG59_PROXY (rw) register accessor: CFG0_PADCONFIG59_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig59_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig59_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig59_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG59_PROXY")]
pub type Cfg0Padconfig59Proxy = crate::Reg<cfg0_padconfig59_proxy::Cfg0Padconfig59ProxySpec>;
#[doc = "CFG0_PADCONFIG59_PROXY"]
pub mod cfg0_padconfig59_proxy;
#[doc = "CFG0_PADCONFIG60_PROXY (rw) register accessor: CFG0_PADCONFIG60_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig60_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig60_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig60_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG60_PROXY")]
pub type Cfg0Padconfig60Proxy = crate::Reg<cfg0_padconfig60_proxy::Cfg0Padconfig60ProxySpec>;
#[doc = "CFG0_PADCONFIG60_PROXY"]
pub mod cfg0_padconfig60_proxy;
#[doc = "CFG0_PADCONFIG61_PROXY (rw) register accessor: CFG0_PADCONFIG61_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig61_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig61_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig61_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG61_PROXY")]
pub type Cfg0Padconfig61Proxy = crate::Reg<cfg0_padconfig61_proxy::Cfg0Padconfig61ProxySpec>;
#[doc = "CFG0_PADCONFIG61_PROXY"]
pub mod cfg0_padconfig61_proxy;
#[doc = "CFG0_PADCONFIG62_PROXY (rw) register accessor: CFG0_PADCONFIG62_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig62_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig62_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig62_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG62_PROXY")]
pub type Cfg0Padconfig62Proxy = crate::Reg<cfg0_padconfig62_proxy::Cfg0Padconfig62ProxySpec>;
#[doc = "CFG0_PADCONFIG62_PROXY"]
pub mod cfg0_padconfig62_proxy;
#[doc = "CFG0_PADCONFIG63_PROXY (rw) register accessor: CFG0_PADCONFIG63_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig63_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig63_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig63_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG63_PROXY")]
pub type Cfg0Padconfig63Proxy = crate::Reg<cfg0_padconfig63_proxy::Cfg0Padconfig63ProxySpec>;
#[doc = "CFG0_PADCONFIG63_PROXY"]
pub mod cfg0_padconfig63_proxy;
#[doc = "CFG0_PADCONFIG64_PROXY (rw) register accessor: CFG0_PADCONFIG64_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig64_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig64_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig64_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG64_PROXY")]
pub type Cfg0Padconfig64Proxy = crate::Reg<cfg0_padconfig64_proxy::Cfg0Padconfig64ProxySpec>;
#[doc = "CFG0_PADCONFIG64_PROXY"]
pub mod cfg0_padconfig64_proxy;
#[doc = "CFG0_PADCONFIG65_PROXY (rw) register accessor: CFG0_PADCONFIG65_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig65_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig65_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig65_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG65_PROXY")]
pub type Cfg0Padconfig65Proxy = crate::Reg<cfg0_padconfig65_proxy::Cfg0Padconfig65ProxySpec>;
#[doc = "CFG0_PADCONFIG65_PROXY"]
pub mod cfg0_padconfig65_proxy;
#[doc = "CFG0_PADCONFIG66_PROXY (rw) register accessor: CFG0_PADCONFIG66_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig66_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig66_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig66_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG66_PROXY")]
pub type Cfg0Padconfig66Proxy = crate::Reg<cfg0_padconfig66_proxy::Cfg0Padconfig66ProxySpec>;
#[doc = "CFG0_PADCONFIG66_PROXY"]
pub mod cfg0_padconfig66_proxy;
#[doc = "CFG0_PADCONFIG67_PROXY (rw) register accessor: CFG0_PADCONFIG67_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig67_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig67_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig67_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG67_PROXY")]
pub type Cfg0Padconfig67Proxy = crate::Reg<cfg0_padconfig67_proxy::Cfg0Padconfig67ProxySpec>;
#[doc = "CFG0_PADCONFIG67_PROXY"]
pub mod cfg0_padconfig67_proxy;
#[doc = "CFG0_PADCONFIG68_PROXY (rw) register accessor: CFG0_PADCONFIG68_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig68_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig68_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig68_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG68_PROXY")]
pub type Cfg0Padconfig68Proxy = crate::Reg<cfg0_padconfig68_proxy::Cfg0Padconfig68ProxySpec>;
#[doc = "CFG0_PADCONFIG68_PROXY"]
pub mod cfg0_padconfig68_proxy;
#[doc = "CFG0_PADCONFIG69_PROXY (rw) register accessor: CFG0_PADCONFIG69_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig69_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig69_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig69_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG69_PROXY")]
pub type Cfg0Padconfig69Proxy = crate::Reg<cfg0_padconfig69_proxy::Cfg0Padconfig69ProxySpec>;
#[doc = "CFG0_PADCONFIG69_PROXY"]
pub mod cfg0_padconfig69_proxy;
#[doc = "CFG0_PADCONFIG70_PROXY (rw) register accessor: CFG0_PADCONFIG70_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig70_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig70_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig70_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG70_PROXY")]
pub type Cfg0Padconfig70Proxy = crate::Reg<cfg0_padconfig70_proxy::Cfg0Padconfig70ProxySpec>;
#[doc = "CFG0_PADCONFIG70_PROXY"]
pub mod cfg0_padconfig70_proxy;
#[doc = "CFG0_PADCONFIG71_PROXY (rw) register accessor: CFG0_PADCONFIG71_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig71_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig71_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig71_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG71_PROXY")]
pub type Cfg0Padconfig71Proxy = crate::Reg<cfg0_padconfig71_proxy::Cfg0Padconfig71ProxySpec>;
#[doc = "CFG0_PADCONFIG71_PROXY"]
pub mod cfg0_padconfig71_proxy;
#[doc = "CFG0_PADCONFIG72_PROXY (rw) register accessor: CFG0_PADCONFIG72_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig72_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig72_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig72_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG72_PROXY")]
pub type Cfg0Padconfig72Proxy = crate::Reg<cfg0_padconfig72_proxy::Cfg0Padconfig72ProxySpec>;
#[doc = "CFG0_PADCONFIG72_PROXY"]
pub mod cfg0_padconfig72_proxy;
#[doc = "CFG0_PADCONFIG73_PROXY (rw) register accessor: CFG0_PADCONFIG73_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig73_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig73_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig73_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG73_PROXY")]
pub type Cfg0Padconfig73Proxy = crate::Reg<cfg0_padconfig73_proxy::Cfg0Padconfig73ProxySpec>;
#[doc = "CFG0_PADCONFIG73_PROXY"]
pub mod cfg0_padconfig73_proxy;
#[doc = "CFG0_PADCONFIG74_PROXY (rw) register accessor: CFG0_PADCONFIG74_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig74_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig74_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig74_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG74_PROXY")]
pub type Cfg0Padconfig74Proxy = crate::Reg<cfg0_padconfig74_proxy::Cfg0Padconfig74ProxySpec>;
#[doc = "CFG0_PADCONFIG74_PROXY"]
pub mod cfg0_padconfig74_proxy;
#[doc = "CFG0_PADCONFIG75_PROXY (rw) register accessor: CFG0_PADCONFIG75_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig75_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig75_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig75_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG75_PROXY")]
pub type Cfg0Padconfig75Proxy = crate::Reg<cfg0_padconfig75_proxy::Cfg0Padconfig75ProxySpec>;
#[doc = "CFG0_PADCONFIG75_PROXY"]
pub mod cfg0_padconfig75_proxy;
#[doc = "CFG0_PADCONFIG76_PROXY (rw) register accessor: CFG0_PADCONFIG76_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig76_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig76_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig76_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG76_PROXY")]
pub type Cfg0Padconfig76Proxy = crate::Reg<cfg0_padconfig76_proxy::Cfg0Padconfig76ProxySpec>;
#[doc = "CFG0_PADCONFIG76_PROXY"]
pub mod cfg0_padconfig76_proxy;
#[doc = "CFG0_PADCONFIG77_PROXY (rw) register accessor: CFG0_PADCONFIG77_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig77_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig77_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig77_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG77_PROXY")]
pub type Cfg0Padconfig77Proxy = crate::Reg<cfg0_padconfig77_proxy::Cfg0Padconfig77ProxySpec>;
#[doc = "CFG0_PADCONFIG77_PROXY"]
pub mod cfg0_padconfig77_proxy;
#[doc = "CFG0_PADCONFIG78_PROXY (rw) register accessor: CFG0_PADCONFIG78_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig78_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig78_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig78_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG78_PROXY")]
pub type Cfg0Padconfig78Proxy = crate::Reg<cfg0_padconfig78_proxy::Cfg0Padconfig78ProxySpec>;
#[doc = "CFG0_PADCONFIG78_PROXY"]
pub mod cfg0_padconfig78_proxy;
#[doc = "CFG0_PADCONFIG79_PROXY (rw) register accessor: CFG0_PADCONFIG79_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig79_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig79_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig79_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG79_PROXY")]
pub type Cfg0Padconfig79Proxy = crate::Reg<cfg0_padconfig79_proxy::Cfg0Padconfig79ProxySpec>;
#[doc = "CFG0_PADCONFIG79_PROXY"]
pub mod cfg0_padconfig79_proxy;
#[doc = "CFG0_PADCONFIG80_PROXY (rw) register accessor: CFG0_PADCONFIG80_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig80_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig80_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig80_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG80_PROXY")]
pub type Cfg0Padconfig80Proxy = crate::Reg<cfg0_padconfig80_proxy::Cfg0Padconfig80ProxySpec>;
#[doc = "CFG0_PADCONFIG80_PROXY"]
pub mod cfg0_padconfig80_proxy;
#[doc = "CFG0_PADCONFIG81_PROXY (rw) register accessor: CFG0_PADCONFIG81_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig81_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig81_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig81_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG81_PROXY")]
pub type Cfg0Padconfig81Proxy = crate::Reg<cfg0_padconfig81_proxy::Cfg0Padconfig81ProxySpec>;
#[doc = "CFG0_PADCONFIG81_PROXY"]
pub mod cfg0_padconfig81_proxy;
#[doc = "CFG0_PADCONFIG82_PROXY (rw) register accessor: CFG0_PADCONFIG82_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig82_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig82_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig82_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG82_PROXY")]
pub type Cfg0Padconfig82Proxy = crate::Reg<cfg0_padconfig82_proxy::Cfg0Padconfig82ProxySpec>;
#[doc = "CFG0_PADCONFIG82_PROXY"]
pub mod cfg0_padconfig82_proxy;
#[doc = "CFG0_PADCONFIG83_PROXY (rw) register accessor: CFG0_PADCONFIG83_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig83_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig83_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig83_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG83_PROXY")]
pub type Cfg0Padconfig83Proxy = crate::Reg<cfg0_padconfig83_proxy::Cfg0Padconfig83ProxySpec>;
#[doc = "CFG0_PADCONFIG83_PROXY"]
pub mod cfg0_padconfig83_proxy;
#[doc = "CFG0_PADCONFIG84_PROXY (rw) register accessor: CFG0_PADCONFIG84_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig84_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig84_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig84_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG84_PROXY")]
pub type Cfg0Padconfig84Proxy = crate::Reg<cfg0_padconfig84_proxy::Cfg0Padconfig84ProxySpec>;
#[doc = "CFG0_PADCONFIG84_PROXY"]
pub mod cfg0_padconfig84_proxy;
#[doc = "CFG0_PADCONFIG85_PROXY (rw) register accessor: CFG0_PADCONFIG85_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig85_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig85_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig85_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG85_PROXY")]
pub type Cfg0Padconfig85Proxy = crate::Reg<cfg0_padconfig85_proxy::Cfg0Padconfig85ProxySpec>;
#[doc = "CFG0_PADCONFIG85_PROXY"]
pub mod cfg0_padconfig85_proxy;
#[doc = "CFG0_PADCONFIG86_PROXY (rw) register accessor: CFG0_PADCONFIG86_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig86_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig86_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig86_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG86_PROXY")]
pub type Cfg0Padconfig86Proxy = crate::Reg<cfg0_padconfig86_proxy::Cfg0Padconfig86ProxySpec>;
#[doc = "CFG0_PADCONFIG86_PROXY"]
pub mod cfg0_padconfig86_proxy;
#[doc = "CFG0_PADCONFIG87_PROXY (rw) register accessor: CFG0_PADCONFIG87_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig87_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig87_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig87_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG87_PROXY")]
pub type Cfg0Padconfig87Proxy = crate::Reg<cfg0_padconfig87_proxy::Cfg0Padconfig87ProxySpec>;
#[doc = "CFG0_PADCONFIG87_PROXY"]
pub mod cfg0_padconfig87_proxy;
#[doc = "CFG0_PADCONFIG88_PROXY (rw) register accessor: CFG0_PADCONFIG88_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig88_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig88_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig88_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG88_PROXY")]
pub type Cfg0Padconfig88Proxy = crate::Reg<cfg0_padconfig88_proxy::Cfg0Padconfig88ProxySpec>;
#[doc = "CFG0_PADCONFIG88_PROXY"]
pub mod cfg0_padconfig88_proxy;
#[doc = "CFG0_PADCONFIG89_PROXY (rw) register accessor: CFG0_PADCONFIG89_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig89_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig89_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig89_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG89_PROXY")]
pub type Cfg0Padconfig89Proxy = crate::Reg<cfg0_padconfig89_proxy::Cfg0Padconfig89ProxySpec>;
#[doc = "CFG0_PADCONFIG89_PROXY"]
pub mod cfg0_padconfig89_proxy;
#[doc = "CFG0_PADCONFIG90_PROXY (rw) register accessor: CFG0_PADCONFIG90_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig90_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig90_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig90_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG90_PROXY")]
pub type Cfg0Padconfig90Proxy = crate::Reg<cfg0_padconfig90_proxy::Cfg0Padconfig90ProxySpec>;
#[doc = "CFG0_PADCONFIG90_PROXY"]
pub mod cfg0_padconfig90_proxy;
#[doc = "CFG0_PADCONFIG91_PROXY (rw) register accessor: CFG0_PADCONFIG91_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig91_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig91_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig91_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG91_PROXY")]
pub type Cfg0Padconfig91Proxy = crate::Reg<cfg0_padconfig91_proxy::Cfg0Padconfig91ProxySpec>;
#[doc = "CFG0_PADCONFIG91_PROXY"]
pub mod cfg0_padconfig91_proxy;
#[doc = "CFG0_PADCONFIG92_PROXY (rw) register accessor: CFG0_PADCONFIG92_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig92_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig92_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig92_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG92_PROXY")]
pub type Cfg0Padconfig92Proxy = crate::Reg<cfg0_padconfig92_proxy::Cfg0Padconfig92ProxySpec>;
#[doc = "CFG0_PADCONFIG92_PROXY"]
pub mod cfg0_padconfig92_proxy;
#[doc = "CFG0_PADCONFIG93_PROXY (rw) register accessor: CFG0_PADCONFIG93_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig93_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig93_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig93_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG93_PROXY")]
pub type Cfg0Padconfig93Proxy = crate::Reg<cfg0_padconfig93_proxy::Cfg0Padconfig93ProxySpec>;
#[doc = "CFG0_PADCONFIG93_PROXY"]
pub mod cfg0_padconfig93_proxy;
#[doc = "CFG0_PADCONFIG94_PROXY (rw) register accessor: CFG0_PADCONFIG94_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig94_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig94_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig94_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG94_PROXY")]
pub type Cfg0Padconfig94Proxy = crate::Reg<cfg0_padconfig94_proxy::Cfg0Padconfig94ProxySpec>;
#[doc = "CFG0_PADCONFIG94_PROXY"]
pub mod cfg0_padconfig94_proxy;
#[doc = "CFG0_PADCONFIG95_PROXY (rw) register accessor: CFG0_PADCONFIG95_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig95_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig95_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig95_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG95_PROXY")]
pub type Cfg0Padconfig95Proxy = crate::Reg<cfg0_padconfig95_proxy::Cfg0Padconfig95ProxySpec>;
#[doc = "CFG0_PADCONFIG95_PROXY"]
pub mod cfg0_padconfig95_proxy;
#[doc = "CFG0_PADCONFIG96_PROXY (rw) register accessor: CFG0_PADCONFIG96_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig96_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig96_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig96_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG96_PROXY")]
pub type Cfg0Padconfig96Proxy = crate::Reg<cfg0_padconfig96_proxy::Cfg0Padconfig96ProxySpec>;
#[doc = "CFG0_PADCONFIG96_PROXY"]
pub mod cfg0_padconfig96_proxy;
#[doc = "CFG0_PADCONFIG97_PROXY (rw) register accessor: CFG0_PADCONFIG97_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig97_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig97_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig97_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG97_PROXY")]
pub type Cfg0Padconfig97Proxy = crate::Reg<cfg0_padconfig97_proxy::Cfg0Padconfig97ProxySpec>;
#[doc = "CFG0_PADCONFIG97_PROXY"]
pub mod cfg0_padconfig97_proxy;
#[doc = "CFG0_PADCONFIG98_PROXY (rw) register accessor: CFG0_PADCONFIG98_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig98_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig98_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig98_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG98_PROXY")]
pub type Cfg0Padconfig98Proxy = crate::Reg<cfg0_padconfig98_proxy::Cfg0Padconfig98ProxySpec>;
#[doc = "CFG0_PADCONFIG98_PROXY"]
pub mod cfg0_padconfig98_proxy;
#[doc = "CFG0_PADCONFIG99_PROXY (rw) register accessor: CFG0_PADCONFIG99_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig99_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig99_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig99_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG99_PROXY")]
pub type Cfg0Padconfig99Proxy = crate::Reg<cfg0_padconfig99_proxy::Cfg0Padconfig99ProxySpec>;
#[doc = "CFG0_PADCONFIG99_PROXY"]
pub mod cfg0_padconfig99_proxy;
#[doc = "CFG0_PADCONFIG100_PROXY (rw) register accessor: CFG0_PADCONFIG100_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig100_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig100_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig100_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG100_PROXY")]
pub type Cfg0Padconfig100Proxy = crate::Reg<cfg0_padconfig100_proxy::Cfg0Padconfig100ProxySpec>;
#[doc = "CFG0_PADCONFIG100_PROXY"]
pub mod cfg0_padconfig100_proxy;
#[doc = "CFG0_PADCONFIG101_PROXY (rw) register accessor: CFG0_PADCONFIG101_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig101_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig101_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig101_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG101_PROXY")]
pub type Cfg0Padconfig101Proxy = crate::Reg<cfg0_padconfig101_proxy::Cfg0Padconfig101ProxySpec>;
#[doc = "CFG0_PADCONFIG101_PROXY"]
pub mod cfg0_padconfig101_proxy;
#[doc = "CFG0_PADCONFIG102_PROXY (rw) register accessor: CFG0_PADCONFIG102_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig102_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig102_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig102_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG102_PROXY")]
pub type Cfg0Padconfig102Proxy = crate::Reg<cfg0_padconfig102_proxy::Cfg0Padconfig102ProxySpec>;
#[doc = "CFG0_PADCONFIG102_PROXY"]
pub mod cfg0_padconfig102_proxy;
#[doc = "CFG0_PADCONFIG103_PROXY (rw) register accessor: CFG0_PADCONFIG103_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig103_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig103_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig103_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG103_PROXY")]
pub type Cfg0Padconfig103Proxy = crate::Reg<cfg0_padconfig103_proxy::Cfg0Padconfig103ProxySpec>;
#[doc = "CFG0_PADCONFIG103_PROXY"]
pub mod cfg0_padconfig103_proxy;
#[doc = "CFG0_PADCONFIG104_PROXY (rw) register accessor: CFG0_PADCONFIG104_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig104_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig104_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig104_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG104_PROXY")]
pub type Cfg0Padconfig104Proxy = crate::Reg<cfg0_padconfig104_proxy::Cfg0Padconfig104ProxySpec>;
#[doc = "CFG0_PADCONFIG104_PROXY"]
pub mod cfg0_padconfig104_proxy;
#[doc = "CFG0_PADCONFIG105_PROXY (rw) register accessor: CFG0_PADCONFIG105_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig105_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig105_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig105_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG105_PROXY")]
pub type Cfg0Padconfig105Proxy = crate::Reg<cfg0_padconfig105_proxy::Cfg0Padconfig105ProxySpec>;
#[doc = "CFG0_PADCONFIG105_PROXY"]
pub mod cfg0_padconfig105_proxy;
#[doc = "CFG0_PADCONFIG106_PROXY (rw) register accessor: CFG0_PADCONFIG106_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig106_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig106_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig106_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG106_PROXY")]
pub type Cfg0Padconfig106Proxy = crate::Reg<cfg0_padconfig106_proxy::Cfg0Padconfig106ProxySpec>;
#[doc = "CFG0_PADCONFIG106_PROXY"]
pub mod cfg0_padconfig106_proxy;
#[doc = "CFG0_PADCONFIG107_PROXY (rw) register accessor: CFG0_PADCONFIG107_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig107_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig107_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig107_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG107_PROXY")]
pub type Cfg0Padconfig107Proxy = crate::Reg<cfg0_padconfig107_proxy::Cfg0Padconfig107ProxySpec>;
#[doc = "CFG0_PADCONFIG107_PROXY"]
pub mod cfg0_padconfig107_proxy;
#[doc = "CFG0_PADCONFIG108_PROXY (rw) register accessor: CFG0_PADCONFIG108_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig108_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig108_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig108_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG108_PROXY")]
pub type Cfg0Padconfig108Proxy = crate::Reg<cfg0_padconfig108_proxy::Cfg0Padconfig108ProxySpec>;
#[doc = "CFG0_PADCONFIG108_PROXY"]
pub mod cfg0_padconfig108_proxy;
#[doc = "CFG0_PADCONFIG109_PROXY (rw) register accessor: CFG0_PADCONFIG109_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig109_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig109_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig109_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG109_PROXY")]
pub type Cfg0Padconfig109Proxy = crate::Reg<cfg0_padconfig109_proxy::Cfg0Padconfig109ProxySpec>;
#[doc = "CFG0_PADCONFIG109_PROXY"]
pub mod cfg0_padconfig109_proxy;
#[doc = "CFG0_PADCONFIG110_PROXY (rw) register accessor: CFG0_PADCONFIG110_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig110_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig110_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig110_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG110_PROXY")]
pub type Cfg0Padconfig110Proxy = crate::Reg<cfg0_padconfig110_proxy::Cfg0Padconfig110ProxySpec>;
#[doc = "CFG0_PADCONFIG110_PROXY"]
pub mod cfg0_padconfig110_proxy;
#[doc = "CFG0_PADCONFIG111_PROXY (rw) register accessor: CFG0_PADCONFIG111_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig111_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig111_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig111_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG111_PROXY")]
pub type Cfg0Padconfig111Proxy = crate::Reg<cfg0_padconfig111_proxy::Cfg0Padconfig111ProxySpec>;
#[doc = "CFG0_PADCONFIG111_PROXY"]
pub mod cfg0_padconfig111_proxy;
#[doc = "CFG0_PADCONFIG112_PROXY (rw) register accessor: CFG0_PADCONFIG112_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig112_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig112_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig112_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG112_PROXY")]
pub type Cfg0Padconfig112Proxy = crate::Reg<cfg0_padconfig112_proxy::Cfg0Padconfig112ProxySpec>;
#[doc = "CFG0_PADCONFIG112_PROXY"]
pub mod cfg0_padconfig112_proxy;
#[doc = "CFG0_PADCONFIG113_PROXY (rw) register accessor: CFG0_PADCONFIG113_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig113_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig113_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig113_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG113_PROXY")]
pub type Cfg0Padconfig113Proxy = crate::Reg<cfg0_padconfig113_proxy::Cfg0Padconfig113ProxySpec>;
#[doc = "CFG0_PADCONFIG113_PROXY"]
pub mod cfg0_padconfig113_proxy;
#[doc = "CFG0_PADCONFIG114_PROXY (rw) register accessor: CFG0_PADCONFIG114_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig114_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig114_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig114_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG114_PROXY")]
pub type Cfg0Padconfig114Proxy = crate::Reg<cfg0_padconfig114_proxy::Cfg0Padconfig114ProxySpec>;
#[doc = "CFG0_PADCONFIG114_PROXY"]
pub mod cfg0_padconfig114_proxy;
#[doc = "CFG0_PADCONFIG115_PROXY (rw) register accessor: CFG0_PADCONFIG115_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig115_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig115_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig115_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG115_PROXY")]
pub type Cfg0Padconfig115Proxy = crate::Reg<cfg0_padconfig115_proxy::Cfg0Padconfig115ProxySpec>;
#[doc = "CFG0_PADCONFIG115_PROXY"]
pub mod cfg0_padconfig115_proxy;
#[doc = "CFG0_PADCONFIG116_PROXY (rw) register accessor: CFG0_PADCONFIG116_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig116_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig116_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig116_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG116_PROXY")]
pub type Cfg0Padconfig116Proxy = crate::Reg<cfg0_padconfig116_proxy::Cfg0Padconfig116ProxySpec>;
#[doc = "CFG0_PADCONFIG116_PROXY"]
pub mod cfg0_padconfig116_proxy;
#[doc = "CFG0_PADCONFIG117_PROXY (rw) register accessor: CFG0_PADCONFIG117_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig117_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig117_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig117_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG117_PROXY")]
pub type Cfg0Padconfig117Proxy = crate::Reg<cfg0_padconfig117_proxy::Cfg0Padconfig117ProxySpec>;
#[doc = "CFG0_PADCONFIG117_PROXY"]
pub mod cfg0_padconfig117_proxy;
#[doc = "CFG0_PADCONFIG118_PROXY (rw) register accessor: CFG0_PADCONFIG118_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig118_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig118_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig118_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG118_PROXY")]
pub type Cfg0Padconfig118Proxy = crate::Reg<cfg0_padconfig118_proxy::Cfg0Padconfig118ProxySpec>;
#[doc = "CFG0_PADCONFIG118_PROXY"]
pub mod cfg0_padconfig118_proxy;
#[doc = "CFG0_PADCONFIG119_PROXY (rw) register accessor: CFG0_PADCONFIG119_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig119_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig119_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig119_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG119_PROXY")]
pub type Cfg0Padconfig119Proxy = crate::Reg<cfg0_padconfig119_proxy::Cfg0Padconfig119ProxySpec>;
#[doc = "CFG0_PADCONFIG119_PROXY"]
pub mod cfg0_padconfig119_proxy;
#[doc = "CFG0_PADCONFIG120_PROXY (rw) register accessor: CFG0_PADCONFIG120_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig120_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig120_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig120_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG120_PROXY")]
pub type Cfg0Padconfig120Proxy = crate::Reg<cfg0_padconfig120_proxy::Cfg0Padconfig120ProxySpec>;
#[doc = "CFG0_PADCONFIG120_PROXY"]
pub mod cfg0_padconfig120_proxy;
#[doc = "CFG0_PADCONFIG121_PROXY (rw) register accessor: CFG0_PADCONFIG121_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig121_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig121_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig121_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG121_PROXY")]
pub type Cfg0Padconfig121Proxy = crate::Reg<cfg0_padconfig121_proxy::Cfg0Padconfig121ProxySpec>;
#[doc = "CFG0_PADCONFIG121_PROXY"]
pub mod cfg0_padconfig121_proxy;
#[doc = "CFG0_PADCONFIG122_PROXY (rw) register accessor: CFG0_PADCONFIG122_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig122_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig122_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig122_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG122_PROXY")]
pub type Cfg0Padconfig122Proxy = crate::Reg<cfg0_padconfig122_proxy::Cfg0Padconfig122ProxySpec>;
#[doc = "CFG0_PADCONFIG122_PROXY"]
pub mod cfg0_padconfig122_proxy;
#[doc = "CFG0_PADCONFIG123_PROXY (rw) register accessor: CFG0_PADCONFIG123_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig123_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig123_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig123_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG123_PROXY")]
pub type Cfg0Padconfig123Proxy = crate::Reg<cfg0_padconfig123_proxy::Cfg0Padconfig123ProxySpec>;
#[doc = "CFG0_PADCONFIG123_PROXY"]
pub mod cfg0_padconfig123_proxy;
#[doc = "CFG0_PADCONFIG124_PROXY (rw) register accessor: CFG0_PADCONFIG124_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig124_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig124_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig124_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG124_PROXY")]
pub type Cfg0Padconfig124Proxy = crate::Reg<cfg0_padconfig124_proxy::Cfg0Padconfig124ProxySpec>;
#[doc = "CFG0_PADCONFIG124_PROXY"]
pub mod cfg0_padconfig124_proxy;
#[doc = "CFG0_PADCONFIG125_PROXY (rw) register accessor: CFG0_PADCONFIG125_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig125_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig125_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig125_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG125_PROXY")]
pub type Cfg0Padconfig125Proxy = crate::Reg<cfg0_padconfig125_proxy::Cfg0Padconfig125ProxySpec>;
#[doc = "CFG0_PADCONFIG125_PROXY"]
pub mod cfg0_padconfig125_proxy;
#[doc = "CFG0_PADCONFIG126_PROXY (rw) register accessor: CFG0_PADCONFIG126_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig126_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig126_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig126_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG126_PROXY")]
pub type Cfg0Padconfig126Proxy = crate::Reg<cfg0_padconfig126_proxy::Cfg0Padconfig126ProxySpec>;
#[doc = "CFG0_PADCONFIG126_PROXY"]
pub mod cfg0_padconfig126_proxy;
#[doc = "CFG0_PADCONFIG127_PROXY (rw) register accessor: CFG0_PADCONFIG127_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig127_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig127_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig127_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG127_PROXY")]
pub type Cfg0Padconfig127Proxy = crate::Reg<cfg0_padconfig127_proxy::Cfg0Padconfig127ProxySpec>;
#[doc = "CFG0_PADCONFIG127_PROXY"]
pub mod cfg0_padconfig127_proxy;
#[doc = "CFG0_PADCONFIG128_PROXY (rw) register accessor: CFG0_PADCONFIG128_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig128_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig128_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig128_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG128_PROXY")]
pub type Cfg0Padconfig128Proxy = crate::Reg<cfg0_padconfig128_proxy::Cfg0Padconfig128ProxySpec>;
#[doc = "CFG0_PADCONFIG128_PROXY"]
pub mod cfg0_padconfig128_proxy;
#[doc = "CFG0_PADCONFIG129_PROXY (rw) register accessor: CFG0_PADCONFIG129_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig129_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig129_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig129_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG129_PROXY")]
pub type Cfg0Padconfig129Proxy = crate::Reg<cfg0_padconfig129_proxy::Cfg0Padconfig129ProxySpec>;
#[doc = "CFG0_PADCONFIG129_PROXY"]
pub mod cfg0_padconfig129_proxy;
#[doc = "CFG0_PADCONFIG130_PROXY (rw) register accessor: CFG0_PADCONFIG130_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig130_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig130_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig130_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG130_PROXY")]
pub type Cfg0Padconfig130Proxy = crate::Reg<cfg0_padconfig130_proxy::Cfg0Padconfig130ProxySpec>;
#[doc = "CFG0_PADCONFIG130_PROXY"]
pub mod cfg0_padconfig130_proxy;
#[doc = "CFG0_PADCONFIG131_PROXY (rw) register accessor: CFG0_PADCONFIG131_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig131_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig131_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig131_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG131_PROXY")]
pub type Cfg0Padconfig131Proxy = crate::Reg<cfg0_padconfig131_proxy::Cfg0Padconfig131ProxySpec>;
#[doc = "CFG0_PADCONFIG131_PROXY"]
pub mod cfg0_padconfig131_proxy;
#[doc = "CFG0_PADCONFIG132_PROXY (rw) register accessor: CFG0_PADCONFIG132_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig132_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig132_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig132_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG132_PROXY")]
pub type Cfg0Padconfig132Proxy = crate::Reg<cfg0_padconfig132_proxy::Cfg0Padconfig132ProxySpec>;
#[doc = "CFG0_PADCONFIG132_PROXY"]
pub mod cfg0_padconfig132_proxy;
#[doc = "CFG0_PADCONFIG133_PROXY (rw) register accessor: CFG0_PADCONFIG133_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig133_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig133_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig133_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG133_PROXY")]
pub type Cfg0Padconfig133Proxy = crate::Reg<cfg0_padconfig133_proxy::Cfg0Padconfig133ProxySpec>;
#[doc = "CFG0_PADCONFIG133_PROXY"]
pub mod cfg0_padconfig133_proxy;
#[doc = "CFG0_PADCONFIG134_PROXY (rw) register accessor: CFG0_PADCONFIG134_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig134_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig134_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig134_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG134_PROXY")]
pub type Cfg0Padconfig134Proxy = crate::Reg<cfg0_padconfig134_proxy::Cfg0Padconfig134ProxySpec>;
#[doc = "CFG0_PADCONFIG134_PROXY"]
pub mod cfg0_padconfig134_proxy;
#[doc = "CFG0_PADCONFIG135_PROXY (rw) register accessor: CFG0_PADCONFIG135_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig135_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig135_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig135_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG135_PROXY")]
pub type Cfg0Padconfig135Proxy = crate::Reg<cfg0_padconfig135_proxy::Cfg0Padconfig135ProxySpec>;
#[doc = "CFG0_PADCONFIG135_PROXY"]
pub mod cfg0_padconfig135_proxy;
#[doc = "CFG0_PADCONFIG136_PROXY (rw) register accessor: CFG0_PADCONFIG136_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig136_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig136_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig136_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG136_PROXY")]
pub type Cfg0Padconfig136Proxy = crate::Reg<cfg0_padconfig136_proxy::Cfg0Padconfig136ProxySpec>;
#[doc = "CFG0_PADCONFIG136_PROXY"]
pub mod cfg0_padconfig136_proxy;
#[doc = "CFG0_PADCONFIG137_PROXY (rw) register accessor: CFG0_PADCONFIG137_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig137_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig137_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig137_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG137_PROXY")]
pub type Cfg0Padconfig137Proxy = crate::Reg<cfg0_padconfig137_proxy::Cfg0Padconfig137ProxySpec>;
#[doc = "CFG0_PADCONFIG137_PROXY"]
pub mod cfg0_padconfig137_proxy;
#[doc = "CFG0_PADCONFIG138_PROXY (rw) register accessor: CFG0_PADCONFIG138_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig138_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig138_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig138_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG138_PROXY")]
pub type Cfg0Padconfig138Proxy = crate::Reg<cfg0_padconfig138_proxy::Cfg0Padconfig138ProxySpec>;
#[doc = "CFG0_PADCONFIG138_PROXY"]
pub mod cfg0_padconfig138_proxy;
#[doc = "CFG0_PADCONFIG139_PROXY (rw) register accessor: CFG0_PADCONFIG139_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig139_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig139_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig139_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG139_PROXY")]
pub type Cfg0Padconfig139Proxy = crate::Reg<cfg0_padconfig139_proxy::Cfg0Padconfig139ProxySpec>;
#[doc = "CFG0_PADCONFIG139_PROXY"]
pub mod cfg0_padconfig139_proxy;
#[doc = "CFG0_PADCONFIG140_PROXY (rw) register accessor: CFG0_PADCONFIG140_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig140_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig140_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig140_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG140_PROXY")]
pub type Cfg0Padconfig140Proxy = crate::Reg<cfg0_padconfig140_proxy::Cfg0Padconfig140ProxySpec>;
#[doc = "CFG0_PADCONFIG140_PROXY"]
pub mod cfg0_padconfig140_proxy;
#[doc = "CFG0_PADCONFIG141_PROXY (rw) register accessor: CFG0_PADCONFIG141_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig141_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig141_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig141_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG141_PROXY")]
pub type Cfg0Padconfig141Proxy = crate::Reg<cfg0_padconfig141_proxy::Cfg0Padconfig141ProxySpec>;
#[doc = "CFG0_PADCONFIG141_PROXY"]
pub mod cfg0_padconfig141_proxy;
#[doc = "CFG0_PADCONFIG142_PROXY (rw) register accessor: CFG0_PADCONFIG142_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig142_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig142_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig142_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG142_PROXY")]
pub type Cfg0Padconfig142Proxy = crate::Reg<cfg0_padconfig142_proxy::Cfg0Padconfig142ProxySpec>;
#[doc = "CFG0_PADCONFIG142_PROXY"]
pub mod cfg0_padconfig142_proxy;
#[doc = "CFG0_PADCONFIG143_PROXY (rw) register accessor: CFG0_PADCONFIG143_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig143_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig143_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig143_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG143_PROXY")]
pub type Cfg0Padconfig143Proxy = crate::Reg<cfg0_padconfig143_proxy::Cfg0Padconfig143ProxySpec>;
#[doc = "CFG0_PADCONFIG143_PROXY"]
pub mod cfg0_padconfig143_proxy;
#[doc = "CFG0_PADCONFIG144_PROXY (rw) register accessor: CFG0_PADCONFIG144_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig144_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig144_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig144_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG144_PROXY")]
pub type Cfg0Padconfig144Proxy = crate::Reg<cfg0_padconfig144_proxy::Cfg0Padconfig144ProxySpec>;
#[doc = "CFG0_PADCONFIG144_PROXY"]
pub mod cfg0_padconfig144_proxy;
#[doc = "CFG0_PADCONFIG145_PROXY (rw) register accessor: CFG0_PADCONFIG145_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig145_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig145_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig145_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG145_PROXY")]
pub type Cfg0Padconfig145Proxy = crate::Reg<cfg0_padconfig145_proxy::Cfg0Padconfig145ProxySpec>;
#[doc = "CFG0_PADCONFIG145_PROXY"]
pub mod cfg0_padconfig145_proxy;
#[doc = "CFG0_PADCONFIG146_PROXY (rw) register accessor: CFG0_PADCONFIG146_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig146_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig146_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig146_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG146_PROXY")]
pub type Cfg0Padconfig146Proxy = crate::Reg<cfg0_padconfig146_proxy::Cfg0Padconfig146ProxySpec>;
#[doc = "CFG0_PADCONFIG146_PROXY"]
pub mod cfg0_padconfig146_proxy;
#[doc = "CFG0_PADCONFIG147_PROXY (rw) register accessor: CFG0_PADCONFIG147_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig147_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig147_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig147_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG147_PROXY")]
pub type Cfg0Padconfig147Proxy = crate::Reg<cfg0_padconfig147_proxy::Cfg0Padconfig147ProxySpec>;
#[doc = "CFG0_PADCONFIG147_PROXY"]
pub mod cfg0_padconfig147_proxy;
#[doc = "CFG0_PADCONFIG148_PROXY (rw) register accessor: CFG0_PADCONFIG148_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig148_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig148_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig148_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG148_PROXY")]
pub type Cfg0Padconfig148Proxy = crate::Reg<cfg0_padconfig148_proxy::Cfg0Padconfig148ProxySpec>;
#[doc = "CFG0_PADCONFIG148_PROXY"]
pub mod cfg0_padconfig148_proxy;
#[doc = "CFG0_PADCONFIG149_PROXY (rw) register accessor: CFG0_PADCONFIG149_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig149_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig149_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig149_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG149_PROXY")]
pub type Cfg0Padconfig149Proxy = crate::Reg<cfg0_padconfig149_proxy::Cfg0Padconfig149ProxySpec>;
#[doc = "CFG0_PADCONFIG149_PROXY"]
pub mod cfg0_padconfig149_proxy;
#[doc = "CFG0_PADCONFIG150_PROXY (rw) register accessor: CFG0_PADCONFIG150_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig150_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig150_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig150_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG150_PROXY")]
pub type Cfg0Padconfig150Proxy = crate::Reg<cfg0_padconfig150_proxy::Cfg0Padconfig150ProxySpec>;
#[doc = "CFG0_PADCONFIG150_PROXY"]
pub mod cfg0_padconfig150_proxy;
#[doc = "CFG0_PADCONFIG151_PROXY (rw) register accessor: CFG0_PADCONFIG151_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig151_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig151_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig151_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG151_PROXY")]
pub type Cfg0Padconfig151Proxy = crate::Reg<cfg0_padconfig151_proxy::Cfg0Padconfig151ProxySpec>;
#[doc = "CFG0_PADCONFIG151_PROXY"]
pub mod cfg0_padconfig151_proxy;
#[doc = "CFG0_PADCONFIG152_PROXY (rw) register accessor: CFG0_PADCONFIG152_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig152_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig152_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig152_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG152_PROXY")]
pub type Cfg0Padconfig152Proxy = crate::Reg<cfg0_padconfig152_proxy::Cfg0Padconfig152ProxySpec>;
#[doc = "CFG0_PADCONFIG152_PROXY"]
pub mod cfg0_padconfig152_proxy;
#[doc = "CFG0_PADCONFIG153_PROXY (rw) register accessor: CFG0_PADCONFIG153_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig153_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig153_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig153_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG153_PROXY")]
pub type Cfg0Padconfig153Proxy = crate::Reg<cfg0_padconfig153_proxy::Cfg0Padconfig153ProxySpec>;
#[doc = "CFG0_PADCONFIG153_PROXY"]
pub mod cfg0_padconfig153_proxy;
#[doc = "CFG0_PADCONFIG154_PROXY (rw) register accessor: CFG0_PADCONFIG154_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig154_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig154_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig154_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG154_PROXY")]
pub type Cfg0Padconfig154Proxy = crate::Reg<cfg0_padconfig154_proxy::Cfg0Padconfig154ProxySpec>;
#[doc = "CFG0_PADCONFIG154_PROXY"]
pub mod cfg0_padconfig154_proxy;
#[doc = "CFG0_PADCONFIG155_PROXY (rw) register accessor: CFG0_PADCONFIG155_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig155_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig155_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig155_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG155_PROXY")]
pub type Cfg0Padconfig155Proxy = crate::Reg<cfg0_padconfig155_proxy::Cfg0Padconfig155ProxySpec>;
#[doc = "CFG0_PADCONFIG155_PROXY"]
pub mod cfg0_padconfig155_proxy;
#[doc = "CFG0_PADCONFIG156_PROXY (rw) register accessor: CFG0_PADCONFIG156_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig156_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig156_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig156_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG156_PROXY")]
pub type Cfg0Padconfig156Proxy = crate::Reg<cfg0_padconfig156_proxy::Cfg0Padconfig156ProxySpec>;
#[doc = "CFG0_PADCONFIG156_PROXY"]
pub mod cfg0_padconfig156_proxy;
#[doc = "CFG0_PADCONFIG157_PROXY (rw) register accessor: CFG0_PADCONFIG157_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig157_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig157_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig157_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG157_PROXY")]
pub type Cfg0Padconfig157Proxy = crate::Reg<cfg0_padconfig157_proxy::Cfg0Padconfig157ProxySpec>;
#[doc = "CFG0_PADCONFIG157_PROXY"]
pub mod cfg0_padconfig157_proxy;
#[doc = "CFG0_PADCONFIG158_PROXY (rw) register accessor: CFG0_PADCONFIG158_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig158_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig158_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig158_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG158_PROXY")]
pub type Cfg0Padconfig158Proxy = crate::Reg<cfg0_padconfig158_proxy::Cfg0Padconfig158ProxySpec>;
#[doc = "CFG0_PADCONFIG158_PROXY"]
pub mod cfg0_padconfig158_proxy;
#[doc = "CFG0_PADCONFIG159_PROXY (rw) register accessor: CFG0_PADCONFIG159_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig159_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig159_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig159_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG159_PROXY")]
pub type Cfg0Padconfig159Proxy = crate::Reg<cfg0_padconfig159_proxy::Cfg0Padconfig159ProxySpec>;
#[doc = "CFG0_PADCONFIG159_PROXY"]
pub mod cfg0_padconfig159_proxy;
#[doc = "CFG0_PADCONFIG160_PROXY (rw) register accessor: CFG0_PADCONFIG160_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig160_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig160_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig160_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG160_PROXY")]
pub type Cfg0Padconfig160Proxy = crate::Reg<cfg0_padconfig160_proxy::Cfg0Padconfig160ProxySpec>;
#[doc = "CFG0_PADCONFIG160_PROXY"]
pub mod cfg0_padconfig160_proxy;
#[doc = "CFG0_PADCONFIG161_PROXY (rw) register accessor: CFG0_PADCONFIG161_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig161_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig161_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig161_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG161_PROXY")]
pub type Cfg0Padconfig161Proxy = crate::Reg<cfg0_padconfig161_proxy::Cfg0Padconfig161ProxySpec>;
#[doc = "CFG0_PADCONFIG161_PROXY"]
pub mod cfg0_padconfig161_proxy;
#[doc = "CFG0_PADCONFIG162_PROXY (rw) register accessor: CFG0_PADCONFIG162_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig162_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig162_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig162_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG162_PROXY")]
pub type Cfg0Padconfig162Proxy = crate::Reg<cfg0_padconfig162_proxy::Cfg0Padconfig162ProxySpec>;
#[doc = "CFG0_PADCONFIG162_PROXY"]
pub mod cfg0_padconfig162_proxy;
#[doc = "CFG0_PADCONFIG163_PROXY (rw) register accessor: CFG0_PADCONFIG163_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig163_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig163_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig163_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG163_PROXY")]
pub type Cfg0Padconfig163Proxy = crate::Reg<cfg0_padconfig163_proxy::Cfg0Padconfig163ProxySpec>;
#[doc = "CFG0_PADCONFIG163_PROXY"]
pub mod cfg0_padconfig163_proxy;
#[doc = "CFG0_PADCONFIG164_PROXY (rw) register accessor: CFG0_PADCONFIG164_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig164_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig164_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig164_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG164_PROXY")]
pub type Cfg0Padconfig164Proxy = crate::Reg<cfg0_padconfig164_proxy::Cfg0Padconfig164ProxySpec>;
#[doc = "CFG0_PADCONFIG164_PROXY"]
pub mod cfg0_padconfig164_proxy;
#[doc = "CFG0_PADCONFIG165_PROXY (rw) register accessor: CFG0_PADCONFIG165_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig165_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig165_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig165_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG165_PROXY")]
pub type Cfg0Padconfig165Proxy = crate::Reg<cfg0_padconfig165_proxy::Cfg0Padconfig165ProxySpec>;
#[doc = "CFG0_PADCONFIG165_PROXY"]
pub mod cfg0_padconfig165_proxy;
#[doc = "CFG0_PADCONFIG166_PROXY (rw) register accessor: CFG0_PADCONFIG166_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig166_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig166_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig166_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG166_PROXY")]
pub type Cfg0Padconfig166Proxy = crate::Reg<cfg0_padconfig166_proxy::Cfg0Padconfig166ProxySpec>;
#[doc = "CFG0_PADCONFIG166_PROXY"]
pub mod cfg0_padconfig166_proxy;
#[doc = "CFG0_PADCONFIG167_PROXY (rw) register accessor: CFG0_PADCONFIG167_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig167_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig167_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig167_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG167_PROXY")]
pub type Cfg0Padconfig167Proxy = crate::Reg<cfg0_padconfig167_proxy::Cfg0Padconfig167ProxySpec>;
#[doc = "CFG0_PADCONFIG167_PROXY"]
pub mod cfg0_padconfig167_proxy;
#[doc = "CFG0_PADCONFIG168_PROXY (rw) register accessor: CFG0_PADCONFIG168_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig168_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig168_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig168_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG168_PROXY")]
pub type Cfg0Padconfig168Proxy = crate::Reg<cfg0_padconfig168_proxy::Cfg0Padconfig168ProxySpec>;
#[doc = "CFG0_PADCONFIG168_PROXY"]
pub mod cfg0_padconfig168_proxy;
#[doc = "CFG0_PADCONFIG169_PROXY (rw) register accessor: CFG0_PADCONFIG169_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig169_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig169_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig169_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG169_PROXY")]
pub type Cfg0Padconfig169Proxy = crate::Reg<cfg0_padconfig169_proxy::Cfg0Padconfig169ProxySpec>;
#[doc = "CFG0_PADCONFIG169_PROXY"]
pub mod cfg0_padconfig169_proxy;
#[doc = "CFG0_PADCONFIG170_PROXY (rw) register accessor: CFG0_PADCONFIG170_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig170_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig170_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig170_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG170_PROXY")]
pub type Cfg0Padconfig170Proxy = crate::Reg<cfg0_padconfig170_proxy::Cfg0Padconfig170ProxySpec>;
#[doc = "CFG0_PADCONFIG170_PROXY"]
pub mod cfg0_padconfig170_proxy;
#[doc = "CFG0_PADCONFIG171_PROXY (rw) register accessor: CFG0_PADCONFIG171_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig171_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig171_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig171_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG171_PROXY")]
pub type Cfg0Padconfig171Proxy = crate::Reg<cfg0_padconfig171_proxy::Cfg0Padconfig171ProxySpec>;
#[doc = "CFG0_PADCONFIG171_PROXY"]
pub mod cfg0_padconfig171_proxy;
#[doc = "CFG0_PADCONFIG172_PROXY (rw) register accessor: CFG0_PADCONFIG172_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig172_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig172_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig172_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG172_PROXY")]
pub type Cfg0Padconfig172Proxy = crate::Reg<cfg0_padconfig172_proxy::Cfg0Padconfig172ProxySpec>;
#[doc = "CFG0_PADCONFIG172_PROXY"]
pub mod cfg0_padconfig172_proxy;
#[doc = "CFG0_PADCONFIG173_PROXY (rw) register accessor: CFG0_PADCONFIG173_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig173_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig173_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig173_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG173_PROXY")]
pub type Cfg0Padconfig173Proxy = crate::Reg<cfg0_padconfig173_proxy::Cfg0Padconfig173ProxySpec>;
#[doc = "CFG0_PADCONFIG173_PROXY"]
pub mod cfg0_padconfig173_proxy;
#[doc = "CFG0_PADCONFIG174_PROXY (rw) register accessor: CFG0_PADCONFIG174_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig174_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig174_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig174_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG174_PROXY")]
pub type Cfg0Padconfig174Proxy = crate::Reg<cfg0_padconfig174_proxy::Cfg0Padconfig174ProxySpec>;
#[doc = "CFG0_PADCONFIG174_PROXY"]
pub mod cfg0_padconfig174_proxy;
#[doc = "CFG0_PADCONFIG175_PROXY (rw) register accessor: CFG0_PADCONFIG175_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig175_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig175_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig175_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG175_PROXY")]
pub type Cfg0Padconfig175Proxy = crate::Reg<cfg0_padconfig175_proxy::Cfg0Padconfig175ProxySpec>;
#[doc = "CFG0_PADCONFIG175_PROXY"]
pub mod cfg0_padconfig175_proxy;
#[doc = "CFG0_PADCONFIG176_PROXY (rw) register accessor: CFG0_PADCONFIG176_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig176_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig176_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig176_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG176_PROXY")]
pub type Cfg0Padconfig176Proxy = crate::Reg<cfg0_padconfig176_proxy::Cfg0Padconfig176ProxySpec>;
#[doc = "CFG0_PADCONFIG176_PROXY"]
pub mod cfg0_padconfig176_proxy;
#[doc = "CFG0_PADCONFIG177_PROXY (rw) register accessor: CFG0_PADCONFIG177_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig177_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig177_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig177_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG177_PROXY")]
pub type Cfg0Padconfig177Proxy = crate::Reg<cfg0_padconfig177_proxy::Cfg0Padconfig177ProxySpec>;
#[doc = "CFG0_PADCONFIG177_PROXY"]
pub mod cfg0_padconfig177_proxy;
#[doc = "CFG0_PADCONFIG178_PROXY (rw) register accessor: CFG0_PADCONFIG178_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig178_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig178_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig178_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG178_PROXY")]
pub type Cfg0Padconfig178Proxy = crate::Reg<cfg0_padconfig178_proxy::Cfg0Padconfig178ProxySpec>;
#[doc = "CFG0_PADCONFIG178_PROXY"]
pub mod cfg0_padconfig178_proxy;
#[doc = "CFG0_PADCONFIG179_PROXY (rw) register accessor: CFG0_PADCONFIG179_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig179_proxy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig179_proxy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_padconfig179_proxy`]
module"]
#[doc(alias = "CFG0_PADCONFIG179_PROXY")]
pub type Cfg0Padconfig179Proxy = crate::Reg<cfg0_padconfig179_proxy::Cfg0Padconfig179ProxySpec>;
#[doc = "CFG0_PADCONFIG179_PROXY"]
pub mod cfg0_padconfig179_proxy;
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
