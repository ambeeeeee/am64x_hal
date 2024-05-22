#[doc = "Register `MEM_INTSTAT45` reader"]
pub type R = crate::R<MemIntstat45Spec>;
#[doc = "Register `MEM_INTSTAT45` writer"]
pub type W = crate::W<MemIntstat45Spec>;
#[doc = "Field `STAT4` reader - 15:0\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat4R = crate::FieldReader<u16>;
#[doc = "Field `STAT4` writer - 15:0\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STAT5` reader - 31:16\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat5R = crate::FieldReader<u16>;
#[doc = "Field `STAT5` writer - 31:16\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat4(&self) -> Stat4R {
        Stat4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat5(&self) -> Stat5R {
        Stat5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat4(&mut self) -> Stat4W<MemIntstat45Spec> {
        Stat4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 4 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat5(&mut self) -> Stat5W<MemIntstat45Spec> {
        Stat5W::new(self, 16)
    }
}
#[doc = "Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIntstat45Spec;
impl crate::RegisterSpec for MemIntstat45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_intstat45::R`](R) reader structure"]
impl crate::Readable for MemIntstat45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_intstat45::W`](W) writer structure"]
impl crate::Writable for MemIntstat45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INTSTAT45 to value 0"]
impl crate::Resettable for MemIntstat45Spec {
    const RESET_VALUE: u32 = 0;
}
