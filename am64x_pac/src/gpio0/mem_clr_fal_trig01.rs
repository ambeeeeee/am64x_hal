#[doc = "Register `MEM_CLR_FAL_TRIG01` reader"]
pub type R = crate::R<MemClrFalTrig01Spec>;
#[doc = "Register `MEM_CLR_FAL_TRIG01` writer"]
pub type W = crate::W<MemClrFalTrig01Spec>;
#[doc = "Field `CLRFAL0` reader - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 0 bits"]
pub type Clrfal0R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL0` writer - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 0 bits"]
pub type Clrfal0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRFAL1` reader - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 1 bits"]
pub type Clrfal1R = crate::FieldReader<u16>;
#[doc = "Field `CLRFAL1` writer - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 1 bits"]
pub type Clrfal1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 0 bits"]
    #[inline(always)]
    pub fn clrfal0(&self) -> Clrfal0R {
        Clrfal0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 1 bits"]
    #[inline(always)]
    pub fn clrfal1(&self) -> Clrfal1R {
        Clrfal1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears falling edge detection for for GPIO bank 0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal0(&mut self) -> Clrfal0W<MemClrFalTrig01Spec> {
        Clrfal0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears falling edge detection for for GPIO bank 1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrfal1(&mut self) -> Clrfal1W<MemClrFalTrig01Spec> {
        Clrfal1W::new(self, 16)
    }
}
#[doc = "Clear Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_fal_trig01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_fal_trig01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrFalTrig01Spec;
impl crate::RegisterSpec for MemClrFalTrig01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_fal_trig01::R`](R) reader structure"]
impl crate::Readable for MemClrFalTrig01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_fal_trig01::W`](W) writer structure"]
impl crate::Writable for MemClrFalTrig01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_FAL_TRIG01 to value 0"]
impl crate::Resettable for MemClrFalTrig01Spec {
    const RESET_VALUE: u32 = 0;
}
