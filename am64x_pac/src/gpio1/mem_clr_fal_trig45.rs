#[doc = "Register `MEM_CLR_FAL_TRIG45` reader"]
pub type R = crate::R<MemClrFalTrig45Spec>;
#[doc = "Register `MEM_CLR_FAL_TRIG45` writer"]
pub type W = crate::W<MemClrFalTrig45Spec>;
#[doc = "Field `CLRFAL4` reader - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 4 bits"]
pub type Clrfal4R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL4` writer - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 4 bits"]
pub type Clrfal4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRFAL5` reader - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 5 bits"]
pub type Clrfal5R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL5` writer - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 5 bits"]
pub type Clrfal5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 4 bits"]
    #[inline(always)]
    pub fn clrfal4(&self) -> Clrfal4R {
        Clrfal4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 5 bits"]
    #[inline(always)]
    pub fn clrfal5(&self) -> Clrfal5R {
        Clrfal5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 4 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal4(&mut self) -> Clrfal4W<MemClrFalTrig45Spec> {
        Clrfal4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 5 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal5(&mut self) -> Clrfal5W<MemClrFalTrig45Spec> {
        Clrfal5W::new(self, 16)
    }
}
#[doc = "Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrFalTrig45Spec;
impl crate::RegisterSpec for MemClrFalTrig45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_fal_trig45::R`](R) reader structure"]
impl crate::Readable for MemClrFalTrig45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_fal_trig45::W`](W) writer structure"]
impl crate::Writable for MemClrFalTrig45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_FAL_TRIG45 to value 0"]
impl crate::Resettable for MemClrFalTrig45Spec {
    const RESET_VALUE: u32 = 0;
}
