#[doc = "Register `MEM_SET_RIS_TRIG67` reader"]
pub type R = crate::R<MemSetRisTrig67Spec>;
#[doc = "Register `MEM_SET_RIS_TRIG67` writer"]
pub type W = crate::W<MemSetRisTrig67Spec>;
#[doc = "Field `SETRIS6` reader - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 6 bits"]
pub type Setris6R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS6` writer - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 6 bits"]
pub type Setris6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETRIS7` reader - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 7 bits"]
pub type Setris7R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS7` writer - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 7 bits"]
pub type Setris7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 6 bits"]
    #[inline(always)]
    pub fn setris6(&self) -> Setris6R {
        Setris6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 7 bits"]
    #[inline(always)]
    pub fn setris7(&self) -> Setris7R {
        Setris7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 6 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris6(&mut self) -> Setris6W<MemSetRisTrig67Spec> {
        Setris6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 7 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris7(&mut self) -> Setris7W<MemSetRisTrig67Spec> {
        Setris7W::new(self, 16)
    }
}
#[doc = "Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetRisTrig67Spec;
impl crate::RegisterSpec for MemSetRisTrig67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_ris_trig67::R`](R) reader structure"]
impl crate::Readable for MemSetRisTrig67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_ris_trig67::W`](W) writer structure"]
impl crate::Writable for MemSetRisTrig67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_RIS_TRIG67 to value 0"]
impl crate::Resettable for MemSetRisTrig67Spec {
    const RESET_VALUE: u32 = 0;
}
