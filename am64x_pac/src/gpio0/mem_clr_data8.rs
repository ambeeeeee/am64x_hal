#[doc = "Register `MEM_CLR_DATA8` reader"]
pub type R = crate::R<MemClrData8Spec>;
#[doc = "Register `MEM_CLR_DATA8` writer"]
pub type W = crate::W<MemClrData8Spec>;
#[doc = "Field `CLR8` reader - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr8R = crate::FieldReader<u16>;
#[doc = "Field `CLR8` writer - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
pub type Clr8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    pub fn clr8(&self) -> Clr8R {
        Clr8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 clears the output drive state of GPIO Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn clr8(&mut self) -> Clr8W<MemClrData8Spec> {
        Clr8W::new(self, 0)
    }
}
#[doc = "Clear Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clr_data8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clr_data8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemClrData8Spec;
impl crate::RegisterSpec for MemClrData8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_clr_data8::R`](R) reader structure"]
impl crate::Readable for MemClrData8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_clr_data8::W`](W) writer structure"]
impl crate::Writable for MemClrData8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CLR_DATA8 to value 0"]
impl crate::Resettable for MemClrData8Spec {
    const RESET_VALUE: u32 = 0;
}
