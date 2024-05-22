#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcrc64_regs_crc_ctrl0: Mcrc64RegsCrcCtrl0,
    _reserved1: [u8; 0x04],
    mcrc64_regs_crc_ctrl1: Mcrc64RegsCrcCtrl1,
    _reserved2: [u8; 0x04],
    mcrc64_regs_crc_ctrl2: Mcrc64RegsCrcCtrl2,
    _reserved3: [u8; 0x04],
    mcrc64_regs_crc_ints: Mcrc64RegsCrcInts,
    _reserved4: [u8; 0x04],
    mcrc64_regs_crc_intr: Mcrc64RegsCrcIntr,
    _reserved5: [u8; 0x04],
    mcrc64_regs_crc_status: Mcrc64RegsCrcStatus,
    _reserved6: [u8; 0x04],
    mcrc64_regs_crc_int_offset_reg: Mcrc64RegsCrcIntOffsetReg,
    _reserved7: [u8; 0x04],
    mcrc64_regs_crc_busy: Mcrc64RegsCrcBusy,
    _reserved8: [u8; 0x04],
    mcrc64_regs_crc_pcount_reg1: Mcrc64RegsCrcPcountReg1,
    mcrc64_regs_crc_scount_reg1: Mcrc64RegsCrcScountReg1,
    mcrc64_regs_crc_cursec_reg1: Mcrc64RegsCrcCursecReg1,
    mcrc64_regs_crc_wdtopld1: Mcrc64RegsCrcWdtopld1,
    mcrc64_regs_crc_bctopld1: Mcrc64RegsCrcBctopld1,
    _reserved13: [u8; 0x0c],
    mcrc64_regs_psa_sigregl1: Mcrc64RegsPsaSigregl1,
    mcrc64_regs_psa_sigregh1: Mcrc64RegsPsaSigregh1,
    mcrc64_regs_crc_regl1: Mcrc64RegsCrcRegl1,
    mcrc64_regs_crc_regh1: Mcrc64RegsCrcRegh1,
    mcrc64_regs_psa_secsigregl1: Mcrc64RegsPsaSecsigregl1,
    mcrc64_regs_psa_secsigregh1: Mcrc64RegsPsaSecsigregh1,
    mcrc64_regs_raw_dataregl1: Mcrc64RegsRawDataregl1,
    mcrc64_regs_raw_dataregh1: Mcrc64RegsRawDataregh1,
    mcrc64_regs_crc_pcount_reg2: Mcrc64RegsCrcPcountReg2,
    mcrc64_regs_crc_scount_reg2: Mcrc64RegsCrcScountReg2,
    mcrc64_regs_crc_cursec_reg2: Mcrc64RegsCrcCursecReg2,
    mcrc64_regs_crc_wdtopld2: Mcrc64RegsCrcWdtopld2,
    mcrc64_regs_crc_bctopld2: Mcrc64RegsCrcBctopld2,
    _reserved26: [u8; 0x0c],
    mcrc64_regs_psa_sigregl2: Mcrc64RegsPsaSigregl2,
    mcrc64_regs_psa_sigregh2: Mcrc64RegsPsaSigregh2,
    mcrc64_regs_crc_regl2: Mcrc64RegsCrcRegl2,
    mcrc64_regs_crc_regh2: Mcrc64RegsCrcRegh2,
    mcrc64_regs_psa_secsigregl2: Mcrc64RegsPsaSecsigregl2,
    mcrc64_regs_psa_secsigregh2: Mcrc64RegsPsaSecsigregh2,
    mcrc64_regs_raw_dataregl2: Mcrc64RegsRawDataregl2,
    mcrc64_regs_raw_dataregh2: Mcrc64RegsRawDataregh2,
    mcrc64_regs_crc_pcount_reg3: Mcrc64RegsCrcPcountReg3,
    mcrc64_regs_crc_scount_reg3: Mcrc64RegsCrcScountReg3,
    mcrc64_regs_crc_cursec_reg3: Mcrc64RegsCrcCursecReg3,
    mcrc64_regs_crc_wdtopld3: Mcrc64RegsCrcWdtopld3,
    mcrc64_regs_crc_bctopld3: Mcrc64RegsCrcBctopld3,
    _reserved39: [u8; 0x0c],
    mcrc64_regs_psa_sigregl3: Mcrc64RegsPsaSigregl3,
    mcrc64_regs_psa_sigregh3: Mcrc64RegsPsaSigregh3,
    mcrc64_regs_crc_regl3: Mcrc64RegsCrcRegl3,
    mcrc64_regs_crc_regh3: Mcrc64RegsCrcRegh3,
    mcrc64_regs_psa_secsigregl3: Mcrc64RegsPsaSecsigregl3,
    mcrc64_regs_psa_secsigregh3: Mcrc64RegsPsaSecsigregh3,
    mcrc64_regs_raw_dataregl3: Mcrc64RegsRawDataregl3,
    mcrc64_regs_raw_dataregh3: Mcrc64RegsRawDataregh3,
    mcrc64_regs_crc_pcount_reg4: Mcrc64RegsCrcPcountReg4,
    mcrc64_regs_crc_scount_reg4: Mcrc64RegsCrcScountReg4,
    mcrc64_regs_crc_cursec_reg4: Mcrc64RegsCrcCursecReg4,
    mcrc64_regs_crc_wdtopld4: Mcrc64RegsCrcWdtopld4,
    mcrc64_regs_crc_bctopld4: Mcrc64RegsCrcBctopld4,
    _reserved52: [u8; 0x0c],
    mcrc64_regs_psa_sigregl4: Mcrc64RegsPsaSigregl4,
    mcrc64_regs_psa_sigregh4: Mcrc64RegsPsaSigregh4,
    mcrc64_regs_crc_regl4: Mcrc64RegsCrcRegl4,
    mcrc64_regs_crc_regh4: Mcrc64RegsCrcRegh4,
    mcrc64_regs_psa_secsigregl4: Mcrc64RegsPsaSecsigregl4,
    mcrc64_regs_psa_secsigregh4: Mcrc64RegsPsaSecsigregh4,
    mcrc64_regs_raw_dataregl4: Mcrc64RegsRawDataregl4,
    mcrc64_regs_raw_dataregh4: Mcrc64RegsRawDataregh4,
    mcrc64_regs_mcrc_bus_sel: Mcrc64RegsMcrcBusSel,
    _reserved61: [u8; 0xbc],
    mcrc64_regs_i0_psa_sigreg1_cpy: Mcrc64RegsI0PsaSigreg1Cpy,
    _reserved62: [u8; 0x7c],
    mcrc64_regs_i0_psa_sigreg2_cpy: Mcrc64RegsI0PsaSigreg2Cpy,
    _reserved63: [u8; 0x7c],
    mcrc64_regs_i0_psa_sigreg3_cpy: Mcrc64RegsI0PsaSigreg3Cpy,
    _reserved64: [u8; 0x7c],
    mcrc64_regs_i0_psa_sigreg4_cpy: Mcrc64RegsI0PsaSigreg4Cpy,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Global Control Register 0"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_ctrl0(&self) -> &Mcrc64RegsCrcCtrl0 {
        &self.mcrc64_regs_crc_ctrl0
    }
    #[doc = "0x08 - CRC Global Control Register 1"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_ctrl1(&self) -> &Mcrc64RegsCrcCtrl1 {
        &self.mcrc64_regs_crc_ctrl1
    }
    #[doc = "0x10 - Data capture mode is especially useful when it is used in conjunction when data trace (CH1_TRACEEN) for channel 1. The seed value can be planted in PSA Signature Register during data capture mode by writing a seed value into PSA Signature Register. Data trace enable bit is then set to snoop and compress any read transaction on Data buses. When trace enable bit is set, CH1_MODE is automatically reset to data capture mode. To change mode from one to another in the middle of an on-going mode user should take the following steps:"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_ctrl2(&self) -> &Mcrc64RegsCrcCtrl2 {
        &self.mcrc64_regs_crc_ctrl2
    }
    #[doc = "0x18 - CRC Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_ints(&self) -> &Mcrc64RegsCrcInts {
        &self.mcrc64_regs_crc_ints
    }
    #[doc = "0x20 - CRC Interrupt Enable Reset Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_intr(&self) -> &Mcrc64RegsCrcIntr {
        &self.mcrc64_regs_crc_intr
    }
    #[doc = "0x28 - CRC Interrupt Status Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_status(&self) -> &Mcrc64RegsCrcStatus {
        &self.mcrc64_regs_crc_status
    }
    #[doc = "0x30 - CRC Interrupt Offset"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_int_offset_reg(&self) -> &Mcrc64RegsCrcIntOffsetReg {
        &self.mcrc64_regs_crc_int_offset_reg
    }
    #[doc = "0x38 - CRC Busy Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_busy(&self) -> &Mcrc64RegsCrcBusy {
        &self.mcrc64_regs_crc_busy
    }
    #[doc = "0x40 - CRC Pattern Counter Preload Register1"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_pcount_reg1(&self) -> &Mcrc64RegsCrcPcountReg1 {
        &self.mcrc64_regs_crc_pcount_reg1
    }
    #[doc = "0x44 - CRC Sector Counter Preload Register1"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_scount_reg1(&self) -> &Mcrc64RegsCrcScountReg1 {
        &self.mcrc64_regs_crc_scount_reg1
    }
    #[doc = "0x48 - CRC Current Sector Register 1"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_cursec_reg1(&self) -> &Mcrc64RegsCrcCursecReg1 {
        &self.mcrc64_regs_crc_cursec_reg1
    }
    #[doc = "0x4c - CRC channel 1 Watchdog Timeout Preload Register A"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_wdtopld1(&self) -> &Mcrc64RegsCrcWdtopld1 {
        &self.mcrc64_regs_crc_wdtopld1
    }
    #[doc = "0x50 - CRC channel 1 Block Complete Timeout Preload Register B"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_bctopld1(&self) -> &Mcrc64RegsCrcBctopld1 {
        &self.mcrc64_regs_crc_bctopld1
    }
    #[doc = "0x60 - Channel 1 PSA signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregl1(&self) -> &Mcrc64RegsPsaSigregl1 {
        &self.mcrc64_regs_psa_sigregl1
    }
    #[doc = "0x64 - Channel 1 PSA signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregh1(&self) -> &Mcrc64RegsPsaSigregh1 {
        &self.mcrc64_regs_psa_sigregh1
    }
    #[doc = "0x68 - Channel 1 CRC value low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regl1(&self) -> &Mcrc64RegsCrcRegl1 {
        &self.mcrc64_regs_crc_regl1
    }
    #[doc = "0x6c - Channel 1 CRC value high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regh1(&self) -> &Mcrc64RegsCrcRegh1 {
        &self.mcrc64_regs_crc_regh1
    }
    #[doc = "0x70 - Channel 1 PSA sector signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregl1(&self) -> &Mcrc64RegsPsaSecsigregl1 {
        &self.mcrc64_regs_psa_secsigregl1
    }
    #[doc = "0x74 - Channel 1 PSA sector signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregh1(&self) -> &Mcrc64RegsPsaSecsigregh1 {
        &self.mcrc64_regs_psa_secsigregh1
    }
    #[doc = "0x78 - Channel 1 Raw Data Low Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregl1(&self) -> &Mcrc64RegsRawDataregl1 {
        &self.mcrc64_regs_raw_dataregl1
    }
    #[doc = "0x7c - Channel 1 Raw Data High Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregh1(&self) -> &Mcrc64RegsRawDataregh1 {
        &self.mcrc64_regs_raw_dataregh1
    }
    #[doc = "0x80 - CRC Pattern Counter Preload Register2"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_pcount_reg2(&self) -> &Mcrc64RegsCrcPcountReg2 {
        &self.mcrc64_regs_crc_pcount_reg2
    }
    #[doc = "0x84 - CRC Sector Counter Preload Register2"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_scount_reg2(&self) -> &Mcrc64RegsCrcScountReg2 {
        &self.mcrc64_regs_crc_scount_reg2
    }
    #[doc = "0x88 - CRC Current Sector Register 2"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_cursec_reg2(&self) -> &Mcrc64RegsCrcCursecReg2 {
        &self.mcrc64_regs_crc_cursec_reg2
    }
    #[doc = "0x8c - CRC channel 2 Watchdog Timeout Preload Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_wdtopld2(&self) -> &Mcrc64RegsCrcWdtopld2 {
        &self.mcrc64_regs_crc_wdtopld2
    }
    #[doc = "0x90 - CRC channel 2 Block Complete Timeout Preload Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_bctopld2(&self) -> &Mcrc64RegsCrcBctopld2 {
        &self.mcrc64_regs_crc_bctopld2
    }
    #[doc = "0xa0 - Channel 2 PSA signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregl2(&self) -> &Mcrc64RegsPsaSigregl2 {
        &self.mcrc64_regs_psa_sigregl2
    }
    #[doc = "0xa4 - Channel 2 PSA signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregh2(&self) -> &Mcrc64RegsPsaSigregh2 {
        &self.mcrc64_regs_psa_sigregh2
    }
    #[doc = "0xa8 - Channel 2 CRC value low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regl2(&self) -> &Mcrc64RegsCrcRegl2 {
        &self.mcrc64_regs_crc_regl2
    }
    #[doc = "0xac - Channel 2 CRC value high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regh2(&self) -> &Mcrc64RegsCrcRegh2 {
        &self.mcrc64_regs_crc_regh2
    }
    #[doc = "0xb0 - Channel 2 PSA sector signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregl2(&self) -> &Mcrc64RegsPsaSecsigregl2 {
        &self.mcrc64_regs_psa_secsigregl2
    }
    #[doc = "0xb4 - Channel 2 PSA sector signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregh2(&self) -> &Mcrc64RegsPsaSecsigregh2 {
        &self.mcrc64_regs_psa_secsigregh2
    }
    #[doc = "0xb8 - Channel 2 Raw Data Low Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregl2(&self) -> &Mcrc64RegsRawDataregl2 {
        &self.mcrc64_regs_raw_dataregl2
    }
    #[doc = "0xbc - Channel 2 Raw Data High Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregh2(&self) -> &Mcrc64RegsRawDataregh2 {
        &self.mcrc64_regs_raw_dataregh2
    }
    #[doc = "0xc0 - CRC Pattern Counter Preload Register3"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_pcount_reg3(&self) -> &Mcrc64RegsCrcPcountReg3 {
        &self.mcrc64_regs_crc_pcount_reg3
    }
    #[doc = "0xc4 - CRC Sector Counter Preload Register3"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_scount_reg3(&self) -> &Mcrc64RegsCrcScountReg3 {
        &self.mcrc64_regs_crc_scount_reg3
    }
    #[doc = "0xc8 - CRC Current Sector Register 3"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_cursec_reg3(&self) -> &Mcrc64RegsCrcCursecReg3 {
        &self.mcrc64_regs_crc_cursec_reg3
    }
    #[doc = "0xcc - CRC channel 3 Watchdog Timeout Preload Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_wdtopld3(&self) -> &Mcrc64RegsCrcWdtopld3 {
        &self.mcrc64_regs_crc_wdtopld3
    }
    #[doc = "0xd0 - CRC channel 3 Block Complete Timeout Preload Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_bctopld3(&self) -> &Mcrc64RegsCrcBctopld3 {
        &self.mcrc64_regs_crc_bctopld3
    }
    #[doc = "0xe0 - Channel 3 PSA signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregl3(&self) -> &Mcrc64RegsPsaSigregl3 {
        &self.mcrc64_regs_psa_sigregl3
    }
    #[doc = "0xe4 - Channel 3 PSA signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregh3(&self) -> &Mcrc64RegsPsaSigregh3 {
        &self.mcrc64_regs_psa_sigregh3
    }
    #[doc = "0xe8 - Channel 3 CRC value low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regl3(&self) -> &Mcrc64RegsCrcRegl3 {
        &self.mcrc64_regs_crc_regl3
    }
    #[doc = "0xec - Channel 3 CRC value high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regh3(&self) -> &Mcrc64RegsCrcRegh3 {
        &self.mcrc64_regs_crc_regh3
    }
    #[doc = "0xf0 - Channel 3 PSA sector signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregl3(&self) -> &Mcrc64RegsPsaSecsigregl3 {
        &self.mcrc64_regs_psa_secsigregl3
    }
    #[doc = "0xf4 - Channel 3 PSA sector signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregh3(&self) -> &Mcrc64RegsPsaSecsigregh3 {
        &self.mcrc64_regs_psa_secsigregh3
    }
    #[doc = "0xf8 - Channel 3 Raw Data Low Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregl3(&self) -> &Mcrc64RegsRawDataregl3 {
        &self.mcrc64_regs_raw_dataregl3
    }
    #[doc = "0xfc - Channel 3 Raw Data High Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregh3(&self) -> &Mcrc64RegsRawDataregh3 {
        &self.mcrc64_regs_raw_dataregh3
    }
    #[doc = "0x100 - CRC Pattern Counter Preload Register4"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_pcount_reg4(&self) -> &Mcrc64RegsCrcPcountReg4 {
        &self.mcrc64_regs_crc_pcount_reg4
    }
    #[doc = "0x104 - CRC Sector Counter Preload Register4"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_scount_reg4(&self) -> &Mcrc64RegsCrcScountReg4 {
        &self.mcrc64_regs_crc_scount_reg4
    }
    #[doc = "0x108 - CRC Current Sector Register 4"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_cursec_reg4(&self) -> &Mcrc64RegsCrcCursecReg4 {
        &self.mcrc64_regs_crc_cursec_reg4
    }
    #[doc = "0x10c - CRC channel 4 Watchdog Timeout Preload Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_wdtopld4(&self) -> &Mcrc64RegsCrcWdtopld4 {
        &self.mcrc64_regs_crc_wdtopld4
    }
    #[doc = "0x110 - CRC channel 4 Block Complete Timeout Preload Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_bctopld4(&self) -> &Mcrc64RegsCrcBctopld4 {
        &self.mcrc64_regs_crc_bctopld4
    }
    #[doc = "0x120 - Channel 4 PSA signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregl4(&self) -> &Mcrc64RegsPsaSigregl4 {
        &self.mcrc64_regs_psa_sigregl4
    }
    #[doc = "0x124 - Channel 4 PSA signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_sigregh4(&self) -> &Mcrc64RegsPsaSigregh4 {
        &self.mcrc64_regs_psa_sigregh4
    }
    #[doc = "0x128 - Channel 4 CRC value low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regl4(&self) -> &Mcrc64RegsCrcRegl4 {
        &self.mcrc64_regs_crc_regl4
    }
    #[doc = "0x12c - Channel 4 CRC value high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_crc_regh4(&self) -> &Mcrc64RegsCrcRegh4 {
        &self.mcrc64_regs_crc_regh4
    }
    #[doc = "0x130 - Channel 4 PSA sector signature low register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregl4(&self) -> &Mcrc64RegsPsaSecsigregl4 {
        &self.mcrc64_regs_psa_secsigregl4
    }
    #[doc = "0x134 - Channel 4 PSA sector signature high register"]
    #[inline(always)]
    pub const fn mcrc64_regs_psa_secsigregh4(&self) -> &Mcrc64RegsPsaSecsigregh4 {
        &self.mcrc64_regs_psa_secsigregh4
    }
    #[doc = "0x138 - Channel 4 Raw Data Low Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregl4(&self) -> &Mcrc64RegsRawDataregl4 {
        &self.mcrc64_regs_raw_dataregl4
    }
    #[doc = "0x13c - Channel 4 Raw Data High Register"]
    #[inline(always)]
    pub const fn mcrc64_regs_raw_dataregh4(&self) -> &Mcrc64RegsRawDataregh4 {
        &self.mcrc64_regs_raw_dataregh4
    }
    #[doc = "0x140 - Data bus tracing selection"]
    #[inline(always)]
    pub const fn mcrc64_regs_mcrc_bus_sel(&self) -> &Mcrc64RegsMcrcBusSel {
        &self.mcrc64_regs_mcrc_bus_sel
    }
    #[doc = "0x200 - Region for Channel 1 PSA signature block used by DMA based systems."]
    #[inline(always)]
    pub const fn mcrc64_regs_i0_psa_sigreg1_cpy(&self) -> &Mcrc64RegsI0PsaSigreg1Cpy {
        &self.mcrc64_regs_i0_psa_sigreg1_cpy
    }
    #[doc = "0x280 - Region for Channel 2 PSA signature block used by DMA based systems."]
    #[inline(always)]
    pub const fn mcrc64_regs_i0_psa_sigreg2_cpy(&self) -> &Mcrc64RegsI0PsaSigreg2Cpy {
        &self.mcrc64_regs_i0_psa_sigreg2_cpy
    }
    #[doc = "0x300 - Region for Channel 3 PSA signature block used by DMA based systems."]
    #[inline(always)]
    pub const fn mcrc64_regs_i0_psa_sigreg3_cpy(&self) -> &Mcrc64RegsI0PsaSigreg3Cpy {
        &self.mcrc64_regs_i0_psa_sigreg3_cpy
    }
    #[doc = "0x380 - Region for Channel 4 PSA signature block used by DMA based systems."]
    #[inline(always)]
    pub const fn mcrc64_regs_i0_psa_sigreg4_cpy(&self) -> &Mcrc64RegsI0PsaSigreg4Cpy {
        &self.mcrc64_regs_i0_psa_sigreg4_cpy
    }
}
#[doc = "MCRC64_REGS_CRC_CTRL0 (rw) register accessor: CRC Global Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_ctrl0`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CTRL0")]
pub type Mcrc64RegsCrcCtrl0 = crate::Reg<mcrc64_regs_crc_ctrl0::Mcrc64RegsCrcCtrl0Spec>;
#[doc = "CRC Global Control Register 0"]
pub mod mcrc64_regs_crc_ctrl0;
#[doc = "MCRC64_REGS_CRC_CTRL1 (rw) register accessor: CRC Global Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_ctrl1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CTRL1")]
pub type Mcrc64RegsCrcCtrl1 = crate::Reg<mcrc64_regs_crc_ctrl1::Mcrc64RegsCrcCtrl1Spec>;
#[doc = "CRC Global Control Register 1"]
pub mod mcrc64_regs_crc_ctrl1;
#[doc = "MCRC64_REGS_CRC_CTRL2 (rw) register accessor: Data capture mode is especially useful when it is used in conjunction when data trace (CH1_TRACEEN) for channel 1. The seed value can be planted in PSA Signature Register during data capture mode by writing a seed value into PSA Signature Register. Data trace enable bit is then set to snoop and compress any read transaction on Data buses. When trace enable bit is set, CH1_MODE is automatically reset to data capture mode. To change mode from one to another in the middle of an on-going mode user should take the following steps:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_ctrl2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CTRL2")]
pub type Mcrc64RegsCrcCtrl2 = crate::Reg<mcrc64_regs_crc_ctrl2::Mcrc64RegsCrcCtrl2Spec>;
#[doc = "Data capture mode is especially useful when it is used in conjunction when data trace (CH1_TRACEEN) for channel 1. The seed value can be planted in PSA Signature Register during data capture mode by writing a seed value into PSA Signature Register. Data trace enable bit is then set to snoop and compress any read transaction on Data buses. When trace enable bit is set, CH1_MODE is automatically reset to data capture mode. To change mode from one to another in the middle of an on-going mode user should take the following steps:"]
pub mod mcrc64_regs_crc_ctrl2;
#[doc = "MCRC64_REGS_CRC_INTS (rw) register accessor: CRC Interrupt Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ints::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ints::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_ints`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_INTS")]
pub type Mcrc64RegsCrcInts = crate::Reg<mcrc64_regs_crc_ints::Mcrc64RegsCrcIntsSpec>;
#[doc = "CRC Interrupt Enable Set Register"]
pub mod mcrc64_regs_crc_ints;
#[doc = "MCRC64_REGS_CRC_INTR (rw) register accessor: CRC Interrupt Enable Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_intr`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_INTR")]
pub type Mcrc64RegsCrcIntr = crate::Reg<mcrc64_regs_crc_intr::Mcrc64RegsCrcIntrSpec>;
#[doc = "CRC Interrupt Enable Reset Register"]
pub mod mcrc64_regs_crc_intr;
#[doc = "MCRC64_REGS_CRC_STATUS (rw) register accessor: CRC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_status`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_STATUS")]
pub type Mcrc64RegsCrcStatus = crate::Reg<mcrc64_regs_crc_status::Mcrc64RegsCrcStatusSpec>;
#[doc = "CRC Interrupt Status Register"]
pub mod mcrc64_regs_crc_status;
#[doc = "MCRC64_REGS_CRC_INT_OFFSET_REG (rw) register accessor: CRC Interrupt Offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_int_offset_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_int_offset_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_int_offset_reg`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_INT_OFFSET_REG")]
pub type Mcrc64RegsCrcIntOffsetReg =
    crate::Reg<mcrc64_regs_crc_int_offset_reg::Mcrc64RegsCrcIntOffsetRegSpec>;
