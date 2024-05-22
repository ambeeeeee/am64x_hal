#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_vendor_register` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_vendor_register` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec>;
#[doc = "Field `ENHANCED_STROBE` reader - 0:0\\]
This bit enables the enhanced strobe logic of the Host Controller"]
pub type EnhancedStrobeR = crate::BitReader;
#[doc = "Field `ENHANCED_STROBE` writer - 0:0\\]
This bit enables the enhanced strobe logic of the Host Controller"]
pub type EnhancedStrobeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMMC_HW_RESET` reader - 1:1\\]
Hardware reset signal is generared for eMMC card when this bit is set"]
pub type EmmcHwResetR = crate::BitReader;
#[doc = "Field `EMMC_HW_RESET` writer - 1:1\\]
Hardware reset signal is generared for eMMC card when this bit is set"]
pub type EmmcHwResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD11_PD_TIMER` reader - 15:2\\]
cmd11 power-down timer value"]
pub type Cmd11PdTimerR = crate::FieldReader<u16>;
#[doc = "Field `CMD11_PD_TIMER` writer - 15:2\\]
cmd11 power-down timer value"]
pub type Cmd11PdTimerW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `AUTOGATE_SDCLK` reader - 16:16\\]
If this bit is set, SD CLK will be gated automatically when there is no transfer. This is applicable only for Embedded Device"]
pub type AutogateSdclkR = crate::BitReader;
#[doc = "Field `AUTOGATE_SDCLK` writer - 16:16\\]
If this bit is set, SD CLK will be gated automatically when there is no transfer. This is applicable only for Embedded Device"]
pub type AutogateSdclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit enables the enhanced strobe logic of the Host Controller"]
    #[inline(always)]
    pub fn enhanced_strobe(&self) -> EnhancedStrobeR {
        EnhancedStrobeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Hardware reset signal is generared for eMMC card when this bit is set"]
    #[inline(always)]
    pub fn emmc_hw_reset(&self) -> EmmcHwResetR {
        EmmcHwResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
cmd11 power-down timer value"]
    #[inline(always)]
    pub fn cmd11_pd_timer(&self) -> Cmd11PdTimerR {
        Cmd11PdTimerR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
If this bit is set, SD CLK will be gated automatically when there is no transfer. This is applicable only for Embedded Device"]
    #[inline(always)]
    pub fn autogate_sdclk(&self) -> AutogateSdclkR {
        AutogateSdclkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit enables the enhanced strobe logic of the Host Controller"]
    #[inline(always)]
    #[must_use]
    pub fn enhanced_strobe(&mut self) -> EnhancedStrobeW<SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec> {
        EnhancedStrobeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Hardware reset signal is generared for eMMC card when this bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_hw_reset(&mut self) -> EmmcHwResetW<SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec> {
        EmmcHwResetW::new(self, 1)
    }
    #[doc = "Bits 2:15 - 15:2\\]
cmd11 power-down timer value"]
    #[inline(always)]
    #[must_use]
    pub fn cmd11_pd_timer(&mut self) -> Cmd11PdTimerW<SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec> {
        Cmd11PdTimerW::new(self, 2)
    }
    #[doc = "Bit 16 - 16:16\\]
If this bit is set, SD CLK will be gated automatically when there is no transfer. This is applicable only for Embedded Device"]
    #[inline(always)]
    #[must_use]
    pub fn autogate_sdclk(&mut self) -> AutogateSdclkW<SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec> {
        AutogateSdclkW::new(self, 16)
    }
}
#[doc = "Vendor register added for autogate sdclk, cmd11 power down timer, enhancedstrobe and eMMC hardware reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_register::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_vendor_register to value 0x0001_4000"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgVendorRegisterSpec {
    const RESET_VALUE: u32 = 0x0001_4000;
}
