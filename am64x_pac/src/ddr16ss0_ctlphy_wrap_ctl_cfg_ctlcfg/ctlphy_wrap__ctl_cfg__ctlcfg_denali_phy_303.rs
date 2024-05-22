#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_303` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_303` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec>;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_1` reader - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 1. READ-ONLY"]
pub type PhyRddqSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_1` writer - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 1. READ-ONLY"]
pub type PhyRddqSlvDlyEncObs1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_1` reader - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyRddqsBaseSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_1` writer - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyRddqsBaseSlvDlyEncObs1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_MEAS_DLY_STEP_VALUE_1` reader - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 1. READ-ONLY"]
pub type PhyMeasDlyStepValue1R = crate::FieldReader;
#[doc = "Field `PHY_MEAS_DLY_STEP_VALUE_1` writer - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 1. READ-ONLY"]
pub type PhyMeasDlyStepValue1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_1` reader - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_1` writer - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddq_slv_dly_enc_obs_1(&self) -> PhyRddqSlvDlyEncObs1R {
        PhyRddqSlvDlyEncObs1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_base_slv_dly_enc_obs_1(&self) -> PhyRddqsBaseSlvDlyEncObs1R {
        PhyRddqsBaseSlvDlyEncObs1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_meas_dly_step_value_1(&self) -> PhyMeasDlyStepValue1R {
        PhyMeasDlyStepValue1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_1(&self) -> PhyRddqsDqRiseAdderSlvDlyEncObs1R {
        PhyRddqsDqRiseAdderSlvDlyEncObs1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq_slv_dly_enc_obs_1(
        &mut self,
    ) -> PhyRddqSlvDlyEncObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec> {
        PhyRddqSlvDlyEncObs1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_base_slv_dly_enc_obs_1(
        &mut self,
    ) -> PhyRddqsBaseSlvDlyEncObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec> {
        PhyRddqsBaseSlvDlyEncObs1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_meas_dly_step_value_1(
        &mut self,
    ) -> PhyMeasDlyStepValue1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec> {
        PhyMeasDlyStepValue1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_1(
        &mut self,
    ) -> PhyRddqsDqRiseAdderSlvDlyEncObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec> {
        PhyRddqsDqRiseAdderSlvDlyEncObs1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_303\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_303::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_303::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_303::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_303::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_303 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy303Spec {
    const RESET_VALUE: u32 = 0;
}
