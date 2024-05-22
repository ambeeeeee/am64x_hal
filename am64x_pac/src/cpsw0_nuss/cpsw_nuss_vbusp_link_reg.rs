#[doc = "Register `CPSW_NUSS_VBUSP_LINK_REG` reader"]
pub type R = crate::R<CpswNussVbuspLinkRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_LINK_REG` writer"]
pub type W = crate::W<CpswNussVbuspLinkRegSpec>;
#[doc = "Field `LINK` reader - 31:0\\]
MDIO link state"]
pub type LinkR = crate::FieldReader<u32>;
#[doc = "Field `LINK` writer - 31:0\\]
MDIO link state"]
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO link state"]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO link state"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LinkW<CpswNussVbuspLinkRegSpec> {
        LinkW::new(self, 0)
    }
}
#[doc = "MDIO Link Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspLinkRegSpec;
impl crate::RegisterSpec for CpswNussVbuspLinkRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_link_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspLinkRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_link_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspLinkRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_LINK_REG to value 0"]
impl crate::Resettable for CpswNussVbuspLinkRegSpec {
    const RESET_VALUE: u32 = 0;
}
