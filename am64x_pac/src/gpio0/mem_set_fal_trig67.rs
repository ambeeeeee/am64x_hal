#[doc = "Register `MEM_SET_FAL_TRIG67` reader"]
pub type R = crate::R<MemSetFalTrig67Spec>;
#[doc = "Register `MEM_SET_FAL_TRIG67` writer"]
pub type W = crate::W<MemSetFalTrig67Spec>;
#[doc = "Field `SETFAL6` reader - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 6 bits"]
pub type Setfal6R = crate::FieldReader<u16>;
#[doc = "Field `SETFAL6` writer - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 6 bits"]
pub type Setfal6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETFAL7` reader - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 7 bits"]
pub type Setfal7R = crate::FieldReader<u16>;
#[doc = "Field `SETFAL7` writer - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 7 bits"]
pub type Setfal7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 6 bits"]
    #[inline(always)]
    pub fn setfal6(&self) -> Setfal6R {
        Setfal6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 7 bits"]
    #[inline(always)]
    pub fn setfal7(&self) -> Setfal7R {
        Setfal7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 6 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setfal6(&mut self) -> Setfal6W<MemSetFalTrig67Spec> {
        Setfal6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 7 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setfal7(&mut self) -> Setfal7W<MemSetFalTrig67Spec> {
        Setfal7W::new(self, 16)
    }
}
#[doc = "Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetFalTrig67Spec;
impl crate::RegisterSpec for MemSetFalTrig67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_fal_trig67::R`](R) reader structure"]
impl crate::Readable for MemSetFalTrig67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_fal_trig67::W`](W) writer structure"]
impl crate::Writable for MemSetFalTrig67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_FAL_TRIG67 to value 0"]
impl crate::Resettable for MemSetFalTrig67Spec {
    const RESET_VALUE: u32 = 0;
}
