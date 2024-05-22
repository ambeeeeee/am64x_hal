#[doc = "Register `APBADDR_PMU_CPU1_PMPIDR5` reader"]
pub type R = crate::R<ApbaddrPmuCpu1Pmpidr5Spec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMPIDR5` writer"]
pub type W = crate::W<ApbaddrPmuCpu1Pmpidr5Spec>;
impl W {}
#[doc = "Performance Monitors Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1Pmpidr5Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu1Pmpidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmpidr5::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1Pmpidr5Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmpidr5::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1Pmpidr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMPIDR5 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu1Pmpidr5Spec {
    const RESET_VALUE: u32 = 0;
}
