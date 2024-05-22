#[doc = "Register `CTL_STS_CAP4` reader"]
pub type R = crate::R<CtlStsCap4Spec>;
#[doc = "Register `CTL_STS_CAP4` writer"]
pub type W = crate::W<CtlStsCap4Spec>;
#[doc = "Field `CAP4` reader - 31:0\\]
In CMP mode this is a time-stamp capture register In APMW mode this is the Compare Shadow (ACMP) register. User updates the PWM Compare value via this register. In this mode CAP4 (ACMP) shadows CAP2"]
pub type Cap4R = crate::FieldReader<u32>;
#[doc = "Field `CAP4` writer - 31:0\\]
In CMP mode this is a time-stamp capture register In APMW mode this is the Compare Shadow (ACMP) register. User updates the PWM Compare value via this register. In this mode CAP4 (ACMP) shadows CAP2"]
pub type Cap4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
In CMP mode this is a time-stamp capture register In APMW mode this is the Compare Shadow (ACMP) register. User updates the PWM Compare value via this register. In this mode CAP4 (ACMP) shadows CAP2"]
    #[inline(always)]
    pub fn cap4(&self) -> Cap4R {
        Cap4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
In CMP mode this is a time-stamp capture register In APMW mode this is the Compare Shadow (ACMP) register. User updates the PWM Compare value via this register. In this mode CAP4 (ACMP) shadows CAP2"]
    #[inline(always)]
    #[must_use]
    pub fn cap4(&mut self) -> Cap4W<CtlStsCap4Spec> {
        Cap4W::new(self, 0)
    }
}
#[doc = "CTL_STS_CAP4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsCap4Spec;
impl crate::RegisterSpec for CtlStsCap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_cap4::R`](R) reader structure"]
impl crate::Readable for CtlStsCap4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_cap4::W`](W) writer structure"]
impl crate::Writable for CtlStsCap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_CAP4 to value 0"]
impl crate::Resettable for CtlStsCap4Spec {
    const RESET_VALUE: u32 = 0;
}
