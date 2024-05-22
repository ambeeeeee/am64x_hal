#[doc = "Register `CPSW_NUSS_VBUSP_USER_INT_MASK_CLEAR_REG` reader"]
pub type R = crate::R<CpswNussVbuspUserIntMaskClearRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_USER_INT_MASK_CLEAR_REG` writer"]
pub type W = crate::W<CpswNussVbuspUserIntMaskClearRegSpec>;
#[doc = "Field `USERINTMASKCLR` reader - 1:0\\]
MDIO user interrupt mask clear"]
pub type UserintmaskclrR = crate::FieldReader;
#[doc = "Field `USERINTMASKCLR` writer - 1:0\\]
MDIO user interrupt mask clear"]
pub type UserintmaskclrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask clear"]
    #[inline(always)]
    pub fn userintmaskclr(&self) -> UserintmaskclrR {
        UserintmaskclrR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn userintmaskclr(&mut self) -> UserintmaskclrW<CpswNussVbuspUserIntMaskClearRegSpec> {
        UserintmaskclrW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Mask Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_mask_clear_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_mask_clear_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspUserIntMaskClearRegSpec;
impl crate::RegisterSpec for CpswNussVbuspUserIntMaskClearRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_user_int_mask_clear_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspUserIntMaskClearRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_user_int_mask_clear_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspUserIntMaskClearRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_USER_INT_MASK_CLEAR_REG to value 0"]
impl crate::Resettable for CpswNussVbuspUserIntMaskClearRegSpec {
    const RESET_VALUE: u32 = 0;
}
