#[doc = "Register `APBADDR_ETM_CPU0_TRCPIDR7` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcpidr7Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCPIDR7` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcpidr7Spec>;
#[doc = "Field `RES0_TRCPIDR7_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr7_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPIDR7_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr7_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpidr7_31_8(&self) -> Res0Trcpidr7_31_8R {
        Res0Trcpidr7_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpidr7_31_8(&mut self) -> Res0Trcpidr7_31_8W<ApbaddrEtmCpu0Trcpidr7Spec> {
        Res0Trcpidr7_31_8W::new(self, 8)
    }
}
#[doc = "Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcpidr7Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcpidr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcpidr7::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcpidr7Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcpidr7::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcpidr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCPIDR7 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcpidr7Spec {
    const RESET_VALUE: u32 = 0;
}
