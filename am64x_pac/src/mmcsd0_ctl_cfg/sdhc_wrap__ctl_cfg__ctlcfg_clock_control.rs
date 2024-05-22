#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_clock_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgClockControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_clock_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgClockControlSpec>;
#[doc = "Field `INT_CLK_ENA` reader - 0:0\\]
This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
pub type IntClkEnaR = crate::BitReader;
#[doc = "Field `INT_CLK_ENA` writer - 0:0\\]
This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
pub type IntClkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLK_STABLE` reader - 1:1\\]
This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
pub type IntClkStableR = crate::BitReader;
#[doc = "Field `INT_CLK_STABLE` writer - 1:1\\]
This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
pub type IntClkStableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD_CLK_ENA` reader - 2:2\\]
The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped \\[Stop at SDCLK = 0\\]. If the HC detects the No Card state, this bit shall be cleared."]
pub type SdClkEnaR = crate::BitReader;
#[doc = "Field `SD_CLK_ENA` writer - 2:2\\]
The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped \\[Stop at SDCLK = 0\\]. If the HC detects the No Card state, this bit shall be cleared."]
pub type SdClkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_ENA` reader - 3:3\\]
This bit is added from Version 4.10 for Host Controller using PLL. This feature allows Host Controller to initialize clock generator in two steps: by Internal Clock Enable and PLL Enable and to minimize output latency \\[ex. SDCLK/RCLK, D0lane\\]
from SD Clock Enable. There are two modes to keep Host Drivers compatibility. In both modes, PLL Locked timing is indicated by Internal Clock Stable. \\[1\\]
When Host Version 4 Enable =0 \\[Host Driver Version 3, which does not support this bit\\]
or this bit is not implemented,Internal Clock Enable \\[or SD Clock Enable\\]
may activate PLL \\[exit low power mode and start locking clock\\]. \\[2\\]
When Host Version 4 Enable =1 \\[Host Driver Version 4\\],Internal Clock Enable is set before setting this bit and then setting this bit may activate PLL \\[exit low power mode and start locking clock\\]."]
pub type PllEnaR = crate::BitReader;
#[doc = "Field `PLL_ENA` writer - 3:3\\]
This bit is added from Version 4.10 for Host Controller using PLL. This feature allows Host Controller to initialize clock generator in two steps: by Internal Clock Enable and PLL Enable and to minimize output latency \\[ex. SDCLK/RCLK, D0lane\\]
from SD Clock Enable. There are two modes to keep Host Drivers compatibility. In both modes, PLL Locked timing is indicated by Internal Clock Stable. \\[1\\]
When Host Version 4 Enable =0 \\[Host Driver Version 3, which does not support this bit\\]
or this bit is not implemented,Internal Clock Enable \\[or SD Clock Enable\\]
may activate PLL \\[exit low power mode and start locking clock\\]. \\[2\\]
When Host Version 4 Enable =1 \\[Host Driver Version 4\\],Internal Clock Enable is set before setting this bit and then setting this bit may activate PLL \\[exit low power mode and start locking clock\\]."]
pub type PllEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGEN_SEL` reader - 5:5\\]
This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported \\[non-zero value is set to Clock Multiplier in the Capabilities register\\], this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers. '0' Divided Clock Mode '1' Programmable Clock Mode"]
pub type ClkgenSelR = crate::BitReader;
#[doc = "Field `CLKGEN_SEL` writer - 5:5\\]
This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported \\[non-zero value is set to Clock Multiplier in the Capabilities register\\], this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers. '0' Divided Clock Mode '1' Programmable Clock Mode"]
pub type ClkgenSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDCLK_FRQSEL_UPBITS` reader - 7:6\\]
Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select."]
pub type SdclkFrqselUpbitsR = crate::FieldReader;
#[doc = "Field `SDCLK_FRQSEL_UPBITS` writer - 7:6\\]
Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select."]
pub type SdclkFrqselUpbitsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLK_FRQSEL` reader - 15:8\\]
This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. \\[1\\]
8-bit Divided Clock Mode 80h - base clock divided by 256 40h - base clock divided by 128 20h - base clock divided by 64 10h - base clock divided by 32 08h - base clock divided by 16 04h - base clock divided by 8 02h - base clock divided by 4 01h - base clock divided by 2 00h - base clock\\[10MHz-63MHz\\]
Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. \\[1\\]
25 MHz divider value. \\[2\\]
400 KHz divider value. The frequency of the SDCLK is set by the following formula: Clock Frequency = \\[Baseclock\\]
/ divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz \\[base clock\\]
Maximum Frequency for MMC = 52Mhz \\[base clock\\]
Minimum Frequency = 195.3125Khz \\[50Mhz / 256\\], same calculation for MMC also. \\[2\\]
10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock \\[Duty 50%\\]
002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock \\[10MHz-254MHz\\]"]
pub type SdclkFrqselR = crate::FieldReader;
#[doc = "Field `SDCLK_FRQSEL` writer - 15:8\\]
This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. \\[1\\]
8-bit Divided Clock Mode 80h - base clock divided by 256 40h - base clock divided by 128 20h - base clock divided by 64 10h - base clock divided by 32 08h - base clock divided by 16 04h - base clock divided by 8 02h - base clock divided by 4 01h - base clock divided by 2 00h - base clock\\[10MHz-63MHz\\]
Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. \\[1\\]
25 MHz divider value. \\[2\\]
400 KHz divider value. The frequency of the SDCLK is set by the following formula: Clock Frequency = \\[Baseclock\\]
/ divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz \\[base clock\\]
Maximum Frequency for MMC = 52Mhz \\[base clock\\]
Minimum Frequency = 195.3125Khz \\[50Mhz / 256\\], same calculation for MMC also. \\[2\\]
10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock \\[Duty 50%\\]
002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock \\[10MHz-254MHz\\]"]
pub type SdclkFrqselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    pub fn int_clk_ena(&self) -> IntClkEnaR {
        IntClkEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    #[inline(always)]
    pub fn int_clk_stable(&self) -> IntClkStableR {
        IntClkStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped \\[Stop at SDCLK = 0\\]. If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    pub fn sd_clk_ena(&self) -> SdClkEnaR {
        SdClkEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit is added from Version 4.10 for Host Controller using PLL. This feature allows Host Controller to initialize clock generator in two steps: by Internal Clock Enable and PLL Enable and to minimize output latency \\[ex. SDCLK/RCLK, D0lane\\]
from SD Clock Enable. There are two modes to keep Host Drivers compatibility. In both modes, PLL Locked timing is indicated by Internal Clock Stable. \\[1\\]
When Host Version 4 Enable =0 \\[Host Driver Version 3, which does not support this bit\\]
or this bit is not implemented,Internal Clock Enable \\[or SD Clock Enable\\]
may activate PLL \\[exit low power mode and start locking clock\\]. \\[2\\]
When Host Version 4 Enable =1 \\[Host Driver Version 4\\],Internal Clock Enable is set before setting this bit and then setting this bit may activate PLL \\[exit low power mode and start locking clock\\]."]
    #[inline(always)]
    pub fn pll_ena(&self) -> PllEnaR {
        PllEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported \\[non-zero value is set to Clock Multiplier in the Capabilities register\\], this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers. '0' Divided Clock Mode '1' Programmable Clock Mode"]
    #[inline(always)]
    pub fn clkgen_sel(&self) -> ClkgenSelR {
        ClkgenSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select."]
    #[inline(always)]
    pub fn sdclk_frqsel_upbits(&self) -> SdclkFrqselUpbitsR {
        SdclkFrqselUpbitsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. \\[1\\]
8-bit Divided Clock Mode 80h - base clock divided by 256 40h - base clock divided by 128 20h - base clock divided by 64 10h - base clock divided by 32 08h - base clock divided by 16 04h - base clock divided by 8 02h - base clock divided by 4 01h - base clock divided by 2 00h - base clock\\[10MHz-63MHz\\]
Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. \\[1\\]
25 MHz divider value. \\[2\\]
400 KHz divider value. The frequency of the SDCLK is set by the following formula: Clock Frequency = \\[Baseclock\\]
/ divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz \\[base clock\\]
Maximum Frequency for MMC = 52Mhz \\[base clock\\]
Minimum Frequency = 195.3125Khz \\[50Mhz / 256\\], same calculation for MMC also. \\[2\\]
10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock \\[Duty 50%\\]
002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock \\[10MHz-254MHz\\]"]
    #[inline(always)]
    pub fn sdclk_frqsel(&self) -> SdclkFrqselR {
        SdclkFrqselR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_ena(&mut self) -> IntClkEnaW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        IntClkEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_stable(&mut self) -> IntClkStableW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        IntClkStableW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped \\[Stop at SDCLK = 0\\]. If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn sd_clk_ena(&mut self) -> SdClkEnaW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        SdClkEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit is added from Version 4.10 for Host Controller using PLL. This feature allows Host Controller to initialize clock generator in two steps: by Internal Clock Enable and PLL Enable and to minimize output latency \\[ex. SDCLK/RCLK, D0lane\\]
from SD Clock Enable. There are two modes to keep Host Drivers compatibility. In both modes, PLL Locked timing is indicated by Internal Clock Stable. \\[1\\]
When Host Version 4 Enable =0 \\[Host Driver Version 3, which does not support this bit\\]
or this bit is not implemented,Internal Clock Enable \\[or SD Clock Enable\\]
may activate PLL \\[exit low power mode and start locking clock\\]. \\[2\\]
When Host Version 4 Enable =1 \\[Host Driver Version 4\\],Internal Clock Enable is set before setting this bit and then setting this bit may activate PLL \\[exit low power mode and start locking clock\\]."]
    #[inline(always)]
    #[must_use]
    pub fn pll_ena(&mut self) -> PllEnaW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        PllEnaW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported \\[non-zero value is set to Clock Multiplier in the Capabilities register\\], this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable = 0, this bit is set by Host Driver. If the Preset Value Enable = 1, this bit is automatically set to a value specified in one of Preset Value registers. '0' Divided Clock Mode '1' Programmable Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clkgen_sel(&mut self) -> ClkgenSelW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        ClkgenSelW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select."]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_frqsel_upbits(
        &mut self,
    ) -> SdclkFrqselUpbitsW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        SdclkFrqselUpbitsW::new(self, 6)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. \\[1\\]
8-bit Divided Clock Mode 80h - base clock divided by 256 40h - base clock divided by 128 20h - base clock divided by 64 10h - base clock divided by 32 08h - base clock divided by 16 04h - base clock divided by 8 02h - base clock divided by 4 01h - base clock divided by 2 00h - base clock\\[10MHz-63MHz\\]
Setting 00h specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. \\[1\\]
25 MHz divider value. \\[2\\]
400 KHz divider value. The frequency of the SDCLK is set by the following formula: Clock Frequency = \\[Baseclock\\]
/ divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz \\[base clock\\]
Maximum Frequency for MMC = 52Mhz \\[base clock\\]
Minimum Frequency = 195.3125Khz \\[50Mhz / 256\\], same calculation for MMC also. \\[2\\]
10-bit Divided Clock Mode Host Controller Version 3.00 supports this mandatory mode instead of the 8-bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 3FFh --1/2046 Divided Clock N -------1/2N Divided Clock \\[Duty 50%\\]
002h -- 1/4 Divided Clock 001h ---1/2 Divided Clock 000h --- Base Clock \\[10MHz-254MHz\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_frqsel(&mut self) -> SdclkFrqselW<SdhcWrap_CtlCfg_CtlcfgClockControlSpec> {
        SdclkFrqselW::new(self, 8)
    }
}
#[doc = "This register is used to program the Clock frequency select, generator select, Clock enable, Internal Clock state fields This register controls SDCLK in SD Mode and RCLK in UHS-II mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_clock_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_clock_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgClockControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgClockControlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_clock_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgClockControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_clock_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgClockControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_clock_control to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgClockControlSpec {
    const RESET_VALUE: u16 = 0;
}
