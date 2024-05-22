#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_slot_int_sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_slot_int_sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec>;
#[doc = "Field `INTR_SIG` reader - 7:0\\]
These status bits indicate the logical OR of Interrupt signal and Wakeup signal for each slot."]
pub type IntrSigR = crate::FieldReader;
#[doc = "Field `INTR_SIG` writer - 7:0\\]
These status bits indicate the logical OR of Interrupt signal and Wakeup signal for each slot."]
pub type IntrSigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These status bits indicate the logical OR of Interrupt signal and Wakeup signal for each slot."]
    #[inline(always)]
    pub fn intr_sig(&self) -> IntrSigR {
        IntrSigR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
These status bits indicate the logical OR of Interrupt signal and Wakeup signal for each slot."]
    #[inline(always)]
    #[must_use]
    pub fn intr_sig(&mut self) -> IntrSigW<SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec> {
        IntrSigW::new(self, 0)
    }
}
#[doc = "This register is used to read the interrupt signal for each slot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_slot_int_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_slot_int_sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgSlotIntStsSpec {
    const RESET_VALUE: u16 = 0;
}
