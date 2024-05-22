#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_MASK_CLEAR_REG` reader"]
pub type R = crate::R<CpswNussVbuspLinkIntMaskClearRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_MASK_CLEAR_REG` writer"]
pub type W = crate::W<CpswNussVbuspLinkIntMaskClearRegSpec>;
#[doc = "Field `LINKINTMASKCLR` reader - 0:0\\]
MDIO link interrupt mask clear"]
pub type LinkintmaskclrR = crate::BitReader;
#[doc = "Field `LINKINTMASKCLR` writer - 0:0\\]
MDIO link interrupt mask clear"]
pub type LinkintmaskclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MDIO link interrupt mask clear"]
    #[inline(always)]
    pub fn linkintmaskclr(&self) -> LinkintmaskclrR {
        LinkintmaskclrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MDIO link interrupt mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn linkintmaskclr(&mut self) -> LinkintmaskclrW<CpswNussVbuspLinkIntMaskClearRegSpec> {
        LinkintmaskclrW::new(self, 0)
    }
}
#[doc = "MDIO Link Interrupt Mask Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_mask_clear_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_mask_clear_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspLinkIntMaskClearRegSpec;
impl crate::RegisterSpec for CpswNussVbuspLinkIntMaskClearRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_link_int_mask_clear_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspLinkIntMaskClearRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_link_int_mask_clear_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspLinkIntMaskClearRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_LINK_INT_MASK_CLEAR_REG to value 0"]
impl crate::Resettable for CpswNussVbuspLinkIntMaskClearRegSpec {
    const RESET_VALUE: u32 = 0;
}
