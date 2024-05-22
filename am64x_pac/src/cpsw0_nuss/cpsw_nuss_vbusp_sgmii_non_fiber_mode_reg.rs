#[doc = "Register `CPSW_NUSS_VBUSP_SGMII_NON_FIBER_MODE_REG` reader"]
pub type R = crate::R<CpswNussVbuspSgmiiNonFiberModeRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SGMII_NON_FIBER_MODE_REG` writer"]
pub type W = crate::W<CpswNussVbuspSgmiiNonFiberModeRegSpec>;
#[doc = "Field `SGMII_NON_FIBER_MODE` reader - 1:0\\]
This register bit goes to the CPSGMII mode input only"]
pub type SgmiiNonFiberModeR = crate::FieldReader;
#[doc = "Field `SGMII_NON_FIBER_MODE` writer - 1:0\\]
This register bit goes to the CPSGMII mode input only"]
pub type SgmiiNonFiberModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
This register bit goes to the CPSGMII mode input only"]
    #[inline(always)]
    pub fn sgmii_non_fiber_mode(&self) -> SgmiiNonFiberModeR {
        SgmiiNonFiberModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
This register bit goes to the CPSGMII mode input only"]
    #[inline(always)]
    #[must_use]
    pub fn sgmii_non_fiber_mode(
        &mut self,
    ) -> SgmiiNonFiberModeW<CpswNussVbuspSgmiiNonFiberModeRegSpec> {
        SgmiiNonFiberModeW::new(self, 0)
    }
}
#[doc = "SGMII NON FIBER Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSgmiiNonFiberModeRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSgmiiNonFiberModeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSgmiiNonFiberModeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_sgmii_non_fiber_mode_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSgmiiNonFiberModeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SGMII_NON_FIBER_MODE_REG to value 0x03"]
impl crate::Resettable for CpswNussVbuspSgmiiNonFiberModeRegSpec {
    const RESET_VALUE: u32 = 0x03;
}
