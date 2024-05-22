#[doc = "Register `RINGACC_GCFG_error_evt` reader"]
pub type R = crate::R<RingaccGcfgErrorEvtSpec>;
#[doc = "Register `RINGACC_GCFG_error_evt` writer"]
pub type W = crate::W<RingaccGcfgErrorEvtSpec>;
#[doc = "Field `EVT` reader - 15:0\\]
Event to send when detecting a bus error."]
pub type EvtR = crate::FieldReader<u16>;
#[doc = "Field `EVT` writer - 15:0\\]
Event to send when detecting a bus error."]
pub type EvtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Event to send when detecting a bus error."]
    #[inline(always)]
    pub fn evt(&self) -> EvtR {
        EvtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Event to send when detecting a bus error."]
    #[inline(always)]
    #[must_use]
    pub fn evt(&mut self) -> EvtW<RingaccGcfgErrorEvtSpec> {
        EvtW::new(self, 0)
    }
}
#[doc = "The Error Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote detection of a ring memory transaction bus error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_gcfg_error_evt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_gcfg_error_evt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccGcfgErrorEvtSpec;
impl crate::RegisterSpec for RingaccGcfgErrorEvtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_gcfg_error_evt::R`](R) reader structure"]
impl crate::Readable for RingaccGcfgErrorEvtSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_gcfg_error_evt::W`](W) writer structure"]
impl crate::Writable for RingaccGcfgErrorEvtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_GCFG_error_evt to value 0x0006_5535"]
impl crate::Resettable for RingaccGcfgErrorEvtSpec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