#[doc = "CRC Interrupt Offset"]
pub mod mcrc64_regs_crc_int_offset_reg;
#[doc = "MCRC64_REGS_CRC_BUSY (rw) register accessor: CRC Busy Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_busy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_busy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_busy`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_BUSY")]
pub type Mcrc64RegsCrcBusy = crate::Reg<mcrc64_regs_crc_busy::Mcrc64RegsCrcBusySpec>;
#[doc = "CRC Busy Register"]
pub mod mcrc64_regs_crc_busy;
#[doc = "MCRC64_REGS_CRC_PCOUNT_REG1 (rw) register accessor: CRC Pattern Counter Preload Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_pcount_reg1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_PCOUNT_REG1")]
pub type Mcrc64RegsCrcPcountReg1 =
    crate::Reg<mcrc64_regs_crc_pcount_reg1::Mcrc64RegsCrcPcountReg1Spec>;
#[doc = "CRC Pattern Counter Preload Register1"]
pub mod mcrc64_regs_crc_pcount_reg1;
#[doc = "MCRC64_REGS_CRC_SCOUNT_REG1 (rw) register accessor: CRC Sector Counter Preload Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_scount_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_scount_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_scount_reg1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_SCOUNT_REG1")]
pub type Mcrc64RegsCrcScountReg1 =
    crate::Reg<mcrc64_regs_crc_scount_reg1::Mcrc64RegsCrcScountReg1Spec>;
