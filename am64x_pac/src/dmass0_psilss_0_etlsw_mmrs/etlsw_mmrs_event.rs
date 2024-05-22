#[doc = "Register `ETLSW_MMRS_event` reader"]
pub type R = crate::R<EtlswMmrsEventSpec>;
#[doc = "Register `ETLSW_MMRS_event` writer"]
pub type W = crate::W<EtlswMmrsEventSpec>;
#[doc = "Field `EVT` reader - 15:0\\]
The event to produce."]
pub type EvtR = crate::FieldReader<u16>;
#[doc = "Field `EVT` writer - 15:0\\]
The event to produce."]
pub type EvtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The event to produce."]
    #[inline(always)]
    pub fn evt(&self) -> EvtR {
        EvtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The event to produce."]
    #[inline(always)]
    #[must_use]
    pub fn evt(&mut self) -> EvtW<EtlswMmrsEventSpec> {
        EvtW::new(self, 0)
    }
}
#[doc = "The Event Register defines the event to produce for a link down event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_event::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_event::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtlswMmrsEventSpec;
impl crate::RegisterSpec for EtlswMmrsEventSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etlsw_mmrs_event::R`](R) reader structure"]
impl crate::Readable for EtlswMmrsEventSpec {}
#[doc = "`write(|w| ..)` method takes [`etlsw_mmrs_event::W`](W) writer structure"]
impl crate::Writable for EtlswMmrsEventSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETLSW_MMRS_event to value 0x0006_5535"]
impl crate::Resettable for EtlswMmrsEventSpec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
