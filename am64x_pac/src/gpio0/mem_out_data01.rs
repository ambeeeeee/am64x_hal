#[doc = "Register `MEM_OUT_DATA01` reader"]
pub type R = crate::R<MemOutData01Spec>;
#[doc = "Register `MEM_OUT_DATA01` writer"]
pub type W = crate::W<MemOutData01Spec>;
#[doc = "Field `OUT0` reader - 15:0\\]
Output drive state of GPIO bank 0 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out0R = crate::FieldReader<u16>;
#[doc = "Field `OUT0` writer - 15:0\\]
Output drive state of GPIO bank 0 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUT1` reader - 31:16\\]
Output drive state of GPIO bank 1 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out1R = crate::FieldReader<u16>;
#[doc = "Field `OUT1` writer - 31:16\\]
Output drive state of GPIO bank 1 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 0 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 1 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 0 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<MemOutData01Spec> {
        Out0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 1 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<MemOutData01Spec> {
        Out1W::new(self, 16)
    }
}
#[doc = "Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemOutData01Spec;
impl crate::RegisterSpec for MemOutData01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_out_data01::R`](R) reader structure"]
impl crate::Readable for MemOutData01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_out_data01::W`](W) writer structure"]
impl crate::Writable for MemOutData01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_OUT_DATA01 to value 0"]
impl crate::Resettable for MemOutData01Spec {
    const RESET_VALUE: u32 = 0;
}
