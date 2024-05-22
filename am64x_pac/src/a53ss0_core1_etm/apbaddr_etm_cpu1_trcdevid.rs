#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVID` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcdevidSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCDEVID` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcdevidSpec>;
#[doc = "Field `DEVID` reader - 31:0\\]
Indicates the capabilities of the trace unit. The implemented width of this field and its bit assignments are IMPLEMENTATION DEFINED. Unimplemented bits are RAZ/WI.If a component is configurable then ARM recommends that this field can also indicate which configuration options are implemented that differ from the standard configuration."]
pub type DevidR = crate::FieldReader<u32>;
#[doc = "Field `DEVID` writer - 31:0\\]
Indicates the capabilities of the trace unit. The implemented width of this field and its bit assignments are IMPLEMENTATION DEFINED. Unimplemented bits are RAZ/WI.If a component is configurable then ARM recommends that this field can also indicate which configuration options are implemented that differ from the standard configuration."]
pub type DevidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the capabilities of the trace unit. The implemented width of this field and its bit assignments are IMPLEMENTATION DEFINED. Unimplemented bits are RAZ/WI.If a component is configurable then ARM recommends that this field can also indicate which configuration options are implemented that differ from the standard configuration."]
    #[inline(always)]
    pub fn devid(&self) -> DevidR {
        DevidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the capabilities of the trace unit. The implemented width of this field and its bit assignments are IMPLEMENTATION DEFINED. Unimplemented bits are RAZ/WI.If a component is configurable then ARM recommends that this field can also indicate which configuration options are implemented that differ from the standard configuration."]
    #[inline(always)]
    #[must_use]
    pub fn devid(&mut self) -> DevidW<ApbaddrEtmCpu1TrcdevidSpec> {
        DevidW::new(self, 0)
    }
}
#[doc = "Device ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcdevidSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcdevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcdevid::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcdevidSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcdevid::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcdevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCDEVID to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcdevidSpec {
    const RESET_VALUE: u32 = 0;
}
