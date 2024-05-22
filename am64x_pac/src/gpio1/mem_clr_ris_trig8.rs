#[doc = "Register `MEM_CLR_RIS_TRIG8` reader"]
pub type R = crate::R<MemClrRisTrig8Spec>;
#[doc = "Register `MEM_CLR_RIS_TRIG8` writer"]
pub type W = crate::W<MemClrRisTrig8Spec>;
#[doc = "Field `CLRRIS8` reader - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 8 bits"]
pub type Clrris8R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS8` writer - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 8 bits"]
pub type Clrris8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 8 bits"]
    #[inline(always)]
    pub fn clrris8(&self) -> Clrris8R {
        Clrris8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris8(&mut self) -> Clrris8W<MemClrRisTrig8Spec> {
        Clrris8W::new(self, 0)
    }
}
#[doc = "Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrRisTrig8Spec;
impl crate::RegisterSpec for MemClrRisTrig8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_ris_trig8::R`](R) reader structure"]
impl crate::Readable for MemClrRisTrig8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_ris_trig8::W`](W) writer structure"]
impl crate::Writable for MemClrRisTrig8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_RIS_TRIG8 to value 0"]
impl crate::Resettable for MemClrRisTrig8Spec {
    const RESET_VALUE: u32 = 0;
}
