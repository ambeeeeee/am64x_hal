#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_capabilities` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_capabilities` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec>;
#[doc = "Field `TIMEOUT_CLK_FREQ` reader - 5:0\\]
This bit shows the base clock frequency used to detect Data Timeout Error. '000000' Get Information via another method, 'not 0' 1KHz to 63KHz/1MHz to 63MHz"]
pub type TimeoutClkFreqR = crate::FieldReader;
#[doc = "Field `TIMEOUT_CLK_FREQ` writer - 5:0\\]
This bit shows the base clock frequency used to detect Data Timeout Error. '000000' Get Information via another method, 'not 0' 1KHz to 63KHz/1MHz to 63MHz"]
pub type TimeoutClkFreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TIMEOUT_CLK_UNIT` reader - 7:7\\]
This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
pub type TimeoutClkUnitR = crate::BitReader;
#[doc = "Field `TIMEOUT_CLK_UNIT` writer - 7:7\\]
This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
pub type TimeoutClkUnitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BASE_CLK_FREQ` reader - 15:8\\]
\\[1\\]6-bit Base Clock Frequency: This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. '11xx xxxxb' Not Supported '0011 1111b' 63MHz '0000 0010b' 2MHz '0000 0001b' 1MHz '0000 0000b'Get Information via another method \\[2\\]8-bit Base Clock Frequency: This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. 'FFh' 255MHz '02h' 2MHz '01h' 1MHz '00h' Get Information via another method. If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b \\[17MHz\\]
because the Host Driver use this value to calculate the clock divider value \\[Refer to the SDCLK Frequency Select in the Clock Control register.\\]
and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method."]
pub type BaseClkFreqR = crate::FieldReader;
#[doc = "Field `BASE_CLK_FREQ` writer - 15:8\\]
\\[1\\]6-bit Base Clock Frequency: This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. '11xx xxxxb' Not Supported '0011 1111b' 63MHz '0000 0010b' 2MHz '0000 0001b' 1MHz '0000 0000b'Get Information via another method \\[2\\]8-bit Base Clock Frequency: This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. 'FFh' 255MHz '02h' 2MHz '01h' 1MHz '00h' Get Information via another method. If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b \\[17MHz\\]
because the Host Driver use this value to calculate the clock divider value \\[Refer to the SDCLK Frequency Select in the Clock Control register.\\]
and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method."]
pub type BaseClkFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAX_BLK_LENGTH` reader - 17:16\\]
This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
pub type MaxBlkLengthR = crate::FieldReader;
#[doc = "Field `MAX_BLK_LENGTH` writer - 17:16\\]
This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
pub type MaxBlkLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUS_8BIT_SUPPORT` reader - 18:18\\]
This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister."]
pub type Bus8bitSupportR = crate::BitReader;
#[doc = "Field `BUS_8BIT_SUPPORT` writer - 18:18\\]
This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister."]
pub type Bus8bitSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA2_SUPPORT` reader - 19:19\\]
'0' ADMA2 Not Supported '1' ADMA2 Supported"]
pub type Adma2SupportR = crate::BitReader;
#[doc = "Field `ADMA2_SUPPORT` writer - 19:19\\]
'0' ADMA2 Not Supported '1' ADMA2 Supported"]
pub type Adma2SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGH_SPEED_SUPPORT` reader - 21:21\\]
This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz \\[for SD\\]/ 20MHz to 52MHz \\[for MMC\\]."]
pub type HighSpeedSupportR = crate::BitReader;
#[doc = "Field `HIGH_SPEED_SUPPORT` writer - 21:21\\]
This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz \\[for SD\\]/ 20MHz to 52MHz \\[for MMC\\]."]
pub type HighSpeedSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMA_SUPPORT` reader - 22:22\\]
This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly.Version 4.10 Host Controller shall support SDMA if ADMA2 is supported."]
pub type SdmaSupportR = crate::BitReader;
#[doc = "Field `SDMA_SUPPORT` writer - 22:22\\]
This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly.Version 4.10 Host Controller shall support SDMA if ADMA2 is supported."]
pub type SdmaSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP_RES_SUPPORT` reader - 23:23\\]
This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
pub type SuspResSupportR = crate::BitReader;
#[doc = "Field `SUSP_RES_SUPPORT` writer - 23:23\\]
This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
pub type SuspResSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOLT_3P3_SUPPORT` reader - 24:24\\]
This bit indicates whether the HC supports 3.3V."]
pub type Volt3p3SupportR = crate::BitReader;
#[doc = "Field `VOLT_3P3_SUPPORT` writer - 24:24\\]
This bit indicates whether the HC supports 3.3V."]
pub type Volt3p3SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOLT_3P0_SUPPORT` reader - 25:25\\]
This bit indicates whether the HC supports 3.0V."]
pub type Volt3p0SupportR = crate::BitReader;
#[doc = "Field `VOLT_3P0_SUPPORT` writer - 25:25\\]
This bit indicates whether the HC supports 3.0V."]
pub type Volt3p0SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOLT_1P8_SUPPORT` reader - 26:26\\]
This bit indicates whether the HC supports 1.8V."]
pub type Volt1p8SupportR = crate::BitReader;
#[doc = "Field `VOLT_1P8_SUPPORT` writer - 26:26\\]
This bit indicates whether the HC supports 1.8V."]
pub type Volt1p8SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_64BIT_SUPPORT_V4` reader - 27:27\\]
This bit is added from Version 4.10. Set-ting 1 to this bit indicates that the Host Controller supports 64-bit System Addressing of Version 4 mode \\[Refer to Table 2-35 for the summary of 64-bit sys-tem address support\\].. When this bit is set to 1, full or a part of 64-bit address should be used to decode Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller Registers is effective regardless of setting to 64bit Addressing in Host Control 2. If this bit is set to 1, 64-bit DMA Address-ing for Version 4 is enabled by setting Host Version 4 Enable =1, 64-bit Address-ing =1 in the Host Control 2 register.SDMA can be used and ADMA2 uses 128-bit Descriptor."]
pub type Addr64bitSupportV4R = crate::BitReader;
#[doc = "Field `ADDR_64BIT_SUPPORT_V4` writer - 27:27\\]
This bit is added from Version 4.10. Set-ting 1 to this bit indicates that the Host Controller supports 64-bit System Addressing of Version 4 mode \\[Refer to Table 2-35 for the summary of 64-bit sys-tem address support\\].. When this bit is set to 1, full or a part of 64-bit address should be used to decode Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller Registers is effective regardless of setting to 64bit Addressing in Host Control 2. If this bit is set to 1, 64-bit DMA Address-ing for Version 4 is enabled by setting Host Version 4 Enable =1, 64-bit Address-ing =1 in the Host Control 2 register.SDMA can be used and ADMA2 uses 128-bit Descriptor."]
pub type Addr64bitSupportV4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_64BIT_SUPPORT_V3` reader - 28:28\\]
IMeaning of this bit is different depends on Versions \\[Refer to Table 2-35 for more details\\]. Host Controller Version 3.00 and Ver4.10 use this bit as 64-bit System Address support for V3 mode. Host Con- troller Version 4.00 uses this bit as 64-bit System Address support for both V3 and V4 modes. SDMA cannot be used in 64-bit Address-ing in Version 3 mode. If this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor may be enabled as follows: In case of Host Controller Version 3, 64-bit ADMA2 is enabled by DMA Select =11b in the Host Control 1 register. In case of Host Controller Version 4, 64-bitADMA2 for Version 3 is enabled by setting Host Version 4 Enable =0 and DMA Select = 11b. 1 - 64-bit System Address for V3 is Supported 0 - 64-bit System Address for V3 is not Supported"]
pub type Addr64bitSupportV3R = crate::BitReader;
#[doc = "Field `ADDR_64BIT_SUPPORT_V3` writer - 28:28\\]
IMeaning of this bit is different depends on Versions \\[Refer to Table 2-35 for more details\\]. Host Controller Version 3.00 and Ver4.10 use this bit as 64-bit System Address support for V3 mode. Host Con- troller Version 4.00 uses this bit as 64-bit System Address support for both V3 and V4 modes. SDMA cannot be used in 64-bit Address-ing in Version 3 mode. If this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor may be enabled as follows: In case of Host Controller Version 3, 64-bit ADMA2 is enabled by DMA Select =11b in the Host Control 1 register. In case of Host Controller Version 4, 64-bitADMA2 for Version 3 is enabled by setting Host Version 4 Enable =0 and DMA Select = 11b. 1 - 64-bit System Address for V3 is Supported 0 - 64-bit System Address for V3 is not Supported"]
pub type Addr64bitSupportV3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH_INTR_SUPPORT` reader - 29:29\\]
Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
pub type AsynchIntrSupportR = crate::BitReader;
#[doc = "Field `ASYNCH_INTR_SUPPORT` writer - 29:29\\]
Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
pub type AsynchIntrSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOT_TYPE` reader - 31:30\\]
This field indicates usage of a slot by a specific Host System. \\[A host controller register set is defined perslot.\\]
Embedded slot for one device \\[01b\\]
means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot \\[10b\\]
can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus \\[10b\\], the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
pub type SlotTypeR = crate::FieldReader;
#[doc = "Field `SLOT_TYPE` writer - 31:30\\]
This field indicates usage of a slot by a specific Host System. \\[A host controller register set is defined perslot.\\]
Embedded slot for one device \\[01b\\]
means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot \\[10b\\]
can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus \\[10b\\], the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
pub type SlotTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDR50_SUPPORT` reader - 32:32\\]
If SDR104 is supported, this bit shall be set to 1. Bit 40 indicates whether SDR50 requires tuning or not."]
pub type Sdr50SupportR = crate::BitReader;
#[doc = "Field `SDR50_SUPPORT` writer - 32:32\\]
If SDR104 is supported, this bit shall be set to 1. Bit 40 indicates whether SDR50 requires tuning or not."]
pub type Sdr50SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR104_SUPPORT` reader - 33:33\\]
This bit indicates whether SDR104 is supported or not.SDR104 requires tuning."]
pub type Sdr104SupportR = crate::BitReader;
#[doc = "Field `SDR104_SUPPORT` writer - 33:33\\]
This bit indicates whether SDR104 is supported or not.SDR104 requires tuning."]
pub type Sdr104SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR50_SUPPORT` reader - 34:34\\]
This bit indicates whether DDR50 is supported or not."]
pub type Ddr50SupportR = crate::BitReader;
#[doc = "Field `DDR50_SUPPORT` writer - 34:34\\]
This bit indicates whether DDR50 is supported or not."]
pub type Ddr50SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_SUPPORT` reader - 35:35\\]
This bit indicates whether Host controller supports UHS-II. If this bit is set to 1, 1.8V VDD2 Support shall be set to 1 \\[Host Sys- tem shall support VDD2 power supply\\]. 1 UHS-II is Supported 0 UHS-II is Not Supported"]
pub type Uhs2SupportR = crate::BitReader;
#[doc = "Field `UHS2_SUPPORT` writer - 35:35\\]
This bit indicates whether Host controller supports UHS-II. If this bit is set to 1, 1.8V VDD2 Support shall be set to 1 \\[Host Sys- tem shall support VDD2 power supply\\]. 1 UHS-II is Supported 0 UHS-II is Not Supported"]
pub type Uhs2SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVERA_SUPPORT` reader - 36:36\\]
This bit indicates support of Driver Type A for 1.8 Signaling. '0' Driver Type A is Not supported, '1' Driver Type A is supported"]
pub type DriveraSupportR = crate::BitReader;
#[doc = "Field `DRIVERA_SUPPORT` writer - 36:36\\]
This bit indicates support of Driver Type A for 1.8 Signaling. '0' Driver Type A is Not supported, '1' Driver Type A is supported"]
pub type DriveraSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVERC_SUPPORT` reader - 37:37\\]
This bit indicates support of Driver Type C for 1.8 Signaling. '0' Driver Type C is Not supported, '1' Driver Type C is supported"]
pub type DrivercSupportR = crate::BitReader;
#[doc = "Field `DRIVERC_SUPPORT` writer - 37:37\\]
This bit indicates support of Driver Type C for 1.8 Signaling. '0' Driver Type C is Not supported, '1' Driver Type C is supported"]
pub type DrivercSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVERD_SUPPORT` reader - 38:38\\]
This bit indicates support of Driver Type D for 1.8 Signaling. '0' Driver Type D is Not supported, '1' Driver Type D is supported"]
pub type DriverdSupportR = crate::BitReader;
#[doc = "Field `DRIVERD_SUPPORT` writer - 38:38\\]
This bit indicates support of Driver Type D for 1.8 Signaling. '0' Driver Type D is Not supported, '1' Driver Type D is supported"]
pub type DriverdSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING_TIMER_CNT` reader - 43:40\\]
This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source 1h = 1 seconds 2h = 2 seconds 3h = 4 seconds 4h = 8 seconds ------ n = 2\\[n-1\\]
seconds ------ Bh = 1024 seconds Fh - Ch = Reserved"]
pub type RetuningTimerCntR = crate::FieldReader;
#[doc = "Field `RETUNING_TIMER_CNT` writer - 43:40\\]
This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source 1h = 1 seconds 2h = 2 seconds 3h = 4 seconds 4h = 8 seconds ------ n = 2\\[n-1\\]
seconds ------ Bh = 1024 seconds Fh - Ch = Reserved"]
pub type RetuningTimerCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TUNING_FOR_SDR50` reader - 45:45\\]
If this bit is set to 1, this Host Controller requires tuning to operate SDR50. \\[Tuning is always required to operate SDR104\\]. '0' '1'"]
pub type TuningForSdr50R = crate::BitReader;
#[doc = "Field `TUNING_FOR_SDR50` writer - 45:45\\]
If this bit is set to 1, this Host Controller requires tuning to operate SDR50. \\[Tuning is always required to operate SDR104\\]. '0' '1'"]
pub type TuningForSdr50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING_MODES` reader - 47:46\\]
This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver. '00' Mode 1 '01' Mode 2 '10' Mode 3 '11' Reserved. There are two re-tuning timings:Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue."]
pub type RetuningModesR = crate::FieldReader;
#[doc = "Field `RETUNING_MODES` writer - 47:46\\]
This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver. '00' Mode 1 '01' Mode 2 '10' Mode 3 '11' Reserved. There are two re-tuning timings:Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue."]
pub type RetuningModesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLOCK_MULTIPLIER` reader - 55:48\\]
This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. 'FF' Clock Multiplier M = 256 --------- '02' Clock Multiplier M = 3 '01' Clock Multiplier M = 2 '00' Clock Multiplier is Not Supported."]
pub type ClockMultiplierR = crate::FieldReader;
#[doc = "Field `CLOCK_MULTIPLIER` writer - 55:48\\]
This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. 'FF' Clock Multiplier M = 256 --------- '02' Clock Multiplier M = 3 '01' Clock Multiplier M = 2 '00' Clock Multiplier is Not Supported."]
pub type ClockMultiplierW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_SUPPORT` reader - 56:56\\]
This field indicates whether SPI Mode is supported or not."]
pub type SpiSupportR = crate::BitReader;
#[doc = "Field `SPI_SUPPORT` writer - 56:56\\]
This field indicates whether SPI Mode is supported or not."]
pub type SpiSupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_BLK_MODE` reader - 57:57\\]
This field indicates whether SPI Block Mode is supported or not."]
pub type SpiBlkModeR = crate::BitReader;
#[doc = "Field `SPI_BLK_MODE` writer - 57:57\\]
This field indicates whether SPI Block Mode is supported or not."]
pub type SpiBlkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA3_SUPPORT` reader - 59:59\\]
This field indicates that support of ADMA3 on Host Controller."]
pub type Adma3SupportR = crate::BitReader;
#[doc = "Field `ADMA3_SUPPORT` writer - 59:59\\]
This field indicates that support of ADMA3 on Host Controller."]
pub type Adma3SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD2_1P8_SUPPORT` reader - 60:60\\]
This field indicates that support of VDD2 on Host system."]
pub type Vdd2_1p8SupportR = crate::BitReader;
#[doc = "Field `VDD2_1P8_SUPPORT` writer - 60:60\\]
This field indicates that support of VDD2 on Host system."]
pub type Vdd2_1p8SupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS400_SUPPORT` reader - 63:63\\]
1 HS400 is Supported 0 HS400 is Not Supported"]
pub type Hs400SupportR = crate::BitReader;
#[doc = "Field `HS400_SUPPORT` writer - 63:63\\]
1 HS400 is Supported 0 HS400 is Not Supported"]
pub type Hs400SupportW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
This bit shows the base clock frequency used to detect Data Timeout Error. '000000' Get Information via another method, 'not 0' 1KHz to 63KHz/1MHz to 63MHz"]
    #[inline(always)]
    pub fn timeout_clk_freq(&self) -> TimeoutClkFreqR {
        TimeoutClkFreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
    #[inline(always)]
    pub fn timeout_clk_unit(&self) -> TimeoutClkUnitR {
        TimeoutClkUnitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
\\[1\\]6-bit Base Clock Frequency: This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. '11xx xxxxb' Not Supported '0011 1111b' 63MHz '0000 0010b' 2MHz '0000 0001b' 1MHz '0000 0000b'Get Information via another method \\[2\\]8-bit Base Clock Frequency: This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. 'FFh' 255MHz '02h' 2MHz '01h' 1MHz '00h' Get Information via another method. If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b \\[17MHz\\]
because the Host Driver use this value to calculate the clock divider value \\[Refer to the SDCLK Frequency Select in the Clock Control register.\\]
and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method."]
    #[inline(always)]
    pub fn base_clk_freq(&self) -> BaseClkFreqR {
        BaseClkFreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
    #[inline(always)]
    pub fn max_blk_length(&self) -> MaxBlkLengthR {
        MaxBlkLengthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister."]
    #[inline(always)]
    pub fn bus_8bit_support(&self) -> Bus8bitSupportR {
        Bus8bitSupportR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
'0' ADMA2 Not Supported '1' ADMA2 Supported"]
    #[inline(always)]
    pub fn adma2_support(&self) -> Adma2SupportR {
        Adma2SupportR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz \\[for SD\\]/ 20MHz to 52MHz \\[for MMC\\]."]
    #[inline(always)]
    pub fn high_speed_support(&self) -> HighSpeedSupportR {
        HighSpeedSupportR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly.Version 4.10 Host Controller shall support SDMA if ADMA2 is supported."]
    #[inline(always)]
    pub fn sdma_support(&self) -> SdmaSupportR {
        SdmaSupportR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
    #[inline(always)]
    pub fn susp_res_support(&self) -> SuspResSupportR {
        SuspResSupportR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit indicates whether the HC supports 3.3V."]
    #[inline(always)]
    pub fn volt_3p3_support(&self) -> Volt3p3SupportR {
        Volt3p3SupportR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
This bit indicates whether the HC supports 3.0V."]
    #[inline(always)]
    pub fn volt_3p0_support(&self) -> Volt3p0SupportR {
        Volt3p0SupportR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
This bit indicates whether the HC supports 1.8V."]
    #[inline(always)]
    pub fn volt_1p8_support(&self) -> Volt1p8SupportR {
        Volt1p8SupportR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
This bit is added from Version 4.10. Set-ting 1 to this bit indicates that the Host Controller supports 64-bit System Addressing of Version 4 mode \\[Refer to Table 2-35 for the summary of 64-bit sys-tem address support\\].. When this bit is set to 1, full or a part of 64-bit address should be used to decode Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller Registers is effective regardless of setting to 64bit Addressing in Host Control 2. If this bit is set to 1, 64-bit DMA Address-ing for Version 4 is enabled by setting Host Version 4 Enable =1, 64-bit Address-ing =1 in the Host Control 2 register.SDMA can be used and ADMA2 uses 128-bit Descriptor."]
    #[inline(always)]
    pub fn addr_64bit_support_v4(&self) -> Addr64bitSupportV4R {
        Addr64bitSupportV4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
IMeaning of this bit is different depends on Versions \\[Refer to Table 2-35 for more details\\]. Host Controller Version 3.00 and Ver4.10 use this bit as 64-bit System Address support for V3 mode. Host Con- troller Version 4.00 uses this bit as 64-bit System Address support for both V3 and V4 modes. SDMA cannot be used in 64-bit Address-ing in Version 3 mode. If this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor may be enabled as follows: In case of Host Controller Version 3, 64-bit ADMA2 is enabled by DMA Select =11b in the Host Control 1 register. In case of Host Controller Version 4, 64-bitADMA2 for Version 3 is enabled by setting Host Version 4 Enable =0 and DMA Select = 11b. 1 - 64-bit System Address for V3 is Supported 0 - 64-bit System Address for V3 is not Supported"]
    #[inline(always)]
    pub fn addr_64bit_support_v3(&self) -> Addr64bitSupportV3R {
        Addr64bitSupportV3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
    #[inline(always)]
    pub fn asynch_intr_support(&self) -> AsynchIntrSupportR {
        AsynchIntrSupportR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This field indicates usage of a slot by a specific Host System. \\[A host controller register set is defined perslot.\\]
Embedded slot for one device \\[01b\\]
means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot \\[10b\\]
can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus \\[10b\\], the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
    #[inline(always)]
    pub fn slot_type(&self) -> SlotTypeR {
        SlotTypeR::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bit 32 - 32:32\\]
If SDR104 is supported, this bit shall be set to 1. Bit 40 indicates whether SDR50 requires tuning or not."]
    #[inline(always)]
    pub fn sdr50_support(&self) -> Sdr50SupportR {
        Sdr50SupportR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - 33:33\\]
This bit indicates whether SDR104 is supported or not.SDR104 requires tuning."]
    #[inline(always)]
    pub fn sdr104_support(&self) -> Sdr104SupportR {
        Sdr104SupportR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bit 34 - 34:34\\]
This bit indicates whether DDR50 is supported or not."]
    #[inline(always)]
    pub fn ddr50_support(&self) -> Ddr50SupportR {
        Ddr50SupportR::new(((self.bits >> 34) & 1) != 0)
    }
    #[doc = "Bit 35 - 35:35\\]
This bit indicates whether Host controller supports UHS-II. If this bit is set to 1, 1.8V VDD2 Support shall be set to 1 \\[Host Sys- tem shall support VDD2 power supply\\]. 1 UHS-II is Supported 0 UHS-II is Not Supported"]
    #[inline(always)]
    pub fn uhs2_support(&self) -> Uhs2SupportR {
        Uhs2SupportR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - 36:36\\]
This bit indicates support of Driver Type A for 1.8 Signaling. '0' Driver Type A is Not supported, '1' Driver Type A is supported"]
    #[inline(always)]
    pub fn drivera_support(&self) -> DriveraSupportR {
        DriveraSupportR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - 37:37\\]
This bit indicates support of Driver Type C for 1.8 Signaling. '0' Driver Type C is Not supported, '1' Driver Type C is supported"]
    #[inline(always)]
    pub fn driverc_support(&self) -> DrivercSupportR {
        DrivercSupportR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - 38:38\\]
This bit indicates support of Driver Type D for 1.8 Signaling. '0' Driver Type D is Not supported, '1' Driver Type D is supported"]
    #[inline(always)]
    pub fn driverd_support(&self) -> DriverdSupportR {
        DriverdSupportR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bits 40:43 - 43:40\\]
This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source 1h = 1 seconds 2h = 2 seconds 3h = 4 seconds 4h = 8 seconds ------ n = 2\\[n-1\\]
seconds ------ Bh = 1024 seconds Fh - Ch = Reserved"]
    #[inline(always)]
    pub fn retuning_timer_cnt(&self) -> RetuningTimerCntR {
        RetuningTimerCntR::new(((self.bits >> 40) & 0x0f) as u8)
    }
    #[doc = "Bit 45 - 45:45\\]
If this bit is set to 1, this Host Controller requires tuning to operate SDR50. \\[Tuning is always required to operate SDR104\\]. '0' '1'"]
    #[inline(always)]
    pub fn tuning_for_sdr50(&self) -> TuningForSdr50R {
        TuningForSdr50R::new(((self.bits >> 45) & 1) != 0)
    }
    #[doc = "Bits 46:47 - 47:46\\]
This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver. '00' Mode 1 '01' Mode 2 '10' Mode 3 '11' Reserved. There are two re-tuning timings:Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue."]
    #[inline(always)]
    pub fn retuning_modes(&self) -> RetuningModesR {
        RetuningModesR::new(((self.bits >> 46) & 3) as u8)
    }
    #[doc = "Bits 48:55 - 55:48\\]
This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. 'FF' Clock Multiplier M = 256 --------- '02' Clock Multiplier M = 3 '01' Clock Multiplier M = 2 '00' Clock Multiplier is Not Supported."]
    #[inline(always)]
    pub fn clock_multiplier(&self) -> ClockMultiplierR {
        ClockMultiplierR::new(((self.bits >> 48) & 0xff) as u8)
    }
    #[doc = "Bit 56 - 56:56\\]
This field indicates whether SPI Mode is supported or not."]
    #[inline(always)]
    pub fn spi_support(&self) -> SpiSupportR {
        SpiSupportR::new(((self.bits >> 56) & 1) != 0)
    }
    #[doc = "Bit 57 - 57:57\\]
This field indicates whether SPI Block Mode is supported or not."]
    #[inline(always)]
    pub fn spi_blk_mode(&self) -> SpiBlkModeR {
        SpiBlkModeR::new(((self.bits >> 57) & 1) != 0)
    }
    #[doc = "Bit 59 - 59:59\\]
This field indicates that support of ADMA3 on Host Controller."]
    #[inline(always)]
    pub fn adma3_support(&self) -> Adma3SupportR {
        Adma3SupportR::new(((self.bits >> 59) & 1) != 0)
    }
    #[doc = "Bit 60 - 60:60\\]
This field indicates that support of VDD2 on Host system."]
    #[inline(always)]
    pub fn vdd2_1p8_support(&self) -> Vdd2_1p8SupportR {
        Vdd2_1p8SupportR::new(((self.bits >> 60) & 1) != 0)
    }
    #[doc = "Bit 63 - 63:63\\]
1 HS400 is Supported 0 HS400 is Not Supported"]
    #[inline(always)]
    pub fn hs400_support(&self) -> Hs400SupportR {
        Hs400SupportR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
This bit shows the base clock frequency used to detect Data Timeout Error. '000000' Get Information via another method, 'not 0' 1KHz to 63KHz/1MHz to 63MHz"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_clk_freq(&mut self) -> TimeoutClkFreqW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        TimeoutClkFreqW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit shows the unit of base clock frequency used to detect Data Timeout Error."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_clk_unit(&mut self) -> TimeoutClkUnitW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        TimeoutClkUnitW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
\\[1\\]6-bit Base Clock Frequency: This mode is supported by the Host Controller Version 1.00 and 2.00. Upper 2-bit is not effective and always 0. Unit values are 1MHz. The supported clock range is 10MHz to 63MHz. '11xx xxxxb' Not Supported '0011 1111b' 63MHz '0000 0010b' 2MHz '0000 0001b' 1MHz '0000 0000b'Get Information via another method \\[2\\]8-bit Base Clock Frequency: This mode is supported by the Host Controller Version 3.00.Unit values are 1MHz. The supported clock range is 10MHz to 255MHz. 'FFh' 255MHz '02h' 2MHz '01h' 1MHz '00h' Get Information via another method. If the real frequency is 16.5MHz, the lager value shall be set 0001 0001b \\[17MHz\\]
because the Host Driver use this value to calculate the clock divider value \\[Refer to the SDCLK Frequency Select in the Clock Control register.\\]
and it shall not exceed upper limit of the SD Clock frequency. If these bits are all 0, the Host System has to get information via another method."]
    #[inline(always)]
    #[must_use]
    pub fn base_clk_freq(&mut self) -> BaseClkFreqW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        BaseClkFreqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
This value indicates the maximum block size that the HD can read and write to the buffer in the HC. The buffer shall transfer this block size without wait cycles. Three sizes can be defined as indicated below."]
    #[inline(always)]
    #[must_use]
    pub fn max_blk_length(&mut self) -> MaxBlkLengthW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        MaxBlkLengthW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
This bit indicates whether the Host Controller is capable of using 8-bit bus width mode. This bit is not effective when Slot Type is set to 10b. In this case, refer to Bus Width Preset in the Shared Bus resister."]
    #[inline(always)]
    #[must_use]
    pub fn bus_8bit_support(&mut self) -> Bus8bitSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Bus8bitSupportW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
'0' ADMA2 Not Supported '1' ADMA2 Supported"]
    #[inline(always)]
    #[must_use]
    pub fn adma2_support(&mut self) -> Adma2SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Adma2SupportW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
This bit indicates whether the HC and the Host System support High Speed mode and they can supply SD Clock frequency from 25Mhz to 50 Mhz \\[for SD\\]/ 20MHz to 52MHz \\[for MMC\\]."]
    #[inline(always)]
    #[must_use]
    pub fn high_speed_support(
        &mut self,
    ) -> HighSpeedSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        HighSpeedSupportW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
This bit indicates whether the HC is capable of using DMA to transfer data between system memory and the HC directly.Version 4.10 Host Controller shall support SDMA if ADMA2 is supported."]
    #[inline(always)]
    #[must_use]
    pub fn sdma_support(&mut self) -> SdmaSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        SdmaSupportW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
This bit indicates whether the HC supports Suspend / Resume functionality. If this bit is 0, the Suspend and Resume mechanism are not supported and the HD shall not issue either Suspend / Resume commands."]
    #[inline(always)]
    #[must_use]
    pub fn susp_res_support(&mut self) -> SuspResSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        SuspResSupportW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
This bit indicates whether the HC supports 3.3V."]
    #[inline(always)]
    #[must_use]
    pub fn volt_3p3_support(&mut self) -> Volt3p3SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Volt3p3SupportW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
This bit indicates whether the HC supports 3.0V."]
    #[inline(always)]
    #[must_use]
    pub fn volt_3p0_support(&mut self) -> Volt3p0SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Volt3p0SupportW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
This bit indicates whether the HC supports 1.8V."]
    #[inline(always)]
    #[must_use]
    pub fn volt_1p8_support(&mut self) -> Volt1p8SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Volt1p8SupportW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
This bit is added from Version 4.10. Set-ting 1 to this bit indicates that the Host Controller supports 64-bit System Addressing of Version 4 mode \\[Refer to Table 2-35 for the summary of 64-bit sys-tem address support\\].. When this bit is set to 1, full or a part of 64-bit address should be used to decode Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller Registers is effective regardless of setting to 64bit Addressing in Host Control 2. If this bit is set to 1, 64-bit DMA Address-ing for Version 4 is enabled by setting Host Version 4 Enable =1, 64-bit Address-ing =1 in the Host Control 2 register.SDMA can be used and ADMA2 uses 128-bit Descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn addr_64bit_support_v4(
        &mut self,
    ) -> Addr64bitSupportV4W<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Addr64bitSupportV4W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
IMeaning of this bit is different depends on Versions \\[Refer to Table 2-35 for more details\\]. Host Controller Version 3.00 and Ver4.10 use this bit as 64-bit System Address support for V3 mode. Host Con- troller Version 4.00 uses this bit as 64-bit System Address support for both V3 and V4 modes. SDMA cannot be used in 64-bit Address-ing in Version 3 mode. If this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor may be enabled as follows: In case of Host Controller Version 3, 64-bit ADMA2 is enabled by DMA Select =11b in the Host Control 1 register. In case of Host Controller Version 4, 64-bitADMA2 for Version 3 is enabled by setting Host Version 4 Enable =0 and DMA Select = 11b. 1 - 64-bit System Address for V3 is Supported 0 - 64-bit System Address for V3 is not Supported"]
    #[inline(always)]
    #[must_use]
    pub fn addr_64bit_support_v3(
        &mut self,
    ) -> Addr64bitSupportV3W<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Addr64bitSupportV3W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Refer to SDIO Specification Version 3.00 about asynchronous interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn asynch_intr_support(
        &mut self,
    ) -> AsynchIntrSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        AsynchIntrSupportW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This field indicates usage of a slot by a specific Host System. \\[A host controller register set is defined perslot.\\]
Embedded slot for one device \\[01b\\]
means that only one non-removable device is connected to a SD bus slot. Shared Bus Slot \\[10b\\]
can be set if Host Controller supports Shared Bus Control register. The Standard Host Driver controls only a removable card or one embedded device is connected to a SD bus slot. If a slot is configured for shared bus \\[10b\\], the Standard Host Driver does not control embedded devices connected to a shared bus. Shared bus slot is controlled by a specific host driver developed by a Host System."]
    #[inline(always)]
    #[must_use]
    pub fn slot_type(&mut self) -> SlotTypeW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        SlotTypeW::new(self, 30)
    }
    #[doc = "Bit 32 - 32:32\\]
If SDR104 is supported, this bit shall be set to 1. Bit 40 indicates whether SDR50 requires tuning or not."]
    #[inline(always)]
    #[must_use]
    pub fn sdr50_support(&mut self) -> Sdr50SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Sdr50SupportW::new(self, 32)
    }
    #[doc = "Bit 33 - 33:33\\]
This bit indicates whether SDR104 is supported or not.SDR104 requires tuning."]
    #[inline(always)]
    #[must_use]
    pub fn sdr104_support(&mut self) -> Sdr104SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Sdr104SupportW::new(self, 33)
    }
    #[doc = "Bit 34 - 34:34\\]
This bit indicates whether DDR50 is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn ddr50_support(&mut self) -> Ddr50SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Ddr50SupportW::new(self, 34)
    }
    #[doc = "Bit 35 - 35:35\\]
This bit indicates whether Host controller supports UHS-II. If this bit is set to 1, 1.8V VDD2 Support shall be set to 1 \\[Host Sys- tem shall support VDD2 power supply\\]. 1 UHS-II is Supported 0 UHS-II is Not Supported"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_support(&mut self) -> Uhs2SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Uhs2SupportW::new(self, 35)
    }
    #[doc = "Bit 36 - 36:36\\]
This bit indicates support of Driver Type A for 1.8 Signaling. '0' Driver Type A is Not supported, '1' Driver Type A is supported"]
    #[inline(always)]
    #[must_use]
    pub fn drivera_support(&mut self) -> DriveraSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        DriveraSupportW::new(self, 36)
    }
    #[doc = "Bit 37 - 37:37\\]
This bit indicates support of Driver Type C for 1.8 Signaling. '0' Driver Type C is Not supported, '1' Driver Type C is supported"]
    #[inline(always)]
    #[must_use]
    pub fn driverc_support(&mut self) -> DrivercSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        DrivercSupportW::new(self, 37)
    }
    #[doc = "Bit 38 - 38:38\\]
This bit indicates support of Driver Type D for 1.8 Signaling. '0' Driver Type D is Not supported, '1' Driver Type D is supported"]
    #[inline(always)]
    #[must_use]
    pub fn driverd_support(&mut self) -> DriverdSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        DriverdSupportW::new(self, 38)
    }
    #[doc = "Bits 40:43 - 43:40\\]
This field indicates an initial value of the Re-Tuning Timer for Re-Tuning Mode 1 to 3. 0h - Get information via other source 1h = 1 seconds 2h = 2 seconds 3h = 4 seconds 4h = 8 seconds ------ n = 2\\[n-1\\]
seconds ------ Bh = 1024 seconds Fh - Ch = Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn retuning_timer_cnt(
        &mut self,
    ) -> RetuningTimerCntW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        RetuningTimerCntW::new(self, 40)
    }
    #[doc = "Bit 45 - 45:45\\]
If this bit is set to 1, this Host Controller requires tuning to operate SDR50. \\[Tuning is always required to operate SDR104\\]. '0' '1'"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_for_sdr50(&mut self) -> TuningForSdr50W<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        TuningForSdr50W::new(self, 45)
    }
    #[doc = "Bits 46:47 - 47:46\\]
This field defines the re-tuning capability of a Host Controller and how to manage the data transfer length and a Re-Tuning Timer by the Host Driver. '00' Mode 1 '01' Mode 2 '10' Mode 3 '11' Reserved. There are two re-tuning timings:Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue."]
    #[inline(always)]
    #[must_use]
    pub fn retuning_modes(&mut self) -> RetuningModesW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        RetuningModesW::new(self, 46)
    }
    #[doc = "Bits 48:55 - 55:48\\]
This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. 'FF' Clock Multiplier M = 256 --------- '02' Clock Multiplier M = 3 '01' Clock Multiplier M = 2 '00' Clock Multiplier is Not Supported."]
    #[inline(always)]
    #[must_use]
    pub fn clock_multiplier(&mut self) -> ClockMultiplierW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        ClockMultiplierW::new(self, 48)
    }
    #[doc = "Bit 56 - 56:56\\]
This field indicates whether SPI Mode is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn spi_support(&mut self) -> SpiSupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        SpiSupportW::new(self, 56)
    }
    #[doc = "Bit 57 - 57:57\\]
This field indicates whether SPI Block Mode is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn spi_blk_mode(&mut self) -> SpiBlkModeW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        SpiBlkModeW::new(self, 57)
    }
    #[doc = "Bit 59 - 59:59\\]
This field indicates that support of ADMA3 on Host Controller."]
    #[inline(always)]
    #[must_use]
    pub fn adma3_support(&mut self) -> Adma3SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Adma3SupportW::new(self, 59)
    }
    #[doc = "Bit 60 - 60:60\\]
This field indicates that support of VDD2 on Host system."]
    #[inline(always)]
    #[must_use]
    pub fn vdd2_1p8_support(&mut self) -> Vdd2_1p8SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Vdd2_1p8SupportW::new(self, 60)
    }
    #[doc = "Bit 63 - 63:63\\]
1 HS400 is Supported 0 HS400 is Not Supported"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_support(&mut self) -> Hs400SupportW<SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec> {
        Hs400SupportW::new(self, 63)
    }
}
#[doc = "This register provides the HD with information specific to the HC implementation. The HC may implement these values as fixed or loaded from flash memory during power on initializa-tion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_capabilities::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_capabilities::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_capabilities::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_capabilities::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_capabilities to value 0x9800_0407_3cee_0000"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCapabilitiesSpec {
    const RESET_VALUE: u64 = 0x9800_0407_3cee_0000;
}
