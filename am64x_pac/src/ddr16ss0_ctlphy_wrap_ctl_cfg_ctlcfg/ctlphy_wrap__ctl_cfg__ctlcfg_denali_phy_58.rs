#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_58` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_58` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_0` reader - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyWdqlvlDqdmLeDlyObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_LE_DLY_OBS_0` writer - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyWdqlvlDqdmLeDlyObs0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_0` reader - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyWdqlvlDqdmTeDlyObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_TE_DLY_OBS_0` writer - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyWdqlvlDqdmTeDlyObs0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_0(&self) -> PhyWdqlvlDqdmLeDlyObs0R {
        PhyWdqlvlDqdmLeDlyObs0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_0(&self) -> PhyWdqlvlDqdmTeDlyObs0R {
        PhyWdqlvlDqdmTeDlyObs0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Observation register containing write data leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_le_dly_obs_0(
        &mut self,
    ) -> PhyWdqlvlDqdmLeDlyObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec> {
        PhyWdqlvlDqdmLeDlyObs0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing write data leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_te_dly_obs_0(
        &mut self,
    ) -> PhyWdqlvlDqdmTeDlyObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec> {
        PhyWdqlvlDqdmTeDlyObs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_58::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_58::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_58 to value 0x2047_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy58Spec {
    const RESET_VALUE: u32 = 0x2047_0000;
}
