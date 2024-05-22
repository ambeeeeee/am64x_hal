#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_MASKED_REG` reader"]
pub type R = crate::R<CpswNussVbuspLinkIntMaskedRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_MASKED_REG` writer"]
pub type W = crate::W<CpswNussVbuspLinkIntMaskedRegSpec>;
#[doc = "Field `LINKINTMASKED` reader - 1:0\\]
MDIO link change interrupt masked value"]
pub type LinkintmaskedR = crate::FieldReader;
#[doc = "Field `LINKINTMASKED` writer - 1:0\\]
MDIO link change interrupt masked value"]
pub type LinkintmaskedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO link change interrupt masked value"]
    #[inline(always)]
    pub fn linkintmasked(&self) -> LinkintmaskedR {
        LinkintmaskedR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO link change interrupt masked value"]
    #[inline(always)]
    #[must_use]
    pub fn linkintmasked(&mut self) -> LinkintmaskedW<CpswNussVbuspLinkIntMaskedRegSpec> {
        LinkintmaskedW::new(self, 0)
    }
}
#[doc = "MDIO Link Interrupt Masked Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_masked_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_masked_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspLinkIntMaskedRegSpec;
impl crate::RegisterSpec for CpswNussVbuspLinkIntMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_link_int_masked_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspLinkIntMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_link_int_masked_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspLinkIntMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_LINK_INT_MASKED_REG to value 0"]
impl crate::Resettable for CpswNussVbuspLinkIntMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
