#[doc = "Register `CPSW_NUSS_VBUSP_DIAG_CONTROL_REG` reader"]
pub type R = crate::R<CpswNussVbuspDiagControlRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_DIAG_CONTROL_REG` writer"]
pub type W = crate::W<CpswNussVbuspDiagControlRegSpec>;
#[doc = "Field `DIAG_EDGE_SEL` reader - 1:0\\]
Diagnostics hold signals edge select"]
pub type DiagEdgeSelR = crate::FieldReader;
#[doc = "Field `DIAG_EDGE_SEL` writer - 1:0\\]
Diagnostics hold signals edge select"]
pub type DiagEdgeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIAG_SM_SEL` reader - 6:4\\]
Diagnostic select"]
pub type DiagSmSelR = crate::FieldReader;
#[doc = "Field `DIAG_SM_SEL` writer - 6:4\\]
Diagnostic select"]
pub type DiagSmSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Diagnostics hold signals edge select"]
    #[inline(always)]
    pub fn diag_edge_sel(&self) -> DiagEdgeSelR {
        DiagEdgeSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Diagnostic select"]
    #[inline(always)]
    pub fn diag_sm_sel(&self) -> DiagSmSelR {
        DiagSmSelR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Diagnostics hold signals edge select"]
    #[inline(always)]
    #[must_use]
    pub fn diag_edge_sel(&mut self) -> DiagEdgeSelW<CpswNussVbuspDiagControlRegSpec> {
        DiagEdgeSelW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Diagnostic select"]
    #[inline(always)]
    #[must_use]
    pub fn diag_sm_sel(&mut self) -> DiagSmSelW<CpswNussVbuspDiagControlRegSpec> {
        DiagSmSelW::new(self, 4)
    }
}
#[doc = "SGMII Diagnostics Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_diag_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_diag_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspDiagControlRegSpec;
impl crate::RegisterSpec for CpswNussVbuspDiagControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_diag_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspDiagControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_diag_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspDiagControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_DIAG_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNussVbuspDiagControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
