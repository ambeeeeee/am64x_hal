#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_good_port1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_good_port1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec>;
#[doc = "Field `TX_GOOD_FRM_CNT` reader - 31:0\\]
TX Good Frame Count Inc if no min size err max size err or mii odd nibble"]
pub type TxGoodFrmCntR = crate::FieldReader<u32>;
#[doc = "Field `TX_GOOD_FRM_CNT` writer - 31:0\\]
TX Good Frame Count Inc if no min size err max size err or mii odd nibble"]
pub type TxGoodFrmCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TX Good Frame Count Inc if no min size err max size err or mii odd nibble"]
    #[inline(always)]
    pub fn tx_good_frm_cnt(&self) -> TxGoodFrmCntR {
        TxGoodFrmCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TX Good Frame Count Inc if no min size err max size err or mii odd nibble"]
    #[inline(always)]
    #[must_use]
    pub fn tx_good_frm_cnt(
        &mut self,
    ) -> TxGoodFrmCntW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec> {
        TxGoodFrmCntW::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_good_port1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_good_port1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_good_port1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_good_port1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_good_port1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_good_port1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatGoodPort1Spec {
    const RESET_VALUE: u32 = 0;
}
