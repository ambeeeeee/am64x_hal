#[doc = "Register `MEM_CLR_DATA45` reader"]
pub type R = crate::R<MemClrData45Spec>;
#[doc = "Register `MEM_CLR_DATA45` writer"]
pub type W = crate::W<MemClrData45Spec>;
#[doc = "Field `CLR4` reader - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr4R = crate::FieldReader<u16>;
#[doc = "Field `CLR4` writer - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLR5` reader - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr5R = crate::FieldReader<u16>;
#[doc = "Field `CLR5` writer - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr4(&self) -> Clr4R {
        Clr4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr5(&self) -> Clr5R {
        Clr5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr4(&mut self) -> Clr4W<MemClrData45Spec> {
        Clr4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr5(&mut self) -> Clr5W<MemClrData45Spec> {
        Clr5W::new(self, 16)
    }
}
#[doc = "Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrData45Spec;
impl crate::RegisterSpec for MemClrData45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_data45::R`](R) reader structure"]
impl crate::Readable for MemClrData45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_data45::W`](W) writer structure"]
impl crate::Writable for MemClrData45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_DATA45 to value 0"]
impl crate::Resettable for MemClrData45Spec {
    const RESET_VALUE: u32 = 0;
}
