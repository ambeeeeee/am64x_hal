#[doc = "Register `CPTS_VBUSP_EVENT_2_REG` reader"]
pub type R = crate::R<CptsVbuspEvent2RegSpec>;
#[doc = "Register `CPTS_VBUSP_EVENT_2_REG` writer"]
pub type W = crate::W<CptsVbuspEvent2RegSpec>;
#[doc = "Field `DOMAIN` reader - 7:0\\]
Domain"]
pub type DomainR = crate::FieldReader;
#[doc = "Field `DOMAIN` writer - 7:0\\]
Domain"]
pub type DomainW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Domain"]
    #[inline(always)]
    pub fn domain(&self) -> DomainR {
        DomainR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Domain"]
    #[inline(always)]
    #[must_use]
    pub fn domain(&mut self) -> DomainW<CptsVbuspEvent2RegSpec> {
        DomainW::new(self, 0)
    }
}
#[doc = "Event 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_event_2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_event_2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspEvent2RegSpec;
impl crate::RegisterSpec for CptsVbuspEvent2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_event_2_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspEvent2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_event_2_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspEvent2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_EVENT_2_REG to value 0"]
impl crate::Resettable for CptsVbuspEvent2RegSpec {
    const RESET_VALUE: u32 = 0;
}
