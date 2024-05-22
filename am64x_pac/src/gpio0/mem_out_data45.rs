#[doc = "Register `MEM_OUT_DATA45` reader"]
pub type R = crate::R<MemOutData45Spec>;
#[doc = "Register `MEM_OUT_DATA45` writer"]
pub type W = crate::W<MemOutData45Spec>;
#[doc = "Field `OUT4` reader - 15:0\\]
Output drive state of GPIO bank 4 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out4R = crate::FieldReader<u16>;
#[doc = "Field `OUT4` writer - 15:0\\]
Output drive state of GPIO bank 4 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUT5` reader - 31:16\\]
Output drive state of GPIO bank 5 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out5R = crate::FieldReader<u16>;
#[doc = "Field `OUT5` writer - 31:16\\]
Output drive state of GPIO bank 5 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 4 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out4(&self) -> Out4R {
        Out4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 5 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out5(&self) -> Out5R {
        Out5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 4 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out4(&mut self) -> Out4W<MemOutData45Spec> {
        Out4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Output drive state of GPIO bank 5 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out5(&mut self) -> Out5W<MemOutData45Spec> {
        Out5W::new(self, 16)
    }
}
#[doc = "Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemOutData45Spec;
impl crate::RegisterSpec for MemOutData45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_out_data45::R`](R) reader structure"]
impl crate::Readable for MemOutData45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_out_data45::W`](W) writer structure"]
impl crate::Writable for MemOutData45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_OUT_DATA45 to value 0"]
impl crate::Resettable for MemOutData45Spec {
    const RESET_VALUE: u32 = 0;
}