#[doc = "CRC Sector Counter Preload Register1"]
pub mod mcrc64_regs_crc_scount_reg1;
#[doc = "MCRC64_REGS_CRC_CURSEC_REG1 (rw) register accessor: CRC Current Sector Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_cursec_reg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_cursec_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_cursec_reg1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CURSEC_REG1")]
pub type Mcrc64RegsCrcCursecReg1 =
    crate::Reg<mcrc64_regs_crc_cursec_reg1::Mcrc64RegsCrcCursecReg1Spec>;
#[doc = "CRC Current Sector Register 1"]
pub mod mcrc64_regs_crc_cursec_reg1;
#[doc = "MCRC64_REGS_CRC_WDTOPLD1 (rw) register accessor: CRC channel 1 Watchdog Timeout Preload Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_wdtopld1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_wdtopld1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_wdtopld1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_WDTOPLD1")]
pub type Mcrc64RegsCrcWdtopld1 = crate::Reg<mcrc64_regs_crc_wdtopld1::Mcrc64RegsCrcWdtopld1Spec>;
#[doc = "CRC channel 1 Watchdog Timeout Preload Register A"]
pub mod mcrc64_regs_crc_wdtopld1;
#[doc = "MCRC64_REGS_CRC_BCTOPLD1 (rw) register accessor: CRC channel 1 Block Complete Timeout Preload Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_bctopld1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_bctopld1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_bctopld1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_BCTOPLD1")]
pub type Mcrc64RegsCrcBctopld1 = crate::Reg<mcrc64_regs_crc_bctopld1::Mcrc64RegsCrcBctopld1Spec>;
#[doc = "CRC channel 1 Block Complete Timeout Preload Register B"]
pub mod mcrc64_regs_crc_bctopld1;
#[doc = "MCRC64_REGS_PSA_SIGREGL1 (rw) register accessor: Channel 1 PSA signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregl1`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGL1")]
pub type Mcrc64RegsPsaSigregl1 = crate::Reg<mcrc64_regs_psa_sigregl1::Mcrc64RegsPsaSigregl1Spec>;
#[doc = "Channel 1 PSA signature low register"]
pub mod mcrc64_regs_psa_sigregl1;
#[doc = "MCRC64_REGS_PSA_SIGREGH1 (rw) register accessor: Channel 1 PSA signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregh1`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGH1")]
pub type Mcrc64RegsPsaSigregh1 = crate::Reg<mcrc64_regs_psa_sigregh1::Mcrc64RegsPsaSigregh1Spec>;
#[doc = "Channel 1 PSA signature high register"]
pub mod mcrc64_regs_psa_sigregh1;
#[doc = "MCRC64_REGS_CRC_REGL1 (rw) register accessor: Channel 1 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regl1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGL1")]
pub type Mcrc64RegsCrcRegl1 = crate::Reg<mcrc64_regs_crc_regl1::Mcrc64RegsCrcRegl1Spec>;
#[doc = "Channel 1 CRC value low register"]
pub mod mcrc64_regs_crc_regl1;
#[doc = "MCRC64_REGS_CRC_REGH1 (rw) register accessor: Channel 1 CRC value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regh1`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGH1")]
pub type Mcrc64RegsCrcRegh1 = crate::Reg<mcrc64_regs_crc_regh1::Mcrc64RegsCrcRegh1Spec>;
#[doc = "Channel 1 CRC value high register"]
pub mod mcrc64_regs_crc_regh1;
#[doc = "MCRC64_REGS_PSA_SECSIGREGL1 (rw) register accessor: Channel 1 PSA sector signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregl1`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGL1")]
pub type Mcrc64RegsPsaSecsigregl1 =
    crate::Reg<mcrc64_regs_psa_secsigregl1::Mcrc64RegsPsaSecsigregl1Spec>;
