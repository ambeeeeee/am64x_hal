#[doc = "Register `APBADDR_PMU_CPU1_PMCEID1_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu1Pmceid1El0Spec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMCEID1_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu1Pmceid1El0Spec>;
#[doc = "Field `CE_32` reader - 0:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For the bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic00x020L2D_CACHE_ALLOCATE"]
pub type Ce32R = crate::BitReader;
#[doc = "Field `CE_32` writer - 0:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For the bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic00x020L2D_CACHE_ALLOCATE"]
pub type Ce32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_PMCEID1_EL0_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Pmceid1El0_31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMCEID1_EL0_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Pmceid1El0_31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For the bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic00x020L2D_CACHE_ALLOCATE"]
    #[inline(always)]
    pub fn ce_32(&self) -> Ce32R {
        Ce32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmceid1_el0_31_1(&self) -> Res0Pmceid1El0_31_1R {
        Res0Pmceid1El0_31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Common architectural and microarchitectural feature events that can be counted by the PMU event counters.For the bit described in the following table, the event is implemented if the bit is set to 1, or not implemented if the bit is set to 0.BitEvent numberEvent mnemonic00x020L2D_CACHE_ALLOCATE"]
    #[inline(always)]
    #[must_use]
    pub fn ce_32(&mut self) -> Ce32W<ApbaddrPmuCpu1Pmceid1El0Spec> {
        Ce32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmceid1_el0_31_1(&mut self) -> Res0Pmceid1El0_31_1W<ApbaddrPmuCpu1Pmceid1El0Spec> {
        Res0Pmceid1El0_31_1W::new(self, 1)
    }
}
#[doc = "Performance Monitors Common Event Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmceid1_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmceid1_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1Pmceid1El0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu1Pmceid1El0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmceid1_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1Pmceid1El0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmceid1_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1Pmceid1El0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMCEID1_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu1Pmceid1El0Spec {
    const RESET_VALUE: u32 = 0;
}
