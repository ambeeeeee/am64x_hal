#[doc = "Register `APBADDR_CTI_CPU0_CTIDEVAFF0` reader"]
pub type R = crate::R<ApbaddrCtiCpu0Ctidevaff0Spec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIDEVAFF0` writer"]
pub type W = crate::W<ApbaddrCtiCpu0Ctidevaff0Spec>;
#[doc = "Field `CTIDEVAFF0` reader - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Ctidevaff0R = crate::FieldReader<u32>;
#[doc = "Field `CTIDEVAFF0` writer - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Ctidevaff0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn ctidevaff0(&self) -> Ctidevaff0R {
        Ctidevaff0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn ctidevaff0(&mut self) -> Ctidevaff0W<ApbaddrCtiCpu0Ctidevaff0Spec> {
        Ctidevaff0W::new(self, 0)
    }
}
#[doc = "CTI Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevaff0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevaff0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0Ctidevaff0Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu0Ctidevaff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctidevaff0::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0Ctidevaff0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctidevaff0::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0Ctidevaff0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIDEVAFF0 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0Ctidevaff0Spec {
    const RESET_VALUE: u32 = 0;
}
