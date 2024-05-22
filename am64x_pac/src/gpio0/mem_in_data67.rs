#[doc = "Register `MEM_IN_DATA67` reader"]
pub type R = crate::R<MemInData67Spec>;
#[doc = "Register `MEM_IN_DATA67` writer"]
pub type W = crate::W<MemInData67Spec>;
#[doc = "Field `IN6` reader - 15:0\\]
Status of GPIO bank 6 bits"]
pub type In6R = crate::FieldReader<u16>;
#[doc = "Field `IN6` writer - 15:0\\]
Status of GPIO bank 6 bits"]
pub type In6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IN7` reader - 31:16\\]
Status of GPIO bank 7 bits"]
pub type In7R = crate::FieldReader<u16>;
#[doc = "Field `IN7` writer - 31:16\\]
Status of GPIO bank 7 bits"]
pub type In7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 6 bits"]
    #[inline(always)]
    pub fn in6(&self) -> In6R {
        In6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 7 bits"]
    #[inline(always)]
    pub fn in7(&self) -> In7R {
        In7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 6 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in6(&mut self) -> In6W<MemInData67Spec> {
        In6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 7 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in7(&mut self) -> In7W<MemInData67Spec> {
        In7W::new(self, 16)
    }
}
#[doc = "Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemInData67Spec;
impl crate::RegisterSpec for MemInData67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_in_data67::R`](R) reader structure"]
impl crate::Readable for MemInData67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_in_data67::W`](W) writer structure"]
impl crate::Writable for MemInData67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IN_DATA67 to value 0"]
impl crate::Resettable for MemInData67Spec {
    const RESET_VALUE: u32 = 0;
}
