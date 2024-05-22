#[doc = "Register `APBADDR_PMU_CPU0_PMCCNTR_EL0_31_0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0PmccntrEl0_31_0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMCCNTR_EL0_31_0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0PmccntrEl0_31_0Spec>;
#[doc = "Field `CCNT` reader - 31:0\\]
Cycle count. Depending on the values of PMCR_EL0.{LC,D}, the cycle count increments in one of the following ways:Every processor clock cycle.Every 64th processor clock cycle.The cycle count can be reset to zero by writing 1 to PMCR_EL0.C."]
pub type CcntR = crate::FieldReader<u32>;
#[doc = "Field `CCNT` writer - 31:0\\]
Cycle count. Depending on the values of PMCR_EL0.{LC,D}, the cycle count increments in one of the following ways:Every processor clock cycle.Every 64th processor clock cycle.The cycle count can be reset to zero by writing 1 to PMCR_EL0.C."]
pub type CcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Cycle count. Depending on the values of PMCR_EL0.{LC,D}, the cycle count increments in one of the following ways:Every processor clock cycle.Every 64th processor clock cycle.The cycle count can be reset to zero by writing 1 to PMCR_EL0.C."]
    #[inline(always)]
    pub fn ccnt(&self) -> CcntR {
        CcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Cycle count. Depending on the values of PMCR_EL0.{LC,D}, the cycle count increments in one of the following ways:Every processor clock cycle.Every 64th processor clock cycle.The cycle count can be reset to zero by writing 1 to PMCR_EL0.C."]
    #[inline(always)]
    #[must_use]
    pub fn ccnt(&mut self) -> CcntW<ApbaddrPmuCpu0PmccntrEl0_31_0Spec> {
        CcntW::new(self, 0)
    }
}
#[doc = "Performance Monitors Cycle Counter (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmccntr_el0_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmccntr_el0_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0PmccntrEl0_31_0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0PmccntrEl0_31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmccntr_el0_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0PmccntrEl0_31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmccntr_el0_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0PmccntrEl0_31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMCCNTR_EL0_31_0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0PmccntrEl0_31_0Spec {
    const RESET_VALUE: u32 = 0;
}
