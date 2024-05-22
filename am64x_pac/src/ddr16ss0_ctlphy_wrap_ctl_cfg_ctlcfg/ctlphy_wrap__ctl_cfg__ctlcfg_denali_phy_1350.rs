#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1350` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1350` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec>;
#[doc = "Field `PHY_CAL_RANGE_PASS2_RX_MAX_DELTA_0` reader - 4:0\\]
Pad calibration pass2 rx results won't update if out of max delta range ."]
pub type PhyCalRangePass2RxMaxDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS2_RX_MAX_DELTA_0` writer - 4:0\\]
Pad calibration pass2 rx results won't update if out of max delta range ."]
pub type PhyCalRangePass2RxMaxDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PU_MIN_DELTA_0` reader - 13:8\\]
Pad calibration pass1 pu results won't update if out of min delta range ."]
pub type PhyCalRangePass1PuMinDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PU_MIN_DELTA_0` writer - 13:8\\]
Pad calibration pass1 pu results won't update if out of min delta range ."]
pub type PhyCalRangePass1PuMinDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PD_MIN_DELTA_0` reader - 21:16\\]
Pad calibration pass1 pd results won't update if out of min delta range ."]
pub type PhyCalRangePass1PdMinDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PD_MIN_DELTA_0` writer - 21:16\\]
Pad calibration pass1 pd results won't update if out of min delta range ."]
pub type PhyCalRangePass1PdMinDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_CAL_RANGE_PASS1_RX_MIN_DELTA_0` reader - 28:24\\]
Pad calibration pass1 rx results won't update if out of min delta range ."]
pub type PhyCalRangePass1RxMinDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS1_RX_MIN_DELTA_0` writer - 28:24\\]
Pad calibration pass1 rx results won't update if out of min delta range ."]
pub type PhyCalRangePass1RxMinDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Pad calibration pass2 rx results won't update if out of max delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass2_rx_max_delta_0(&self) -> PhyCalRangePass2RxMaxDelta0R {
        PhyCalRangePass2RxMaxDelta0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Pad calibration pass1 pu results won't update if out of min delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass1_pu_min_delta_0(&self) -> PhyCalRangePass1PuMinDelta0R {
        PhyCalRangePass1PuMinDelta0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Pad calibration pass1 pd results won't update if out of min delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass1_pd_min_delta_0(&self) -> PhyCalRangePass1PdMinDelta0R {
        PhyCalRangePass1PdMinDelta0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Pad calibration pass1 rx results won't update if out of min delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass1_rx_min_delta_0(&self) -> PhyCalRangePass1RxMinDelta0R {
        PhyCalRangePass1RxMinDelta0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Pad calibration pass2 rx results won't update if out of max delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass2_rx_max_delta_0(
        &mut self,
    ) -> PhyCalRangePass2RxMaxDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec> {
        PhyCalRangePass2RxMaxDelta0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Pad calibration pass1 pu results won't update if out of min delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass1_pu_min_delta_0(
        &mut self,
    ) -> PhyCalRangePass1PuMinDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec> {
        PhyCalRangePass1PuMinDelta0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Pad calibration pass1 pd results won't update if out of min delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass1_pd_min_delta_0(
        &mut self,
    ) -> PhyCalRangePass1PdMinDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec> {
        PhyCalRangePass1PdMinDelta0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Pad calibration pass1 rx results won't update if out of min delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass1_rx_min_delta_0(
        &mut self,
    ) -> PhyCalRangePass1RxMinDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec> {
        PhyCalRangePass1RxMinDelta0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1350\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1350::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1350::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1350::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1350::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1350 to value 0x31"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1350Spec {
    const RESET_VALUE: u32 = 0x31;
}
