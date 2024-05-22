#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_max_err_port0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_max_err_port0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec>;
#[doc = "Field `TX_MAX_ERR_FRM_CNT` reader - 15:0\\]
TX MAX Size Err Frame Count Inc if > max Limit"]
pub type TxMaxErrFrmCntR = crate::FieldReader<u16>;
#[doc = "Field `TX_MAX_ERR_FRM_CNT` writer - 15:0\\]
TX MAX Size Err Frame Count Inc if > max Limit"]
pub type TxMaxErrFrmCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TX MAX Size Err Frame Count Inc if > max Limit"]
    #[inline(always)]
    pub fn tx_max_err_frm_cnt(&self) -> TxMaxErrFrmCntR {
        TxMaxErrFrmCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TX MAX Size Err Frame Count Inc if > max Limit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_max_err_frm_cnt(
        &mut self,
    ) -> TxMaxErrFrmCntW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec> {
        TxMaxErrFrmCntW::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_max_err_port0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_max_err_port0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_max_err_port0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_max_err_port0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_tx_stat_max_err_port0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_tx_stat_max_err_port0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGTxStatMaxErrPort0Spec {
    const RESET_VALUE: u32 = 0;
}
