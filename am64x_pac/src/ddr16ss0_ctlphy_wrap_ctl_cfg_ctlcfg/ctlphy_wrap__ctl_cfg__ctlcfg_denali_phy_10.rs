#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_10` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_10` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec>;
#[doc = "Field `PHY_AUTO_TIMING_MARGIN_OBS_0` reader - 27:0\\]
Observation register for the auto_timing_margin for slice 0. READ-ONLY"]
pub type PhyAutoTimingMarginObs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_AUTO_TIMING_MARGIN_OBS_0` writer - 27:0\\]
Observation register for the auto_timing_margin for slice 0. READ-ONLY"]
pub type PhyAutoTimingMarginObs0W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Observation register for the auto_timing_margin for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_auto_timing_margin_obs_0(&self) -> PhyAutoTimingMarginObs0R {
        PhyAutoTimingMarginObs0R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Observation register for the auto_timing_margin for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_auto_timing_margin_obs_0(
        &mut self,
    ) -> PhyAutoTimingMarginObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec> {
        PhyAutoTimingMarginObs0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_10::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_10::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_10 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy10Spec {
    const RESET_VALUE: u32 = 0;
}
