#[doc = "Register `APBADDR_PMU_CPU1_PMSWINC_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu1PmswincEl0Spec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMSWINC_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu1PmswincEl0Spec>;
#[doc = "Field `P_X` reader - 5:0\\]
Event counter software increment bit for PMEVCNTR&lt;x>"]
pub type PXR = crate::FieldReader;
#[doc = "Field `P_X` writer - 5:0\\]
Event counter software increment bit for PMEVCNTR&lt;x>"]
pub type PXW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Event counter software increment bit for PMEVCNTR&lt;x>"]
    #[inline(always)]
    pub fn p_x(&self) -> PXR {
        PXR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Event counter software increment bit for PMEVCNTR&lt;x>"]
    #[inline(always)]
    #[must_use]
    pub fn p_x(&mut self) -> PXW<ApbaddrPmuCpu1PmswincEl0Spec> {
        PXW::new(self, 0)
    }
}
#[doc = "Performance Monitors Software Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmswinc_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmswinc_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1PmswincEl0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu1PmswincEl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmswinc_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1PmswincEl0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmswinc_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1PmswincEl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMSWINC_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu1PmswincEl0Spec {
    const RESET_VALUE: u32 = 0;
}
