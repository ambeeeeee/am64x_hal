#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1026` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1026` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec>;
#[doc = "Field `PHY_ADR_LPBK_ERROR_COUNT_OBS_2` reader - 15:0\\]
Observation register containing total number of loopback error data for address slice 2. READ-ONLY"]
pub type PhyAdrLpbkErrorCountObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_LPBK_ERROR_COUNT_OBS_2` writer - 15:0\\]
Observation register containing total number of loopback error data for address slice 2. READ-ONLY"]
pub type PhyAdrLpbkErrorCountObs2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_ADR_MEAS_DLY_STEP_VALUE_2` reader - 23:16\\]
Contains the fraction of a cycle in 1 delay element numerator with demominator of 512, for address slice 2. READ-ONLY"]
pub type PhyAdrMeasDlyStepValue2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MEAS_DLY_STEP_VALUE_2` writer - 23:16\\]
Contains the fraction of a cycle in 1 delay element numerator with demominator of 512, for address slice 2. READ-ONLY"]
pub type PhyAdrMeasDlyStepValue2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_2` reader - 27:24\\]
Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
pub type PhyAdrMasterDlyLockObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_MASTER_DLY_LOCK_OBS_SELECT_2` writer - 27:24\\]
Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
pub type PhyAdrMasterDlyLockObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register containing total number of loopback error data for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_lpbk_error_count_obs_2(&self) -> PhyAdrLpbkErrorCountObs2R {
        PhyAdrLpbkErrorCountObs2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Contains the fraction of a cycle in 1 delay element numerator with demominator of 512, for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_meas_dly_step_value_2(&self) -> PhyAdrMeasDlyStepValue2R {
        PhyAdrMeasDlyStepValue2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_master_dly_lock_obs_select_2(&self) -> PhyAdrMasterDlyLockObsSelect2R {
        PhyAdrMasterDlyLockObsSelect2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register containing total number of loopback error data for address slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_error_count_obs_2(
        &mut self,
    ) -> PhyAdrLpbkErrorCountObs2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec> {
        PhyAdrLpbkErrorCountObs2W::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Contains the fraction of a cycle in 1 delay element numerator with demominator of 512, for address slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_meas_dly_step_value_2(
        &mut self,
    ) -> PhyAdrMeasDlyStepValue2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec> {
        PhyAdrMeasDlyStepValue2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map the internal master delay observation registers to the accessible master delay observation register for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_master_dly_lock_obs_select_2(
        &mut self,
    ) -> PhyAdrMasterDlyLockObsSelect2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec> {
        PhyAdrMasterDlyLockObsSelect2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1026\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1026::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1026::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1026::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1026::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1026 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1026Spec {
    const RESET_VALUE: u32 = 0;
}
