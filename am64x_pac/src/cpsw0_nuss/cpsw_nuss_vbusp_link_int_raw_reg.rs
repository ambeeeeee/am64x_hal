#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_RAW_REG` reader"]
pub type R = crate::R<CpswNussVbuspLinkIntRawRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_LINK_INT_RAW_REG` writer"]
pub type W = crate::W<CpswNussVbuspLinkIntRawRegSpec>;
#[doc = "Field `LINKINTRAW` reader - 1:0\\]
MDIO link change event raw value"]
pub type LinkintrawR = crate::FieldReader;
#[doc = "Field `LINKINTRAW` writer - 1:0\\]
MDIO link change event raw value"]
pub type LinkintrawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO link change event raw value"]
    #[inline(always)]
    pub fn linkintraw(&self) -> LinkintrawR {
        LinkintrawR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MDIO link change event raw value"]
    #[inline(always)]
    #[must_use]
    pub fn linkintraw(&mut self) -> LinkintrawW<CpswNussVbuspLinkIntRawRegSpec> {
        LinkintrawW::new(self, 0)
    }
}
#[doc = "MDIO Link Interrupt Raw Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_link_int_raw_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_link_int_raw_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspLinkIntRawRegSpec;
impl crate::RegisterSpec for CpswNussVbuspLinkIntRawRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_link_int_raw_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspLinkIntRawRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_link_int_raw_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspLinkIntRawRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_LINK_INT_RAW_REG to value 0"]
impl crate::Resettable for CpswNussVbuspLinkIntRawRegSpec {
    const RESET_VALUE: u32 = 0;
}
