#[doc = "Register `CPSW_NUSS_VBUSP_SERDES_RESET_ISO_REG` reader"]
pub type R = crate::R<CpswNussVbuspSerdesResetIsoRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SERDES_RESET_ISO_REG` writer"]
pub type W = crate::W<CpswNussVbuspSerdesResetIsoRegSpec>;
#[doc = "Field `SERDES_RESET_ISO` reader - 1:0\\]
These bits control whether the SERDES ignores the hard reset for isolation or not"]
pub type SerdesResetIsoR = crate::FieldReader;
#[doc = "Field `SERDES_RESET_ISO` writer - 1:0\\]
These bits control whether the SERDES ignores the hard reset for isolation or not"]
pub type SerdesResetIsoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
These bits control whether the SERDES ignores the hard reset for isolation or not"]
    #[inline(always)]
    pub fn serdes_reset_iso(&self) -> SerdesResetIsoR {
        SerdesResetIsoR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
These bits control whether the SERDES ignores the hard reset for isolation or not"]
    #[inline(always)]
    #[must_use]
    pub fn serdes_reset_iso(&mut self) -> SerdesResetIsoW<CpswNussVbuspSerdesResetIsoRegSpec> {
        SerdesResetIsoW::new(self, 0)
    }
}
#[doc = "SyncE Mux Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_serdes_reset_iso_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_serdes_reset_iso_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSerdesResetIsoRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSerdesResetIsoRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_serdes_reset_iso_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSerdesResetIsoRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_serdes_reset_iso_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSerdesResetIsoRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SERDES_RESET_ISO_REG to value 0"]
impl crate::Resettable for CpswNussVbuspSerdesResetIsoRegSpec {
    const RESET_VALUE: u32 = 0;
}
