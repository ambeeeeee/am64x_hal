#[doc = "Register `MEM_CLR_FAL_TRIG67` reader"]
pub type R = crate::R<MemClrFalTrig67Spec>;
#[doc = "Register `MEM_CLR_FAL_TRIG67` writer"]
pub type W = crate::W<MemClrFalTrig67Spec>;
#[doc = "Field `CLRFAL6` reader - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 6 bits"]
pub type Clrfal6R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL6` writer - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 6 bits"]
pub type Clrfal6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRFAL7` reader - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 7 bits"]
pub type Clrfal7R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL7` writer - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 7 bits"]
pub type Clrfal7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 6 bits"]
    #[inline(always)]
    pub fn clrfal6(&self) -> Clrfal6R {
        Clrfal6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 7 bits"]
    #[inline(always)]
    pub fn clrfal7(&self) -> Clrfal7R {
        Clrfal7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 6 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal6(&mut self) -> Clrfal6W<MemClrFalTrig67Spec> {
        Clrfal6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 7 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal7(&mut self) -> Clrfal7W<MemClrFalTrig67Spec> {
        Clrfal7W::new(self, 16)
    }
}
#[doc = "Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrFalTrig67Spec;
impl crate::RegisterSpec for MemClrFalTrig67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_fal_trig67::R`](R) reader structure"]
impl crate::Readable for MemClrFalTrig67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_fal_trig67::W`](W) writer structure"]
impl crate::Writable for MemClrFalTrig67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_FAL_TRIG67 to value 0"]
impl crate::Resettable for MemClrFalTrig67Spec {
    const RESET_VALUE: u32 = 0;
}
