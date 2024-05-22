#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_crc_err_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_crc_err_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec>;
#[doc = "Field `RX_CRC_ERR_FRM_CNT` reader - 15:0\\]
RX CRC Err Frame Count Inc on crc err, Wrt subtracts"]
pub type RxCrcErrFrmCntR = crate::FieldReader<u16>;
#[doc = "Field `RX_CRC_ERR_FRM_CNT` writer - 15:0\\]
RX CRC Err Frame Count Inc on crc err, Wrt subtracts"]
pub type RxCrcErrFrmCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
RX CRC Err Frame Count Inc on crc err, Wrt subtracts"]
    #[inline(always)]
    pub fn rx_crc_err_frm_cnt(&self) -> RxCrcErrFrmCntR {
        RxCrcErrFrmCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
RX CRC Err Frame Count Inc on crc err, Wrt subtracts"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_err_frm_cnt(
        &mut self,
    ) -> RxCrcErrFrmCntW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec> {
        RxCrcErrFrmCntW::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_crc_err_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_crc_err_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_crc_err_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_crc_err_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_crc_err_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_crc_err_pru0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatCrcErrPru0Spec {
    const RESET_VALUE: u32 = 0;
}