#[doc = "Channel 1 PSA sector signature low register"]
pub mod mcrc64_regs_psa_secsigregl1;
#[doc = "MCRC64_REGS_PSA_SECSIGREGH1 (rw) register accessor: Channel 1 PSA sector signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregh1`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGH1")]
pub type Mcrc64RegsPsaSecsigregh1 =
    crate::Reg<mcrc64_regs_psa_secsigregh1::Mcrc64RegsPsaSecsigregh1Spec>;
#[doc = "Channel 1 PSA sector signature high register"]
pub mod mcrc64_regs_psa_secsigregh1;
#[doc = "MCRC64_REGS_RAW_DATAREGL1 (rw) register accessor: Channel 1 Raw Data Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregl1`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGL1")]
pub type Mcrc64RegsRawDataregl1 = crate::Reg<mcrc64_regs_raw_dataregl1::Mcrc64RegsRawDataregl1Spec>;
#[doc = "Channel 1 Raw Data Low Register"]
pub mod mcrc64_regs_raw_dataregl1;
#[doc = "MCRC64_REGS_RAW_DATAREGH1 (rw) register accessor: Channel 1 Raw Data High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregh1`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGH1")]
pub type Mcrc64RegsRawDataregh1 = crate::Reg<mcrc64_regs_raw_dataregh1::Mcrc64RegsRawDataregh1Spec>;
#[doc = "Channel 1 Raw Data High Register"]
pub mod mcrc64_regs_raw_dataregh1;
#[doc = "MCRC64_REGS_CRC_PCOUNT_REG2 (rw) register accessor: CRC Pattern Counter Preload Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_pcount_reg2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_PCOUNT_REG2")]
pub type Mcrc64RegsCrcPcountReg2 =
    crate::Reg<mcrc64_regs_crc_pcount_reg2::Mcrc64RegsCrcPcountReg2Spec>;
