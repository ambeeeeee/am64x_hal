#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_1` reader"]
pub type R = crate::R<MemElmSyndromeFragment1Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_1` writer"]
pub type W = crate::W<MemElmSyndromeFragment1Spec>;
#[doc = "Field `SYNDROME_1` reader - 31:0\\]
Syndrome bits 32 to 63"]
pub type Syndrome1R = crate::FieldReader<u32>;
#[doc = "Field `SYNDROME_1` writer - 31:0\\]
Syndrome bits 32 to 63"]
pub type Syndrome1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 32 to 63"]
    #[inline(always)]
    pub fn syndrome_1(&self) -> Syndrome1R {
        Syndrome1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 32 to 63"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_1(&mut self) -> Syndrome1W<MemElmSyndromeFragment1Spec> {
        Syndrome1W::new(self, 0)
    }
}
#[doc = "Input syndrome polynomial bits 32 to 63.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment1Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_1::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment1Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_1::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_1 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment1Spec {
    const RESET_VALUE: u32 = 0;
}
