#[doc = "Register `MEM_OUT_DATA23` reader"]
pub type R = crate::R<MemOutData23Spec>;
#[doc = "Register `MEM_OUT_DATA23` writer"]
pub type W = crate::W<MemOutData23Spec>;
#[doc = "Field `OUT2` reader - 15:0\\]
Output drive state of GPIO bank 2 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out2R = crate::FieldReader<u16>;
#[doc = "Field `OUT2` writer - 15:0\\]
Output drive state of GPIO bank 2 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUT3` reader - 31:16\\]
Output drive state of GPIO bank 3 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out3R = crate::FieldReader<u16>;
#[doc = "Field `OUT3` writer - 31:16\\]
Output drive state of GPIO bank 3 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 2 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 3 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out3(&self) -> Out3R {
        Out3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 2 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<MemOutData23Spec> {
        Out2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 3 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out3(&mut self) -> Out3W<MemOutData23Spec> {
        Out3W::new(self, 16)
    }
}
#[doc = "Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemOutData23Spec;
impl crate::RegisterSpec for MemOutData23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_out_data23::R`](R) reader structure"]
impl crate::Readable for MemOutData23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_out_data23::W`](W) writer structure"]
impl crate::Writable for MemOutData23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_OUT_DATA23 to value 0"]
impl crate::Resettable for MemOutData23Spec {
    const RESET_VALUE: u32 = 0;
}
