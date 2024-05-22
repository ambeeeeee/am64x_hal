#[doc = "Register `APBADDR_PMU_CPU1_PMPIDR7` reader"]
pub type R = crate::R<ApbaddrPmuCpu1Pmpidr7Spec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMPIDR7` writer"]
pub type W = crate::W<ApbaddrPmuCpu1Pmpidr7Spec>;
impl W {}
#[doc = "Performance Monitors Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1Pmpidr7Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu1Pmpidr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmpidr7::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1Pmpidr7Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmpidr7::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1Pmpidr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMPIDR7 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu1Pmpidr7Spec {
    const RESET_VALUE: u32 = 0;
}
