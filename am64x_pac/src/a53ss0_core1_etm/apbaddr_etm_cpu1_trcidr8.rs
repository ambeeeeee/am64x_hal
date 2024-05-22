#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR8` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr8Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR8` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr8Spec>;
#[doc = "Field `MAXSPEC` reader - 31:0\\]
Indicates the maximum speculation depth of the instruction trace stream. This is the maximum number of P0 elements in the trace stream that can be speculative at any time."]
pub type MaxspecR = crate::FieldReader<u32>;
#[doc = "Field `MAXSPEC` writer - 31:0\\]
Indicates the maximum speculation depth of the instruction trace stream. This is the maximum number of P0 elements in the trace stream that can be speculative at any time."]
pub type MaxspecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the maximum speculation depth of the instruction trace stream. This is the maximum number of P0 elements in the trace stream that can be speculative at any time."]
    #[inline(always)]
    pub fn maxspec(&self) -> MaxspecR {
        MaxspecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the maximum speculation depth of the instruction trace stream. This is the maximum number of P0 elements in the trace stream that can be speculative at any time."]
    #[inline(always)]
    #[must_use]
    pub fn maxspec(&mut self) -> MaxspecW<ApbaddrEtmCpu1Trcidr8Spec> {
        MaxspecW::new(self, 0)
    }
}
#[doc = "ID Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr8Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr8::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr8Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr8::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR8 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr8Spec {
    const RESET_VALUE: u32 = 0;
}
