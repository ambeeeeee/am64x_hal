#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_preset_value5` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_preset_value5` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec>;
#[doc = "Field `SDCLK_FRQSEL` reader - 9:0\\]
10-bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system."]
pub type SdclkFrqselR = crate::FieldReader<u16>;
#[doc = "Field `SDCLK_FRQSEL` writer - 9:0\\]
10-bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system."]
pub type SdclkFrqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CLOCK_GENSEL` reader - 10:10\\]
This bit is effective when Host Controller supports programmable clock '0' Host Controller Ver2.00 Compatible Clock Generator '1' Programmable Clock Generator"]
pub type ClockGenselR = crate::BitReader;
#[doc = "Field `CLOCK_GENSEL` writer - 10:10\\]
This bit is effective when Host Controller supports programmable clock '0' Host Controller Ver2.00 Compatible Clock Generator '1' Programmable Clock Generator"]
pub type ClockGenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVER_STRENGTH_SEL` reader - 15:14\\]
Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type DriverStrengthSelR = crate::FieldReader;
#[doc = "Field `DRIVER_STRENGTH_SEL` writer - 15:14\\]
Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type DriverStrengthSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
10-bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system."]
    #[inline(always)]
    pub fn sdclk_frqsel(&self) -> SdclkFrqselR {
        SdclkFrqselR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is effective when Host Controller supports programmable clock '0' Host Controller Ver2.00 Compatible Clock Generator '1' Programmable Clock Generator"]
    #[inline(always)]
    pub fn clock_gensel(&self) -> ClockGenselR {
        ClockGenselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn driver_strength_sel(&self) -> DriverStrengthSelR {
        DriverStrengthSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
10-bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system."]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_frqsel(&mut self) -> SdclkFrqselW<SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec> {
        SdclkFrqselW::new(self, 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is effective when Host Controller supports programmable clock '0' Host Controller Ver2.00 Compatible Clock Generator '1' Programmable Clock Generator"]
    #[inline(always)]
    #[must_use]
    pub fn clock_gensel(&mut self) -> ClockGenselW<SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec> {
        ClockGenselW::new(self, 10)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn driver_strength_sel(
        &mut self,
    ) -> DriverStrengthSelW<SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec> {
        DriverStrengthSelW::new(self, 14)
    }
}
#[doc = "This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_preset_value5::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_preset_value5 to value 0x01"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgPresetValue5Spec {
    const RESET_VALUE: u16 = 0x01;
}
