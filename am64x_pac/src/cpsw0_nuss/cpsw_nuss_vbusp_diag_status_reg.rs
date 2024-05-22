#[doc = "Register `CPSW_NUSS_VBUSP_DIAG_STATUS_REG` reader"]
pub type R = crate::R<CpswNussVbuspDiagStatusRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_DIAG_STATUS_REG` writer"]
pub type W = crate::W<CpswNussVbuspDiagStatusRegSpec>;
#[doc = "Field `DIAG_STATUS` reader - 15:0\\]
Diagnostics status"]
pub type DiagStatusR = crate::FieldReader<u16>;
#[doc = "Field `DIAG_STATUS` writer - 15:0\\]
Diagnostics status"]
pub type DiagStatusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Diagnostics status"]
    #[inline(always)]
    pub fn diag_status(&self) -> DiagStatusR {
        DiagStatusR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Diagnostics status"]
    #[inline(always)]
    #[must_use]
    pub fn diag_status(&mut self) -> DiagStatusW<CpswNussVbuspDiagStatusRegSpec> {
        DiagStatusW::new(self, 0)
    }
}
#[doc = "SGMII Diagnostics Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_diag_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_diag_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspDiagStatusRegSpec;
impl crate::RegisterSpec for CpswNussVbuspDiagStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_diag_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspDiagStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_diag_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspDiagStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_DIAG_STATUS_REG to value 0"]
impl crate::Resettable for CpswNussVbuspDiagStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