#[doc = "CRC Pattern Counter Preload Register2"]
pub mod mcrc64_regs_crc_pcount_reg2;
#[doc = "MCRC64_REGS_CRC_SCOUNT_REG2 (rw) register accessor: CRC Sector Counter Preload Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_scount_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_scount_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_scount_reg2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_SCOUNT_REG2")]
pub type Mcrc64RegsCrcScountReg2 =
    crate::Reg<mcrc64_regs_crc_scount_reg2::Mcrc64RegsCrcScountReg2Spec>;
#[doc = "CRC Sector Counter Preload Register2"]
pub mod mcrc64_regs_crc_scount_reg2;
#[doc = "MCRC64_REGS_CRC_CURSEC_REG2 (rw) register accessor: CRC Current Sector Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_cursec_reg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_cursec_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_cursec_reg2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CURSEC_REG2")]
pub type Mcrc64RegsCrcCursecReg2 =
    crate::Reg<mcrc64_regs_crc_cursec_reg2::Mcrc64RegsCrcCursecReg2Spec>;
#[doc = "CRC Current Sector Register 2"]
pub mod mcrc64_regs_crc_cursec_reg2;
#[doc = "MCRC64_REGS_CRC_WDTOPLD2 (rw) register accessor: CRC channel 2 Watchdog Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_wdtopld2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_wdtopld2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_wdtopld2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_WDTOPLD2")]
pub type Mcrc64RegsCrcWdtopld2 = crate::Reg<mcrc64_regs_crc_wdtopld2::Mcrc64RegsCrcWdtopld2Spec>;
#[doc = "CRC channel 2 Watchdog Timeout Preload Register"]
pub mod mcrc64_regs_crc_wdtopld2;
#[doc = "MCRC64_REGS_CRC_BCTOPLD2 (rw) register accessor: CRC channel 2 Block Complete Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_bctopld2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_bctopld2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_bctopld2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_BCTOPLD2")]
pub type Mcrc64RegsCrcBctopld2 = crate::Reg<mcrc64_regs_crc_bctopld2::Mcrc64RegsCrcBctopld2Spec>;
#[doc = "CRC channel 2 Block Complete Timeout Preload Register"]
pub mod mcrc64_regs_crc_bctopld2;
#[doc = "MCRC64_REGS_PSA_SIGREGL2 (rw) register accessor: Channel 2 PSA signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregl2`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGL2")]
pub type Mcrc64RegsPsaSigregl2 = crate::Reg<mcrc64_regs_psa_sigregl2::Mcrc64RegsPsaSigregl2Spec>;
#[doc = "Channel 2 PSA signature low register"]
pub mod mcrc64_regs_psa_sigregl2;
#[doc = "MCRC64_REGS_PSA_SIGREGH2 (rw) register accessor: Channel 2 PSA signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregh2`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGH2")]
pub type Mcrc64RegsPsaSigregh2 = crate::Reg<mcrc64_regs_psa_sigregh2::Mcrc64RegsPsaSigregh2Spec>;
#[doc = "Channel 2 PSA signature high register"]
pub mod mcrc64_regs_psa_sigregh2;
#[doc = "MCRC64_REGS_CRC_REGL2 (rw) register accessor: Channel 2 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regl2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGL2")]
pub type Mcrc64RegsCrcRegl2 = crate::Reg<mcrc64_regs_crc_regl2::Mcrc64RegsCrcRegl2Spec>;
#[doc = "Channel 2 CRC value low register"]
pub mod mcrc64_regs_crc_regl2;
#[doc = "MCRC64_REGS_CRC_REGH2 (rw) register accessor: Channel 2 CRC value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regh2`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGH2")]
pub type Mcrc64RegsCrcRegh2 = crate::Reg<mcrc64_regs_crc_regh2::Mcrc64RegsCrcRegh2Spec>;
#[doc = "Channel 2 CRC value high register"]
pub mod mcrc64_regs_crc_regh2;
#[doc = "MCRC64_REGS_PSA_SECSIGREGL2 (rw) register accessor: Channel 2 PSA sector signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregl2`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGL2")]
pub type Mcrc64RegsPsaSecsigregl2 =
    crate::Reg<mcrc64_regs_psa_secsigregl2::Mcrc64RegsPsaSecsigregl2Spec>;
