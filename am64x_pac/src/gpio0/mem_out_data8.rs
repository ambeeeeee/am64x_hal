#[doc = "Register `MEM_OUT_DATA8` reader"]
pub type R = crate::R<MemOutData8Spec>;
#[doc = "Register `MEM_OUT_DATA8` writer"]
pub type W = crate::W<MemOutData8Spec>;
#[doc = "Field `OUT8` reader - 15:0\\]
Output drive state of GPIO bank 8 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out8R = crate::FieldReader<u16>;
#[doc = "Field `OUT8` writer - 15:0\\]
Output drive state of GPIO bank 8 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
pub type Out8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 8 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    pub fn out8(&self) -> Out8R {
        Out8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Output drive state of GPIO bank 8 bits, does not affect operation when it is configured as input Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn out8(&mut self) -> Out8W<MemOutData8Spec> {
        Out8W::new(self, 0)
    }
}
#[doc = "Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_out_data8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_out_data8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemOutData8Spec;
impl crate::RegisterSpec for MemOutData8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_out_data8::R`](R) reader structure"]
impl crate::Readable for MemOutData8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_out_data8::W`](W) writer structure"]
impl crate::Writable for MemOutData8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_OUT_DATA8 to value 0"]
impl crate::Resettable for MemOutData8Spec {
    const RESET_VALUE: u32 = 0;
}
