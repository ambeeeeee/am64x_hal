#[doc = "Register `CPSW_NUSS_VBUSP_TX_PRI5_MAXLEN_REG` reader"]
pub type R = crate::R<CpswNussVbuspTxPri5MaxlenRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_TX_PRI5_MAXLEN_REG` writer"]
pub type W = crate::W<CpswNussVbuspTxPri5MaxlenRegSpec>;
#[doc = "Field `TX_PRI5_MAXLEN` reader - 13:0\\]
Transmit Priority 5 Maximum Length"]
pub type TxPri5MaxlenR = crate::FieldReader<u16>;
#[doc = "Field `TX_PRI5_MAXLEN` writer - 13:0\\]
Transmit Priority 5 Maximum Length"]
pub type TxPri5MaxlenW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 5 Maximum Length"]
    #[inline(always)]
    pub fn tx_pri5_maxlen(&self) -> TxPri5MaxlenR {
        TxPri5MaxlenR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 5 Maximum Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pri5_maxlen(&mut self) -> TxPri5MaxlenW<CpswNussVbuspTxPri5MaxlenRegSpec> {
        TxPri5MaxlenW::new(self, 0)
    }
}
#[doc = "Transmit Priority 5 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri5_maxlen_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri5_maxlen_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspTxPri5MaxlenRegSpec;
impl crate::RegisterSpec for CpswNussVbuspTxPri5MaxlenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_tx_pri5_maxlen_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspTxPri5MaxlenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_tx_pri5_maxlen_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspTxPri5MaxlenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_TX_PRI5_MAXLEN_REG to value 0x2024"]
impl crate::Resettable for CpswNussVbuspTxPri5MaxlenRegSpec {
    const RESET_VALUE: u32 = 0x2024;
}
