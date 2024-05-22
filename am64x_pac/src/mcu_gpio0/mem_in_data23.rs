#[doc = "Register `MEM_IN_DATA23` reader"]
pub type R = crate::R<MemInData23Spec>;
#[doc = "Register `MEM_IN_DATA23` writer"]
pub type W = crate::W<MemInData23Spec>;
#[doc = "Field `IN2` reader - 15:0\\]
Status of GPIO bank 2 bits"]
pub type In2R = crate::FieldReader<u16>;
#[doc = "Field `IN2` writer - 15:0\\]
Status of GPIO bank 2 bits"]
pub type In2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IN3` reader - 31:16\\]
Status of GPIO bank 3 bits"]
pub type In3R = crate::FieldReader<u16>;
#[doc = "Field `IN3` writer - 31:16\\]
Status of GPIO bank 3 bits"]
pub type In3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 2 bits"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 3 bits"]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in2(&mut self) -> In2W<MemInData23Spec> {
        In2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 3 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in3(&mut self) -> In3W<MemInData23Spec> {
        In3W::new(self, 16)
    }
}
#[doc = "Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemInData23Spec;
impl crate::RegisterSpec for MemInData23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_in_data23::R`](R) reader structure"]
impl crate::Readable for MemInData23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_in_data23::W`](W) writer structure"]
impl crate::Writable for MemInData23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IN_DATA23 to value 0"]
impl crate::Resettable for MemInData23Spec {
    const RESET_VALUE: u32 = 0;
}
