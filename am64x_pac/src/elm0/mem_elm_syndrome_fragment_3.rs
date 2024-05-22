#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_3` reader"]
pub type R = crate::R<MemElmSyndromeFragment3Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_3` writer"]
pub type W = crate::W<MemElmSyndromeFragment3Spec>;
#[doc = "Field `SYNDROME_3` reader - 31:0\\]
Syndrome bits 96 to 127"]
pub type Syndrome3R = crate::FieldReader<u32>;
#[doc = "Field `SYNDROME_3` writer - 31:0\\]
Syndrome bits 96 to 127"]
pub type Syndrome3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 96 to 127"]
    #[inline(always)]
    pub fn syndrome_3(&self) -> Syndrome3R {
        Syndrome3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 96 to 127"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_3(&mut self) -> Syndrome3W<MemElmSyndromeFragment3Spec> {
        Syndrome3W::new(self, 0)
    }
}
#[doc = "Input syndrome polynomial bits 96 to 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment3Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_3::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment3Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_3::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_3 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment3Spec {
    const RESET_VALUE: u32 = 0;
}
