#[doc = "Register `MEM_SET_RIS_TRIG45` reader"]
pub type R = crate::R<MemSetRisTrig45Spec>;
#[doc = "Register `MEM_SET_RIS_TRIG45` writer"]
pub type W = crate::W<MemSetRisTrig45Spec>;
#[doc = "Field `SETRIS4` reader - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 4 bits"]
pub type Setris4R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS4` writer - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 4 bits"]
pub type Setris4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETRIS5` reader - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 5 bits"]
pub type Setris5R = crate::FieldReader<u16>;
#[doc = "Field `SETRIS5` writer - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 5 bits"]
pub type Setris5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 4 bits"]
    #[inline(always)]
    pub fn setris4(&self) -> Setris4R {
        Setris4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 5 bits"]
    #[inline(always)]
    pub fn setris5(&self) -> Setris5R {
        Setris5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 enables rising edge detection for GPIO bank 4 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris4(&mut self) -> Setris4W<MemSetRisTrig45Spec> {
        Setris4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 enables rising edge detection for GPIO bank 5 bits"]
    #[inline(always)]
    #[must_use]
    pub fn setris5(&mut self) -> Setris5W<MemSetRisTrig45Spec> {
        Setris5W::new(self, 16)
    }
}
#[doc = "Set Rising Edge Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_ris_trig45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_ris_trig45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetRisTrig45Spec;
impl crate::RegisterSpec for MemSetRisTrig45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_ris_trig45::R`](R) reader structure"]
impl crate::Readable for MemSetRisTrig45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_ris_trig45::W`](W) writer structure"]
impl crate::Writable for MemSetRisTrig45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_RIS_TRIG45 to value 0"]
impl crate::Resettable for MemSetRisTrig45Spec {
    const RESET_VALUE: u32 = 0;
}
