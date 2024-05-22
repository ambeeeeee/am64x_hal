#[doc = "Register `MEM_CLR_RIS_TRIG67` reader"]
pub type R = crate::R<MemClrRisTrig67Spec>;
#[doc = "Register `MEM_CLR_RIS_TRIG67` writer"]
pub type W = crate::W<MemClrRisTrig67Spec>;
#[doc = "Field `CLRRIS6` reader - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 6 bits"]
pub type Clrris6R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS6` writer - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 6 bits"]
pub type Clrris6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRRIS7` reader - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 7 bits"]
pub type Clrris7R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS7` writer - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 7 bits"]
pub type Clrris7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 6 bits"]
    #[inline(always)]
    pub fn clrris6(&self) -> Clrris6R {
        Clrris6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 7 bits"]
    #[inline(always)]
    pub fn clrris7(&self) -> Clrris7R {
        Clrris7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 6 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris6(&mut self) -> Clrris6W<MemClrRisTrig67Spec> {
        Clrris6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 7 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris7(&mut self) -> Clrris7W<MemClrRisTrig67Spec> {
        Clrris7W::new(self, 16)
    }
}
#[doc = "Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrRisTrig67Spec;
impl crate::RegisterSpec for MemClrRisTrig67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_ris_trig67::R`](R) reader structure"]
impl crate::Readable for MemClrRisTrig67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_ris_trig67::W`](W) writer structure"]
impl crate::Writable for MemClrRisTrig67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_RIS_TRIG67 to value 0"]
impl crate::Resettable for MemClrRisTrig67Spec {
    const RESET_VALUE: u32 = 0;
}
