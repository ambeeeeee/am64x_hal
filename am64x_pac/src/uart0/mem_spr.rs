#[doc = "Register `MEM_SPR` reader"]
pub type R = crate::R<MemSprSpec>;
#[doc = "Register `MEM_SPR` writer"]
pub type W = crate::W<MemSprSpec>;
#[doc = "Field `SPR_WORD` reader - 7:0\\]
Scratchpad register"]
pub type SprWordR = crate::FieldReader;
#[doc = "Field `SPR_WORD` writer - 7:0\\]
Scratchpad register"]
pub type SprWordW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Scratchpad register"]
    #[inline(always)]
    pub fn spr_word(&self) -> SprWordR {
        SprWordR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Scratchpad register"]
    #[inline(always)]
    #[must_use]
    pub fn spr_word(&mut self) -> SprWordW<MemSprSpec> {
        SprWordW::new(self, 0)
    }
}
#[doc = "This read/write register does not control the module in anyway. It is intended as a scratchpad register to be used by the programmer to hold temporary data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_spr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_spr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSprSpec;
impl crate::RegisterSpec for MemSprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_spr::R`](R) reader structure"]
impl crate::Readable for MemSprSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_spr::W`](W) writer structure"]
impl crate::Writable for MemSprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SPR to value 0"]
impl crate::Resettable for MemSprSpec {
    const RESET_VALUE: u32 = 0;
}
