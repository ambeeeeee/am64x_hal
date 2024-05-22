#[doc = "Register `MEM_IN_DATA45` reader"]
pub type R = crate::R<MemInData45Spec>;
#[doc = "Register `MEM_IN_DATA45` writer"]
pub type W = crate::W<MemInData45Spec>;
#[doc = "Field `IN4` reader - 15:0\\]
Status of GPIO bank 4 bits"]
pub type In4R = crate::FieldReader<u16>;
#[doc = "Field `IN4` writer - 15:0\\]
Status of GPIO bank 4 bits"]
pub type In4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IN5` reader - 31:16\\]
Status of GPIO bank 5 bits"]
pub type In5R = crate::FieldReader<u16>;
#[doc = "Field `IN5` writer - 31:16\\]
Status of GPIO bank 5 bits"]
pub type In5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 4 bits"]
    #[inline(always)]
    pub fn in4(&self) -> In4R {
        In4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 5 bits"]
    #[inline(always)]
    pub fn in5(&self) -> In5R {
        In5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 4 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in4(&mut self) -> In4W<MemInData45Spec> {
        In4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 5 bits"]
    #[inline(always)]
    #[must_use]
    pub fn in5(&mut self) -> In5W<MemInData45Spec> {
        In5W::new(self, 16)
    }
}
#[doc = "Bank Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_in_data45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_in_data45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemInData45Spec;
impl crate::RegisterSpec for MemInData45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_in_data45::R`](R) reader structure"]
impl crate::Readable for MemInData45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_in_data45::W`](W) writer structure"]
impl crate::Writable for MemInData45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IN_DATA45 to value 0"]
impl crate::Resettable for MemInData45Spec {
    const RESET_VALUE: u32 = 0;
}
