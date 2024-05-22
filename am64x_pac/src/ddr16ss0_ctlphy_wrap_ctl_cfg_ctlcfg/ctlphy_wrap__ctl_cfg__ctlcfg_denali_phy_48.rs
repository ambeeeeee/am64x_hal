#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_48` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_48` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec>;
#[doc = "Field `PHY_RDDQS_DQ_FALL_ADDER_SLV_DLY_ENC_OBS_0` reader - 7:0\\]
Observation register containing read DQS DQ falling edge adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsDqFallAdderSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_FALL_ADDER_SLV_DLY_ENC_OBS_0` writer - 7:0\\]
Observation register containing read DQS DQ falling edge adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsDqFallAdderSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDQS_GATE_SLV_DLY_ENC_OBS_0` reader - 18:8\\]
Observation register containing read DQS gate slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsGateSlvDlyEncObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLV_DLY_ENC_OBS_0` writer - 18:8\\]
Observation register containing read DQS gate slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsGateSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WRDQS_BASE_SLV_DLY_ENC_OBS_0` reader - 30:24\\]
Observation register containing write DQS base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrdqsBaseSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_WRDQS_BASE_SLV_DLY_ENC_OBS_0` writer - 30:24\\]
Observation register containing write DQS base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrdqsBaseSlvDlyEncObs0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Observation register containing read DQS DQ falling edge adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_dq_fall_adder_slv_dly_enc_obs_0(&self) -> PhyRddqsDqFallAdderSlvDlyEncObs0R {
        PhyRddqsDqFallAdderSlvDlyEncObs0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Observation register containing read DQS gate slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_gate_slv_dly_enc_obs_0(&self) -> PhyRddqsGateSlvDlyEncObs0R {
        PhyRddqsGateSlvDlyEncObs0R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Observation register containing write DQS base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrdqs_base_slv_dly_enc_obs_0(&self) -> PhyWrdqsBaseSlvDlyEncObs0R {
        PhyWrdqsBaseSlvDlyEncObs0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Observation register containing read DQS DQ falling edge adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_fall_adder_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyRddqsDqFallAdderSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec> {
        PhyRddqsDqFallAdderSlvDlyEncObs0W::new(self, 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Observation register containing read DQS gate slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyRddqsGateSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec> {
        PhyRddqsGateSlvDlyEncObs0W::new(self, 8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Observation register containing write DQS base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrdqs_base_slv_dly_enc_obs_0(
        &mut self,
    ) -> PhyWrdqsBaseSlvDlyEncObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec> {
        PhyWrdqsBaseSlvDlyEncObs0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_48::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_48::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_48 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy48Spec {
    const RESET_VALUE: u32 = 0;
}
