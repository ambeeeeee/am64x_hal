#[doc = "Register `GTC_CFG0_PUSHEVT` reader"]
pub type R = crate::R<GtcCfg0PushevtSpec>;
#[doc = "Register `GTC_CFG0_PUSHEVT` writer"]
pub type W = crate::W<GtcCfg0PushevtSpec>;
#[doc = "Field `PUSHEVT_EXPBIT_SEL` reader - 5:0\\]
Selects which bit \\[63:0\\]
of the System Counter value is exported on the push_evt output. This field controls the 64:1 mux that drives the push_evt output."]
pub type PushevtExpbitSelR = crate::FieldReader;
#[doc = "Field `PUSHEVT_EXPBIT_SEL` writer - 5:0\\]
Selects which bit \\[63:0\\]
of the System Counter value is exported on the push_evt output. This field controls the 64:1 mux that drives the push_evt output."]
pub type PushevtExpbitSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Selects which bit \\[63:0\\]
of the System Counter value is exported on the push_evt output. This field controls the 64:1 mux that drives the push_evt output."]
    #[inline(always)]
    pub fn pushevt_expbit_sel(&self) -> PushevtExpbitSelR {
        PushevtExpbitSelR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Selects which bit \\[63:0\\]
of the System Counter value is exported on the push_evt output. This field controls the 64:1 mux that drives the push_evt output."]
    #[inline(always)]
    #[must_use]
    pub fn pushevt_expbit_sel(&mut self) -> PushevtExpbitSelW<GtcCfg0PushevtSpec> {
        PushevtExpbitSelW::new(self, 0)
    }
}
#[doc = "GTC_CFG0_PUSHEVT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg0_pushevt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg0_pushevt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg0PushevtSpec;
impl crate::RegisterSpec for GtcCfg0PushevtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg0_pushevt::R`](R) reader structure"]
impl crate::Readable for GtcCfg0PushevtSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg0_pushevt::W`](W) writer structure"]
impl crate::Writable for GtcCfg0PushevtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG0_PUSHEVT to value 0"]
impl crate::Resettable for GtcCfg0PushevtSpec {
    const RESET_VALUE: u32 = 0;
}
