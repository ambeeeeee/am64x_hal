#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_smd_frag_err_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_smd_frag_err_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec>;
#[doc = "Field `RX_STAT_SMDS_ERR_PRU0` reader - 7:0\\]
RX SMDSx Seq Error Count, Wrt subtracts"]
pub type RxStatSmdsErrPru0R = crate::FieldReader;
#[doc = "Field `RX_STAT_SMDS_ERR_PRU0` writer - 7:0\\]
RX SMDSx Seq Error Count, Wrt subtracts"]
pub type RxStatSmdsErrPru0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_STAT_SMDC_ERR_PRU0` reader - 15:8\\]
RX SMDCx Seq Error Count, Wrt subtracts"]
pub type RxStatSmdcErrPru0R = crate::FieldReader;
#[doc = "Field `RX_STAT_SMDC_ERR_PRU0` writer - 15:8\\]
RX SMDCx Seq Error Count, Wrt subtracts"]
pub type RxStatSmdcErrPru0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_STAT_FRAG_ERR_PRU0` reader - 23:16\\]
RX Frag_Cnt Seq Error Count, Wrt subtracts"]
pub type RxStatFragErrPru0R = crate::FieldReader;
#[doc = "Field `RX_STAT_FRAG_ERR_PRU0` writer - 23:16\\]
RX Frag_Cnt Seq Error Count, Wrt subtracts"]
pub type RxStatFragErrPru0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_STAT_SMD_ERR_PRU0` reader - 31:24\\]
RX SMDS Error Count, Inc when first none 0x55 does not match any valid SMD, Wrt subtracts"]
pub type RxStatSmdErrPru0R = crate::FieldReader;
#[doc = "Field `RX_STAT_SMD_ERR_PRU0` writer - 31:24\\]
RX SMDS Error Count, Inc when first none 0x55 does not match any valid SMD, Wrt subtracts"]
pub type RxStatSmdErrPru0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
RX SMDSx Seq Error Count, Wrt subtracts"]
    #[inline(always)]
    pub fn rx_stat_smds_err_pru0(&self) -> RxStatSmdsErrPru0R {
        RxStatSmdsErrPru0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RX SMDCx Seq Error Count, Wrt subtracts"]
    #[inline(always)]
    pub fn rx_stat_smdc_err_pru0(&self) -> RxStatSmdcErrPru0R {
        RxStatSmdcErrPru0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
RX Frag_Cnt Seq Error Count, Wrt subtracts"]
    #[inline(always)]
    pub fn rx_stat_frag_err_pru0(&self) -> RxStatFragErrPru0R {
        RxStatFragErrPru0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RX SMDS Error Count, Inc when first none 0x55 does not match any valid SMD, Wrt subtracts"]
    #[inline(always)]
    pub fn rx_stat_smd_err_pru0(&self) -> RxStatSmdErrPru0R {
        RxStatSmdErrPru0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
RX SMDSx Seq Error Count, Wrt subtracts"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stat_smds_err_pru0(
        &mut self,
    ) -> RxStatSmdsErrPru0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec> {
        RxStatSmdsErrPru0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RX SMDCx Seq Error Count, Wrt subtracts"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stat_smdc_err_pru0(
        &mut self,
    ) -> RxStatSmdcErrPru0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec> {
        RxStatSmdcErrPru0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
RX Frag_Cnt Seq Error Count, Wrt subtracts"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stat_frag_err_pru0(
        &mut self,
    ) -> RxStatFragErrPru0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec> {
        RxStatFragErrPru0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RX SMDS Error Count, Inc when first none 0x55 does not match any valid SMD, Wrt subtracts"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stat_smd_err_pru0(
        &mut self,
    ) -> RxStatSmdErrPru0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec> {
        RxStatSmdErrPru0W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_smd_frag_err_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_smd_frag_err_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_smd_frag_err_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_smd_frag_err_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_smd_frag_err_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_smd_frag_err_pru0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatSmdFragErrPru0Spec {
    const RESET_VALUE: u32 = 0;
}
