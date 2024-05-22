#[doc = "Register `APBADDR_PMU_CPU0_PMDEVAFF1` reader"]
pub type R = crate::R<ApbaddrPmuCpu0Pmdevaff1Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMDEVAFF1` writer"]
pub type W = crate::W<ApbaddrPmuCpu0Pmdevaff1Spec>;
#[doc = "Field `PMDEVAFF1` reader - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Pmdevaff1R = crate::FieldReader<u32>;
#[doc = "Field `PMDEVAFF1` writer - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Pmdevaff1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn pmdevaff1(&self) -> Pmdevaff1R {
        Pmdevaff1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn pmdevaff1(&mut self) -> Pmdevaff1W<ApbaddrPmuCpu0Pmdevaff1Spec> {
        Pmdevaff1W::new(self, 0)
    }
}
#[doc = "Performance Monitors Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmdevaff1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmdevaff1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0Pmdevaff1Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0Pmdevaff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmdevaff1::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0Pmdevaff1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmdevaff1::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0Pmdevaff1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMDEVAFF1 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0Pmdevaff1Spec {
    const RESET_VALUE: u32 = 0;
}