#[doc = "Channel 2 PSA sector signature low register"]
pub mod mcrc64_regs_psa_secsigregl2;
#[doc = "MCRC64_REGS_PSA_SECSIGREGH2 (rw) register accessor: Channel 2 PSA sector signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregh2`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGH2")]
pub type Mcrc64RegsPsaSecsigregh2 =
    crate::Reg<mcrc64_regs_psa_secsigregh2::Mcrc64RegsPsaSecsigregh2Spec>;
#[doc = "Channel 2 PSA sector signature high register"]
pub mod mcrc64_regs_psa_secsigregh2;
#[doc = "MCRC64_REGS_RAW_DATAREGL2 (rw) register accessor: Channel 2 Raw Data Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregl2`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGL2")]
pub type Mcrc64RegsRawDataregl2 = crate::Reg<mcrc64_regs_raw_dataregl2::Mcrc64RegsRawDataregl2Spec>;
#[doc = "Channel 2 Raw Data Low Register"]
pub mod mcrc64_regs_raw_dataregl2;
#[doc = "MCRC64_REGS_RAW_DATAREGH2 (rw) register accessor: Channel 2 Raw Data High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregh2`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGH2")]
pub type Mcrc64RegsRawDataregh2 = crate::Reg<mcrc64_regs_raw_dataregh2::Mcrc64RegsRawDataregh2Spec>;
#[doc = "Channel 2 Raw Data High Register"]
pub mod mcrc64_regs_raw_dataregh2;
#[doc = "MCRC64_REGS_CRC_PCOUNT_REG3 (rw) register accessor: CRC Pattern Counter Preload Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_pcount_reg3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_PCOUNT_REG3")]
pub type Mcrc64RegsCrcPcountReg3 =
    crate::Reg<mcrc64_regs_crc_pcount_reg3::Mcrc64RegsCrcPcountReg3Spec>;
#[doc = "CRC Pattern Counter Preload Register3"]
pub mod mcrc64_regs_crc_pcount_reg3;
#[doc = "MCRC64_REGS_CRC_SCOUNT_REG3 (rw) register accessor: CRC Sector Counter Preload Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_scount_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_scount_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_scount_reg3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_SCOUNT_REG3")]
pub type Mcrc64RegsCrcScountReg3 =
    crate::Reg<mcrc64_regs_crc_scount_reg3::Mcrc64RegsCrcScountReg3Spec>;
#[doc = "CRC Sector Counter Preload Register3"]
pub mod mcrc64_regs_crc_scount_reg3;
#[doc = "MCRC64_REGS_CRC_CURSEC_REG3 (rw) register accessor: CRC Current Sector Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_cursec_reg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_cursec_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_cursec_reg3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CURSEC_REG3")]
pub type Mcrc64RegsCrcCursecReg3 =
    crate::Reg<mcrc64_regs_crc_cursec_reg3::Mcrc64RegsCrcCursecReg3Spec>;
