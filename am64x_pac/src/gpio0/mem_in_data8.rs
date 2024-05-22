#[doc = "Register `MEM_IN_DATA8` reader"]
pub type R = crate::R<MemInData8Spec>;
#[doc = "Register `MEM_IN_DATA8` writer"]
pub type W = crate::W<MemInData8Spec>;
#[doc = "Field `IN8` reader - 15:0\\]
Status of GPIO bank 8 bits"]
pub type In8R = crate::FieldReader<u16>;
#[doc = "Field `IN8` writer - 15:0\\]
Status of GPIO bank 8 bits"]
pub type In8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 8 bits"]
    #[inline(always)]
    pub fn in8(&self) -> In8R {
        In8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in8(&mut self) -> In8W<MemInData8Spec> {
        In8W::new(self, 0)
    }
}
#[doc = "Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemInData8Spec;
impl crate::RegisterSpec for MemInData8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_in_data8::R`](R) reader structure"]
impl crate::Readable for MemInData8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_in_data8::W`](W) writer structure"]
impl crate::Writable for MemInData8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IN_DATA8 to value 0"]
impl crate::Resettable for MemInData8Spec {
    const RESET_VALUE: u32 = 0;
}
