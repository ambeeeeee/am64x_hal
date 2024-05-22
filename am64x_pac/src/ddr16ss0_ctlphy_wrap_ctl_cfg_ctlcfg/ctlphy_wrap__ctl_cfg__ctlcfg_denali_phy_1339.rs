#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1339` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1339` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec>;
#[doc = "Field `PHY_CAL_RESULT5_OBS_0` reader - 23:0\\]
Pad calibration pass2 shadow results observation values for block 0. READ-ONLY"]
pub type PhyCalResult5Obs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_RESULT5_OBS_0` writer - 23:0\\]
Pad calibration pass2 shadow results observation values for block 0. READ-ONLY"]
pub type PhyCalResult5Obs0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Pad calibration pass2 shadow results observation values for block 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_cal_result5_obs_0(&self) -> PhyCalResult5Obs0R {
        PhyCalResult5Obs0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Pad calibration pass2 shadow results observation values for block 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_result5_obs_0(
        &mut self,
    ) -> PhyCalResult5Obs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec> {
        PhyCalResult5Obs0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1339\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1339::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1339::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1339::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1339::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1339 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1339Spec {
    const RESET_VALUE: u32 = 0;
}