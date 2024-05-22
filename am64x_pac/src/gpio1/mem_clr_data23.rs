#[doc = "Register `MEM_CLR_DATA23` reader"]
pub type R = crate::R<MemClrData23Spec>;
#[doc = "Register `MEM_CLR_DATA23` writer"]
pub type W = crate::W<MemClrData23Spec>;
#[doc = "Field `CLR2` reader - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr2R = crate::FieldReader<u16>;
#[doc = "Field `CLR2` writer - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLR3` reader - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr3R = crate::FieldReader<u16>;
#[doc = "Field `CLR3` writer - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr2(&self) -> Clr2R {
        Clr2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr3(&self) -> Clr3R {
        Clr3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr2(&mut self) -> Clr2W<MemClrData23Spec> {
        Clr2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr3(&mut self) -> Clr3W<MemClrData23Spec> {
        Clr3W::new(self, 16)
    }
}
#[doc = "Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrData23Spec;
impl crate::RegisterSpec for MemClrData23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_data23::R`](R) reader structure"]
impl crate::Readable for MemClrData23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_data23::W`](W) writer structure"]
impl crate::Writable for MemClrData23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_DATA23 to value 0"]
impl crate::Resettable for MemClrData23Spec {
    const RESET_VALUE: u32 = 0;
}
