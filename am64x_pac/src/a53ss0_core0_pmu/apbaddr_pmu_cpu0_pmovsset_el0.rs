#[doc = "Register `APBADDR_PMU_CPU0_PMOVSSET_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0PmovssetEl0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMOVSSET_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0PmovssetEl0Spec>;
#[doc = "Field `P_X` reader - 30:0\\]
Event counter overflow set bit for PMEVCNTR&lt;x>.N is the value in PMCR_EL0.N. Bits \\[30:N\\]
are RAZ/WI.Possible values are: 0 When read, means that PMEVCNTR&lt;x> has not overflowed. When written, has no effect. 1 When read, means that PMEVCNTR&lt;x> has overflowed. When written, sets the PMEVCNTR&lt;x> overflow bit to 1."]
pub type PXR = crate::FieldReader<u32>;
#[doc = "Field `P_X` writer - 30:0\\]
Event counter overflow set bit for PMEVCNTR&lt;x>.N is the value in PMCR_EL0.N. Bits \\[30:N\\]
are RAZ/WI.Possible values are: 0 When read, means that PMEVCNTR&lt;x> has not overflowed. When written, has no effect. 1 When read, means that PMEVCNTR&lt;x> has overflowed. When written, sets the PMEVCNTR&lt;x> overflow bit to 1."]
pub type PXW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `C` reader - 31:31\\]
PMCCNTR_EL0 overflow bit. Possible values are: 0 When read, means the cycle counter has not overflowed. When written, has no effect. 1 When read, means the cycle counter has overflowed. When written, sets the overflow bit to 1."]
pub type CR = crate::BitReader;
#[doc = "Field `C` writer - 31:31\\]
PMCCNTR_EL0 overflow bit. Possible values are: 0 When read, means the cycle counter has not overflowed. When written, has no effect. 1 When read, means the cycle counter has overflowed. When written, sets the overflow bit to 1."]
pub type CW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - 30:0\\]
Event counter overflow set bit for PMEVCNTR&lt;x>.N is the value in PMCR_EL0.N. Bits \\[30:N\\]
are RAZ/WI.Possible values are: 0 When read, means that PMEVCNTR&lt;x> has not overflowed. When written, has no effect. 1 When read, means that PMEVCNTR&lt;x> has overflowed. When written, sets the PMEVCNTR&lt;x> overflow bit to 1."]
    #[inline(always)]
    pub fn p_x(&self) -> PXR {
        PXR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
PMCCNTR_EL0 overflow bit. Possible values are: 0 When read, means the cycle counter has not overflowed. When written, has no effect. 1 When read, means the cycle counter has overflowed. When written, sets the overflow bit to 1."]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - 30:0\\]
Event counter overflow set bit for PMEVCNTR&lt;x>.N is the value in PMCR_EL0.N. Bits \\[30:N\\]
are RAZ/WI.Possible values are: 0 When read, means that PMEVCNTR&lt;x> has not overflowed. When written, has no effect. 1 When read, means that PMEVCNTR&lt;x> has overflowed. When written, sets the PMEVCNTR&lt;x> overflow bit to 1."]
    #[inline(always)]
    #[must_use]
    pub fn p_x(&mut self) -> PXW<ApbaddrPmuCpu0PmovssetEl0Spec> {
        PXW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
PMCCNTR_EL0 overflow bit. Possible values are: 0 When read, means the cycle counter has not overflowed. When written, has no effect. 1 When read, means the cycle counter has overflowed. When written, sets the overflow bit to 1."]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> CW<ApbaddrPmuCpu0PmovssetEl0Spec> {
        CW::new(self, 31)
    }
}
#[doc = "Performance Monitors Overflow Flag Status Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmovsset_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmovsset_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0PmovssetEl0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0PmovssetEl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmovsset_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0PmovssetEl0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmovsset_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0PmovssetEl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMOVSSET_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0PmovssetEl0Spec {
    const RESET_VALUE: u32 = 0;
}
