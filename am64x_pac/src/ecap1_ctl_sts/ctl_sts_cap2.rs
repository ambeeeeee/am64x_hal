#[doc = "Register `CTL_STS_CAP2` reader"]
pub type R = crate::R<CtlStsCap2Spec>;
#[doc = "Register `CTL_STS_CAP2` writer"]
pub type W = crate::W<CtlStsCap2Spec>;
#[doc = "Field `CAP2` reader - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes 3. ACMP shadow register (i.e. CAP4) when used in APWM mode"]
pub type Cap2R = crate::FieldReader<u32>;
#[doc = "Field `CAP2` writer - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes 3. ACMP shadow register (i.e. CAP4) when used in APWM mode"]
pub type Cap2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes 3. ACMP shadow register (i.e. CAP4) when used in APWM mode"]
    #[inline(always)]
    pub fn cap2(&self) -> Cap2R {
        Cap2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register can be loaded (written) by : 1. Time-Stamp (i.e. counter value) during a Capture event 2. S/W - may be useful for test purposes 3. ACMP shadow register (i.e. CAP4) when used in APWM mode"]
    #[inline(always)]
    #[must_use]
    pub fn cap2(&mut self) -> Cap2W<CtlStsCap2Spec> {
        Cap2W::new(self, 0)
    }
}
#[doc = "CTL_STS_CAP2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsCap2Spec;
impl crate::RegisterSpec for CtlStsCap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_cap2::R`](R) reader structure"]
impl crate::Readable for CtlStsCap2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_cap2::W`](W) writer structure"]
impl crate::Writable for CtlStsCap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_CAP2 to value 0"]
impl crate::Resettable for CtlStsCap2Spec {
    const RESET_VALUE: u32 = 0;
}
