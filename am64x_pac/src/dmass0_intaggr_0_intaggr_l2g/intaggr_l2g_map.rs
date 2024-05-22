#[doc = "Register `INTAGGR_L2G_map` reader"]
pub type R = crate::R<IntaggrL2gMapSpec>;
#[doc = "Register `INTAGGR_L2G_map` writer"]
pub type W = crate::W<IntaggrL2gMapSpec>;
#[doc = "Field `GEVIDX` reader - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
pub type GevidxR = crate::FieldReader<u16>;
#[doc = "Field `GEVIDX` writer - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
pub type GevidxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MODE` reader - 31:31\\]
Local event detection mode. This field is set to 0 for pulsed events, and to 1 for rising edge eventss"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - 31:31\\]
Local event detection mode. This field is set to 0 for pulsed events, and to 1 for rising edge eventss"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
    #[inline(always)]
    pub fn gevidx(&self) -> GevidxR {
        GevidxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Local event detection mode. This field is set to 0 for pulsed events, and to 1 for rising edge eventss"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Global event index. This field specifies the index of the outgoing global event. Set to 0xFFFF to disable."]
    #[inline(always)]
    #[must_use]
    pub fn gevidx(&mut self) -> GevidxW<IntaggrL2gMapSpec> {
        GevidxW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Local event detection mode. This field is set to 0 for pulsed events, and to 1 for rising edge eventss"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<IntaggrL2gMapSpec> {
        ModeW::new(self, 31)
    }
}
#[doc = "This register determines how the ordinal local event is translated to a global event on the outgoing event transport lane. Both pulse and rising edge local event types are supported. With pulsed events, the event count is determined by the number of cycles for which the event signal remains high. For rising edge events, the count represents the total number of rising edge transitions. The index field of the register determines the outgoing global event index, and the mode bit specifies either pulsed or rising edge local event detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_l2g_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_l2g_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrL2gMapSpec;
impl crate::RegisterSpec for IntaggrL2gMapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_l2g_map::R`](R) reader structure"]
impl crate::Readable for IntaggrL2gMapSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_l2g_map::W`](W) writer structure"]
impl crate::Writable for IntaggrL2gMapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_L2G_map to value 0"]
impl crate::Resettable for IntaggrL2gMapSpec {
    const RESET_VALUE: u64 = 0;
}
