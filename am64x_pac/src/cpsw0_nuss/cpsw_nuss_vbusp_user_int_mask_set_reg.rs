#[doc = "Register `CPSW_NUSS_VBUSP_USER_INT_MASK_SET_REG` reader"]
pub type R = crate::R<CpswNussVbuspUserIntMaskSetRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_USER_INT_MASK_SET_REG` writer"]
pub type W = crate::W<CpswNussVbuspUserIntMaskSetRegSpec>;
#[doc = "Field `USERINTMASKSET` reader - 1:0\\]
MDIO user interrupt mask set"]
pub type UserintmasksetR = crate::FieldReader;
#[doc = "Field `USERINTMASKSET` writer - 1:0\\]
MDIO user interrupt mask set"]
pub type UserintmasksetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask set"]
    #[inline(always)]
    pub fn userintmaskset(&self) -> UserintmasksetR {
        UserintmasksetR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO user interrupt mask set"]
    #[inline(always)]
    #[must_use]
    pub fn userintmaskset(&mut self) -> UserintmasksetW<CpswNussVbuspUserIntMaskSetRegSpec> {
        UserintmasksetW::new(self, 0)
    }
}
#[doc = "MDIO User Interrupt Mask Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_user_int_mask_set_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_user_int_mask_set_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspUserIntMaskSetRegSpec;
impl crate::RegisterSpec for CpswNussVbuspUserIntMaskSetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_user_int_mask_set_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspUserIntMaskSetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_user_int_mask_set_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspUserIntMaskSetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_USER_INT_MASK_SET_REG to value 0"]
impl crate::Resettable for CpswNussVbuspUserIntMaskSetRegSpec {
    const RESET_VALUE: u32 = 0;
}
