#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_4` reader"]
pub type R = crate::R<MemElmSyndromeFragment4Spec>;
#[doc = "Register `MEM_ELM_SYNDROME_FRAGMENT_4` writer"]
pub type W = crate::W<MemElmSyndromeFragment4Spec>;
#[doc = "Field `SYNDROME_4` reader - 31:0\\]
Syndrome bits 128 to 159"]
pub type Syndrome4R = crate::FieldReader<u32>;
#[doc = "Field `SYNDROME_4` writer - 31:0\\]
Syndrome bits 128 to 159"]
pub type Syndrome4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 128 to 159"]
    #[inline(always)]
    pub fn syndrome_4(&self) -> Syndrome4R {
        Syndrome4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Syndrome bits 128 to 159"]
    #[inline(always)]
    #[must_use]
    pub fn syndrome_4(&mut self) -> Syndrome4W<MemElmSyndromeFragment4Spec> {
        Syndrome4W::new(self, 0)
    }
}
#[doc = "Input syndrome polynomial bits 128 to 159.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmSyndromeFragment4Spec;
impl crate::RegisterSpec for MemElmSyndromeFragment4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_syndrome_fragment_4::R`](R) reader structure"]
impl crate::Readable for MemElmSyndromeFragment4Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_syndrome_fragment_4::W`](W) writer structure"]
impl crate::Writable for MemElmSyndromeFragment4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_SYNDROME_FRAGMENT_4 to value 0"]
impl crate::Resettable for MemElmSyndromeFragment4Spec {
    const RESET_VALUE: u32 = 0;
}
