#[doc = "Register `REG_QEPSTROBESEL` reader"]
pub type R = crate::R<RegQepstrobeselSpec>;
#[doc = "Register `REG_QEPSTROBESEL` writer"]
pub type W = crate::W<RegQepstrobeselSpec>;
#[doc = "Field `STROBESEL` reader - 1:0\\]
Strobe source select:"]
pub type StrobeselR = crate::FieldReader;
#[doc = "Field `STROBESEL` writer - 1:0\\]
Strobe source select:"]
pub type StrobeselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Strobe source select:"]
    #[inline(always)]
    pub fn strobesel(&self) -> StrobeselR {
        StrobeselR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Strobe source select:"]
    #[inline(always)]
    #[must_use]
    pub fn strobesel(&mut self) -> StrobeselW<RegQepstrobeselSpec> {
        StrobeselW::new(self, 0)
    }
}
#[doc = "QEP Strobe select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepstrobesel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepstrobesel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQepstrobeselSpec;
impl crate::RegisterSpec for RegQepstrobeselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_qepstrobesel::R`](R) reader structure"]
impl crate::Readable for RegQepstrobeselSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_qepstrobesel::W`](W) writer structure"]
impl crate::Writable for RegQepstrobeselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_QEPSTROBESEL to value 0"]
impl crate::Resettable for RegQepstrobeselSpec {
    const RESET_VALUE: u32 = 0;
}
