#[doc = "Register `MEM_INTSTAT67` reader"]
pub type R = crate::R<MemIntstat67Spec>;
#[doc = "Register `MEM_INTSTAT67` writer"]
pub type W = crate::W<MemIntstat67Spec>;
#[doc = "Field `STAT6` reader - 15:0\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat6R = crate::FieldReader<u16>;
#[doc = "Field `STAT6` writer - 15:0\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STAT7` reader - 31:16\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat7R = crate::FieldReader<u16>;
#[doc = "Field `STAT7` writer - 31:16\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat6(&self) -> Stat6R {
        Stat6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat7(&self) -> Stat7R {
        Stat7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat6(&mut self) -> Stat6W<MemIntstat67Spec> {
        Stat6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 6 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat7(&mut self) -> Stat7W<MemIntstat67Spec> {
        Stat7W::new(self, 16)
    }
}
#[doc = "Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIntstat67Spec;
impl crate::RegisterSpec for MemIntstat67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_intstat67::R`](R) reader structure"]
impl crate::Readable for MemIntstat67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_intstat67::W`](W) writer structure"]
impl crate::Writable for MemIntstat67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INTSTAT67 to value 0"]
impl crate::Resettable for MemIntstat67Spec {
    const RESET_VALUE: u32 = 0;
}
