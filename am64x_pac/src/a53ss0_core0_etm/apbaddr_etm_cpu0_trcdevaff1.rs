#[doc = "Register `APBADDR_ETM_CPU0_TRCDEVAFF1` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcdevaff1Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCDEVAFF1` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcdevaff1Spec>;
#[doc = "Field `MPIDR_EL1_63_32` reader - 31:0\\]
Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type MpidrEl1_63_32R = crate::FieldReader<u32>;
#[doc = "Field `MPIDR_EL1_63_32` writer - 31:0\\]
Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type MpidrEl1_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn mpidr_el1_63_32(&self) -> MpidrEl1_63_32R {
        MpidrEl1_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn mpidr_el1_63_32(&mut self) -> MpidrEl1_63_32W<ApbaddrEtmCpu0Trcdevaff1Spec> {
        MpidrEl1_63_32W::new(self, 0)
    }
}
#[doc = "Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcdevaff1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcdevaff1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcdevaff1Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcdevaff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcdevaff1::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcdevaff1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcdevaff1::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcdevaff1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCDEVAFF1 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcdevaff1Spec {
    const RESET_VALUE: u32 = 0;
}
