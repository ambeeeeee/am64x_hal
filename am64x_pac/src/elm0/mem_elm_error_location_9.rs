#[doc = "Register `MEM_ELM_ERROR_LOCATION_9` reader"]
pub type R = crate::R<MemElmErrorLocation9Spec>;
#[doc = "Register `MEM_ELM_ERROR_LOCATION_9` writer"]
pub type W = crate::W<MemElmErrorLocation9Spec>;
#[doc = "Field `ECC_ERROR_LOCATION` reader - 12:0\\]
Error location bit address"]
pub type EccErrorLocationR = crate::FieldReader<u16>;
#[doc = "Field `ECC_ERROR_LOCATION` writer - 12:0\\]
Error location bit address"]
pub type EccErrorLocationW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Error location bit address"]
    #[inline(always)]
    pub fn ecc_error_location(&self) -> EccErrorLocationR {
        EccErrorLocationR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Error location bit address"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_error_location(&mut self) -> EccErrorLocationW<MemElmErrorLocation9Spec> {
        EccErrorLocationW::new(self, 0)
    }
}
#[doc = "Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmErrorLocation9Spec;
impl crate::RegisterSpec for MemElmErrorLocation9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_error_location_9::R`](R) reader structure"]
impl crate::Readable for MemElmErrorLocation9Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_error_location_9::W`](W) writer structure"]
impl crate::Writable for MemElmErrorLocation9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_ERROR_LOCATION_9 to value 0"]
impl crate::Resettable for MemElmErrorLocation9Spec {
    const RESET_VALUE: u32 = 0;
}
