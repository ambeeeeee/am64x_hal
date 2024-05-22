#[doc = "Register `MEM_CLR_FAL_TRIG8` reader"]
pub type R = crate::R<MemClrFalTrig8Spec>;
#[doc = "Register `MEM_CLR_FAL_TRIG8` writer"]
pub type W = crate::W<MemClrFalTrig8Spec>;
#[doc = "Field `CLRFAL8` reader - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 8 bits"]
pub type Clrfal8R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL8` writer - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 8 bits"]
pub type Clrfal8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 8 bits"]
    #[inline(always)]
    pub fn clrfal8(&self) -> Clrfal8R {
        Clrfal8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal8(&mut self) -> Clrfal8W<MemClrFalTrig8Spec> {
        Clrfal8W::new(self, 0)
    }
}
#[doc = "Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrFalTrig8Spec;
impl crate::RegisterSpec for MemClrFalTrig8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_fal_trig8::R`](R) reader structure"]
impl crate::Readable for MemClrFalTrig8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_fal_trig8::W`](W) writer structure"]
impl crate::Writable for MemClrFalTrig8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_FAL_TRIG8 to value 0"]
impl crate::Resettable for MemClrFalTrig8Spec {
    const RESET_VALUE: u32 = 0;
}