#[doc = "CRC Current Sector Register 3"]
pub mod mcrc64_regs_crc_cursec_reg3;
#[doc = "MCRC64_REGS_CRC_WDTOPLD3 (rw) register accessor: CRC channel 3 Watchdog Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_wdtopld3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_wdtopld3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_wdtopld3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_WDTOPLD3")]
pub type Mcrc64RegsCrcWdtopld3 = crate::Reg<mcrc64_regs_crc_wdtopld3::Mcrc64RegsCrcWdtopld3Spec>;
#[doc = "CRC channel 3 Watchdog Timeout Preload Register"]
pub mod mcrc64_regs_crc_wdtopld3;
#[doc = "MCRC64_REGS_CRC_BCTOPLD3 (rw) register accessor: CRC channel 3 Block Complete Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_bctopld3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_bctopld3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_bctopld3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_BCTOPLD3")]
pub type Mcrc64RegsCrcBctopld3 = crate::Reg<mcrc64_regs_crc_bctopld3::Mcrc64RegsCrcBctopld3Spec>;
#[doc = "CRC channel 3 Block Complete Timeout Preload Register"]
pub mod mcrc64_regs_crc_bctopld3;
#[doc = "MCRC64_REGS_PSA_SIGREGL3 (rw) register accessor: Channel 3 PSA signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregl3`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGL3")]
pub type Mcrc64RegsPsaSigregl3 = crate::Reg<mcrc64_regs_psa_sigregl3::Mcrc64RegsPsaSigregl3Spec>;
#[doc = "Channel 3 PSA signature low register"]
pub mod mcrc64_regs_psa_sigregl3;
#[doc = "MCRC64_REGS_PSA_SIGREGH3 (rw) register accessor: Channel 3 PSA signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregh3`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGH3")]
pub type Mcrc64RegsPsaSigregh3 = crate::Reg<mcrc64_regs_psa_sigregh3::Mcrc64RegsPsaSigregh3Spec>;
#[doc = "Channel 3 PSA signature high register"]
pub mod mcrc64_regs_psa_sigregh3;
#[doc = "MCRC64_REGS_CRC_REGL3 (rw) register accessor: Channel 3 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regl3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGL3")]
pub type Mcrc64RegsCrcRegl3 = crate::Reg<mcrc64_regs_crc_regl3::Mcrc64RegsCrcRegl3Spec>;
#[doc = "Channel 3 CRC value low register"]
pub mod mcrc64_regs_crc_regl3;
#[doc = "MCRC64_REGS_CRC_REGH3 (rw) register accessor: Channel 3 CRC value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regh3`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGH3")]
pub type Mcrc64RegsCrcRegh3 = crate::Reg<mcrc64_regs_crc_regh3::Mcrc64RegsCrcRegh3Spec>;
#[doc = "Channel 3 CRC value high register"]
pub mod mcrc64_regs_crc_regh3;
#[doc = "MCRC64_REGS_PSA_SECSIGREGL3 (rw) register accessor: Channel 3 PSA sector signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregl3`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGL3")]
pub type Mcrc64RegsPsaSecsigregl3 =
    crate::Reg<mcrc64_regs_psa_secsigregl3::Mcrc64RegsPsaSecsigregl3Spec>;
#[doc = "Channel 3 PSA sector signature low register"]
pub mod mcrc64_regs_psa_secsigregl3;
#[doc = "MCRC64_REGS_PSA_SECSIGREGH3 (rw) register accessor: Channel 3 PSA sector signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregh3`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGH3")]
pub type Mcrc64RegsPsaSecsigregh3 =
    crate::Reg<mcrc64_regs_psa_secsigregh3::Mcrc64RegsPsaSecsigregh3Spec>;
#[doc = "Channel 3 PSA sector signature high register"]
pub mod mcrc64_regs_psa_secsigregh3;
#[doc = "MCRC64_REGS_RAW_DATAREGL3 (rw) register accessor: Channel 3 Raw Data Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregl3`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGL3")]
pub type Mcrc64RegsRawDataregl3 = crate::Reg<mcrc64_regs_raw_dataregl3::Mcrc64RegsRawDataregl3Spec>;
#[doc = "Channel 3 Raw Data Low Register"]
pub mod mcrc64_regs_raw_dataregl3;
#[doc = "MCRC64_REGS_RAW_DATAREGH3 (rw) register accessor: Channel 3 Raw Data High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregh3`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGH3")]
pub type Mcrc64RegsRawDataregh3 = crate::Reg<mcrc64_regs_raw_dataregh3::Mcrc64RegsRawDataregh3Spec>;
#[doc = "Channel 3 Raw Data High Register"]
pub mod mcrc64_regs_raw_dataregh3;
#[doc = "MCRC64_REGS_CRC_PCOUNT_REG4 (rw) register accessor: CRC Pattern Counter Preload Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_pcount_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_pcount_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_pcount_reg4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_PCOUNT_REG4")]
pub type Mcrc64RegsCrcPcountReg4 =
    crate::Reg<mcrc64_regs_crc_pcount_reg4::Mcrc64RegsCrcPcountReg4Spec>;
#[doc = "CRC Pattern Counter Preload Register4"]
pub mod mcrc64_regs_crc_pcount_reg4;
#[doc = "MCRC64_REGS_CRC_SCOUNT_REG4 (rw) register accessor: CRC Sector Counter Preload Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_scount_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_scount_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_scount_reg4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_SCOUNT_REG4")]
pub type Mcrc64RegsCrcScountReg4 =
    crate::Reg<mcrc64_regs_crc_scount_reg4::Mcrc64RegsCrcScountReg4Spec>;
#[doc = "CRC Sector Counter Preload Register4"]
pub mod mcrc64_regs_crc_scount_reg4;
#[doc = "MCRC64_REGS_CRC_CURSEC_REG4 (rw) register accessor: CRC Current Sector Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_cursec_reg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_cursec_reg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_cursec_reg4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_CURSEC_REG4")]
pub type Mcrc64RegsCrcCursecReg4 =
    crate::Reg<mcrc64_regs_crc_cursec_reg4::Mcrc64RegsCrcCursecReg4Spec>;
#[doc = "CRC Current Sector Register 4"]
pub mod mcrc64_regs_crc_cursec_reg4;
#[doc = "MCRC64_REGS_CRC_WDTOPLD4 (rw) register accessor: CRC channel 4 Watchdog Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_wdtopld4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_wdtopld4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_wdtopld4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_WDTOPLD4")]
pub type Mcrc64RegsCrcWdtopld4 = crate::Reg<mcrc64_regs_crc_wdtopld4::Mcrc64RegsCrcWdtopld4Spec>;
#[doc = "CRC channel 4 Watchdog Timeout Preload Register"]
pub mod mcrc64_regs_crc_wdtopld4;
#[doc = "MCRC64_REGS_CRC_BCTOPLD4 (rw) register accessor: CRC channel 4 Block Complete Timeout Preload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_bctopld4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_bctopld4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_bctopld4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_BCTOPLD4")]
pub type Mcrc64RegsCrcBctopld4 = crate::Reg<mcrc64_regs_crc_bctopld4::Mcrc64RegsCrcBctopld4Spec>;
#[doc = "CRC channel 4 Block Complete Timeout Preload Register"]
pub mod mcrc64_regs_crc_bctopld4;
#[doc = "MCRC64_REGS_PSA_SIGREGL4 (rw) register accessor: Channel 4 PSA signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregl4`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGL4")]
pub type Mcrc64RegsPsaSigregl4 = crate::Reg<mcrc64_regs_psa_sigregl4::Mcrc64RegsPsaSigregl4Spec>;
#[doc = "Channel 4 PSA signature low register"]
pub mod mcrc64_regs_psa_sigregl4;
#[doc = "MCRC64_REGS_PSA_SIGREGH4 (rw) register accessor: Channel 4 PSA signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_sigregh4`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SIGREGH4")]
pub type Mcrc64RegsPsaSigregh4 = crate::Reg<mcrc64_regs_psa_sigregh4::Mcrc64RegsPsaSigregh4Spec>;
#[doc = "Channel 4 PSA signature high register"]
pub mod mcrc64_regs_psa_sigregh4;
#[doc = "MCRC64_REGS_CRC_REGL4 (rw) register accessor: Channel 4 CRC value low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regl4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGL4")]
pub type Mcrc64RegsCrcRegl4 = crate::Reg<mcrc64_regs_crc_regl4::Mcrc64RegsCrcRegl4Spec>;
#[doc = "Channel 4 CRC value low register"]
pub mod mcrc64_regs_crc_regl4;
#[doc = "MCRC64_REGS_CRC_REGH4 (rw) register accessor: Channel 4 CRC value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_regh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_regh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_crc_regh4`]
module"]
#[doc(alias = "MCRC64_REGS_CRC_REGH4")]
pub type Mcrc64RegsCrcRegh4 = crate::Reg<mcrc64_regs_crc_regh4::Mcrc64RegsCrcRegh4Spec>;
#[doc = "Channel 4 CRC value high register"]
pub mod mcrc64_regs_crc_regh4;
#[doc = "MCRC64_REGS_PSA_SECSIGREGL4 (rw) register accessor: Channel 4 PSA sector signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregl4`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGL4")]
pub type Mcrc64RegsPsaSecsigregl4 =
    crate::Reg<mcrc64_regs_psa_secsigregl4::Mcrc64RegsPsaSecsigregl4Spec>;
