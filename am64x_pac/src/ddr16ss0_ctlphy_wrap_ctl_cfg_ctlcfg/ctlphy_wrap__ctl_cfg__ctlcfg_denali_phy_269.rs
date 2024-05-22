#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_269` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_269` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec>;
#[doc = "Field `PHY_VREF_INITIAL_STEPSIZE_1` reader - 5:0\\]
Data slice initial VREF training step size for slice 1."]
pub type PhyVrefInitialStepsize1R = crate::FieldReader;
#[doc = "Field `PHY_VREF_INITIAL_STEPSIZE_1` writer - 5:0\\]
Data slice initial VREF training step size for slice 1."]
pub type PhyVrefInitialStepsize1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_VREF_TRAIN_OBS_1` reader - 14:8\\]
Observation register for best vref value for slice 1. READ-ONLY"]
pub type PhyVrefTrainObs1R = crate::FieldReader;
#[doc = "Field `PHY_VREF_TRAIN_OBS_1` writer - 14:8\\]
Observation register for best vref value for slice 1. READ-ONLY"]
pub type PhyVrefTrainObs1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_1` reader - 25:16\\]
Read DQS data clock bypass mode slave delay setting for slice 1."]
pub type PhyRddqsDqBypassSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_1` writer - 25:16\\]
Read DQS data clock bypass mode slave delay setting for slice 1."]
pub type PhyRddqsDqBypassSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Data slice initial VREF training step size for slice 1."]
    #[inline(always)]
    pub fn phy_vref_initial_stepsize_1(&self) -> PhyVrefInitialStepsize1R {
        PhyVrefInitialStepsize1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Observation register for best vref value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_vref_train_obs_1(&self) -> PhyVrefTrainObs1R {
        PhyVrefTrainObs1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Read DQS data clock bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq_bypass_slave_delay_1(&self) -> PhyRddqsDqBypassSlaveDelay1R {
        PhyRddqsDqBypassSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Data slice initial VREF training step size for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_initial_stepsize_1(
        &mut self,
    ) -> PhyVrefInitialStepsize1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec> {
        PhyVrefInitialStepsize1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Observation register for best vref value for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_vref_train_obs_1(
        &mut self,
    ) -> PhyVrefTrainObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec> {
        PhyVrefTrainObs1W::new(self, 8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Read DQS data clock bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_bypass_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDqBypassSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec> {
        PhyRddqsDqBypassSlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_269\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_269::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_269::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_269::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_269::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_269 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy269Spec {
    const RESET_VALUE: u32 = 0;
}
