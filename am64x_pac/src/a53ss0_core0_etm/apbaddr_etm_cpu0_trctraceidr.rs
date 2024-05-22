#[doc = "Register `APBADDR_ETM_CPU0_TRCTRACEIDR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrctraceidrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCTRACEIDR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrctraceidrSpec>;
#[doc = "Field `TRACEID` reader - 6:0\\]
Trace ID value. When only instruction tracing is enabled this provides the trace ID."]
pub type TraceidR = crate::FieldReader;
#[doc = "Field `TRACEID` writer - 6:0\\]
Trace ID value. When only instruction tracing is enabled this provides the trace ID."]
pub type TraceidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Trace ID value. When only instruction tracing is enabled this provides the trace ID."]
    #[inline(always)]
    pub fn traceid(&self) -> TraceidR {
        TraceidR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Trace ID value. When only instruction tracing is enabled this provides the trace ID."]
    #[inline(always)]
    #[must_use]
    pub fn traceid(&mut self) -> TraceidW<ApbaddrEtmCpu0TrctraceidrSpec> {
        TraceidW::new(self, 0)
    }
}
#[doc = "Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trctraceidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trctraceidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrctraceidrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrctraceidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trctraceidr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrctraceidrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trctraceidr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrctraceidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCTRACEIDR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrctraceidrSpec {
    const RESET_VALUE: u32 = 0;
}
