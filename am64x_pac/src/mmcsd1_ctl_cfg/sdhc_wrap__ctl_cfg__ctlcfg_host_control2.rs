#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_host_control2` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_host_control2` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec>;
#[doc = "Field `UHS_MODE_SELECT` reader - 2:0\\]
This field is used to select one of UHS-I modes or UHS-II mode.In case of UHS-I mode, this field is effective when 1.8V Signal-ing Enable is set to 1. In case of UHS-II mode, 1.8V Signaling Enable shall be set to 0. Setting of this field is used to select one of preset values in UHS-I or UHS-II mode. If Preset Value Enable in the Host Control 2 register is set to 1,Host Controller sets SDCLK/RCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select according to Preset Value registers. In this case, one of preset value registers is selected by this field. Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitch. After setting this field, Host Driver sets SD Clock Enable again. When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
pub type UhsModeSelectR = crate::FieldReader;
#[doc = "Field `UHS_MODE_SELECT` writer - 2:0\\]
This field is used to select one of UHS-I modes or UHS-II mode.In case of UHS-I mode, this field is effective when 1.8V Signal-ing Enable is set to 1. In case of UHS-II mode, 1.8V Signaling Enable shall be set to 0. Setting of this field is used to select one of preset values in UHS-I or UHS-II mode. If Preset Value Enable in the Host Control 2 register is set to 1,Host Controller sets SDCLK/RCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select according to Preset Value registers. In this case, one of preset value registers is selected by this field. Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitch. After setting this field, Host Driver sets SD Clock Enable again. When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
pub type UhsModeSelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `V1P8_SIGNAL_ENA` reader - 3:3\\]
This bit controls voltage regulator for I/O cell. 3.3V is supplied to the card regardless of signaling voltage. Setting this bit from 0 to 1 starts changing signal voltage from 3.3V to 1.8V. 1.8V regulator output shall be stable within 5ms. Host Controller clears this bit if switching to 1.8V signaling fails. Clearing this bit from 1 to 0 starts changing signal voltage from 1.8V to 3.3V. 3.3V regulator output shall be stable within 5ms. Host Driver can set this bit to 1 when Host Controller supports 1.8V signaling \\[One of support bits is set to 1: SDR50, SDR104 or DDR50 in the Capabilities register\\]
and the card or device supports UHS-I. '0' 3.3V Signalling, '1' 1.8V Signalling"]
pub type V1p8SignalEnaR = crate::BitReader;
#[doc = "Field `V1P8_SIGNAL_ENA` writer - 3:3\\]
This bit controls voltage regulator for I/O cell. 3.3V is supplied to the card regardless of signaling voltage. Setting this bit from 0 to 1 starts changing signal voltage from 3.3V to 1.8V. 1.8V regulator output shall be stable within 5ms. Host Controller clears this bit if switching to 1.8V signaling fails. Clearing this bit from 1 to 0 starts changing signal voltage from 1.8V to 3.3V. 3.3V regulator output shall be stable within 5ms. Host Driver can set this bit to 1 when Host Controller supports 1.8V signaling \\[One of support bits is set to 1: SDR50, SDR104 or DDR50 in the Capabilities register\\]
and the card or device supports UHS-I. '0' 3.3V Signalling, '1' 1.8V Signalling"]
pub type V1p8SignalEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVER_STRENGTH1` reader - 5:4\\]
Host Controller output driver in 1.8V signaling is selected by this bit. In 3.3V signaling, this field is not effective. This field can be set depends on Driver Type A, C and D support bits in the Capabilities register. This bit depends on setting of Preset Value Enable. If Preset Value Enable = 0, this field is set by Host Driver. If Preset Value Enable = 1, this field is automatically set by a value specified in the one of Preset Value registers."]
pub type DriverStrength1R = crate::FieldReader;
#[doc = "Field `DRIVER_STRENGTH1` writer - 5:4\\]
Host Controller output driver in 1.8V signaling is selected by this bit. In 3.3V signaling, this field is not effective. This field can be set depends on Driver Type A, C and D support bits in the Capabilities register. This bit depends on setting of Preset Value Enable. If Preset Value Enable = 0, this field is set by Host Driver. If Preset Value Enable = 1, this field is automatically set by a value specified in the one of Preset Value registers."]
pub type DriverStrength1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXECUTE_TUNING` reader - 6:6\\]
This bit is set to 1 to start tuning procedure and automatically cleared when tuning procedure is completed. The result of tuning is indicated to Sampling Clock Select. Tuning procedure is aborted by writing 0 for more detail about tuning procedure. '0' Not Tuned or Tuning Completed '1' Execute Tuning"]
pub type ExecuteTuningR = crate::BitReader;
#[doc = "Field `EXECUTE_TUNING` writer - 6:6\\]
This bit is set to 1 to start tuning procedure and automatically cleared when tuning procedure is completed. The result of tuning is indicated to Sampling Clock Select. Tuning procedure is aborted by writing 0 for more detail about tuning procedure. '0' Not Tuned or Tuning Completed '1' Execute Tuning"]
pub type ExecuteTuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLING_CLK_SELECT` reader - 7:7\\]
This bit is set by tuning procedure when Execute Tuning is cleared. Writing 1 to this bit is meaningless and ignored. Setting 1 means that tuning is completed successfully and setting 0 means that tuning is failed. Host Controller uses this bit to select sampling clock to receive CMD and DAT. This bit is cleared by writing 0. Change of this bit is not allowed while the Host Controller is receiving response or a read data block."]
pub type SamplingClkSelectR = crate::BitReader;
#[doc = "Field `SAMPLING_CLK_SELECT` writer - 7:7\\]
This bit is set by tuning procedure when Execute Tuning is cleared. Writing 1 to this bit is meaningless and ignored. Setting 1 means that tuning is completed successfully and setting 0 means that tuning is failed. Host Controller uses this bit to select sampling clock to receive CMD and DAT. This bit is cleared by writing 0. Change of this bit is not allowed while the Host Controller is receiving response or a read data block."]
pub type SamplingClkSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_INTF_ENABLE` reader - 8:8\\]
This bit is used to enable UHS-II Interface. Before trying to start UHS-II initialization, this bit shall be set to 1. Before trying to start SD mode initialization, this bit shall be set to 0. This bit is used to enable UHS-II IF Detection, Lane Synchroni-zation and In Dormant State in the Present State register, and to select clock source of either SD mode or UHS-II mode. Host Controller shall not leave unused SD 4-bit Interface lines \\[CLK, CMD and DAT\\[3:2\\]\\]
floating in UHS-II mode by using pull-up or driving to low. When DAT\\[2\\]
is used as interrupt input in UHS-II mode, DAT\\[2\\]
of Host Controller is set to input and then DAT\\[2\\]
of SDIO card is set to output to avoid conflict. '0' 4-bit SD Interface enabled '1' UHS-II Interface enabled"]
pub type Uhs2IntfEnableR = crate::BitReader;
#[doc = "Field `UHS2_INTF_ENABLE` writer - 8:8\\]
This bit is used to enable UHS-II Interface. Before trying to start UHS-II initialization, this bit shall be set to 1. Before trying to start SD mode initialization, this bit shall be set to 0. This bit is used to enable UHS-II IF Detection, Lane Synchroni-zation and In Dormant State in the Present State register, and to select clock source of either SD mode or UHS-II mode. Host Controller shall not leave unused SD 4-bit Interface lines \\[CLK, CMD and DAT\\[3:2\\]\\]
floating in UHS-II mode by using pull-up or driving to low. When DAT\\[2\\]
is used as interrupt input in UHS-II mode, DAT\\[2\\]
of Host Controller is set to input and then DAT\\[2\\]
of SDIO card is set to output to avoid conflict. '0' 4-bit SD Interface enabled '1' UHS-II Interface enabled"]
pub type Uhs2IntfEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVER_STRENGTH2` reader - 9:9\\]
This is the programmed Drive STrength output and Bit\\[2\\]
of the sdhccore_drivestrength value."]
pub type DriverStrength2R = crate::BitReader;
#[doc = "Field `DRIVER_STRENGTH2` writer - 9:9\\]
This is the programmed Drive STrength output and Bit\\[2\\]
of the sdhccore_drivestrength value."]
pub type DriverStrength2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA2_LEN_MODE` reader - 10:10\\]
This bit selects one of ADMA2 Length Modes either 16-bit or 26-bit."]
pub type Adma2LenModeR = crate::BitReader;
#[doc = "Field `ADMA2_LEN_MODE` writer - 10:10\\]
This bit selects one of ADMA2 Length Modes either 16-bit or 26-bit."]
pub type Adma2LenModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD23_ENA` reader - 11:11\\]
In memory card initialization, Host Driver Version 4.10 checks whether card supports CMD23 by checking a bit SCR\\[33\\]. If the card supports CMD23 \\[SCR\\[33\\]=1\\], this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 datatransfer. Refer to Auto CMD Enable in the Transfer Mode regis-ter."]
pub type Cmd23EnaR = crate::BitReader;
#[doc = "Field `CMD23_ENA` writer - 11:11\\]
In memory card initialization, Host Driver Version 4.10 checks whether card supports CMD23 by checking a bit SCR\\[33\\]. If the card supports CMD23 \\[SCR\\[33\\]=1\\], this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 datatransfer. Refer to Auto CMD Enable in the Transfer Mode regis-ter."]
pub type Cmd23EnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_VER40_ENA` reader - 12:12\\]
This bit selects either Version 3.00 compatible mode or Ver4.mode. In Version 4.00, support of 64-bit System Addressing is modified. All DMAs support 64-bit System Addressing. UHS-II supported Host Driver shall enable this bit. In Version 4.10, supported 32-bit Block Count for all operations. Functions of following fields are modified. SDMA Address SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of SDMA System Address register \\[Offset 003-000h\\]
ADMA2 / ADMA3 Selection ADMA3 is selected by DMA Select in the Host Control 1 regis-ter. 64bit ADMA Descriptor Size 128bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1. Selection of 32-bit / 64-bit System Addressing Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register instead of DMA Select in the Host Control 1 register. 32-bit Block Count SDMA System Address register \\[003h-000h\\]
is modified to 32-bit Block Count register."]
pub type HostVer40EnaR = crate::BitReader;
#[doc = "Field `HOST_VER40_ENA` writer - 12:12\\]
This bit selects either Version 3.00 compatible mode or Ver4.mode. In Version 4.00, support of 64-bit System Addressing is modified. All DMAs support 64-bit System Addressing. UHS-II supported Host Driver shall enable this bit. In Version 4.10, supported 32-bit Block Count for all operations. Functions of following fields are modified. SDMA Address SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of SDMA System Address register \\[Offset 003-000h\\]
ADMA2 / ADMA3 Selection ADMA3 is selected by DMA Select in the Host Control 1 regis-ter. 64bit ADMA Descriptor Size 128bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1. Selection of 32-bit / 64-bit System Addressing Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register instead of DMA Select in the Host Control 1 register. 32-bit Block Count SDMA System Address register \\[003h-000h\\]
is modified to 32-bit Block Count register."]
pub type HostVer40EnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT64_ADDRESSING` reader - 13:13\\]
This field is effective when Host Version 4.00 Enable is set to 1. Host Controller selects either of 32-bit or 64-bit addressing modes to access system memory. Whether 32-bit or 64-bit is determined by OS installed in a host system. Host Driver sets this bit depends on addressing mode of installed OS. Refer to 64-bit System Address Support in the Capabilities register. ."]
pub type Bit64AddressingR = crate::BitReader;
#[doc = "Field `BIT64_ADDRESSING` writer - 13:13\\]
This field is effective when Host Version 4.00 Enable is set to 1. Host Controller selects either of 32-bit or 64-bit addressing modes to access system memory. Whether 32-bit or 64-bit is determined by OS installed in a host system. Host Driver sets this bit depends on addressing mode of installed OS. Refer to 64-bit System Address Support in the Capabilities register. ."]
pub type Bit64AddressingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH_INTR_ENA` reader - 14:14\\]
This bit can be set to 1 if a card support asynchronous interrupt and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Asynchronous interrupt is effective when DAT\\[1\\]
interrupt is used in 4-bit SD mode \\[and zero is set to Interrupt Pin Select in the Shared Bus Control register\\]. If this bit is set to 1, the Host Driver can stop the SDCLK during asynchronous interrupt period to save power. During this period, the Host Controller continues to deliver Card Interrupt to the host when it is asserted by the Card."]
pub type AsynchIntrEnaR = crate::BitReader;
#[doc = "Field `ASYNCH_INTR_ENA` writer - 14:14\\]
This bit can be set to 1 if a card support asynchronous interrupt and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Asynchronous interrupt is effective when DAT\\[1\\]
interrupt is used in 4-bit SD mode \\[and zero is set to Interrupt Pin Select in the Shared Bus Control register\\]. If this bit is set to 1, the Host Driver can stop the SDCLK during asynchronous interrupt period to save power. During this period, the Host Controller continues to deliver Card Interrupt to the host when it is asserted by the Card."]
pub type AsynchIntrEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESET_VALUE_ENA` reader - 15:15\\]
Host Controller Version 3.00 supports this bit. As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enables the functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
pub type PresetValueEnaR = crate::BitReader;
#[doc = "Field `PRESET_VALUE_ENA` writer - 15:15\\]
Host Controller Version 3.00 supports this bit. As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enables the functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
pub type PresetValueEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This field is used to select one of UHS-I modes or UHS-II mode.In case of UHS-I mode, this field is effective when 1.8V Signal-ing Enable is set to 1. In case of UHS-II mode, 1.8V Signaling Enable shall be set to 0. Setting of this field is used to select one of preset values in UHS-I or UHS-II mode. If Preset Value Enable in the Host Control 2 register is set to 1,Host Controller sets SDCLK/RCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select according to Preset Value registers. In this case, one of preset value registers is selected by this field. Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitch. After setting this field, Host Driver sets SD Clock Enable again. When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
    #[inline(always)]
    pub fn uhs_mode_select(&self) -> UhsModeSelectR {
        UhsModeSelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit controls voltage regulator for I/O cell. 3.3V is supplied to the card regardless of signaling voltage. Setting this bit from 0 to 1 starts changing signal voltage from 3.3V to 1.8V. 1.8V regulator output shall be stable within 5ms. Host Controller clears this bit if switching to 1.8V signaling fails. Clearing this bit from 1 to 0 starts changing signal voltage from 1.8V to 3.3V. 3.3V regulator output shall be stable within 5ms. Host Driver can set this bit to 1 when Host Controller supports 1.8V signaling \\[One of support bits is set to 1: SDR50, SDR104 or DDR50 in the Capabilities register\\]
and the card or device supports UHS-I. '0' 3.3V Signalling, '1' 1.8V Signalling"]
    #[inline(always)]
    pub fn v1p8_signal_ena(&self) -> V1p8SignalEnaR {
        V1p8SignalEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Host Controller output driver in 1.8V signaling is selected by this bit. In 3.3V signaling, this field is not effective. This field can be set depends on Driver Type A, C and D support bits in the Capabilities register. This bit depends on setting of Preset Value Enable. If Preset Value Enable = 0, this field is set by Host Driver. If Preset Value Enable = 1, this field is automatically set by a value specified in the one of Preset Value registers."]
    #[inline(always)]
    pub fn driver_strength1(&self) -> DriverStrength1R {
        DriverStrength1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is set to 1 to start tuning procedure and automatically cleared when tuning procedure is completed. The result of tuning is indicated to Sampling Clock Select. Tuning procedure is aborted by writing 0 for more detail about tuning procedure. '0' Not Tuned or Tuning Completed '1' Execute Tuning"]
    #[inline(always)]
    pub fn execute_tuning(&self) -> ExecuteTuningR {
        ExecuteTuningR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit is set by tuning procedure when Execute Tuning is cleared. Writing 1 to this bit is meaningless and ignored. Setting 1 means that tuning is completed successfully and setting 0 means that tuning is failed. Host Controller uses this bit to select sampling clock to receive CMD and DAT. This bit is cleared by writing 0. Change of this bit is not allowed while the Host Controller is receiving response or a read data block."]
    #[inline(always)]
    pub fn sampling_clk_select(&self) -> SamplingClkSelectR {
        SamplingClkSelectR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit is used to enable UHS-II Interface. Before trying to start UHS-II initialization, this bit shall be set to 1. Before trying to start SD mode initialization, this bit shall be set to 0. This bit is used to enable UHS-II IF Detection, Lane Synchroni-zation and In Dormant State in the Present State register, and to select clock source of either SD mode or UHS-II mode. Host Controller shall not leave unused SD 4-bit Interface lines \\[CLK, CMD and DAT\\[3:2\\]\\]
floating in UHS-II mode by using pull-up or driving to low. When DAT\\[2\\]
is used as interrupt input in UHS-II mode, DAT\\[2\\]
of Host Controller is set to input and then DAT\\[2\\]
of SDIO card is set to output to avoid conflict. '0' 4-bit SD Interface enabled '1' UHS-II Interface enabled"]
    #[inline(always)]
    pub fn uhs2_intf_enable(&self) -> Uhs2IntfEnableR {
        Uhs2IntfEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This is the programmed Drive STrength output and Bit\\[2\\]
of the sdhccore_drivestrength value."]
    #[inline(always)]
    pub fn driver_strength2(&self) -> DriverStrength2R {
        DriverStrength2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit selects one of ADMA2 Length Modes either 16-bit or 26-bit."]
    #[inline(always)]
    pub fn adma2_len_mode(&self) -> Adma2LenModeR {
        Adma2LenModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
In memory card initialization, Host Driver Version 4.10 checks whether card supports CMD23 by checking a bit SCR\\[33\\]. If the card supports CMD23 \\[SCR\\[33\\]=1\\], this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 datatransfer. Refer to Auto CMD Enable in the Transfer Mode regis-ter."]
    #[inline(always)]
    pub fn cmd23_ena(&self) -> Cmd23EnaR {
        Cmd23EnaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This bit selects either Version 3.00 compatible mode or Ver4.mode. In Version 4.00, support of 64-bit System Addressing is modified. All DMAs support 64-bit System Addressing. UHS-II supported Host Driver shall enable this bit. In Version 4.10, supported 32-bit Block Count for all operations. Functions of following fields are modified. SDMA Address SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of SDMA System Address register \\[Offset 003-000h\\]
ADMA2 / ADMA3 Selection ADMA3 is selected by DMA Select in the Host Control 1 regis-ter. 64bit ADMA Descriptor Size 128bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1. Selection of 32-bit / 64-bit System Addressing Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register instead of DMA Select in the Host Control 1 register. 32-bit Block Count SDMA System Address register \\[003h-000h\\]
is modified to 32-bit Block Count register."]
    #[inline(always)]
    pub fn host_ver40_ena(&self) -> HostVer40EnaR {
        HostVer40EnaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
This field is effective when Host Version 4.00 Enable is set to 1. Host Controller selects either of 32-bit or 64-bit addressing modes to access system memory. Whether 32-bit or 64-bit is determined by OS installed in a host system. Host Driver sets this bit depends on addressing mode of installed OS. Refer to 64-bit System Address Support in the Capabilities register. ."]
    #[inline(always)]
    pub fn bit64_addressing(&self) -> Bit64AddressingR {
        Bit64AddressingR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
This bit can be set to 1 if a card support asynchronous interrupt and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Asynchronous interrupt is effective when DAT\\[1\\]
interrupt is used in 4-bit SD mode \\[and zero is set to Interrupt Pin Select in the Shared Bus Control register\\]. If this bit is set to 1, the Host Driver can stop the SDCLK during asynchronous interrupt period to save power. During this period, the Host Controller continues to deliver Card Interrupt to the host when it is asserted by the Card."]
    #[inline(always)]
    pub fn asynch_intr_ena(&self) -> AsynchIntrEnaR {
        AsynchIntrEnaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Host Controller Version 3.00 supports this bit. As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enables the functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
    #[inline(always)]
    pub fn preset_value_ena(&self) -> PresetValueEnaR {
        PresetValueEnaR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This field is used to select one of UHS-I modes or UHS-II mode.In case of UHS-I mode, this field is effective when 1.8V Signal-ing Enable is set to 1. In case of UHS-II mode, 1.8V Signaling Enable shall be set to 0. Setting of this field is used to select one of preset values in UHS-I or UHS-II mode. If Preset Value Enable in the Host Control 2 register is set to 1,Host Controller sets SDCLK/RCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select according to Preset Value registers. In this case, one of preset value registers is selected by this field. Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitch. After setting this field, Host Driver sets SD Clock Enable again. When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
    #[inline(always)]
    #[must_use]
    pub fn uhs_mode_select(&mut self) -> UhsModeSelectW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        UhsModeSelectW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit controls voltage regulator for I/O cell. 3.3V is supplied to the card regardless of signaling voltage. Setting this bit from 0 to 1 starts changing signal voltage from 3.3V to 1.8V. 1.8V regulator output shall be stable within 5ms. Host Controller clears this bit if switching to 1.8V signaling fails. Clearing this bit from 1 to 0 starts changing signal voltage from 1.8V to 3.3V. 3.3V regulator output shall be stable within 5ms. Host Driver can set this bit to 1 when Host Controller supports 1.8V signaling \\[One of support bits is set to 1: SDR50, SDR104 or DDR50 in the Capabilities register\\]
and the card or device supports UHS-I. '0' 3.3V Signalling, '1' 1.8V Signalling"]
    #[inline(always)]
    #[must_use]
    pub fn v1p8_signal_ena(&mut self) -> V1p8SignalEnaW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        V1p8SignalEnaW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Host Controller output driver in 1.8V signaling is selected by this bit. In 3.3V signaling, this field is not effective. This field can be set depends on Driver Type A, C and D support bits in the Capabilities register. This bit depends on setting of Preset Value Enable. If Preset Value Enable = 0, this field is set by Host Driver. If Preset Value Enable = 1, this field is automatically set by a value specified in the one of Preset Value registers."]
    #[inline(always)]
    #[must_use]
    pub fn driver_strength1(&mut self) -> DriverStrength1W<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        DriverStrength1W::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is set to 1 to start tuning procedure and automatically cleared when tuning procedure is completed. The result of tuning is indicated to Sampling Clock Select. Tuning procedure is aborted by writing 0 for more detail about tuning procedure. '0' Not Tuned or Tuning Completed '1' Execute Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn execute_tuning(&mut self) -> ExecuteTuningW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        ExecuteTuningW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit is set by tuning procedure when Execute Tuning is cleared. Writing 1 to this bit is meaningless and ignored. Setting 1 means that tuning is completed successfully and setting 0 means that tuning is failed. Host Controller uses this bit to select sampling clock to receive CMD and DAT. This bit is cleared by writing 0. Change of this bit is not allowed while the Host Controller is receiving response or a read data block."]
    #[inline(always)]
    #[must_use]
    pub fn sampling_clk_select(
        &mut self,
    ) -> SamplingClkSelectW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        SamplingClkSelectW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit is used to enable UHS-II Interface. Before trying to start UHS-II initialization, this bit shall be set to 1. Before trying to start SD mode initialization, this bit shall be set to 0. This bit is used to enable UHS-II IF Detection, Lane Synchroni-zation and In Dormant State in the Present State register, and to select clock source of either SD mode or UHS-II mode. Host Controller shall not leave unused SD 4-bit Interface lines \\[CLK, CMD and DAT\\[3:2\\]\\]
floating in UHS-II mode by using pull-up or driving to low. When DAT\\[2\\]
is used as interrupt input in UHS-II mode, DAT\\[2\\]
of Host Controller is set to input and then DAT\\[2\\]
of SDIO card is set to output to avoid conflict. '0' 4-bit SD Interface enabled '1' UHS-II Interface enabled"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_intf_enable(&mut self) -> Uhs2IntfEnableW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        Uhs2IntfEnableW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This is the programmed Drive STrength output and Bit\\[2\\]
of the sdhccore_drivestrength value."]
    #[inline(always)]
    #[must_use]
    pub fn driver_strength2(&mut self) -> DriverStrength2W<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        DriverStrength2W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit selects one of ADMA2 Length Modes either 16-bit or 26-bit."]
    #[inline(always)]
    #[must_use]
    pub fn adma2_len_mode(&mut self) -> Adma2LenModeW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        Adma2LenModeW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
In memory card initialization, Host Driver Version 4.10 checks whether card supports CMD23 by checking a bit SCR\\[33\\]. If the card supports CMD23 \\[SCR\\[33\\]=1\\], this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 datatransfer. Refer to Auto CMD Enable in the Transfer Mode regis-ter."]
    #[inline(always)]
    #[must_use]
    pub fn cmd23_ena(&mut self) -> Cmd23EnaW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        Cmd23EnaW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
This bit selects either Version 3.00 compatible mode or Ver4.mode. In Version 4.00, support of 64-bit System Addressing is modified. All DMAs support 64-bit System Addressing. UHS-II supported Host Driver shall enable this bit. In Version 4.10, supported 32-bit Block Count for all operations. Functions of following fields are modified. SDMA Address SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of SDMA System Address register \\[Offset 003-000h\\]
ADMA2 / ADMA3 Selection ADMA3 is selected by DMA Select in the Host Control 1 regis-ter. 64bit ADMA Descriptor Size 128bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1. Selection of 32-bit / 64-bit System Addressing Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register instead of DMA Select in the Host Control 1 register. 32-bit Block Count SDMA System Address register \\[003h-000h\\]
is modified to 32-bit Block Count register."]
    #[inline(always)]
    #[must_use]
    pub fn host_ver40_ena(&mut self) -> HostVer40EnaW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        HostVer40EnaW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
This field is effective when Host Version 4.00 Enable is set to 1. Host Controller selects either of 32-bit or 64-bit addressing modes to access system memory. Whether 32-bit or 64-bit is determined by OS installed in a host system. Host Driver sets this bit depends on addressing mode of installed OS. Refer to 64-bit System Address Support in the Capabilities register. ."]
    #[inline(always)]
    #[must_use]
    pub fn bit64_addressing(&mut self) -> Bit64AddressingW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        Bit64AddressingW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
This bit can be set to 1 if a card support asynchronous interrupt and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Asynchronous interrupt is effective when DAT\\[1\\]
interrupt is used in 4-bit SD mode \\[and zero is set to Interrupt Pin Select in the Shared Bus Control register\\]. If this bit is set to 1, the Host Driver can stop the SDCLK during asynchronous interrupt period to save power. During this period, the Host Controller continues to deliver Card Interrupt to the host when it is asserted by the Card."]
    #[inline(always)]
    #[must_use]
    pub fn asynch_intr_ena(&mut self) -> AsynchIntrEnaW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        AsynchIntrEnaW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Host Controller Version 3.00 supports this bit. As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enables the functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
    #[inline(always)]
    #[must_use]
    pub fn preset_value_ena(&mut self) -> PresetValueEnaW<SdhcWrap_CtlCfg_CtlcfgHostControl2Spec> {
        PresetValueEnaW::new(self, 15)
    }
}
#[doc = "This register is used to program UHS Select Mode,UHS Select Mode,Driver Strength Select,Execute Tuning,Sampling Clock Select,Asynchronous Interrupt Enable and Preset value enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_host_control2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_host_control2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgHostControl2Spec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgHostControl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_host_control2::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgHostControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_host_control2::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgHostControl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_host_control2 to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgHostControl2Spec {
    const RESET_VALUE: u16 = 0;
}
