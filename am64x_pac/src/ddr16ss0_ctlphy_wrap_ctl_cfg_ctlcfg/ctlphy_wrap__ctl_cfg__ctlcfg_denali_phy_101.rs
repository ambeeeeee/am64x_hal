#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_101` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_101` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec>;
#[doc = "Field `PHY_WRPATH_GATE_DISABLE_0` reader - 1:0\\]
Write path clock gating disable for slice 0. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyWrpathGateDisable0R = crate::FieldReader;
#[doc = "Field `PHY_WRPATH_GATE_DISABLE_0` writer - 1:0\\]
Write path clock gating disable for slice 0. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyWrpathGateDisable0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_WRPATH_GATE_TIMING_0` reader - 10:8\\]
Write path clock gating timing for slice 0. it means additional clock number to write path clock gate"]
pub type PhyWrpathGateTiming0R = crate::FieldReader;
#[doc = "Field `PHY_WRPATH_GATE_TIMING_0` writer - 10:8\\]
Write path clock gating timing for slice 0. it means additional clock number to write path clock gate"]
pub type PhyWrpathGateTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WDQ_OSC_DELTA_0` reader - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 0."]
pub type PhyWdqOscDelta0R = crate::FieldReader;
#[doc = "Field `PHY_WDQ_OSC_DELTA_0` writer - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 0."]
pub type PhyWdqOscDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_MEAS_DLY_STEP_ENABLE_0` reader - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 0."]
pub type PhyMeasDlyStepEnable0R = crate::FieldReader;
#[doc = "Field `PHY_MEAS_DLY_STEP_ENABLE_0` writer - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 0."]
pub type PhyMeasDlyStepEnable0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Write path clock gating disable for slice 0. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    pub fn phy_wrpath_gate_disable_0(&self) -> PhyWrpathGateDisable0R {
        PhyWrpathGateDisable0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write path clock gating timing for slice 0. it means additional clock number to write path clock gate"]
    #[inline(always)]
    pub fn phy_wrpath_gate_timing_0(&self) -> PhyWrpathGateTiming0R {
        PhyWrpathGateTiming0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 0."]
    #[inline(always)]
    pub fn phy_wdq_osc_delta_0(&self) -> PhyWdqOscDelta0R {
        PhyWdqOscDelta0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 0."]
    #[inline(always)]
    pub fn phy_meas_dly_step_enable_0(&self) -> PhyMeasDlyStepEnable0R {
        PhyMeasDlyStepEnable0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Write path clock gating disable for slice 0. \\[0\\]: disable pull in wrdata_en; \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrpath_gate_disable_0(
        &mut self,
    ) -> PhyWrpathGateDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec> {
        PhyWrpathGateDisable0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write path clock gating timing for slice 0. it means additional clock number to write path clock gate"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrpath_gate_timing_0(
        &mut self,
    ) -> PhyWrpathGateTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec> {
        PhyWrpathGateTiming0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Slave delay offset that applies to a 1 bit change of dfi_wdq_osc_code for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdq_osc_delta_0(
        &mut self,
    ) -> PhyWdqOscDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec> {
        PhyWdqOscDelta0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Data slice training step definition using phy_meas_dly_step_value for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_meas_dly_step_enable_0(
        &mut self,
    ) -> PhyMeasDlyStepEnable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec> {
        PhyMeasDlyStepEnable0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_101::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_101::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_101::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_101 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy101Spec {
    const RESET_VALUE: u32 = 0;
}
