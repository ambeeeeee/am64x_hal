#[doc = "Register `MEM_INTSTAT01` reader"]
pub type R = crate::R<MemIntstat01Spec>;
#[doc = "Register `MEM_INTSTAT01` writer"]
pub type W = crate::W<MemIntstat01Spec>;
#[doc = "Field `STAT0` reader - 15:0\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat0R = crate::FieldReader<u16>;
#[doc = "Field `STAT0` writer - 15:0\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STAT1` reader - 31:16\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat1R = crate::FieldReader<u16>;
#[doc = "Field `STAT1` writer - 31:16\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
pub type Stat1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat0(&self) -> Stat0R {
        Stat0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    pub fn stat1(&self) -> Stat1R {
        Stat1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat0(&mut self) -> Stat0W<MemIntstat01Spec> {
        Stat0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Status of GPIO bank 0 bits interrupt Reading back 1 = interrupt occurred 0 = interrupt hasnt occurred since last cleared Writing 1 clears the corresponding interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn stat1(&mut self) -> Stat1W<MemIntstat01Spec> {
        Stat1W::new(self, 16)
    }
}
#[doc = "Bank Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_intstat01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_intstat01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIntstat01Spec;
impl crate::RegisterSpec for MemIntstat01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_intstat01::R`](R) reader structure"]
impl crate::Readable for MemIntstat01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_intstat01::W`](W) writer structure"]
impl crate::Writable for MemIntstat01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INTSTAT01 to value 0"]
impl crate::Resettable for MemIntstat01Spec {
    const RESET_VALUE: u32 = 0;
}
