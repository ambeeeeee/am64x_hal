#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_302` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_302` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_1` reader - 15:0\\]
Observation register containing total number of loopback error data for slice 1. READ-ONLY"]
pub type PhyLpbkErrorCountObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_1` writer - 15:0\\]
Observation register containing total number of loopback error data for slice 1. READ-ONLY"]
pub type PhyLpbkErrorCountObs1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_1` reader - 26:16\\]
Observation register containing master delay results for slice 1. READ-ONLY"]
pub type PhyMasterDlyLockObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_1` writer - 26:16\\]
Observation register containing master delay results for slice 1. READ-ONLY"]
pub type PhyMasterDlyLockObs1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register containing total number of loopback error data for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_lpbk_error_count_obs_1(&self) -> PhyLpbkErrorCountObs1R {
        PhyLpbkErrorCountObs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing master delay results for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_1(&self) -> PhyMasterDlyLockObs1R {
        PhyMasterDlyLockObs1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register containing total number of loopback error data for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_error_count_obs_1(
        &mut self,
    ) -> PhyLpbkErrorCountObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec> {
        PhyLpbkErrorCountObs1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing master delay results for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_dly_lock_obs_1(
        &mut self,
    ) -> PhyMasterDlyLockObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec> {
        PhyMasterDlyLockObs1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_302\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_302::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_302::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_302::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_302::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_302 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy302Spec {
    const RESET_VALUE: u32 = 0;
}
