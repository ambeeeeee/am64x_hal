#[doc = "Register `RINGACC_CFG_EVT` reader"]
pub type R = crate::R<RingaccCfgEvtSpec>;
#[doc = "Register `RINGACC_CFG_EVT` writer"]
pub type W = crate::W<RingaccCfgEvtSpec>;
#[doc = "Field `EVT` reader - 15:0\\]
Defines the event for this ring or queue."]
pub type EvtR = crate::FieldReader<u16>;
#[doc = "Field `EVT` writer - 15:0\\]
Defines the event for this ring or queue."]
pub type EvtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the event for this ring or queue."]
    #[inline(always)]
    pub fn evt(&self) -> EvtR {
        EvtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the event for this ring or queue."]
    #[inline(always)]
    #[must_use]
    pub fn evt(&mut self) -> EvtW<RingaccCfgEvtSpec> {
        EvtW::new(self, 0)
    }
}
#[doc = "The Ring Event Register is an Output Event Steering 'OES' register that specifies the event number used to denote the occurrence of an up event \\[empty to not-empty\\]
or a down event \\[non-empty to empty\\]
for this ring.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_evt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_evt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccCfgEvtSpec;
impl crate::RegisterSpec for RingaccCfgEvtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_cfg_evt::R`](R) reader structure"]
impl crate::Readable for RingaccCfgEvtSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_cfg_evt::W`](W) writer structure"]
impl crate::Writable for RingaccCfgEvtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_CFG_EVT to value 0x0006_5535"]
impl crate::Resettable for RingaccCfgEvtSpec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
