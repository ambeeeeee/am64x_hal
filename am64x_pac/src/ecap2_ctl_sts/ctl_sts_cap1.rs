#[doc = "Register `CTL_STS_CAP1` reader"]
pub type R = crate::R<CtlStsCap1Spec>;
#[doc = "Register `CTL_STS_CAP1` writer"]
pub type W = crate::W<CtlStsCap1Spec>;
#[doc = "Field `CAP1` reader - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes / initialisation 3. APRD shadow register (i.e. CAP3) when used in APWM mode"]
pub type Cap1R = crate::FieldReader<u32>;
#[doc = "Field `CAP1` writer - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes / initialisation 3. APRD shadow register (i.e. CAP3) when used in APWM mode"]
pub type Cap1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes / initialisation 3. APRD shadow register (i.e. CAP3) when used in APWM mode"]
    #[inline(always)]
    pub fn cap1(&self) -> Cap1R {
        Cap1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes / initialisation 3. APRD shadow register (i.e. CAP3) when used in APWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn cap1(&mut self) -> Cap1W<CtlStsCap1Spec> {
        Cap1W::new(self, 0)
    }
}
#[doc = "CTL_STS_CAP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsCap1Spec;
impl crate::RegisterSpec for CtlStsCap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_cap1::R`](R) reader structure"]
impl crate::Readable for CtlStsCap1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_cap1::W`](W) writer structure"]
impl crate::Writable for CtlStsCap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_CAP1 to value 0"]
impl crate::Resettable for CtlStsCap1Spec {
    const RESET_VALUE: u32 = 0;
}
