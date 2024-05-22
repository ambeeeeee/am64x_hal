#[doc = "Register `CFG_TCAR1` reader"]
pub type R = crate::R<CfgTcar1Spec>;
#[doc = "Register `CFG_TCAR1` writer"]
pub type W = crate::W<CfgTcar1Spec>;
#[doc = "Field `CAPTURE_VALUE1` reader - 31:0\\]
The value of first captured counter register"]
pub type CaptureValue1R = crate::FieldReader<u32>;
#[doc = "Field `CAPTURE_VALUE1` writer - 31:0\\]
The value of first captured counter register"]
pub type CaptureValue1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The value of first captured counter register"]
    #[inline(always)]
    pub fn capture_value1(&self) -> CaptureValue1R {
        CaptureValue1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The value of first captured counter register"]
    #[inline(always)]
    #[must_use]
    pub fn capture_value1(&mut self) -> CaptureValue1W<CfgTcar1Spec> {
        CaptureValue1W::new(self, 0)
    }
}
#[doc = "This register holds the value of the first counter register capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTcar1Spec;
impl crate::RegisterSpec for CfgTcar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tcar1::R`](R) reader structure"]
impl crate::Readable for CfgTcar1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tcar1::W`](W) writer structure"]
impl crate::Writable for CfgTcar1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TCAR1 to value 0"]
impl crate::Resettable for CfgTcar1Spec {
    const RESET_VALUE: u32 = 0;
}
