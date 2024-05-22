#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_46` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_46` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_0` reader - 15:0\\]
Observation register containing total number of loopback error data for slice 0. READ-ONLY"]
pub type PhyLpbkErrorCountObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_LPBK_ERROR_COUNT_OBS_0` writer - 15:0\\]
Observation register containing total number of loopback error data for slice 0. READ-ONLY"]
pub type PhyLpbkErrorCountObs0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_0` reader - 26:16\\]
Observation register containing master delay results for slice 0. READ-ONLY"]
pub type PhyMasterDlyLockObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DLY_LOCK_OBS_0` writer - 26:16\\]
Observation register containing master delay results for slice 0. READ-ONLY"]
pub type PhyMasterDlyLockObs0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register containing total number of loopback error data for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_lpbk_error_count_obs_0(&self) -> PhyLpbkErrorCountObs0R {
        PhyLpbkErrorCountObs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing master delay results for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_master_dly_lock_obs_0(&self) -> PhyMasterDlyLockObs0R {
        PhyMasterDlyLockObs0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register containing total number of loopback error data for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_error_count_obs_0(
        &mut self,
    ) -> PhyLpbkErrorCountObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec> {
        PhyLpbkErrorCountObs0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Observation register containing master delay results for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_dly_lock_obs_0(
        &mut self,
    ) -> PhyMasterDlyLockObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec> {
        PhyMasterDlyLockObs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_46::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_46::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_46 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy46Spec {
    const RESET_VALUE: u32 = 0;
}
