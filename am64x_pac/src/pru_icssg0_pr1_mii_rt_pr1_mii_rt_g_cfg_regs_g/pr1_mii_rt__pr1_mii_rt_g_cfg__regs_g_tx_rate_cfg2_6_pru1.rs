#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg2_6_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg2_6_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec>;
#[doc = "Field `TX_RATE_LEN6` reader - 15:0\\]
TX Rate Pkt Length"]
pub type TxRateLen6R = crate::FieldReader<u16>;
#[doc = "Field `TX_RATE_LEN6` writer - 15:0\\]
TX Rate Pkt Length"]
pub type TxRateLen6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX_RATE_EN6` reader - 16:16\\]
TX Rate Pkt Enable"]
pub type TxRateEn6R = crate::BitReader;
#[doc = "Field `TX_RATE_EN6` writer - 16:16\\]
TX Rate Pkt Enable"]
pub type TxRateEn6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RATE_ALLOW6` reader - 17:17\\]
TX Rate Pkt Enable"]
pub type TxRateAllow6R = crate::BitReader;
#[doc = "Field `TX_RATE_ALLOW6` writer - 17:17\\]
TX Rate Pkt Enable"]
pub type TxRateAllow6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TX Rate Pkt Length"]
    #[inline(always)]
    pub fn tx_rate_len6(&self) -> TxRateLen6R {
        TxRateLen6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
TX Rate Pkt Enable"]
    #[inline(always)]
    pub fn tx_rate_en6(&self) -> TxRateEn6R {
        TxRateEn6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
TX Rate Pkt Enable"]
    #[inline(always)]
    pub fn tx_rate_allow6(&self) -> TxRateAllow6R {
        TxRateAllow6R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TX Rate Pkt Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rate_len6(&mut self) -> TxRateLen6W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec> {
        TxRateLen6W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
TX Rate Pkt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rate_en6(&mut self) -> TxRateEn6W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec> {
        TxRateEn6W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
TX Rate Pkt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rate_allow6(
        &mut self,
    ) -> TxRateAllow6W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec> {
        TxRateAllow6W::new(self, 17)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg2_6_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg2_6_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg2_6_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg2_6_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg2_6_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg2_6_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg2_6Pru1Spec {
    const RESET_VALUE: u32 = 0;
}
