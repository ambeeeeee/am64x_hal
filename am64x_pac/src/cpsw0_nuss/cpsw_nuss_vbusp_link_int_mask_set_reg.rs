#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_MASK_SET_REG` reader"]
pub type R = crate::R<CpswNussVbuspLinkIntMaskSetRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_MASK_SET_REG` writer"]
pub type W = crate::W<CpswNussVbuspLinkIntMaskSetRegSpec>;
#[doc = "Field `LINKINTMASKSET` reader - 0:0\\]
MDIO link interrupt mask set"]
pub type LinkintmasksetR = crate::BitReader;
#[doc = "Field `LINKINTMASKSET` writer - 0:0\\]
MDIO link interrupt mask set"]
pub type LinkintmasksetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MDIO link interrupt mask set"]
    #[inline(always)]
    pub fn linkintmaskset(&self) -> LinkintmasksetR {
        LinkintmasksetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MDIO link interrupt mask set"]
    #[inline(always)]
    #[must_use]
    pub fn linkintmaskset(&mut self) -> LinkintmasksetW<CpswNussVbuspLinkIntMaskSetRegSpec> {
        LinkintmasksetW::new(self, 0)
    }
}
#[doc = "MDIO Link Interrupt Mask Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_mask_set_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_mask_set_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspLinkIntMaskSetRegSpec;
impl crate::RegisterSpec for CpswNussVbuspLinkIntMaskSetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_link_int_mask_set_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspLinkIntMaskSetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_link_int_mask_set_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspLinkIntMaskSetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_LINK_INT_MASK_SET_REG to value 0"]
impl crate::Resettable for CpswNussVbuspLinkIntMaskSetRegSpec {
    const RESET_VALUE: u32 = 0;
}
