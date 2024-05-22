#[doc = "Register `MEM_SET_FAL_TRIG01` reader"]
pub type R = crate::R<MemSetFalTrig01Spec>;
#[doc = "Register `MEM_SET_FAL_TRIG01` writer"]
pub type W = crate::W<MemSetFalTrig01Spec>;
#[doc = "Field `SETFAL0` reader - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 0 bits"]
pub type Setfal0R = crate::FieldReader<u16>;
#[doc = "Field `SETFAL0` writer - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 0 bits"]
pub type Setfal0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETFAL1` reader - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 1 bits"]
pub type Setfal1R = crate::FieldReader<u16>;
#[doc = "Field `SETFAL1` writer - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 1 bits"]
pub type Setfal1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 0 bits"]
    #[inline(always)]
    pub fn setfal0(&self) -> Setfal0R {
        Setfal0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 1 bits"]
    #[inline(always)]
    pub fn setfal1(&self) -> Setfal1R {
        Setfal1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 0 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setfal0(&mut self) -> Setfal0W<MemSetFalTrig01Spec> {
        Setfal0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 1 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setfal1(&mut self) -> Setfal1W<MemSetFalTrig01Spec> {
        Setfal1W::new(self, 16)
    }
}
#[doc = "Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetFalTrig01Spec;
impl crate::RegisterSpec for MemSetFalTrig01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_fal_trig01::R`](R) reader structure"]
impl crate::Readable for MemSetFalTrig01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_fal_trig01::W`](W) writer structure"]
impl crate::Writable for MemSetFalTrig01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_FAL_TRIG01 to value 0"]
impl crate::Resettable for MemSetFalTrig01Spec {
    const RESET_VALUE: u32 = 0;
}
