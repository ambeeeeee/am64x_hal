#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_6` reader"]
pub type R = crate::R<MemElmSyndromeFragment6Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_6` writer"]
pub type W = crate::W<MemElmSyndromeFragment6Spec>;
#[doc = "Field `SYNDROME_6` reader - 15:0\\]
Syndrome bits 192 to 207"]
pub type Syndrome6R = crate::FieldReader<u16>;
#[doc = "Field `SYNDROME_6` writer - 15:0\\]
Syndrome bits 192 to 207"]
pub type Syndrome6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SYNDROME_VALID` reader - 16:16\\]
Syndrome valid bit 0x0: this syndrome polynomial should not be processed 0x1: this syndrome polynomial must be processed"]
pub type SyndromeValidR = crate::BitReader;
#[doc = "Field `SYNDROME_VALID` writer - 16:16\\]
Syndrome valid bit 0x0: this syndrome polynomial should not be processed 0x1: this syndrome polynomial must be processed"]
pub type SyndromeValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Syndrome bits 192 to 207"]
    #[inline(always)]
    pub fn syndrome_6(&self) -> Syndrome6R {
        Syndrome6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Syndrome valid bit 0x0: this syndrome polynomial should not be processed 0x1: this syndrome polynomial must be processed"]
    #[inline(always)]
    pub fn syndrome_valid(&self) -> SyndromeValidR {
        SyndromeValidR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Syndrome bits 192 to 207"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_6(&mut self) -> Syndrome6W<MemElmSyndromeFragment6Spec> {
        Syndrome6W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Syndrome valid bit 0x0: this syndrome polynomial should not be processed 0x1: this syndrome polynomial must be processed"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_valid(&mut self) -> SyndromeValidW<MemElmSyndromeFragment6Spec> {
        SyndromeValidW::new(self, 16)
    }
}
#[doc = "Input syndrome polynomial bits 192 to 207.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment6Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_6::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment6Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_6::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_6 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment6Spec {
    const RESET_VALUE: u32 = 0;
}
