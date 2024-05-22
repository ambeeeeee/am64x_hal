#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_53` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_53` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec>;
#[doc = "Field `PHY_GTLVL_HARD0_DELAY_OBS_0` reader - 13:0\\]
Observation register containing gate training first hard 0 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyGtlvlHard0DelayObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_HARD0_DELAY_OBS_0` writer - 13:0\\]
Observation register containing gate training first hard 0 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyGtlvlHard0DelayObs0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PHY_GTLVL_HARD1_DELAY_OBS_0` reader - 29:16\\]
Observation register containing gate training last hard 1 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyGtlvlHard1DelayObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_HARD1_DELAY_OBS_0` writer - 29:16\\]
Observation register containing gate training last hard 1 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyGtlvlHard1DelayObs0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Observation register containing gate training first hard 0 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_hard0_delay_obs_0(&self) -> PhyGtlvlHard0DelayObs0R {
        PhyGtlvlHard0DelayObs0R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Observation register containing gate training last hard 1 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_hard1_delay_obs_0(&self) -> PhyGtlvlHard1DelayObs0R {
        PhyGtlvlHard1DelayObs0R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Observation register containing gate training first hard 0 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_hard0_delay_obs_0(
        &mut self,
    ) -> PhyGtlvlHard0DelayObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec> {
        PhyGtlvlHard0DelayObs0W::new(self, 0)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Observation register containing gate training last hard 1 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_hard1_delay_obs_0(
        &mut self,
    ) -> PhyGtlvlHard1DelayObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec> {
        PhyGtlvlHard1DelayObs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_53::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_53::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_53 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy53Spec {
    const RESET_VALUE: u32 = 0;
}