#[doc = "Register `MEM_CLR_RIS_TRIG01` reader"]
pub type R = crate::R<MemClrRisTrig01Spec>;
#[doc = "Register `MEM_CLR_RIS_TRIG01` writer"]
pub type W = crate::W<MemClrRisTrig01Spec>;
#[doc = "Field `CLRRIS0` reader - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 0 bits"]
pub type Clrris0R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS0` writer - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 0 bits"]
pub type Clrris0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRRIS1` reader - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 1 bits"]
pub type Clrris1R = crate::FieldReader<u16>;
#[doc = "Field `CLRRIS1` writer - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 1 bits"]
pub type Clrris1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 0 bits"]
    #[inline(always)]
    pub fn clrris0(&self) -> Clrris0R {
        Clrris0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 1 bits"]
    #[inline(always)]
    pub fn clrris1(&self) -> Clrris1R {
        Clrris1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears rising edge detection for GPIO bank 0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris0(&mut self) -> Clrris0W<MemClrRisTrig01Spec> {
        Clrris0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears rising edge detection for GPIO bank 1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrris1(&mut self) -> Clrris1W<MemClrRisTrig01Spec> {
        Clrris1W::new(self, 16)
    }
}
#[doc = "Clear Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_ris_trig01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_ris_trig01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrRisTrig01Spec;
impl crate::RegisterSpec for MemClrRisTrig01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_ris_trig01::R`](R) reader structure"]
impl crate::Readable for MemClrRisTrig01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_ris_trig01::W`](W) writer structure"]
impl crate::Writable for MemClrRisTrig01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_RIS_TRIG01 to value 0"]
impl crate::Resettable for MemClrRisTrig01Spec {
    const RESET_VALUE: u32 = 0;
}
