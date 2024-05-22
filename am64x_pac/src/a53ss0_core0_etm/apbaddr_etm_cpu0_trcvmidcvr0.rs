#[doc = "Register `APBADDR_ETM_CPU0_TRCVMIDCVR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcvmidcvr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCVMIDCVR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcvmidcvr0Spec>;
#[doc = "Field `VALUE` reader - 7:0\\]
Contains a VMID value."]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - 7:0\\]
Contains a VMID value."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains a VMID value."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Contains a VMID value."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<ApbaddrEtmCpu0Trcvmidcvr0Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "VMID Comparator Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcvmidcvr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcvmidcvr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcvmidcvr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcvmidcvr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcvmidcvr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcvmidcvr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcvmidcvr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcvmidcvr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCVMIDCVR0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcvmidcvr0Spec {
    const RESET_VALUE: u32 = 0;
}
