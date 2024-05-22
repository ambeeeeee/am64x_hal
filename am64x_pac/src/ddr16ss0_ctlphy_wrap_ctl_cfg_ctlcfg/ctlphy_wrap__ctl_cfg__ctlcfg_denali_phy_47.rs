#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_47` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_47` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec>;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_0` reader - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 0. READ-ONLY"]
pub type PhyRddqSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_0` writer - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 0. READ-ONLY"]
pub type PhyRddqSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_0` reader - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsBaseSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_0` writer - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsBaseSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_MEAS_DLY_STEP_VALUE_0` reader - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 0. READ-ONLY"]
pub type PhyMeasDlyStepValue0R = crate::FieldReader;
#[doc = "Field `PHY_MEAS_DLY_STEP_VALUE_0` writer - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 0. READ-ONLY"]
pub type PhyMeasDlyStepValue0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_0` reader - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_0` writer - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddq_slv_dly_enc_obs_0(&self) -> PhyRddqSlvDlyEncObs0R {
        PhyRddqSlvDlyEncObs0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_base_slv_dly_enc_obs_0(&self) -> PhyRddqsBaseSlvDlyEncObs0R {
        PhyRddqsBaseSlvDlyEncObs0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_meas_dly_step_value_0(&self) -> PhyMeasDlyStepValue0R {
        PhyMeasDlyStepValue0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_0(&self) -> PhyRddqsDqRiseAdderSlvDlyEncObs0R {
        PhyRddqsDqRiseAdderSlvDlyEncObs0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Observation register containing read DQ slave delay encoded values for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyRddqSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec> {
        PhyRddqSlvDlyEncObs0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Observation register containing read DQS base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_base_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyRddqsBaseSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec> {
        PhyRddqsBaseSlvDlyEncObs0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Observation register containing fraction of the cycle in 1 delay element, numerator with demominator of 512, for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_meas_dly_step_value_0(
        &mut self,
    ) -> PhyMeasDlyStepValue0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec> {
        PhyMeasDlyStepValue0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Observation register containing read DQS DQ rising edge adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyRddqsDqRiseAdderSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec> {
        PhyRddqsDqRiseAdderSlvDlyEncObs0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_47::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_47::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_47 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy47Spec {
    const RESET_VALUE: u32 = 0;
}
