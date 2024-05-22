#[doc = "Register `MEM_CLR_DATA01` reader"]
pub type R = crate::R<MemClrData01Spec>;
#[doc = "Register `MEM_CLR_DATA01` writer"]
pub type W = crate::W<MemClrData01Spec>;
#[doc = "Field `CLR0` reader - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr0R = crate::FieldReader<u16>;
#[doc = "Field `CLR0` writer - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLR1` reader - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr1R = crate::FieldReader<u16>;
#[doc = "Field `CLR1` writer - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr0(&self) -> Clr0R {
        Clr0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr1(&self) -> Clr1R {
        Clr1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr0(&mut self) -> Clr0W<MemClrData01Spec> {
        Clr0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr1(&mut self) -> Clr1W<MemClrData01Spec> {
        Clr1W::new(self, 16)
    }
}
#[doc = "Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrData01Spec;
impl crate::RegisterSpec for MemClrData01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_data01::R`](R) reader structure"]
impl crate::Readable for MemClrData01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_data01::W`](W) writer structure"]
impl crate::Writable for MemClrData01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_DATA01 to value 0"]
impl crate::Resettable for MemClrData01Spec {
    const RESET_VALUE: u32 = 0;
}
