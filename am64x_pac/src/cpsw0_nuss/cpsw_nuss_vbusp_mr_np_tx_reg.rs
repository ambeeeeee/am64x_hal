#[doc = "Register `CPSW_NUSS_VBUSP_MR_NP_TX_REG` reader"]
pub type R = crate::R<CpswNussVbuspMrNpTxRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_MR_NP_TX_REG` writer"]
pub type W = crate::W<CpswNussVbuspMrNpTxRegSpec>;
#[doc = "Field `MR_NP_TX` reader - 15:0\\]
Next page transmit"]
pub type MrNpTxR = crate::FieldReader<u16>;
#[doc = "Field `MR_NP_TX` writer - 15:0\\]
Next page transmit"]
pub type MrNpTxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Next page transmit"]
    #[inline(always)]
    pub fn mr_np_tx(&self) -> MrNpTxR {
        MrNpTxR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Next page transmit"]
    #[inline(always)]
    #[must_use]
    pub fn mr_np_tx(&mut self) -> MrNpTxW<CpswNussVbuspMrNpTxRegSpec> {
        MrNpTxW::new(self, 0)
    }
}
#[doc = "SGMII Next Pate Transmit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_mr_np_tx_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_mr_np_tx_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspMrNpTxRegSpec;
impl crate::RegisterSpec for CpswNussVbuspMrNpTxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_mr_np_tx_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspMrNpTxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_mr_np_tx_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspMrNpTxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_MR_NP_TX_REG to value 0"]
impl crate::Resettable for CpswNussVbuspMrNpTxRegSpec {
    const RESET_VALUE: u32 = 0;
}
