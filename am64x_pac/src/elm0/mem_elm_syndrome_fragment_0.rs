#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_0` reader"]
pub type R = crate::R<MemElmSyndromeFragment0Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_0` writer"]
pub type W = crate::W<MemElmSyndromeFragment0Spec>;
#[doc = "Field `SYNDROME_0` reader - 31:0\\]
Syndrome bits 0 to 31"]
pub type Syndrome0R = crate::FieldReader<u32>;
#[doc = "Field `SYNDROME_0` writer - 31:0\\]
Syndrome bits 0 to 31"]
pub type Syndrome0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 0 to 31"]
    #[inline(always)]
    pub fn syndrome_0(&self) -> Syndrome0R {
        Syndrome0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 0 to 31"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_0(&mut self) -> Syndrome0W<MemElmSyndromeFragment0Spec> {
        Syndrome0W::new(self, 0)
    }
}
#[doc = "Input syndrome polynomial bits 0 to 31.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment0Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_0::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment0Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_0::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_0 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment0Spec {
    const RESET_VALUE: u32 = 0;
}
