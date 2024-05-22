#[doc = "Register `MEM_INTSTAT23` reader"]
pub type R = crate::R<MemIntstat23Spec>;
#[doc = "Register `MEM_INTSTAT23` writer"]
pub type W = crate::W<MemIntstat23Spec>;
#[doc = "Field `STAT2` reader - 15:0\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat2R = crate::FieldReader<u16>;
#[doc = "Field `STAT2` writer - 15:0\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STAT3` reader - 31:16\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat3R = crate::FieldReader<u16>;
#[doc = "Field `STAT3` writer - 31:16\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat2(&self) -> Stat2R {
        Stat2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat3(&self) -> Stat3R {
        Stat3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat2(&mut self) -> Stat2W<MemIntstat23Spec> {
        Stat2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 2 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat3(&mut self) -> Stat3W<MemIntstat23Spec> {
        Stat3W::new(self, 16)
    }
}
#[doc = "Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIntstat23Spec;
impl crate::RegisterSpec for MemIntstat23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_intstat23::R`](R) reader structure"]
impl crate::Readable for MemIntstat23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_intstat23::W`](W) writer structure"]
impl crate::Writable for MemIntstat23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INTSTAT23 to value 0"]
impl crate::Resettable for MemIntstat23Spec {
    const RESET_VALUE: u32 = 0;
}
