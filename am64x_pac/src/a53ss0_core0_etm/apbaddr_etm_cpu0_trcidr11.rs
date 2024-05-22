#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR11` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcidr11Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR11` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcidr11Spec>;
#[doc = "Field `NUMP1SPC` reader - 31:0\\]
Indicates the number of special P1 right-hand keys that the trace unit can use."]
pub type Nump1spcR = crate::FieldReader<u32>;
#[doc = "Field `NUMP1SPC` writer - 31:0\\]
Indicates the number of special P1 right-hand keys that the trace unit can use."]
pub type Nump1spcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of special P1 right-hand keys that the trace unit can use."]
    #[inline(always)]
    pub fn nump1spc(&self) -> Nump1spcR {
        Nump1spcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of special P1 right-hand keys that the trace unit can use."]
    #[inline(always)]
    #[must_use]
    pub fn nump1spc(&mut self) -> Nump1spcW<ApbaddrEtmCpu0Trcidr11Spec> {
        Nump1spcW::new(self, 0)
    }
}
#[doc = "ID Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcidr11Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcidr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcidr11::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcidr11Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcidr11::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcidr11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCIDR11 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcidr11Spec {
    const RESET_VALUE: u32 = 0;
}
