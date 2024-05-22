#[doc = "Register `CPSW_NUSS_VBUSP_TX_PRI0_MAXLEN_REG` reader"]
pub type R = crate::R<CpswNussVbuspTxPri0MaxlenRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_TX_PRI0_MAXLEN_REG` writer"]
pub type W = crate::W<CpswNussVbuspTxPri0MaxlenRegSpec>;
#[doc = "Field `TX_PRI0_MAXLEN` reader - 13:0\\]
Transmit Priority 0 Maximum Length"]
pub type TxPri0MaxlenR = crate::FieldReader<u16>;
#[doc = "Field `TX_PRI0_MAXLEN` writer - 13:0\\]
Transmit Priority 0 Maximum Length"]
pub type TxPri0MaxlenW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 0 Maximum Length"]
    #[inline(always)]
    pub fn tx_pri0_maxlen(&self) -> TxPri0MaxlenR {
        TxPri0MaxlenR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Transmit Priority 0 Maximum Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pri0_maxlen(&mut self) -> TxPri0MaxlenW<CpswNussVbuspTxPri0MaxlenRegSpec> {
        TxPri0MaxlenW::new(self, 0)
    }
}
#[doc = "Transmit Priority 0 Maximum Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_pri0_maxlen_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_pri0_maxlen_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspTxPri0MaxlenRegSpec;
impl crate::RegisterSpec for CpswNussVbuspTxPri0MaxlenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_tx_pri0_maxlen_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspTxPri0MaxlenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_tx_pri0_maxlen_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspTxPri0MaxlenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_TX_PRI0_MAXLEN_REG to value 0x2024"]
impl crate::Resettable for CpswNussVbuspTxPri0MaxlenRegSpec {
    const RESET_VALUE: u32 = 0x2024;
}
