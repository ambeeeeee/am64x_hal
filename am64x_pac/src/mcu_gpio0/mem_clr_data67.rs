#[doc = "Register `MEM_CLR_DATA67` reader"]
pub type R = crate::R<MemClrData67Spec>;
#[doc = "Register `MEM_CLR_DATA67` writer"]
pub type W = crate::W<MemClrData67Spec>;
#[doc = "Field `CLR6` reader - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr6R = crate::FieldReader<u16>;
#[doc = "Field `CLR6` writer - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLR7` reader - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr7R = crate::FieldReader<u16>;
#[doc = "Field `CLR7` writer - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr6(&self) -> Clr6R {
        Clr6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr7(&self) -> Clr7R {
        Clr7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr6(&mut self) -> Clr6W<MemClrData67Spec> {
        Clr6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr7(&mut self) -> Clr7W<MemClrData67Spec> {
        Clr7W::new(self, 16)
    }
}
#[doc = "Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrData67Spec;
impl crate::RegisterSpec for MemClrData67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_data67::R`](R) reader structure"]
impl crate::Readable for MemClrData67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_data67::W`](W) writer structure"]
impl crate::Writable for MemClrData67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_DATA67 to value 0"]
impl crate::Resettable for MemClrData67Spec {
    const RESET_VALUE: u32 = 0;
}
