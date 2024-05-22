#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_357` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_357` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec>;
#[doc = "Field `PHY_WRPATH_GATE_DISABLE_1` reader - 1:0\\]
Write path clock gating disable for slice 1. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyWrpathGateDisable1R = crate::FieldReader;
#[doc = "Field `PHY_WRPATH_GATE_DISABLE_1` writer - 1:0\\]
Write path clock gating disable for slice 1. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyWrpathGateDisable1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_WRPATH_GATE_TIMING_1` reader - 10:8\\]
Write path clock gating timing for slice 1. it means additional clock number to write path clock gate"]
pub type PhyWrpathGateTiming1R = crate::FieldReader;
#[doc = "Field `PHY_WRPATH_GATE_TIMING_1` writer - 10:8\\]
Write path clock gating timing for slice 1. it means additional clock number to write path clock gate"]
pub type PhyWrpathGateTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WDQ_OSC_DELTA_1` reader - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 1."]
pub type PhyWdqOscDelta1R = crate::FieldReader;
#[doc = "Field `PHY_WDQ_OSC_DELTA_1` writer - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 1."]
pub type PhyWdqOscDelta1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_MEAS_DLY_STEP_ENABLE_1` reader - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 1."]
pub type PhyMeasDlyStepEnable1R = crate::FieldReader;
#[doc = "Field `PHY_MEAS_DLY_STEP_ENABLE_1` writer - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 1."]
pub type PhyMeasDlyStepEnable1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Write path clock gating disable for slice 1. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    pub fn phy_wrpath_gate_disable_1(&self) -> PhyWrpathGateDisable1R {
        PhyWrpathGateDisable1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write path clock gating timing for slice 1. it means additional clock number to write path clock gate"]
    #[inline(always)]
    pub fn phy_wrpath_gate_timing_1(&self) -> PhyWrpathGateTiming1R {
        PhyWrpathGateTiming1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 1."]
    #[inline(always)]
    pub fn phy_wdq_osc_delta_1(&self) -> PhyWdqOscDelta1R {
        PhyWdqOscDelta1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 1."]
    #[inline(always)]
    pub fn phy_meas_dly_step_enable_1(&self) -> PhyMeasDlyStepEnable1R {
        PhyMeasDlyStepEnable1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Write path clock gating disable for slice 1. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrpath_gate_disable_1(
        &mut self,
    ) -> PhyWrpathGateDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec> {
        PhyWrpathGateDisable1W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write path clock gating timing for slice 1. it means additional clock number to write path clock gate"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrpath_gate_timing_1(
        &mut self,
    ) -> PhyWrpathGateTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec> {
        PhyWrpathGateTiming1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdq_osc_delta_1(
        &mut self,
    ) -> PhyWdqOscDelta1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec> {
        PhyWdqOscDelta1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_meas_dly_step_enable_1(
        &mut self,
    ) -> PhyMeasDlyStepEnable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec> {
        PhyMeasDlyStepEnable1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_357\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_357::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_357::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_357::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_357::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_357 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy357Spec {
    const RESET_VALUE: u32 = 0;
}
