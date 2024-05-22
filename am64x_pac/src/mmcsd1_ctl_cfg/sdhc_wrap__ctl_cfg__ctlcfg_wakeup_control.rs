#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_wakeup_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_wakeup_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec>;
#[doc = "Field `CARD_INTERRUPT` reader - 0:0\\]
This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register.This bit can be set to 1 if FN_WUS \\[Wake Up Support\\]
in CIS is set to 1."]
pub type CardInterruptR = crate::BitReader;
#[doc = "Field `CARD_INTERRUPT` writer - 0:0\\]
This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register.This bit can be set to 1 if FN_WUS \\[Wake Up Support\\]
in CIS is set to 1."]
pub type CardInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION` reader - 1:1\\]
This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
pub type CardInsertionR = crate::BitReader;
#[doc = "Field `CARD_INSERTION` writer - 1:1\\]
This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
pub type CardInsertionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - 2:2\\]
This bit enables wakeup event via Card removal assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
pub type CardRemovalR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - 2:2\\]
This bit enables wakeup event via Card removal assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register.This bit can be set to 1 if FN_WUS \\[Wake Up Support\\]
in CIS is set to 1."]
    #[inline(always)]
    pub fn card_interrupt(&self) -> CardInterruptR {
        CardInterruptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
    #[inline(always)]
    pub fn card_insertion(&self) -> CardInsertionR {
        CardInsertionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables wakeup event via Card removal assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit enables wakeup event via Card Interrupt assertion in the Normal Interrupt Status register.This bit can be set to 1 if FN_WUS \\[Wake Up Support\\]
in CIS is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt(&mut self) -> CardInterruptW<SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec> {
        CardInterruptW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit enables wakeup event via Card Insertion assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion(&mut self) -> CardInsertionW<SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec> {
        CardInsertionW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit enables wakeup event via Card removal assertion in the Normal Interrupt Status register.FN_WUS \\[Wake up Support\\]
in CIS does not affect this bit."]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CardRemovalW<SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec> {
        CardRemovalW::new(self, 2)
    }
}
#[doc = "This register is used to program the wakeup functionality\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_wakeup_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_wakeup_control to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgWakeupControlSpec {
    const RESET_VALUE: u8 = 0;
}
