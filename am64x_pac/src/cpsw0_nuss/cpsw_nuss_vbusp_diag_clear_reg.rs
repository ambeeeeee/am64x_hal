#[doc = "Register `CPSW_NUSS_VBUSP_DIAG_CLEAR_REG` reader"]
pub type R = crate::R<CpswNussVbuspDiagClearRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_DIAG_CLEAR_REG` writer"]
pub type W = crate::W<CpswNussVbuspDiagClearRegSpec>;
#[doc = "Field `DIAG_CLEAR` reader - 0:0\\]
Diagnostics clear"]
pub type DiagClearR = crate::BitReader;
#[doc = "Field `DIAG_CLEAR` writer - 0:0\\]
Diagnostics clear"]
pub type DiagClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Diagnostics clear"]
    #[inline(always)]
    pub fn diag_clear(&self) -> DiagClearR {
        DiagClearR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Diagnostics clear"]
    #[inline(always)]
    #[must_use]
    pub fn diag_clear(&mut self) -> DiagClearW<CpswNussVbuspDiagClearRegSpec> {
        DiagClearW::new(self, 0)
    }
}
#[doc = "SGMII Diagnostics Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_diag_clear_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_diag_clear_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspDiagClearRegSpec;
impl crate::RegisterSpec for CpswNussVbuspDiagClearRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_diag_clear_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspDiagClearRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_diag_clear_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspDiagClearRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_DIAG_CLEAR_REG to value 0"]
impl crate::Resettable for CpswNussVbuspDiagClearRegSpec {
    const RESET_VALUE: u32 = 0;
}
