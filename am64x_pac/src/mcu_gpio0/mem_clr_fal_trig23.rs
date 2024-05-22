#[doc = "Register `MEM_CLR_FAL_TRIG23` reader"]
pub type R = crate::R<MemClrFalTrig23Spec>;
#[doc = "Register `MEM_CLR_FAL_TRIG23` writer"]
pub type W = crate::W<MemClrFalTrig23Spec>;
#[doc = "Field `CLRFAL2` reader - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 2 bits"]
pub type Clrfal2R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL2` writer - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 2 bits"]
pub type Clrfal2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRFAL3` reader - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 3 bits"]
pub type Clrfal3R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL3` writer - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 3 bits"]
pub type Clrfal3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 2 bits"]
    #[inline(always)]
    pub fn clrfal2(&self) -> Clrfal2R {
        Clrfal2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 3 bits"]
    #[inline(always)]
    pub fn clrfal3(&self) -> Clrfal3R {
        Clrfal3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal2(&mut self) -> Clrfal2W<MemClrFalTrig23Spec> {
        Clrfal2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal3(&mut self) -> Clrfal3W<MemClrFalTrig23Spec> {
        Clrfal3W::new(self, 16)
    }
}
#[doc = "Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrFalTrig23Spec;
impl crate::RegisterSpec for MemClrFalTrig23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_fal_trig23::R`](R) reader structure"]
impl crate::Readable for MemClrFalTrig23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_fal_trig23::W`](W) writer structure"]
impl crate::Writable for MemClrFalTrig23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_FAL_TRIG23 to value 0"]
impl crate::Resettable for MemClrFalTrig23Spec {
    const RESET_VALUE: u32 = 0;
}
