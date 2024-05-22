#[doc = "Register `CFG_TCAR2` reader"]
pub type R = crate::R<CfgTcar2Spec>;
#[doc = "Register `CFG_TCAR2` writer"]
pub type W = crate::W<CfgTcar2Spec>;
#[doc = "Field `CAPTURE_VALUE2` reader - 31:0\\]
The value of second captured counter register"]
pub type CaptureValue2R = crate::FieldReader<u32>;
#[doc = "Field `CAPTURE_VALUE2` writer - 31:0\\]
The value of second captured counter register"]
pub type CaptureValue2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of second captured counter register"]
    #[inline(always)]
    pub fn capture_value2(&self) -> CaptureValue2R {
        CaptureValue2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of second captured counter register"]
    #[inline(always)]
    #[must_use]
    pub fn capture_value2(&mut self) -> CaptureValue2W<CfgTcar2Spec> {
        CaptureValue2W::new(self, 0)
    }
}
#[doc = "This register holds the value of the second counter register capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTcar2Spec;
impl crate::RegisterSpec for CfgTcar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tcar2::R`](R) reader structure"]
impl crate::Readable for CfgTcar2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tcar2::W`](W) writer structure"]
impl crate::Writable for CfgTcar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TCAR2 to value 0"]
impl crate::Resettable for CfgTcar2Spec {
    const RESET_VALUE: u32 = 0;
}
