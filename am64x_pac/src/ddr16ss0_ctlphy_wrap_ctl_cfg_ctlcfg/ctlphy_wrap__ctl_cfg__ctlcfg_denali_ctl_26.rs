#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_26` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_26` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec>;
#[doc = "Field `TDLL_F2` reader - 15:0\\]
DRAM TDLL value in cycles. FC=2"]
pub type TdllF2R = crate::FieldReader<u16>;
#[doc = "Field `TDLL_F2` writer - 15:0\\]
DRAM TDLL value in cycles. FC=2"]
pub type TdllF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LPC_SW_ENTER_DQS_OSC_IN_PROGRESS_ERR_STATUS` reader - 16:16\\]
Error response for Software issued Low power command while DQS Oscillator is in progress. READ-ONLY"]
pub type LpcSwEnterDqsOscInProgressErrStatusR = crate::BitReader;
#[doc = "Field `LPC_SW_ENTER_DQS_OSC_IN_PROGRESS_ERR_STATUS` writer - 16:16\\]
Error response for Software issued Low power command while DQS Oscillator is in progress. READ-ONLY"]
pub type LpcSwEnterDqsOscInProgressErrStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS_OSC_PER_CS_OOV_TRAINING_STATUS` reader - 25:24\\]
Set the CS information for which DQS oscillator is having out of variance value. READ-ONLY"]
pub type DqsOscPerCsOovTrainingStatusR = crate::FieldReader;
#[doc = "Field `DQS_OSC_PER_CS_OOV_TRAINING_STATUS` writer - 25:24\\]
Set the CS information for which DQS oscillator is having out of variance value. READ-ONLY"]
pub type DqsOscPerCsOovTrainingStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TDLL value in cycles. FC=2"]
    #[inline(always)]
    pub fn tdll_f2(&self) -> TdllF2R {
        TdllF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Error response for Software issued Low power command while DQS Oscillator is in progress. READ-ONLY"]
    #[inline(always)]
    pub fn lpc_sw_enter_dqs_osc_in_progress_err_status(
        &self,
    ) -> LpcSwEnterDqsOscInProgressErrStatusR {
        LpcSwEnterDqsOscInProgressErrStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Set the CS information for which DQS oscillator is having out of variance value. READ-ONLY"]
    #[inline(always)]
    pub fn dqs_osc_per_cs_oov_training_status(&self) -> DqsOscPerCsOovTrainingStatusR {
        DqsOscPerCsOovTrainingStatusR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TDLL value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdll_f2(&mut self) -> TdllF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec> {
        TdllF2W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Error response for Software issued Low power command while DQS Oscillator is in progress. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sw_enter_dqs_osc_in_progress_err_status(
        &mut self,
    ) -> LpcSwEnterDqsOscInProgressErrStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec> {
        LpcSwEnterDqsOscInProgressErrStatusW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Set the CS information for which DQS oscillator is having out of variance value. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_per_cs_oov_training_status(
        &mut self,
    ) -> DqsOscPerCsOovTrainingStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec> {
        DqsOscPerCsOovTrainingStatusW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_26::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_26::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_26 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl26Spec {
    const RESET_VALUE: u32 = 0;
}
