#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR9` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr9Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR9` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr9Spec>;
#[doc = "Field `NUMP0KEY` reader - 31:0\\]
Indicates the number of P0 right-hand keys that the trace unit can use. A value of 0 or 1 indicates one P0 key."]
pub type Nump0keyR = crate::FieldReader<u32>;
#[doc = "Field `NUMP0KEY` writer - 31:0\\]
Indicates the number of P0 right-hand keys that the trace unit can use. A value of 0 or 1 indicates one P0 key."]
pub type Nump0keyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of P0 right-hand keys that the trace unit can use. A value of 0 or 1 indicates one P0 key."]
    #[inline(always)]
    pub fn nump0key(&self) -> Nump0keyR {
        Nump0keyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of P0 right-hand keys that the trace unit can use. A value of 0 or 1 indicates one P0 key."]
    #[inline(always)]
    #[must_use]
    pub fn nump0key(&mut self) -> Nump0keyW<ApbaddrEtmCpu1Trcidr9Spec> {
        Nump0keyW::new(self, 0)
    }
}
#[doc = "ID Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr9Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr9::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr9Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr9::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR9 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr9Spec {
    const RESET_VALUE: u32 = 0;
}
