#[doc = "Register `MEM_INTSTAT8` reader"]
pub type R = crate::R<MemIntstat8Spec>;
#[doc = "Register `MEM_INTSTAT8` writer"]
pub type W = crate::W<MemIntstat8Spec>;
#[doc = "Field `STAT8` reader - 15:0\\]
Status of GPIO bank 8 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat8R = crate::FieldReader<u16>;
#[doc = "Field `STAT8` writer - 15:0\\]
Status of GPIO bank 8 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 8 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat8(&self) -> Stat8R {
        Stat8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 8 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat8(&mut self) -> Stat8W<MemIntstat8Spec> {
        Stat8W::new(self, 0)
    }
}
#[doc = "Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIntstat8Spec;
impl crate::RegisterSpec for MemIntstat8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_intstat8::R`](R) reader structure"]
impl crate::Readable for MemIntstat8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_intstat8::W`](W) writer structure"]
impl crate::Writable for MemIntstat8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INTSTAT8 to value 0"]
impl crate::Resettable for MemIntstat8Spec {
    const RESET_VALUE: u32 = 0;
}
