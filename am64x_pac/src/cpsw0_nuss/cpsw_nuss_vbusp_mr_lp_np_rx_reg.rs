#[doc = "Register `CPSW_NUSS_VBUSP_MR_LP_NP_RX_REG` reader"]
pub type R = crate::R<CpswNussVbuspMrLpNpRxRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_MR_LP_NP_RX_REG` writer"]
pub type W = crate::W<CpswNussVbuspMrLpNpRxRegSpec>;
#[doc = "Field `MR_LP_NP_RX` reader - 15:0\\]
Link Partner Next Page Received"]
pub type MrLpNpRxR = crate::FieldReader<u16>;
#[doc = "Field `MR_LP_NP_RX` writer - 15:0\\]
Link Partner Next Page Received"]
pub type MrLpNpRxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Link Partner Next Page Received"]
    #[inline(always)]
    pub fn mr_lp_np_rx(&self) -> MrLpNpRxR {
        MrLpNpRxR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Link Partner Next Page Received"]
    #[inline(always)]
    #[must_use]
    pub fn mr_lp_np_rx(&mut self) -> MrLpNpRxW<CpswNussVbuspMrLpNpRxRegSpec> {
        MrLpNpRxW::new(self, 0)
    }
}
#[doc = "SGMII Link Partner Next Page Receive Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_lp_np_rx_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_lp_np_rx_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspMrLpNpRxRegSpec;
impl crate::RegisterSpec for CpswNussVbuspMrLpNpRxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_mr_lp_np_rx_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspMrLpNpRxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_mr_lp_np_rx_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspMrLpNpRxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_MR_LP_NP_RX_REG to value 0"]
impl crate::Resettable for CpswNussVbuspMrLpNpRxRegSpec {
    const RESET_VALUE: u32 = 0;
}