#[doc = "Channel 4 PSA sector signature low register"]
pub mod mcrc64_regs_psa_secsigregl4;
#[doc = "MCRC64_REGS_PSA_SECSIGREGH4 (rw) register accessor: Channel 4 PSA sector signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_psa_secsigregh4`]
module"]
#[doc(alias = "MCRC64_REGS_PSA_SECSIGREGH4")]
pub type Mcrc64RegsPsaSecsigregh4 =
    crate::Reg<mcrc64_regs_psa_secsigregh4::Mcrc64RegsPsaSecsigregh4Spec>;
#[doc = "Channel 4 PSA sector signature high register"]
pub mod mcrc64_regs_psa_secsigregh4;
#[doc = "MCRC64_REGS_RAW_DATAREGL4 (rw) register accessor: Channel 4 Raw Data Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregl4`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGL4")]
pub type Mcrc64RegsRawDataregl4 = crate::Reg<mcrc64_regs_raw_dataregl4::Mcrc64RegsRawDataregl4Spec>;
#[doc = "Channel 4 Raw Data Low Register"]
pub mod mcrc64_regs_raw_dataregl4;
#[doc = "MCRC64_REGS_RAW_DATAREGH4 (rw) register accessor: Channel 4 Raw Data High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_raw_dataregh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_raw_dataregh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_raw_dataregh4`]
module"]
#[doc(alias = "MCRC64_REGS_RAW_DATAREGH4")]
pub type Mcrc64RegsRawDataregh4 = crate::Reg<mcrc64_regs_raw_dataregh4::Mcrc64RegsRawDataregh4Spec>;
#[doc = "Channel 4 Raw Data High Register"]
pub mod mcrc64_regs_raw_dataregh4;
#[doc = "MCRC64_REGS_MCRC_BUS_SEL (rw) register accessor: Data bus tracing selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_mcrc_bus_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_mcrc_bus_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_mcrc_bus_sel`]
module"]
#[doc(alias = "MCRC64_REGS_MCRC_BUS_SEL")]
pub type Mcrc64RegsMcrcBusSel = crate::Reg<mcrc64_regs_mcrc_bus_sel::Mcrc64RegsMcrcBusSelSpec>;
#[doc = "Data bus tracing selection"]
pub mod mcrc64_regs_mcrc_bus_sel;
#[doc = "MCRC64_REGS_I0_PSA_SIGREG1_CPY (rw) register accessor: Region for Channel 1 PSA signature block used by DMA based systems.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_i0_psa_sigreg1_cpy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_i0_psa_sigreg1_cpy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_i0_psa_sigreg1_cpy`]
module"]
#[doc(alias = "MCRC64_REGS_I0_PSA_SIGREG1_CPY")]
pub type Mcrc64RegsI0PsaSigreg1Cpy =
    crate::Reg<mcrc64_regs_i0_psa_sigreg1_cpy::Mcrc64RegsI0PsaSigreg1CpySpec>;
#[doc = "Region for Channel 1 PSA signature block used by DMA based systems."]
pub mod mcrc64_regs_i0_psa_sigreg1_cpy;
#[doc = "MCRC64_REGS_I0_PSA_SIGREG2_CPY (rw) register accessor: Region for Channel 2 PSA signature block used by DMA based systems.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_i0_psa_sigreg2_cpy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_i0_psa_sigreg2_cpy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_i0_psa_sigreg2_cpy`]
module"]
#[doc(alias = "MCRC64_REGS_I0_PSA_SIGREG2_CPY")]
pub type Mcrc64RegsI0PsaSigreg2Cpy =
    crate::Reg<mcrc64_regs_i0_psa_sigreg2_cpy::Mcrc64RegsI0PsaSigreg2CpySpec>;
#[doc = "Region for Channel 2 PSA signature block used by DMA based systems."]
pub mod mcrc64_regs_i0_psa_sigreg2_cpy;
#[doc = "MCRC64_REGS_I0_PSA_SIGREG3_CPY (rw) register accessor: Region for Channel 3 PSA signature block used by DMA based systems.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_i0_psa_sigreg3_cpy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_i0_psa_sigreg3_cpy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_i0_psa_sigreg3_cpy`]
module"]
#[doc(alias = "MCRC64_REGS_I0_PSA_SIGREG3_CPY")]
pub type Mcrc64RegsI0PsaSigreg3Cpy =
    crate::Reg<mcrc64_regs_i0_psa_sigreg3_cpy::Mcrc64RegsI0PsaSigreg3CpySpec>;
#[doc = "Region for Channel 3 PSA signature block used by DMA based systems."]
pub mod mcrc64_regs_i0_psa_sigreg3_cpy;
#[doc = "MCRC64_REGS_I0_PSA_SIGREG4_CPY (rw) register accessor: Region for Channel 4 PSA signature block used by DMA based systems.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_i0_psa_sigreg4_cpy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_i0_psa_sigreg4_cpy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcrc64_regs_i0_psa_sigreg4_cpy`]
module"]
#[doc(alias = "MCRC64_REGS_I0_PSA_SIGREG4_CPY")]
pub type Mcrc64RegsI0PsaSigreg4Cpy =
    crate::Reg<mcrc64_regs_i0_psa_sigreg4_cpy::Mcrc64RegsI0PsaSigreg4CpySpec>;
#[doc = "Region for Channel 4 PSA signature block used by DMA based systems."]
pub mod mcrc64_regs_i0_psa_sigreg4_cpy;
