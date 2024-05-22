#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_110` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_110` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec>;
#[doc = "Field `PHY_WRLVL_THRESHOLD_ADJUST_0` reader - 1:0\\]
Write level threshold adjust value based on those thresholds for DQS for slice 0."]
pub type PhyWrlvlThresholdAdjust0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_THRESHOLD_ADJUST_0` writer - 1:0\\]
Write level threshold adjust value based on those thresholds for DQS for slice 0."]
pub type PhyWrlvlThresholdAdjust0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_0` reader - 17:8\\]
Rising edge read DQS slave delay setting for DQ0 for slice 0."]
pub type PhyRddqsDq0RiseSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_0` writer - 17:8\\]
Rising edge read DQS slave delay setting for DQ0 for slice 0."]
pub type PhyRddqsDq0RiseSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Write level threshold adjust value based on those thresholds for DQS for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_threshold_adjust_0(&self) -> PhyWrlvlThresholdAdjust0R {
        PhyWrlvlThresholdAdjust0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Rising edge read DQS slave delay setting for DQ0 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_rise_slave_delay_0(&self) -> PhyRddqsDq0RiseSlaveDelay0R {
        PhyRddqsDq0RiseSlaveDelay0R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Write level threshold adjust value based on those thresholds for DQS for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_threshold_adjust_0(
        &mut self,
    ) -> PhyWrlvlThresholdAdjust0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec> {
        PhyWrlvlThresholdAdjust0W::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Rising edge read DQS slave delay setting for DQ0 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_rise_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq0RiseSlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec> {
        PhyRddqsDq0RiseSlaveDelay0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_110::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_110::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_110::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_110::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_110 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy110Spec {
    const RESET_VALUE: u32 = 0;
}
