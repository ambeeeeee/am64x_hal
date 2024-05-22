#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg1_2_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg1_2_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec>;
#[doc = "Field `TX_RATE_CIR_IDLE2` reader - 31:0\\]
TX Rate Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle including EOF. The PIR counter is disabled with a zero value and packets will not be marked RED"]
pub type TxRateCirIdle2R = crate::FieldReader<u32>;
#[doc = "Field `TX_RATE_CIR_IDLE2` writer - 31:0\\]
TX Rate Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle including EOF. The PIR counter is disabled with a zero value and packets will not be marked RED"]
pub type TxRateCirIdle2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TX Rate Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle including EOF. The PIR counter is disabled with a zero value and packets will not be marked RED"]
    #[inline(always)]
    pub fn tx_rate_cir_idle2(&self) -> TxRateCirIdle2R {
        TxRateCirIdle2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TX Rate Peak Information Rate Idle Increment Value - The number added to the PIR counter every clock cycle including EOF. The PIR counter is disabled with a zero value and packets will not be marked RED"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rate_cir_idle2(
        &mut self,
    ) -> TxRateCirIdle2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec> {
        TxRateCirIdle2W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg1_2_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg1_2_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg1_2_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg1_2_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_rate_cfg1_2_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_rate_cfg1_2_pru0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxRateCfg1_2Pru0Spec {
    const RESET_VALUE: u32 = 0;
}
