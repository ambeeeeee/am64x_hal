#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1344` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1344` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec>;
#[doc = "Field `PHY_CAL_RESULT3_OBS_0` reader - 31:0\\]
Pad calibration results first/last0/1 observation values for block 0. READ-ONLY"]
pub type PhyCalResult3Obs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_RESULT3_OBS_0` writer - 31:0\\]
Pad calibration results first/last0/1 observation values for block 0. READ-ONLY"]
pub type PhyCalResult3Obs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Pad calibration results first/last0/1 observation values for block 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_cal_result3_obs_0(&self) -> PhyCalResult3Obs0R {
        PhyCalResult3Obs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Pad calibration results first/last0/1 observation values for block 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_result3_obs_0(
        &mut self,
    ) -> PhyCalResult3Obs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec> {
        PhyCalResult3Obs0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1344\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1344::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1344::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1344::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1344::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1344 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1344Spec {
    const RESET_VALUE: u32 = 0;
}
