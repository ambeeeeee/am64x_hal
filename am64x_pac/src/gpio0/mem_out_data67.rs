#[doc = "Register `MEM_OUT_DATA67` reader"]
pub type R = crate::R<MemOutData67Spec>;
#[doc = "Register `MEM_OUT_DATA67` writer"]
pub type W = crate::W<MemOutData67Spec>;
#[doc = "Field `OUT6` reader - 15:0\\]
Output drive state of GPIO bank 6 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out6R = crate::FieldReader<u16>;
#[doc = "Field `OUT6` writer - 15:0\\]
Output drive state of GPIO bank 6 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUT7` reader - 31:16\\]
Output drive state of GPIO bank 7 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out7R = crate::FieldReader<u16>;
#[doc = "Field `OUT7` writer - 31:16\\]
Output drive state of GPIO bank 7 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 6 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out6(&self) -> Out6R {
        Out6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 7 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out7(&self) -> Out7R {
        Out7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 6 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out6(&mut self) -> Out6W<MemOutData67Spec> {
        Out6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 7 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out7(&mut self) -> Out7W<MemOutData67Spec> {
        Out7W::new(self, 16)
    }
}
#[doc = "Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemOutData67Spec;
impl crate::RegisterSpec for MemOutData67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_out_data67::R`](R) reader structure"]
impl crate::Readable for MemOutData67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_out_data67::W`](W) writer structure"]
impl crate::Writable for MemOutData67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_OUT_DATA67 to value 0"]
impl crate::Resettable for MemOutData67Spec {
    const RESET_VALUE: u32 = 0;
}
