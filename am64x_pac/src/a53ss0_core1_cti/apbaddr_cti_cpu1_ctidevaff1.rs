#[doc = "Register `APBADDR_CTI_CPU1_CTIDEVAFF1` reader"]
pub type R = crate::R<ApbaddrCtiCpu1Ctidevaff1Spec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIDEVAFF1` writer"]
pub type W = crate::W<ApbaddrCtiCpu1Ctidevaff1Spec>;
#[doc = "Field `CTIDEVAFF1` reader - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Ctidevaff1R = crate::FieldReader<u32>;
#[doc = "Field `CTIDEVAFF1` writer - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Ctidevaff1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn ctidevaff1(&self) -> Ctidevaff1R {
        Ctidevaff1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn ctidevaff1(&mut self) -> Ctidevaff1W<ApbaddrCtiCpu1Ctidevaff1Spec> {
        Ctidevaff1W::new(self, 0)
    }
}
#[doc = "CTI Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevaff1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevaff1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1Ctidevaff1Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu1Ctidevaff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctidevaff1::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1Ctidevaff1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctidevaff1::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1Ctidevaff1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIDEVAFF1 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1Ctidevaff1Spec {
    const RESET_VALUE: u32 = 0;
}
