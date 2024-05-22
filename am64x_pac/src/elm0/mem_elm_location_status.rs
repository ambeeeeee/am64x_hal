#[doc = "Register `MEM_ELM_LOCATION_STATUS` reader"]
pub type R = crate::R<MemElmLocationStatusSpec>;
#[doc = "Register `MEM_ELM_LOCATION_STATUS` writer"]
pub type W = crate::W<MemElmLocationStatusSpec>;
#[doc = "Field `ECC_NB_ERRORS` reader - 4:0\\]
Number of errors detected and located"]
pub type EccNbErrorsR = crate::FieldReader;
#[doc = "Field `ECC_NB_ERRORS` writer - 4:0\\]
Number of errors detected and located"]
pub type EccNbErrorsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ECC_CORRECTABLE` reader - 8:8\\]
Error location process exit status 0x0: ECC error location process failed Number of errors and error locations are invalid 0x1: all errors were successfully located Number of errors and error locations are valid"]
pub type EccCorrectableR = crate::BitReader;
#[doc = "Field `ECC_CORRECTABLE` writer - 8:8\\]
Error location process exit status 0x0: ECC error location process failed Number of errors and error locations are invalid 0x1: all errors were successfully located Number of errors and error locations are valid"]
pub type EccCorrectableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Number of errors detected and located"]
    #[inline(always)]
    pub fn ecc_nb_errors(&self) -> EccNbErrorsR {
        EccNbErrorsR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Error location process exit status 0x0: ECC error location process failed Number of errors and error locations are invalid 0x1: all errors were successfully located Number of errors and error locations are valid"]
    #[inline(always)]
    pub fn ecc_correctable(&self) -> EccCorrectableR {
        EccCorrectableR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Number of errors detected and located"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_nb_errors(&mut self) -> EccNbErrorsW<MemElmLocationStatusSpec> {
        EccNbErrorsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Error location process exit status 0x0: ECC error location process failed Number of errors and error locations are invalid 0x1: all errors were successfully located Number of errors and error locations are valid"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_correctable(&mut self) -> EccCorrectableW<MemElmLocationStatusSpec> {
        EccCorrectableW::new(self, 8)
    }
}
#[doc = "Exit status for the syndrome polynomial processing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_location_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_location_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmLocationStatusSpec;
impl crate::RegisterSpec for MemElmLocationStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_location_status::R`](R) reader structure"]
impl crate::Readable for MemElmLocationStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_location_status::W`](W) writer structure"]
impl crate::Writable for MemElmLocationStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_LOCATION_STATUS to value 0"]
impl crate::Resettable for MemElmLocationStatusSpec {
    const RESET_VALUE: u32 = 0;
}
