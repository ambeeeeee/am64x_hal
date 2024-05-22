#[doc = "Register `APBADDR_PMU_CPU1_PMEVCNTR2_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu1Pmevcntr2El0Spec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMEVCNTR2_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu1Pmevcntr2El0Spec>;
#[doc = "Field `PMEVCNTR2_EL0` reader - 31:0\\]
Event counter n. Value of event counter n, where n is the number of this register and is a number from 0 to 30."]
pub type Pmevcntr2El0R = crate::FieldReader<u32>;
#[doc = "Field `PMEVCNTR2_EL0` writer - 31:0\\]
Event counter n. Value of event counter n, where n is the number of this register and is a number from 0 to 30."]
pub type Pmevcntr2El0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Event counter n. Value of event counter n, where n is the number of this register and is a number from 0 to 30."]
    #[inline(always)]
    pub fn pmevcntr2_el0(&self) -> Pmevcntr2El0R {
        Pmevcntr2El0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Event counter n. Value of event counter n, where n is the number of this register and is a number from 0 to 30."]
    #[inline(always)]
    #[must_use]
    pub fn pmevcntr2_el0(&mut self) -> Pmevcntr2El0W<ApbaddrPmuCpu1Pmevcntr2El0Spec> {
        Pmevcntr2El0W::new(self, 0)
    }
}
#[doc = "Performance Monitors Event Count Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr2_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr2_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1Pmevcntr2El0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu1Pmevcntr2El0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmevcntr2_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1Pmevcntr2El0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmevcntr2_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1Pmevcntr2El0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMEVCNTR2_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu1Pmevcntr2El0Spec {
    const RESET_VALUE: u32 = 0;
}
