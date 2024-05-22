#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_5` reader"]
pub type R = crate::R<MemElmSyndromeFragment5Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_5` writer"]
pub type W = crate::W<MemElmSyndromeFragment5Spec>;
#[doc = "Field `SYNDROME_5` reader - 31:0\\]
Syndrome bits 160 to 191"]
pub type Syndrome5R = crate::FieldReader<u32>;
#[doc = "Field `SYNDROME_5` writer - 31:0\\]
Syndrome bits 160 to 191"]
pub type Syndrome5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 160 to 191"]
    #[inline(always)]
    pub fn syndrome_5(&self) -> Syndrome5R {
        Syndrome5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 160 to 191"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_5(&mut self) -> Syndrome5W<MemElmSyndromeFragment5Spec> {
        Syndrome5W::new(self, 0)
    }
}
#[doc = "Input syndrome polynomial bits 160 to 191.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment5Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_5::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment5Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_5::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_5 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment5Spec {
    const RESET_VALUE: u32 = 0;
}
