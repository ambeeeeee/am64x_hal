#[doc = "Register `MEM_SET_RIS_TRIG23` reader"]
pub type R = crate::R<MemSetRisTrig23Spec>;
#[doc = "Register `MEM_SET_RIS_TRIG23` writer"]
pub type W = crate::W<MemSetRisTrig23Spec>;
#[doc = "Field `SETRIS2` reader - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 2 bits"]
pub type Setris2R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS2` writer - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 2 bits"]
pub type Setris2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETRIS3` reader - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 3 bits"]
pub type Setris3R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS3` writer - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 3 bits"]
pub type Setris3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 2 bits"]
    #[inline(always)]
    pub fn setris2(&self) -> Setris2R {
        Setris2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 3 bits"]
    #[inline(always)]
    pub fn setris3(&self) -> Setris3R {
        Setris3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris2(&mut self) -> Setris2W<MemSetRisTrig23Spec> {
        Setris2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris3(&mut self) -> Setris3W<MemSetRisTrig23Spec> {
        Setris3W::new(self, 16)
    }
}
#[doc = "Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetRisTrig23Spec;
impl crate::RegisterSpec for MemSetRisTrig23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_ris_trig23::R`](R) reader structure"]
impl crate::Readable for MemSetRisTrig23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_ris_trig23::W`](W) writer structure"]
impl crate::Writable for MemSetRisTrig23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_RIS_TRIG23 to value 0"]
impl crate::Resettable for MemSetRisTrig23Spec {
    const RESET_VALUE: u32 = 0;
}
