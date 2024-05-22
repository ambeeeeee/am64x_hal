#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1349` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1349` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec>;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PD_MAX_DELTA_0` reader - 5:0\\]
Pad calibration pass1 pd results won't update if out of max delta range ."]
pub type PhyCalRangePass1PdMaxDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PD_MAX_DELTA_0` writer - 5:0\\]
Pad calibration pass1 pd results won't update if out of max delta range ."]
pub type PhyCalRangePass1PdMaxDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_CAL_RANGE_PASS1_RX_MAX_DELTA_0` reader - 12:8\\]
Pad calibration pass1 rx results won't update if out of max delta range ."]
pub type PhyCalRangePass1RxMaxDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS1_RX_MAX_DELTA_0` writer - 12:8\\]
Pad calibration pass1 rx results won't update if out of max delta range ."]
pub type PhyCalRangePass1RxMaxDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PU_MAX_DELTA_0` reader - 21:16\\]
Pad calibration pass2 pu results won't update if out of max delta range ."]
pub type PhyCalRangePass2PuMaxDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PU_MAX_DELTA_0` writer - 21:16\\]
Pad calibration pass2 pu results won't update if out of max delta range ."]
pub type PhyCalRangePass2PuMaxDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PD_MAX_DELTA_0` reader - 29:24\\]
Pad calibration pass2 pd results won't update if out of max delta range ."]
pub type PhyCalRangePass2PdMaxDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS2_PD_MAX_DELTA_0` writer - 29:24\\]
Pad calibration pass2 pd results won't update if out of max delta range ."]
pub type PhyCalRangePass2PdMaxDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Pad calibration pass1 pd results won't update if out of max delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass1_pd_max_delta_0(&self) -> PhyCalRangePass1PdMaxDelta0R {
        PhyCalRangePass1PdMaxDelta0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Pad calibration pass1 rx results won't update if out of max delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass1_rx_max_delta_0(&self) -> PhyCalRangePass1RxMaxDelta0R {
        PhyCalRangePass1RxMaxDelta0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Pad calibration pass2 pu results won't update if out of max delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass2_pu_max_delta_0(&self) -> PhyCalRangePass2PuMaxDelta0R {
        PhyCalRangePass2PuMaxDelta0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Pad calibration pass2 pd results won't update if out of max delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass2_pd_max_delta_0(&self) -> PhyCalRangePass2PdMaxDelta0R {
        PhyCalRangePass2PdMaxDelta0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Pad calibration pass1 pd results won't update if out of max delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass1_pd_max_delta_0(
        &mut self,
    ) -> PhyCalRangePass1PdMaxDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec> {
        PhyCalRangePass1PdMaxDelta0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Pad calibration pass1 rx results won't update if out of max delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass1_rx_max_delta_0(
        &mut self,
    ) -> PhyCalRangePass1RxMaxDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec> {
        PhyCalRangePass1RxMaxDelta0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Pad calibration pass2 pu results won't update if out of max delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass2_pu_max_delta_0(
        &mut self,
    ) -> PhyCalRangePass2PuMaxDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec> {
        PhyCalRangePass2PuMaxDelta0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Pad calibration pass2 pd results won't update if out of max delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass2_pd_max_delta_0(
        &mut self,
    ) -> PhyCalRangePass2PdMaxDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec> {
        PhyCalRangePass2PdMaxDelta0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1349\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1349::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1349::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1349::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1349::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1349 to value 0x6363_3163"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1349Spec {
    const RESET_VALUE: u32 = 0x6363_3163;
}
