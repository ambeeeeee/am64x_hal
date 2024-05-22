#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_2` reader"]
pub type R = crate::R<MemElmSyndromeFragment2Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_2` writer"]
pub type W = crate::W<MemElmSyndromeFragment2Spec>;
#[doc = "Field `SYNDROME_2` reader - 31:0\\]
Syndrome bits 64 to 95"]
pub type Syndrome2R = crate::FieldReader<u32>;
#[doc = "Field `SYNDROME_2` writer - 31:0\\]
Syndrome bits 64 to 95"]
pub type Syndrome2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 64 to 95"]
    #[inline(always)]
    pub fn syndrome_2(&self) -> Syndrome2R {
        Syndrome2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 64 to 95"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_2(&mut self) -> Syndrome2W<MemElmSyndromeFragment2Spec> {
        Syndrome2W::new(self, 0)
    }
}
#[doc = "Input syndrome polynomial bits 64 to 95.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment2Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_2::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_2::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_2 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment2Spec {
    const RESET_VALUE: u32 = 0;
}
