#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVAFF0` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcdevaff0Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVAFF0` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcdevaff0Spec>;
#[doc = "Field `MPIDR_EL1_31_0` reader - 31:0\\]
Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type MpidrEl1_31_0R = crate::FieldReader<u32>;
#[doc = "Field `MPIDR_EL1_31_0` writer - 31:0\\]
Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type MpidrEl1_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn mpidr_el1_31_0(&self) -> MpidrEl1_31_0R {
        MpidrEl1_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn mpidr_el1_31_0(&mut self) -> MpidrEl1_31_0W<ApbaddrEtmCpu1Trcdevaff0Spec> {
        MpidrEl1_31_0W::new(self, 0)
    }
}
#[doc = "Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevaff0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevaff0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcdevaff0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcdevaff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcdevaff0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcdevaff0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcdevaff0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcdevaff0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCDEVAFF0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcdevaff0Spec {
    const RESET_VALUE: u32 = 0;
}
