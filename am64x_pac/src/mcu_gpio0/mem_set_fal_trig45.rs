#[doc = "Register `MEM_SET_FAL_TRIG45` reader"]
pub type R = crate::R<MemSetFalTrig45Spec>;
#[doc = "Register `MEM_SET_FAL_TRIG45` writer"]
pub type W = crate::W<MemSetFalTrig45Spec>;
#[doc = "Field `SETFAL4` reader - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 4 bits"]
pub type Setfal4R = crate::FieldReader<u16>;
#[doc = "Field `SETFAL4` writer - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 4 bits"]
pub type Setfal4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETFAL5` reader - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 5 bits"]
pub type Setfal5R = crate::FieldReader<u16>;
#[doc = "Field `SETFAL5` writer - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 5 bits"]
pub type Setfal5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 4 bits"]
    #[inline(always)]
    pub fn setfal4(&self) -> Setfal4R {
        Setfal4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 5 bits"]
    #[inline(always)]
    pub fn setfal5(&self) -> Setfal5R {
        Setfal5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables falling edge detection for for GPIO bank 4 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setfal4(&mut self) -> Setfal4W<MemSetFalTrig45Spec> {
        Setfal4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables falling edge detection for for GPIO bank 5 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setfal5(&mut self) -> Setfal5W<MemSetFalTrig45Spec> {
        Setfal5W::new(self, 16)
    }
}
#[doc = "Set Falling Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_fal_trig45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_fal_trig45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetFalTrig45Spec;
impl crate::RegisterSpec for MemSetFalTrig45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_fal_trig45::R`](R) reader structure"]
impl crate::Readable for MemSetFalTrig45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_fal_trig45::W`](W) writer structure"]
impl crate::Writable for MemSetFalTrig45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_FAL_TRIG45 to value 0"]
impl crate::Resettable for MemSetFalTrig45Spec {
    const RESET_VALUE: u32 = 0;
}
