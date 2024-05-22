#[doc = "Register `APBADDR_PMU_CPU0_PMDEVAFF0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0Pmdevaff0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMDEVAFF0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0Pmdevaff0Spec>;
#[doc = "Field `PMDEVAFF0` reader - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Pmdevaff0R = crate::FieldReader<u32>;
#[doc = "Field `PMDEVAFF0` writer - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Pmdevaff0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn pmdevaff0(&self) -> Pmdevaff0R {
        Pmdevaff0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn pmdevaff0(&mut self) -> Pmdevaff0W<ApbaddrPmuCpu0Pmdevaff0Spec> {
        Pmdevaff0W::new(self, 0)
    }
}
#[doc = "Performance Monitors Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmdevaff0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmdevaff0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0Pmdevaff0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0Pmdevaff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmdevaff0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0Pmdevaff0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmdevaff0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0Pmdevaff0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMDEVAFF0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0Pmdevaff0Spec {
    const RESET_VALUE: u32 = 0;
}
