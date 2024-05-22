#[doc = "Register `MEM_CLR_RIS_TRIG23` reader"]
pub type R = crate::R<MemClrRisTrig23Spec>;
#[doc = "Register `MEM_CLR_RIS_TRIG23` writer"]
pub type W = crate::W<MemClrRisTrig23Spec>;
#[doc = "Field `CLRRIS2` reader - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 2 bits"]
pub type Clrris2R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS2` writer - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 2 bits"]
pub type Clrris2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRRIS3` reader - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 3 bits"]
pub type Clrris3R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS3` writer - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 3 bits"]
pub type Clrris3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 2 bits"]
    #[inline(always)]
    pub fn clrris2(&self) -> Clrris2R {
        Clrris2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 3 bits"]
    #[inline(always)]
    pub fn clrris3(&self) -> Clrris3R {
        Clrris3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris2(&mut self) -> Clrris2W<MemClrRisTrig23Spec> {
        Clrris2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris3(&mut self) -> Clrris3W<MemClrRisTrig23Spec> {
        Clrris3W::new(self, 16)
    }
}
#[doc = "Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrRisTrig23Spec;
impl crate::RegisterSpec for MemClrRisTrig23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_ris_trig23::R`](R) reader structure"]
impl crate::Readable for MemClrRisTrig23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_ris_trig23::W`](W) writer structure"]
impl crate::Writable for MemClrRisTrig23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_RIS_TRIG23 to value 0"]
impl crate::Resettable for MemClrRisTrig23Spec {
    const RESET_VALUE: u32 = 0;
}
