#[doc = "Register `CPTS_VBUSP_EVENT_POP_REG` reader"]
pub type R = crate::R<CptsVbuspEventPopRegSpec>;
#[doc = "Register `CPTS_VBUSP_EVENT_POP_REG` writer"]
pub type W = crate::W<CptsVbuspEventPopRegSpec>;
#[doc = "Field `EVENT_POP` reader - 0:0\\]
Event pop"]
pub type EventPopR = crate::BitReader;
#[doc = "Field `EVENT_POP` writer - 0:0\\]
Event pop"]
pub type EventPopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Event pop"]
    #[inline(always)]
    pub fn event_pop(&self) -> EventPopR {
        EventPopR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Event pop"]
    #[inline(always)]
    #[must_use]
    pub fn event_pop(&mut self) -> EventPopW<CptsVbuspEventPopRegSpec> {
        EventPopW::new(self, 0)
    }
}
#[doc = "Event Pop Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_pop_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_pop_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspEventPopRegSpec;
impl crate::RegisterSpec for CptsVbuspEventPopRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_event_pop_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspEventPopRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_event_pop_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspEventPopRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_EVENT_POP_REG to value 0"]
impl crate::Resettable for CptsVbuspEventPopRegSpec {
    const RESET_VALUE: u32 = 0;
}
