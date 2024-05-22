#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_314` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_314` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_1` reader - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 1. READ-ONLY"]
pub type PhyWdqlvlDqdmLeDlyObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_1` writer - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 1. READ-ONLY"]
pub type PhyWdqlvlDqdmLeDlyObs1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_1` reader - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 1. READ-ONLY"]
pub type PhyWdqlvlDqdmTeDlyObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_1` writer - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 1. READ-ONLY"]
pub type PhyWdqlvlDqdmTeDlyObs1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_1(&self) -> PhyWdqlvlDqdmLeDlyObs1R {
        PhyWdqlvlDqdmLeDlyObs1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_1(&self) -> PhyWdqlvlDqdmTeDlyObs1R {
        PhyWdqlvlDqdmTeDlyObs1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_1(
        &mut self,
    ) -> PhyWdqlvlDqdmLeDlyObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec> {
        PhyWdqlvlDqdmLeDlyObs1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_1(
        &mut self,
    ) -> PhyWdqlvlDqdmTeDlyObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec> {
        PhyWdqlvlDqdmTeDlyObs1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_314\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_314::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_314::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_314::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_314::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_314 to value 0x2047_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy314Spec {
    const RESET_VALUE: u32 = 0x2047_0000;
}
