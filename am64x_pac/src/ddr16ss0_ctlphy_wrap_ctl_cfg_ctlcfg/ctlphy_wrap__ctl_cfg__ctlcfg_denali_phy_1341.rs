#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1341` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1341` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec>;
#[doc = "Field `PHY_CAL_RESULT7_OBS_0` reader - 23:0\\]
Pad calibration internal results observation delta values for block 0. READ-ONLY"]
pub type PhyCalResult7Obs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_RESULT7_OBS_0` writer - 23:0\\]
Pad calibration internal results observation delta values for block 0. READ-ONLY"]
pub type PhyCalResult7Obs0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_CAL_CPTR_CNT_0` reader - 30:24\\]
defines sample capture number in pad calibration process"]
pub type PhyCalCptrCnt0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_CPTR_CNT_0` writer - 30:24\\]
defines sample capture number in pad calibration process"]
pub type PhyCalCptrCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Pad calibration internal results observation delta values for block 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_cal_result7_obs_0(&self) -> PhyCalResult7Obs0R {
        PhyCalResult7Obs0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:30 - 30:24\\]
defines sample capture number in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_cptr_cnt_0(&self) -> PhyCalCptrCnt0R {
        PhyCalCptrCnt0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Pad calibration internal results observation delta values for block 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_result7_obs_0(
        &mut self,
    ) -> PhyCalResult7Obs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec> {
        PhyCalResult7Obs0W::new(self, 0)
    }
    #[doc = "Bits 24:30 - 30:24\\]
defines sample capture number in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_cptr_cnt_0(
        &mut self,
    ) -> PhyCalCptrCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec> {
        PhyCalCptrCnt0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1341\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1341::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1341::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1341::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1341::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1341 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1341Spec {
    const RESET_VALUE: u32 = 0;
}
