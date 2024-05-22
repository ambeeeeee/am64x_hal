#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR13` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcidr13Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR13` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcidr13Spec>;
#[doc = "Field `NUMCONDSPC` reader - 31:0\\]
Indicates the number of special conditional instruction right-hand keys that the trace unit can use."]
pub type NumcondspcR = crate::FieldReader<u32>;
#[doc = "Field `NUMCONDSPC` writer - 31:0\\]
Indicates the number of special conditional instruction right-hand keys that the trace unit can use."]
pub type NumcondspcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of special conditional instruction right-hand keys that the trace unit can use."]
    #[inline(always)]
    pub fn numcondspc(&self) -> NumcondspcR {
        NumcondspcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of special conditional instruction right-hand keys that the trace unit can use."]
    #[inline(always)]
    #[must_use]
    pub fn numcondspc(&mut self) -> NumcondspcW<ApbaddrEtmCpu0Trcidr13Spec> {
        NumcondspcW::new(self, 0)
    }
}
#[doc = "ID Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcidr13Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcidr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcidr13::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcidr13Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcidr13::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcidr13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCIDR13 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcidr13Spec {
    const RESET_VALUE: u32 = 0;
}
