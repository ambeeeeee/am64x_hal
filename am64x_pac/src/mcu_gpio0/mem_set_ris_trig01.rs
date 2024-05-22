#[doc = "Register `MEM_SET_RIS_TRIG01` reader"]
pub type R = crate::R<MemSetRisTrig01Spec>;
#[doc = "Register `MEM_SET_RIS_TRIG01` writer"]
pub type W = crate::W<MemSetRisTrig01Spec>;
#[doc = "Field `SETRIS0` reader - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 0 bits"]
pub type Setris0R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS0` writer - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 0 bits"]
pub type Setris0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETRIS1` reader - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 1 bits"]
pub type Setris1R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS1` writer - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 1 bits"]
pub type Setris1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 0 bits"]
    #[inline(always)]
    pub fn setris0(&self) -> Setris0R {
        Setris0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 1 bits"]
    #[inline(always)]
    pub fn setris1(&self) -> Setris1R {
        Setris1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris0(&mut self) -> Setris0W<MemSetRisTrig01Spec> {
        Setris0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris1(&mut self) -> Setris1W<MemSetRisTrig01Spec> {
        Setris1W::new(self, 16)
    }
}
#[doc = "Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetRisTrig01Spec;
impl crate::RegisterSpec for MemSetRisTrig01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_ris_trig01::R`](R) reader structure"]
impl crate::Readable for MemSetRisTrig01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_ris_trig01::W`](W) writer structure"]
impl crate::Writable for MemSetRisTrig01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_RIS_TRIG01 to value 0"]
impl crate::Resettable for MemSetRisTrig01Spec {
    const RESET_VALUE: u32 = 0;
}
