#[doc = "Register `CPSW_NUSS_VBUSP_USER_INT_MASKED_REG` reader"]
pub type R = crate::R<CpswNussVbuspUserIntMaskedRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_USER_INT_MASKED_REG` writer"]
pub type W = crate::W<CpswNussVbuspUserIntMaskedRegSpec>;
#[doc = "Field `USERINTMASKED` reader - 1:0\\]
User interrupt masked"]
pub type UserintmaskedR = crate::FieldReader;
#[doc = "Field `USERINTMASKED` writer - 1:0\\]
User interrupt masked"]
pub type UserintmaskedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt masked"]
    #[inline(always)]
    pub fn userintmasked(&self) -> UserintmaskedR {
        UserintmaskedR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt masked"]
    #[inline(always)]
    #[must_use]
    pub fn userintmasked(&mut self) -> UserintmaskedW<CpswNussVbuspUserIntMaskedRegSpec> {
        UserintmaskedW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_masked_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_masked_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspUserIntMaskedRegSpec;
impl crate::RegisterSpec for CpswNussVbuspUserIntMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_user_int_masked_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspUserIntMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_user_int_masked_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspUserIntMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_USER_INT_MASKED_REG to value 0"]
impl crate::Resettable for CpswNussVbuspUserIntMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
