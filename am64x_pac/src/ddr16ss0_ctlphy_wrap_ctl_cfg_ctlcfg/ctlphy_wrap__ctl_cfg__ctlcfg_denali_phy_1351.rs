#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1351` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1351` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec>;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PU_MIN_DELTA_0` reader - 5:0\\]
Pad calibration pass2 pu results won't update if out of min delta range ."]
pub type PhyCalRangePass2PuMinDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PU_MIN_DELTA_0` writer - 5:0\\]
Pad calibration pass2 pu results won't update if out of min delta range ."]
pub type PhyCalRangePass2PuMinDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PD_MIN_DELTA_0` reader - 13:8\\]
Pad calibration pass2 pd results won't update if out of min delta range ."]
pub type PhyCalRangePass2PdMinDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PD_MIN_DELTA_0` writer - 13:8\\]
Pad calibration pass2 pd results won't update if out of min delta range ."]
pub type PhyCalRangePass2PdMinDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_CAL_RANGE_PASS2_RX_MIN_DELTA_0` reader - 20:16\\]
Pad calibration pass2 rx results won't update if out of min delta range ."]
pub type PhyCalRangePass2RxMinDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS2_RX_MIN_DELTA_0` writer - 20:16\\]
Pad calibration pass2 rx results won't update if out of min delta range ."]
pub type PhyCalRangePass2RxMinDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Pad calibration pass2 pu results won't update if out of min delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass2_pu_min_delta_0(&self) -> PhyCalRangePass2PuMinDelta0R {
        PhyCalRangePass2PuMinDelta0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Pad calibration pass2 pd results won't update if out of min delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass2_pd_min_delta_0(&self) -> PhyCalRangePass2PdMinDelta0R {
        PhyCalRangePass2PdMinDelta0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Pad calibration pass2 rx results won't update if out of min delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass2_rx_min_delta_0(&self) -> PhyCalRangePass2RxMinDelta0R {
        PhyCalRangePass2RxMinDelta0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Pad calibration pass2 pu results won't update if out of min delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass2_pu_min_delta_0(
        &mut self,
    ) -> PhyCalRangePass2PuMinDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec> {
        PhyCalRangePass2PuMinDelta0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Pad calibration pass2 pd results won't update if out of min delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass2_pd_min_delta_0(
        &mut self,
    ) -> PhyCalRangePass2PdMinDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec> {
        PhyCalRangePass2PdMinDelta0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Pad calibration pass2 rx results won't update if out of min delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass2_rx_min_delta_0(
        &mut self,
    ) -> PhyCalRangePass2RxMinDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec> {
        PhyCalRangePass2RxMinDelta0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1351\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1351::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1351::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1351::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1351::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1351 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1351Spec {
    const RESET_VALUE: u32 = 0;
}
